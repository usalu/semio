 fn encode_op_into(&self,options:&protocol::codec::PackEncodeOptions,output:&mut dyn protocol::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),protocol::ProtocolError>{
  let mut limited=protocol::operation_bytes::OperationByteLimitedOutput::new(output,options.limits.max_file_len);
  let output:&mut dyn protocol::operation_bytes::OperationByteOutput=&mut limited;
  let spec=Self::__dsl_spec_producer().encode(control).map_err(protocol::PackRefusal::from)?;
  let record=self.__dsl_to_record_controlled(control).map_err(protocol::PackRefusal::from)?;
  output.write_bytes(&[1,1],control)?;
  pack::record::encode_record_body_into(&spec,&record,options,output,control)?;
  Ok(())
 }
 fn decode_op_span(source:protocol::ByteSpan<'_>,options:&protocol::codec::PackDecodeOptions,canonical_options:&protocol::codec::PackEncodeOptions,decoding:&mut semio_framework_value::NativeDecodeControl<'_>,encoding:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Self,protocol::ProtocolError>{
  decoding.checkpoint().map_err(protocol::PackRefusal::from)?;
  if source.len()as u64>options.limits.max_file_len{return Err(protocol::PackRefusal::LimitExceeded{kind:semio_framework_value::ValueRefusalKind::OwnershipLimit,limit:"Chart operation source byte length"}.into());}
  if source.get(0)!=Some(&1)||source.get(1)!=Some(&1){return Err(protocol::PackRefusal::RetainedMalformed{kind:semio_framework_value::ValueRefusalKind::InvalidValue,what:"Chart operation format/tag",offset:0,detail:"expected exact [1,1] header"}.into());}
  let spec=Self::__dsl_spec_producer().decode(decoding).map_err(protocol::PackRefusal::from)?;
  let record=pack::record::decode_record_body_span_exact_controlled(source.slice(2,source.len()-2)?,&spec,options,decoding)?;
  let operation=Self::__dsl_from_record_controlled(&record,decoding).map_err(protocol::PackRefusal::from)?;
  let mut comparison=protocol::operation_bytes::OperationByteComparison::new(source);
  operation.encode_op_into(canonical_options,&mut comparison,encoding)?;
  comparison.finish()?;
  Ok(operation)
 }
