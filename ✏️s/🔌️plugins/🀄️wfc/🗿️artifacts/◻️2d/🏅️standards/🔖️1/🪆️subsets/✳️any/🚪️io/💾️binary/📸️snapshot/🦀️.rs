//! 📦️ WFC 2D artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::schema::snapshot::{Wfc2dBitmapMedia, Wfc2dTileMedia, Wfc2dSnapshot};
use store::{ErasedSnapshotRetirement, PackError};

/// 📦️ Encodes a `Wfc2dSnapshot` to its binary pack form.
pub fn encode(document: &Wfc2dSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Wfc2dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Wfc2dSnapshot, PackError> {
    <Wfc2dSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region ♻️Retirement
/// 📖️ Decodes into a LIVE projection and hands back the displaced document as a bounded retirement
/// admitted under the caller's own five-currency grant — the only decode entry point a mounted store
/// may call, because it never drops the old snapshot inline. A refused admission restores the exact
/// displaced document into `target` and drops only the freshly decoded, never-published one.
pub fn decode_into(target: &mut Wfc2dSnapshot, bytes: &[u8], grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), PackError> {
    let next = decode(bytes)?;
    let displaced = std::mem::replace(target, next);
    match semio_framework_value::retirement::admit_owned_retirement(displaced, grant) {
        Ok(admitted) => Ok(admitted),
        Err((error, displaced)) => {
            drop(std::mem::replace(target, displaced));
            Err(PackError::from(error))
        }
    }
}
//#endregion ♻️Retirement

//#region 🖼️TileMediaRaster
/// 🖼️ Projects a `Bitmap` tile's palette-indexed pixels into a `data:image/png;base64,…` URL — the
/// exact carrier `Canvas2dHost`'s `kind: "image"` layer reads (`layer.dataUrl`, resolved through its
/// own `imageCache`). Both the editor preview and the viewer board paint through this one function,
/// so a bitmap tile really shows its pixels instead of a labelled rectangle.
///
/// It lives in the BINARY io facet rather than in either window because both surfaces need it and a
/// viewer file may never import through the sibling editor module (`policyViewerPurityBreaches`).
/// Encoding goes through `s.stdio.png`'s own encoder — a hand-rolled PNG writer beside it would be a
/// second, drifting implementation of a format this repo already owns.
///
/// Returns `None` for any media that is not a `Bitmap`, for a zero-sized one, and for a payload whose
/// decoded length is not exactly `width * height` — a malformed tile draws its outline rather than
/// failing the whole surface refresh.
pub fn tile_media_png_data_url(media: &Wfc2dTileMedia) -> Option<String> {
    let Wfc2dTileMedia::Bitmap(Wfc2dBitmapMedia { width, height, palette, pixels }) = media else { return None };
    let (width, height) = (*width, *height);
    if width == 0 || height == 0 || palette.is_empty() {
        return None;
    }
    let count = (width as usize).checked_mul(height as usize)?;
    let indices = base64_codec::base64_standard_decode(pixels).ok()?;
    if indices.len() != count {
        return None;
    }
    let mut rgba = Vec::with_capacity(count.checked_mul(4)?);
    for index in indices {
        let colour = palette.get(usize::from(index)).copied().unwrap_or_default();
        rgba.extend_from_slice(&[colour.r.min(255) as u8, colour.g.min(255) as u8, colour.b.min(255) as u8, colour.a.min(255) as u8]);
    }
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::{io::{author_png_projection, PngProjection, PngChunkMarker}, schema::snapshot::PngColorType};
    let raster = PngProjection { width, height, bit_depth: 8, color_type: PngColorType::Rgba, interlace: false, plte: None, trns: None, gama: None, chrm: None, srgb: None, phys: None, time: None, bkgd: None, text_chunks: Vec::new(), pixels: rgba, chunk_order: vec![PngChunkMarker::Ihdr, PngChunkMarker::Idat, PngChunkMarker::Iend], unknown_chunks: Vec::new() };
    let bytes = author_png_projection(&raster).ok()?;
    Some(format!("data:image/png;base64,{}", base64_codec::base64_standard_encode(bytes)))
}
//#endregion 🖼️TileMediaRaster

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `encode`/`decode` speak, named as the schema names the export.
pub type Wfc2dSnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

mod native_codec {
use super::*;
use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dSnapshot, Wfc2dTile, Wfc2dTileMedia, WFC_2D_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::any::io::text::snapshot::{Wfc2dSnapshotDsl,wfc2d_document_to_dsl,wfc2d_document_from_dsl};

impl store::ArtifactPack for Wfc2dSnapshotDsl {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

impl store::ArtifactPack for Wfc2dSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        <Wfc2dSnapshotDsl as store::ArtifactPack>::encode_pack_with(&wfc2d_document_to_dsl(self), options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let parsed = <Wfc2dSnapshotDsl as store::ArtifactPack>::decode_pack_with(bytes, options)?;
        wfc2d_document_from_dsl(parsed).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Wfc2dSnapshotDsl as store::ArtifactPack>::record_spec()
    }
}
}
