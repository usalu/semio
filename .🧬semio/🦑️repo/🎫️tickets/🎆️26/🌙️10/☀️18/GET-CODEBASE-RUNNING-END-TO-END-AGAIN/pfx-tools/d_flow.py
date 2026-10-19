import dom
B='''/// 🧩️ Fills the retained flow root from borrowed binary octets, inserting every node, parameter and edge before its fields.
pub fn binary_into(output:&mut SemioFlowSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio flow native format"))}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.nodes,&mut reader,control,limits,&mut entities,Default::default,|node,reader,control,entities|{
  native::reader_text_into(&mut node.id,reader,control)?;native::reader_text_into(&mut node.kind,reader,control)?;native::reader_text_into(&mut node.label,reader,control)?;
  native::binary_list_into(&mut node.params,reader,control,limits,entities,Default::default,|param,reader,control,_|{native::reader_text_into(&mut param.key,reader,control)?;native::reader_text_into(&mut param.value,reader,control)})?;
  node.position=SemioPoint2{x:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,y:reader.read_f64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?};Ok(())
 })?;
 native::binary_list_into(&mut output.edges,&mut reader,control,limits,&mut entities,Default::default,|edge,reader,control,_|{native::reader_text_into(&mut edge.id,reader,control)?;native::reader_text_into(&mut edge.from.node,reader,control)?;native::reader_text_into(&mut edge.from.port,reader,control)?;native::reader_text_into(&mut edge.to.node,reader,control)?;native::reader_text_into(&mut edge.to.port,reader,control)?;native::reader_text_into(&mut edge.kind,reader,control)})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native trailing bytes"))}Ok(())
}
'''
D='''/// 📝️ Fills the retained flow root from the borrowed document body, inserting every node, parameter and edge before its fields.
pub fn document_into(output:&mut SemioFlowSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","nodes","edges"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio flow native schema missing"))?,control)?;
 native::text_list_into(&mut output.nodes,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|node,value,control,entities|{
  let[id,kind,label,param_fields,position]=native::record(value,control)?;native::hex_text_into(&mut node.id,id,control)?;native::hex_text_into(&mut node.kind,kind,control)?;native::hex_text_into(&mut node.label,label,control)?;
  native::text_list_into(&mut node.params,param_fields,control,limits,entities,Default::default,|param,value,control,_|{let[key,value]=native::record(value,control)?;native::hex_text_into(&mut param.key,key,control)?;native::hex_text_into(&mut param.value,value,control)})?;
  let[x,y]=native::record(position,control)?;node.position=SemioPoint2{x:native::float(x,control)?,y:native::float(y,control)?};Ok(())
 })?;
 native::text_list_into(&mut output.edges,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|edge,value,control,_|{let[id,from,to,kind]=native::record(value,control)?;native::hex_text_into(&mut edge.id,id,control)?;port_into(&mut edge.from,from,control)?;port_into(&mut edge.to,to,control)?;native::hex_text_into(&mut edge.kind,kind,control)})
}
fn port_into(output:&mut PortRef,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let[node,port]=native::record(value,control)?;native::hex_text_into(&mut output.node,node,control)?;native::hex_text_into(&mut output.port,port,control)}
'''
print(dom.apply("🌊️flow",B,D,remove=("port",)))
print(dom.fmt("🌊️flow"))
