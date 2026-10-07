fn encode_with_into<T:semio_framework_dsl_record::native_encoding::FieldProjectionSource>(operation:&mut crate::os_pack::record::BorrowedProjectedPackOperation<T>,tag_of:impl Fn(&str,usize)->Result<u64,ProtocolError>,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
 protocol::io::binary::operation_bytes::with_operation_encode_policy(options,control,|control|{
  let mut limited=protocol::io::binary::operation_bytes::OperationByteLimitedOutput::new(output,options.limits.max_file_len);let output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput=&mut limited;
  control.checkpoint().map_err(crate::os_pack::PackRefusal::from)?;let(keyword,ordinal)=operation.variant_identity()?;let tag=tag_of(keyword,ordinal)?;
  output.write_bytes(&[OP_BINARY_FORMAT],control)?;let mut remaining=tag;let mut bytes=[0;10];let mut length=0;
  loop{bytes[length]=(remaining as u8)&127;remaining>>=7;if remaining!=0{bytes[length]|=128;}length+=1;if remaining==0{break;}}
  output.write_bytes(&bytes[..length],control)?;operation.write_body(options,output,control)?;Ok(())
 })
}
/// 🏷️ Emits the declared protocol tag from the caller's original retained operation capsule.
pub fn encode_tagged_op_into<T:semio_framework_dsl_record::native_encoding::FieldProjectionSource>(protocol:&str,operation:&mut crate::os_pack::record::BorrowedProjectedPackOperation<T>,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
 encode_with_into(operation,|keyword,_|protocol_record::records(protocol).find(|(kind,_)|*kind==keyword).map(|(_,tag)|tag).ok_or_else(||ProtocolError::Malformed{what:"op tag",offset:1,detail:format!("📡️.protocol.semio declares no record for '{keyword}'")}),options,output,control)
}
/// 🎞️ Emits the exact ordinal header and same-source canonical body while retaining all scratch.
pub fn encode_op_into<T:semio_framework_dsl_record::native_encoding::FieldProjectionSource>(operation:&mut crate::os_pack::record::BorrowedProjectedPackOperation<T>,options:&EncodeOptions,output:&mut dyn protocol::io::binary::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{
 encode_with_into(operation,|_,ordinal|u64::try_from(ordinal).map_err(|_|ProtocolError::Malformed{what:"op variant",offset:1,detail:format!("ordinal {ordinal} exceeds the u64 wire range")}),options,output,control)
}
