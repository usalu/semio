/// 📖️ Decodes one terminal Record directly from a checked source range under the caller's control.
pub fn decode_record_body_span_exact_controlled(bytes:crate::ByteSpan<'_>,spec:&RecordSpec,options:&DecodeOptions,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<RecordValue,PackRefusal>{
    if bytes.len()as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"record body exceeds max_file_len"})}
    control.begin_stage(0).map_err(PackRefusal::from)?;
    let mut reader=ByteReader::from_span(bytes);
    let materialization=ControlledMaterialization{control:std::cell::RefCell::new(control),maximum:options.limits.max_total_alloc};
    let symbols=decode_inline_symbols(&mut reader,&options.limits,&materialization)?;
    let mut context=DecCtx{source:DecSource::Inline{symbols},limits:options.limits.clone(),verification:options.verification,preserve_unknown:false,unknown_field_ids:Vec::new(),materialization:&materialization};
    let record=decode_record_fields(&mut reader,Some(spec),&mut context,0)?;
    if reader.position()!=bytes.len(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"trailing bytes after terminal record"})}
    if !context.unknown_field_ids.is_empty(){return Err(PackRefusal::RetainedMalformed{kind:ValueRefusalKind::InvalidValue,what:"record body",offset:reader.position()as u64,detail:"unknown terminal record fields"})}
    Ok(record)
}
