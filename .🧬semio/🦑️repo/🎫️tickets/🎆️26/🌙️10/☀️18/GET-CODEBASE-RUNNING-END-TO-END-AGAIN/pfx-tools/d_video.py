import dom
B='''/// 🧩️ Fills the retained video root from borrowed binary octets, inserting every stream and sample before its fields.
pub fn binary_into(output:&mut SemioVideoSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio video native format"))}
 native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.streams,&mut reader,control,limits,&mut entities,Default::default,|stream,reader,control,entities|{
  stream.kind=crate::io::binary::snapshot::kind_from_tag(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error))?;native::reader_text_into(&mut stream.codec,reader,control)?;stream.width=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;stream.height=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;stream.rate=SemioRational{num:integer(reader)?,den:integer(reader)?};
  native::binary_list_into(&mut stream.samples,reader,control,limits,entities,Default::default,|sample,reader,control,_|{sample.pts=reader.read_u64_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;sample.key=match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>false,1=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native key flag"))};native::reader_bytes_into(&mut sample.data,reader,control)})
 })?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio video native trailing bytes"))}Ok(())
}
'''
D='''/// 📝️ Fills the retained video root from the borrowed document body, inserting every stream and sample before its fields.
pub fn document_into(output:&mut SemioVideoSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","streams"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio video native schema missing"))?,control)?;
 native::text_list_into(&mut output.streams,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|stream,value,control,entities|{
  let[kind,codec,width,height,rate,sample_fields]=native::record(value,control)?;stream.kind=match kind{"V"=>SemioVideoStreamKind::Video,"A"=>SemioVideoStreamKind::Audio,"S"=>SemioVideoStreamKind::Subtitle,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native stream kind"))};native::hex_text_into(&mut stream.codec,codec,control)?;stream.width=width.parse::<u32>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;stream.height=height.parse::<u32>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;let[num,den]=native::record(rate,control)?;stream.rate=SemioRational{num:num.parse::<i64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?,den:den.parse::<i64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?};
  native::text_list_into(&mut stream.samples,sample_fields,control,limits,entities,Default::default,|sample,value,control,_|{let[pts,key,data]=native::record(value,control)?;sample.pts=pts.parse::<u64>().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;sample.key=match key{"0"=>false,"1"=>true,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio video native key flag"))};native::hex_bytes_into(&mut sample.data,data,control)})
 })
}
'''
print(dom.apply("🎬️video",B,D))
print(dom.fmt("🎬️video"))
