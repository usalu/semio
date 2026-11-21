#[derive(semio_framework_value::RetireOwned)]
struct Buffers<P: semio_framework_value::RetireOwned> {
    projection: P,
    cursor: semio_framework_pack_json::JsonBorrowedWriteCursor,
    prepared: bool,
    failure: Option<&'static str>,
    json: Option<Vec<u8>>,
    encoded: Option<(MediaType, String, Vec<u8>)>,
    page: Option<Vec<u8>>,
    result: Option<ResultPage>,
}

impl<P: Default + semio_framework_value::RetireOwned> Buffers<P> {
    fn new(schema_len: usize, output_limit: usize) -> Self {
        let mut cursor = semio_framework_pack_json::JsonBorrowedWriteCursor::new();
        cursor.set_output_ceiling(output_limit.saturating_sub(schema_len));
        Self {
            projection: P::default(), cursor, prepared: false, failure: None,
            json: None, encoded: None, page: None, result: None,
        }
    }
}

fn advance_projection<S: StructuredMediaSource>(
    request: &Request<S>,
    buffers: &mut Buffers<S::Projection>,
    cx: &mut StepContext<'_>,
) -> Result<(), OriginalTurnFailure> {
    cx.set_stage("structured-media-project");
    let (media_type, schema) = S::descriptor(&request.port)
        .ok_or_else(|| ValueError::invariant("structured media export received a foreign port"))?;
    if let Some(message) = buffers.failure {
        let bytes = request.port.len() + message.len();
        let demand = RetirementDemand { capacity_bytes: bytes, ..header(size_of::<ResultPage>() + bytes) };
        if !cx.retained_work_can_enter(demand) {
            return Ok(());
        }
        buffers.result = Some(Err(MediaError::Payload(request.port.clone(), message.into())));
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: bytes, ..Default::default() })?;
        cx.consume_fuel(1);
        return Ok(());
    }
    let snapshot = request.snapshot_read.get();
    if !buffers.prepared {
        let bytes = S::PREPARATION_COPY_BYTES + size_of::<Option<&'static str>>() + size_of::<bool>();
        advance_structured_preparation(bytes, || {
            match S::prepare(&request.port, snapshot, &mut buffers.projection) {
                Ok(complete) => buffers.prepared = complete,
                Err(message) => buffers.failure = Some(message),
            }
            Ok(buffers.prepared)
        }, cx)?;
        return Ok(());
    }
    if buffers.json.is_none() {
        let view = S::borrow(&request.port, snapshot, &buffers.projection)?;
        advance_structured_json(&mut buffers.cursor, &view, &mut buffers.json, cx)?;
        return Ok(());
    }
    let demand = RetirementDemand {
        capacity_bytes: schema.len(),
        ..header(size_of::<(MediaType, String, Vec<u8>)>() + schema.len())
    };
    if !cx.retained_work_can_enter(demand) {
        return Ok(());
    }
    buffers.encoded = Some((media_type, schema.to_owned(), buffers.json.take().unwrap()));
    cx.consume_retained(RetainedCloneProgress {
        copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: schema.len(), ..Default::default()
    })?;
    cx.consume_fuel(1);
    Ok(())
}
