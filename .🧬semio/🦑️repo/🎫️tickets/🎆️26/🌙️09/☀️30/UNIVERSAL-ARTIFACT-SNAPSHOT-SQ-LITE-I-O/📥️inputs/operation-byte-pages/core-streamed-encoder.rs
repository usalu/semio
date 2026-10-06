pub(super) fn record_body_into(spec:&RecordSpec,record:&RecordValue,options:&EncodeOptions,output:&mut dyn protocol::mutation::bytes::OperationByteOutput,control:&mut NativeEncodeControl<'_>)->Result<usize,PackRefusal>{
    let maximum=usize::try_from(options.limits.max_total_alloc).unwrap_or(usize::MAX).min(control.maximum_bytes());
    control.scoped_maximum(maximum,|control|control.scoped_stage(|control|{
        control.begin_stage(0)?;
        let mut symbols=Symbols{entries:Vec::new()};
        symbols.record(Some(spec),record,0,options.limits.max_depth,control)?;
        symbols.finish(control)?;
        let mut encoder=Encoder{symbols:&symbols,options,control,writer:None,chunking:false,next_chunk:0};
        let mut measure=Output::measure();
        encoder.body(spec,record,&mut measure)?;
        if measure.length as u64>options.limits.max_file_len{return Err(PackRefusal::LimitExceeded{kind:ValueRefusalKind::WorkLimit,limit:"operation Record exceeds max_file_len"})}
        if measure.length>maximum.saturating_sub(encoder.control.owned_bytes()){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"operation Record exceeds caller allocation allowance").into())}
        let mut output=Output{bytes:None,external:Some(output),length:0};
        encoder.body(spec,record,&mut output)?;
        if output.length!=measure.length{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"operation Record length changed between admission and emission").into())}
        Ok(output.length)
    }))
}
