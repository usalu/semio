/// 🖼️ Reserves first, then measures, decodes and queues ONE encoded image backing under `canvas-image:<surface>:<layer>`. The
/// answer is typed ([`RasterUploadRefusal`]): `Busy` is back-pressure (no free slot, the process ledger at capacity, an owner still
/// retiring) and the caller offers the source again later; `Invalid` is the SOURCE's own refusal (undecodable, oversized) — a
/// per-image outcome the caller keeps instead of re-offering. Every refused owner is parked in the surface's refusal ring and retired
/// by the upload cursor without faulting the frame, so one bad image never blocks the next image on the same surface.
pub(crate) fn queue_canvas_image_upload_with(
    surface_id: &str,
    layer_id: &str,
    source_identity: &[u8],
    dimensions: impl FnOnce() -> Result<(u32, u32, Vec<u8>), Vec<u8>>,
    decode: impl FnOnce(&[u8]) -> Option<Vec<u8>>,
) -> Result<String, RasterUploadRefusal> {
    if scene_host_retiring(surface_id) {
        return Err(RasterUploadRefusal::Busy);
    }
    if surface_id.len().saturating_add(layer_id.len()).saturating_add(32) > RASTER_UPLOAD_KEY_BYTE_CAPACITY {
        return Err(RasterUploadRefusal::Invalid("raster key exceeded its fixed credits"));
    }
    if source_identity.len() > RASTER_UPLOAD_BYTE_CAPACITY.saturating_mul(2) {
        return Err(RasterUploadRefusal::Invalid("raster source exceeded Canvas upload credits"));
    }
    let key = format!("canvas-image:{surface_id}:{layer_id}");
    let ready = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(surface_id.to_string(), PendingRasterSurface::default).is_some_and(|surface| surface.admits_upload()));
    if !ready {
        return Err(RasterUploadRefusal::Busy);
    }
    PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id).ok_or(RasterUploadRefusal::Busy)?;
        match PreparedRasterReservation::try_reserve_source(key, source_identity.len()) {
            Ok(reservation) => {
                surface.admission = Some(reservation);
                Ok(())
            }
            Err(rejected) => Err(surface.park_refusal(rejected)),
        }
    })?;
    let (width, height, retained_source) = match dimensions() {
        Ok(dimensions) => dimensions,
        Err(retained_source) => return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("raster source dimensions failed", Vec::new(), retained_source).into_content_refusal())),
    };
    let retained_source = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let surface = surfaces.get_mut(surface_id).ok_or(RasterUploadRefusal::Busy)?;
        let reservation = surface.admission.take().ok_or(RasterUploadRefusal::Busy)?;
        match reservation.claim_with_retained(width, height, retained_source) {
            Ok((reservation, retained_source)) => {
                surface.admission = Some(reservation);
                Ok(retained_source)
            }
            Err(rejected) => Err(surface.park_refusal(rejected.into_content_refusal())),
        }
    })?;
    let Some(pixels) = decode(&retained_source) else {
        return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("raster source decode failed", Vec::new(), retained_source).into_content_refusal()));
    };
    let expected = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if expected > RASTER_UPLOAD_BYTE_CAPACITY || pixels.len() != expected {
        return Err(refuse_raster_admission(surface_id, |reservation| reservation.reject_with_retained("decoded raster exceeded Canvas upload credits", pixels, retained_source).into_content_refusal()));
    }
    let admitted = PENDING_RASTER_STATE.with(|cell| cell.borrow_mut().get_mut(surface_id).and_then(|surface| surface.admission.take()).map(|reservation| reservation.finalize(pixels, retained_source, width, height)));
    let (producer, published_key) = match admitted {
        Some(Ok(admitted)) => admitted,
        Some(Err(rejected)) => {
            PENDING_RASTER_STATE.with(|cell| {
                if let Some(surface) = cell.borrow_mut().get_mut(surface_id) {
                    surface.rejected = Some(rejected);
                }
            });
            return Err(RasterUploadRefusal::Busy);
        }
        None => return Err(RasterUploadRefusal::Busy),
    };
    let accepted = PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return false };
        match surface.queue.push_back(producer) {
            Ok(()) => true,
            Err(mut producer) => {
                producer.begin_close();
                surface.closing = Some(producer);
                false
            }
        }
    });
    if !accepted {
        return Err(RasterUploadRefusal::Busy);
    }
    Ok(published_key)
}

/// 🖼️ Parks `surface_id`'s in-flight admission as the refusal `reject` turns it into, answering the typed outcome.
fn refuse_raster_admission(surface_id: &str, reject: impl FnOnce(PreparedRasterReservation) -> PreparedRasterRejected) -> RasterUploadRefusal {
    PENDING_RASTER_STATE.with(|cell| {
        let mut surfaces = cell.borrow_mut();
        let Some(surface) = surfaces.get_mut(surface_id) else { return RasterUploadRefusal::Busy };
        let Some(reservation) = surface.admission.take() else { return RasterUploadRefusal::Busy };
        surface.park_refusal(reject(reservation))
    })
}

