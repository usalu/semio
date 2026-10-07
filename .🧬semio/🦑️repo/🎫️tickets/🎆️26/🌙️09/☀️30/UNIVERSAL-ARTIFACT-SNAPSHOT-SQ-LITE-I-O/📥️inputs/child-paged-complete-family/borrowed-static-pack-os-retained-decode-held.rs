    /// 📥️ Holds the actual decoded variant and canonical scratch until caller settlement.
    pub type RetainedDecodedOperation<T:DslVariants> = crate::os_pack::record::BorrowedProjectedPackOperation<crate::os_pack::record::OwnedVariantPackSource<T>>;

    fn decode_with_span<T:DslVariants+semio_framework_dsl_record::BorrowedDslVariants>(source:crate::os_pack::ByteSpan<'_>,index_of:impl Fn(u64,&[(String,super::RecordSpecProducer)])->Result<usize,ProtocolError>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>,retained:&mut Option<RetainedDecodedOperation<T>>,reencode:impl FnOnce(&mut RetainedDecodedOperation<T>,&EncodeOptions,&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>)->Result<(),ProtocolError>{
        if retained.is_some(){return Err(crate::os_pack::PackRefusal::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical decoded operation owner is not empty")).into())}
        decoding.checkpoint().map_err(crate::os_pack::PackRefusal::from)?;
        if source.len()as u64>options.limits.max_file_len{return Err(crate::os_pack::PackRefusal::LimitExceeded{kind:semio_framework_value::ValueRefusalKind::OwnershipLimit,limit:"operation source byte length"}.into());}
        let mut reader=ByteReader::from_span(source);
        let format=reader.read_u8()?;
        if format!=OP_BINARY_FORMAT{return Err(ProtocolError::Malformed{what:"op format",offset:0,detail:format!("unsupported op format {format}")});}
        let tag=reader.read_varint_u64()?;
        let variants=T::variants_controlled(decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let index=index_of(tag,&variants)?;
        let(keyword,producer)=&variants[index];
        let spec=producer.decode(decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let body=reader.read_span(reader.remaining())?;
        let record=crate::os_pack::record::decode_record_body_span_exact_controlled(body,&spec,options,decoding)?;
        let decoded=T::from_named_record_controlled(keyword,&record,decoding).map_err(crate::os_pack::PackRefusal::from)?;
        let mut comparison=protocol::io::binary::operation_bytes::OperationByteComparison::new(source);
        *retained=Some(crate::os_pack::record::BorrowedProjectedPackOperation::from_owned_variant(decoded));
        reencode(retained.as_mut().unwrap(),canonical_options,&mut comparison,encoding)?;
        comparison.finish()?;
        Ok(())
    }

    /// 🧾️ Decodes the same admitted source and compares its complete canonical tagged wire without a second byte owner.
    pub fn decode_tagged_op_span<T:DslVariants+semio_framework_dsl_record::BorrowedDslVariants>(protocol:&str,source:crate::os_pack::ByteSpan<'_>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>,retained:&mut Option<RetainedDecodedOperation<T>>)->Result<(),ProtocolError>{
        decode_with_span(source,|tag,variants|{
            let kind=protocol_record::kind(protocol,tag).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("📡️.protocol.semio declares no record with tag {tag}")})?;
            variants.iter().position(|(keyword,_)|keyword==kind).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("record '{kind}' names no variant")})
        },options,canonical_options,decoding,encoding,retained,|decoded,options,output,control|encode_tagged_op_into(protocol,decoded,options,output,control))
    }

    /// 🎞️ Decodes a borrowed operation and checks exact ordinal canonical bytes with caller controls.
    pub fn decode_op_span<T:DslVariants+semio_framework_dsl_record::BorrowedDslVariants>(source:crate::os_pack::ByteSpan<'_>,options:&DecodeOptions,canonical_options:&EncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>,retained:&mut Option<RetainedDecodedOperation<T>>)->Result<(),ProtocolError>{
        decode_with_span(source,|ordinal,variants|{
            let index=usize::try_from(ordinal).map_err(|_|ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} exceeds the native index range")})?;
            if index<variants.len(){Ok(index)}else{Err(ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} out of range for {} declared variants",variants.len())})}
        },options,canonical_options,decoding,encoding,retained,encode_op_into)
    }

