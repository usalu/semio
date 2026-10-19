import dom,rs
OT='''/// 🔺️ Fills one optional texture reference in its original slot from borrowed binary octets.
fn optional_text_into(output:&mut Option<String>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>Ok(()),1=>{*output=Some(String::new());native::reader_text_into(output.as_mut().unwrap(),reader,control)},_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native texture reference presence"))}}
/// 🔺️ Fills one optional texture reference in its original slot from a bracketed hexadecimal record.
fn optional_text_record_into(output:&mut Option<String>,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if value=="[0]"{return Ok(())}let[tag,value]=native::record(value,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native texture reference presence"))}*output=Some(String::new());native::hex_text_into(output.as_mut().unwrap(),value,control)}
'''
B='''/// 🧩️ Fills the retained mesh root from borrowed binary octets, inserting every mesh, primitive, material and texture before its fields.
pub fn binary_into(output:&mut SemioMeshSnapshot,body:&[u8],control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_binary(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let mut reader=store::ByteReader::new(body);if reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unsupported Semio mesh native format"))}native::reader_text_into(&mut output.schema,&mut reader,control)?;
 native::binary_list_into(&mut output.meshes,&mut reader,control,limits,&mut entities,Default::default,|mesh,reader,control,entities|{
  native::reader_text_into(&mut mesh.id,reader,control)?;
  native::binary_list_into(&mut mesh.primitives,reader,control,limits,entities,Default::default,|primitive,reader,control,entities|{
   native::reader_text_into(&mut primitive.id,reader,control)?;primitive.topology=topology(reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?)?;
   native::binary_list_into(&mut primitive.positions,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=point(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.normals,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=point(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.uvs,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=SemioUv{u:f64(reader)?,v:f64(reader)?};Ok(())})?;native::binary_list_into(&mut primitive.colors,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=rgba(reader)?;Ok(())})?;native::binary_list_into(&mut primitive.indices,reader,control,limits,entities,Default::default,|slot,reader,_,_|{*slot=reader.read_u32_le().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?;Ok(())})?;
   match reader.read_u8().map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string()))?{0=>{},1=>{primitive.material_id=Some(String::new());native::reader_text_into(primitive.material_id.as_mut().unwrap(),reader,control)?;},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native material presence"))}
   Ok(())
  })
 })?;
 native::binary_list_into(&mut output.materials,&mut reader,control,limits,&mut entities,Default::default,|material,reader,control,_|{native::reader_text_into(&mut material.id,reader,control)?;material.base_color=rgba(reader)?;material.metallic=f32(reader)?;material.roughness=f32(reader)?;optional_text_into(&mut material.base_color_texture,reader,control)?;optional_text_into(&mut material.metallic_roughness_texture,reader,control)?;optional_text_into(&mut material.normal_texture,reader,control)?;optional_text_into(&mut material.occlusion_texture,reader,control)?;optional_text_into(&mut material.emissive_texture,reader,control)})?;
 native::binary_list_into(&mut output.textures,&mut reader,control,limits,&mut entities,Default::default,|texture,reader,control,_|{native::reader_text_into(&mut texture.id,reader,control)?;native::reader_text_into(&mut texture.mime,reader,control)?;native::reader_bytes_into(&mut texture.bytes,reader,control)})?;
 if reader.remaining()!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio mesh native trailing bytes"))}Ok(())
}
'''
D='''/// 📝️ Fills the retained mesh root from the borrowed document body, inserting every mesh, primitive, material and texture before its fields.
pub fn document_into(output:&mut SemioMeshSnapshot,body:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<(),ValueError>{
 crate::io::sqlite::snapshot::admit_document(body,control,limits)?;
 let mut entities=0;native::entities(&mut entities,1,limits)?;let fields=native::fields(body,["schema","meshes","materials","textures"],control)?;native::hex_text_into(&mut output.schema,fields[0].ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio mesh native schema missing"))?,control)?;
 native::text_list_into(&mut output.meshes,fields[1].unwrap_or("[]"),control,limits,&mut entities,Default::default,|mesh,value,control,entities|{
  let[id,primitives]=native::record(value,control)?;native::hex_text_into(&mut mesh.id,id,control)?;
  native::text_list_into(&mut mesh.primitives,primitives,control,limits,entities,Default::default,|primitive,value,control,entities|{
   let[id,topology_fields,positions,normals,uvs,colors,indices,material_fields]=native::record(value,control)?;native::hex_text_into(&mut primitive.id,id,control)?;let tag=match topology_fields{"P"=>0,"L"=>1,"S"=>2,"T"=>3,"X"=>4,"F"=>5,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native topology"))};primitive.topology=topology(tag)?;
   native::text_list_into(&mut primitive.positions,positions,control,limits,entities,Default::default,|slot,value,control,_|{*slot=point_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.normals,normals,control,limits,entities,Default::default,|slot,value,control,_|{*slot=point_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.uvs,uvs,control,limits,entities,Default::default,|slot,value,control,_|{let[u,v]=native::record(value,control)?;*slot=SemioUv{u:native::float(u,control)?,v:native::float(v,control)?};Ok(())})?;native::text_list_into(&mut primitive.colors,colors,control,limits,entities,Default::default,|slot,value,control,_|{*slot=rgba_text(value,control)?;Ok(())})?;native::text_list_into(&mut primitive.indices,indices,control,limits,entities,Default::default,|slot,value,control,_|{*slot=control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}value.parse::<u32>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native index"))})?;Ok(())})?;
   if material_fields!="[0]"{let[tag,value]=native::record(material_fields,control)?;if tag!="1"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native material presence"))}primitive.material_id=Some(String::new());native::hex_text_into(primitive.material_id.as_mut().unwrap(),value,control)?;}
   Ok(())
  })
 })?;
 native::text_list_into(&mut output.materials,fields[2].unwrap_or("[]"),control,limits,&mut entities,Default::default,|material,value,control,_|{let[id,color,metallic,roughness,base_color_texture,metallic_roughness_texture,normal_texture,occlusion_texture,emissive_texture]=native::record(value,control)?;native::hex_text_into(&mut material.id,id,control)?;material.base_color=rgba_text(color,control)?;material.metallic=native::float32(metallic,control)?;material.roughness=native::float32(roughness,control)?;optional_text_record_into(&mut material.base_color_texture,base_color_texture,control)?;optional_text_record_into(&mut material.metallic_roughness_texture,metallic_roughness_texture,control)?;optional_text_record_into(&mut material.normal_texture,normal_texture,control)?;optional_text_record_into(&mut material.occlusion_texture,occlusion_texture,control)?;optional_text_record_into(&mut material.emissive_texture,emissive_texture,control)})?;
 native::text_list_into(&mut output.textures,fields[3].unwrap_or("[]"),control,limits,&mut entities,Default::default,|texture,value,control,_|{let[id,mime,bytes]=native::record(value,control)?;native::hex_text_into(&mut texture.id,id,control)?;native::hex_text_into(&mut texture.mime,mime,control)?;native::hex_bytes_into(&mut texture.bytes,bytes,control)})
}
'''
p=rs.native("🔺️mesh")
print(dom.apply("🔺️mesh",B,D,remove=("optional_text","optional_text_record")))
s=open(p,encoding="utf-8").read()
s=s.replace("/// 🧩️ Fills the retained mesh root",OT+"/// 🧩️ Fills the retained mesh root",1)
open(p,"w",encoding="utf-8").write(s)
print(dom.fmt("🔺️mesh"))
