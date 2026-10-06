import {readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {join} from "node:path";
import assert from "node:assert/strict";
if(process.argv[2]==="readback"){const pairs=JSON.parse(readFileSync(join(import.meta.dir,"held-pairs.json"),"utf8"));for(const p of pairs)p.after=readFileSync(p.path,"utf8");writeFileSync(join(import.meta.dir,"held-pairs.json"),JSON.stringify(pairs,null,2)+"\n");console.log("[DEBUG] Generation exact current provider readback paths="+pairs.length+" production_mutations=0");process.exit(0)}
if(process.argv[2]==="mount"){
 const pairs=JSON.parse(readFileSync(join(import.meta.dir,"held-pairs.json"),"utf8"));
 for(const p of pairs)assert.equal(readFileSync(p.path,"utf8"),p.before,"Concurrent Generation provider "+p.path);
 for(const p of pairs)writeFileSync(p.path,p.after);
 console.log("[DEBUG] Generation exact typed and borrowed semantic provider mounted paths="+pairs.length);
 process.exit(0);
}
const root="/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native";
const path=join(root,"📏️rows/🦀️.rs"),before=readFileSync(path,"utf8");let after=before;
function replace(a:string,b:string){assert(after.includes(a),a);after=after.replace(a,b)}
replace("use semio_framework_value::DslValue;","use semio_framework_value::DslValue;\nuse semio_framework_value::Number;");
replace("{FlowUi,neural::Tree}","{FlowUi,NodeChrome,neural::Tree}");
replace("maximum:usize,count:usize}","maximum:usize,count:usize,maximum_bytes:usize,bytes:usize}");
replace(" fn add(&mut self,count:usize)",String.raw` fn cells(&mut self,n:usize)->Result<(),ValueError>{self.bytes=self.bytes.checked_add(n).filter(|n|*n<=self.maximum_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation complete SQL semantic value bytes exceeded"))?;self.control.step()}
 fn text(&mut self,v:&str)->Result<(),ValueError>{self.cells(v.len())}
 fn float(&mut self,v:f64)->Result<(),ValueError>{self.cells(if v.is_nan(){11}else if v.is_infinite(){32}else{22})}
 fn texts<'a>(&mut self,values:impl IntoIterator<Item=&'a str>)->Result<(),ValueError>{for v in values{self.text(v)?;}Ok(())}
 fn add(&mut self,count:usize)`);
after=after.replaceAll("maximum:usize,c:&mut C)","maximum:usize,maximum_bytes:usize,c:&mut C)").replaceAll("Rows{control:c,maximum,count:0}","Rows{control:c,maximum,count:0,maximum_bytes,bytes:0}");
replace("let mut pending=Vec::new();r.add(2)?;r.add(snapshot", "let mut pending=Vec::new();r.cells(24)?;r.text(&snapshot.host_snapshot.schema)?;for v in [snapshot.host_snapshot.camera.x,snapshot.host_snapshot.camera.y,snapshot.host_snapshot.camera.zoom]{r.float(v)?;}if let Some(v)=&snapshot.generation.selected_generation_id{r.text(v)?;}if let Some(v)=&snapshot.generation.preview_text{r.text(v)?;}for s in &snapshot.host_snapshot.synapses{r.cells(24)?;r.texts([s.id.as_str(),s.from.as_str(),s.to.as_str(),s.from_port.as_str(),s.to_port.as_str()])?;}for(k,v)in snapshot.host_snapshot.layout.iter(){r.cells(24)?;r.text(k)?;r.float(v.x)?;r.float(v.y)?;}r.add(2)?;r.add(snapshot");
replace("for widget in &snapshot.host_snapshot.widgets{r.add(2)?;match widget{", "for widget in &snapshot.host_snapshot.widgets{r.add(2)?;owned_widget(&mut r,widget)?;match widget{");
replace("for generation in &snapshot.generation.generations{r.add(1)?;for value in generation.values.values(){r.add(1)?;", "for generation in &snapshot.generation.generations{r.add(1)?;r.cells(24)?;r.texts([generation.id.as_str(),generation.name.as_str()])?;for(key,value)in &generation.values{r.add(1)?;r.cells(32)?;r.text(key)?;");
replace("Owned::Dictionary(value)=>{r.add(1)?;for(_,value)in value.iter(){r.add(2)?;", "Owned::Dictionary(value)=>{r.add(1)?;r.cells(8)?;for(key,value)in value.iter(){r.add(2)?;r.cells(40)?;r.text(key)?;match value{NeuralValue::Atom(Atom::Null)=>r.text(\"null\")?,NeuralValue::Atom(Atom::Boolean(_))=>{r.text(\"boolean\")?;r.cells(8)?;},NeuralValue::Atom(Atom::Integer(_))=>{r.text(\"integer\")?;r.cells(8)?;},NeuralValue::Atom(Atom::Decimal(v))=>{r.text(\"decimal\")?;r.float(*v)?;},NeuralValue::Atom(Atom::String(v))=>{r.text(\"string\")?;r.text(v)?;},NeuralValue::Dictionary(_)=>{r.text(\"dictionary\")?;r.cells(8)?;}}");
replace("Owned::Tree(value)=>{r.add(1)?;r.add(value.synapses.len())?;for neuron in &value.neurons{r.add(1)?;", "Owned::Tree(value)=>{r.add(1)?;r.cells(8)?;r.add(value.synapses.len())?;for s in &value.synapses{r.cells(24)?;r.texts([s.id.as_str(),s.from.as_str(),s.to.as_str(),s.from_port.as_str(),s.to_port.as_str()])?;}for neuron in &value.neurons{r.add(1)?;r.cells(32+if neuron.tree.is_some(){8}else{0})?;r.texts([neuron.id.as_str(),neuron.kind.as_str()])?;");
replace("Owned::Gui(value)=>{r.add(1)?;", "Owned::Gui(value)=>{owned_gui(&mut r,value)?;r.add(1)?;");
replace("Owned::Answer(value)=>{r.add(1)?;match value", "Owned::Answer(value)=>{r.add(1)?;answer_cells(&mut r,value)?;match value");
replace("Array(values)=>for value in values{r.add(1)?;r.push(&mut pending,Owned", "Array(values)=>for value in values{r.add(1)?;r.cells(32)?;r.push(&mut pending,Owned");
replace("Object(values)=>for(_,value)in values{r.add(1)?;r.push(&mut pending,Owned", "Object(values)=>for(key,value)in values{r.add(1)?;r.cells(32)?;r.text(key)?;r.push(&mut pending,Owned");
replace("let mut pending=Vec::new();r.add(2)?;r.add(list(field(source,3)", "let mut pending=Vec::new();r.cells(24)?;r.text(text(field(source,0)?)?)?;float_record(&mut r,record(field(source,1)?)?,&[0,1,2])?;for id in [5,6]{if let Some(v)=optional(source.fields.get(&id)){r.text(text(v)?)?;}}for s in list(field(source,3)?)?{let s=record(s)?;r.cells(24)?;r.text(text(field(s,0)?)?)?;let FieldValue::Wire(w)=field(s,1)? else{return Err(invalid())};r.text(&w.from.id)?;if let Some(v)=&w.from.port{r.text(v)?;}if let Some((_,to))=&w.edge{r.text(&to.id)?;if let Some(v)=&to.port{r.text(v)?;}}}for(k,v)in map(field(source,4)?)?{r.cells(24)?;r.text(k)?;float_record(&mut r,record(v)?,&[0,1])?;}r.add(2)?;r.add(list(field(source,3)");
replace("for(kind,widget)in widgets{r.add(2)?;match", "for(kind,widget)in widgets{r.add(2)?;borrowed_widget(&mut r,kind,widget)?;match");
replace("for generation in list(field(source,7)?)?{r.add(1)?;for(_,value)in map(field(record(generation)?,2)?)?{r.add(1)?;", "for generation in list(field(source,7)?)?{r.add(1)?;let g=record(generation)?;r.cells(24)?;r.text(text(field(g,0)?)?)?;r.text(text(field(g,1)?)?)?;for(key,value)in map(field(g,2)?)?{r.add(1)?;r.cells(32)?;r.text(key)?;");
replace("Borrowed::Dictionary(entries)=>{r.add(1)?;for entry in entries{r.add(2)?;let value=record(field(record(entry)?,1)?)?;", "Borrowed::Dictionary(entries)=>{r.add(1)?;r.cells(8)?;for entry in entries{r.add(2)?;let entry=record(entry)?;r.cells(40)?;r.text(text(field(entry,0)?)?)?;let value=record(field(entry,1)?)?;neural_fields(&mut r,value)?;");
replace("Borrowed::DynamicDictionary(value)=>{r.add(1)?;for(_,value)in object(value)?{r.add(2)?;", "Borrowed::DynamicDictionary(value)=>{r.add(1)?;r.cells(8)?;for(key,value)in object(value)?{r.add(2)?;r.cells(40)?;r.text(key)?;neural_dynamic(&mut r,value)?;");
replace("Borrowed::Tree(value)=>{r.add(1)?;r.add(array(member(value,\"synapses\")?)?.len())?;for neuron in array(member(value,\"neurons\")?)?{r.add(1)?;", "Borrowed::Tree(value)=>{r.add(1)?;r.cells(8)?;r.add(array(member(value,\"synapses\")?)?.len())?;for s in array(member(value,\"synapses\")?)?{r.cells(24)?;for name in [\"id\",\"from\",\"to\",\"fromPort\",\"toPort\"]{r.text(dynamic_text(member(s,name)?)?)?;}}for neuron in array(member(value,\"neurons\")?)?{r.add(1)?;r.cells(32+if matches!(member(neuron,\"tree\")?,DslValue::Null){0}else{8})?;r.text(dynamic_text(member(neuron,\"id\")?)?)?;r.text(dynamic_text(member(neuron,\"kind\")?)?)?;");
replace("Borrowed::Gui(value)=>{r.add(1)?;", "Borrowed::Gui(value)=>{borrowed_gui(&mut r,value)?;r.add(1)?;");
replace("Borrowed::Answer(value)=>{r.add(1)?;match value", "Borrowed::Answer(value)=>{r.add(1)?;answer_cells(&mut r,value)?;match value");
replace("Array(values)=>for value in values{r.add(1)?;r.push(&mut pending,Borrowed::Answer", "Array(values)=>for value in values{r.add(1)?;r.cells(32)?;r.push(&mut pending,Borrowed::Answer");
replace("Object(values)=>for(_,value)in values{r.add(1)?;r.push(&mut pending,Borrowed::Answer", "Object(values)=>for(key,value)in values{r.add(1)?;r.cells(32)?;r.text(key)?;r.push(&mut pending,Borrowed::Answer");
after+=String.raw`
fn invalid()->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,"Generation semantic SQL field shape differs")}
fn text(v:&FieldValue)->Result<&str,ValueError>{match v{FieldValue::Text(v)=>Ok(v),_=>Err(invalid())}}
fn dynamic_text(v:&DslValue)->Result<&str,ValueError>{match v{DslValue::String(v)=>Ok(v),_=>Err(invalid())}}
fn dynamic_float(v:&DslValue)->Result<f64,ValueError>{match v{DslValue::Number(v)=>Ok(v.as_f64()),_=>Err(invalid())}}
fn float_record<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&RecordValue,ids:&[u16])->Result<(),ValueError>{for id in ids{let FieldValue::Float(v)=field(v,*id)? else{return Err(invalid())};r.float(*v)?;}Ok(())}
fn dynamic_floats<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&DslValue,names:&[&str])->Result<(),ValueError>{for name in names{r.float(dynamic_float(member(v,name)?)?)?;}Ok(())}
fn digits(mut v:u64)->usize{let mut n=1;while v>=10{v/=10;n+=1;}n}
fn answer_cells<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&DslValue)->Result<(),ValueError>{r.cells(8)?;match v{DslValue::Null=>r.text("null"),DslValue::Bool(_)=>{r.text("boolean")?;r.cells(8)},DslValue::Number(Number::UInt(v))=>{r.text("unsigned")?;r.cells(digits(*v))},DslValue::Number(Number::Int(_))=>{r.text("integer")?;r.cells(8)},DslValue::Number(Number::Float(v))=>{r.text("float")?;r.float(*v)},DslValue::String(v)=>{r.text("string")?;r.text(v)},DslValue::Bytes(v)=>{r.text("bytes")?;r.cells(v.len())},DslValue::Array(_)=>r.text("array"),DslValue::Object(_)=>r.text("object")}}
fn neural_dynamic<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&DslValue)->Result<(),ValueError>{match v{DslValue::Null=>r.text("null"),DslValue::Bool(_)=>{r.text("boolean")?;r.cells(8)},DslValue::Number(Number::Int(_)|Number::UInt(_))=>{r.text("integer")?;r.cells(8)},DslValue::Number(Number::Float(v))=>{r.text("decimal")?;r.float(*v)},DslValue::String(v)=>{r.text("string")?;r.text(v)},DslValue::Object(_)=>{r.text("dictionary")?;r.cells(8)},_=>Err(invalid())}}
fn neural_fields<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&RecordValue)->Result<(),ValueError>{let mut found=0;for id in 0..6{if let Some(v)=optional(v.fields.get(&id)){found+=1;match(id,v){(0,FieldValue::Bool(true))=>r.text("null")?,(1,FieldValue::Bool(_))=>{r.text("boolean")?;r.cells(8)?;},(2,FieldValue::Int(_))=>{r.text("integer")?;r.cells(8)?;},(3,FieldValue::Float(v))=>{r.text("decimal")?;r.float(*v)?;},(4,FieldValue::Text(v))=>{r.text("string")?;r.text(v)?;},(5,FieldValue::List(_))=>{r.text("dictionary")?;r.cells(8)?;},_=>return Err(invalid())}}}if found!=1{return Err(invalid())}Ok(())}
fn owned_widget<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&Widget)->Result<(),ValueError>{r.cells(24)?;let(kind,id)=match v{Widget::Neuron{id,..}=>("neuron",id),Widget::InputSlider{id,..}=>("input_slider",id),Widget::InputNote{id,..}=>("input_note",id),Widget::InputImage{id,..}=>("input_image",id),Widget::Variable{id,..}=>("variable",id),Widget::OutputPreview{id,..}=>("output_preview",id),Widget::OutputAction{id,..}=>("output_action",id),Widget::OutputExport{id,..}=>("output_export",id),Widget::Cluster{id,..}=>("cluster",id)};r.texts([kind,id.as_str()])?;match v{
Widget::Neuron{neuron_kind,input_ports,output_ports,..}=>{r.cells(24)?;r.text(neuron_kind)?;for(dir,ports)in [("input",input_ports),("output",output_ports)]{for name in ports{r.cells(24)?;r.texts([dir,name.as_str()])?;}}},
Widget::InputSlider{label,value,min,max,step,..}=>{r.cells(8)?;r.text(label)?;for v in [value,min,max,step]{r.float(*v)?;}},
Widget::InputNote{text,..}=>{r.cells(8)?;r.text(text)?;},Widget::InputImage{src,..}=>{r.cells(8)?;r.text(src)?;},Widget::Variable{name,schema,..}=>{r.cells(8)?;r.texts([name.as_str(),schema.as_str()])?;},
Widget::OutputPreview{expanded,..}=>{r.cells(16)?;for path in expanded.iter(){r.cells(24)?;r.text(path)?;}},Widget::OutputAction{action,..}=>{r.cells(8)?;r.text(action)?;},Widget::OutputExport{format,..}=>{r.cells(8)?;r.text(format)?;},Widget::Cluster{name,..}=>{r.cells(24)?;r.text(name)?;}}
Ok(())}
fn borrowed_widget<C:NativeSchemaControl>(r:&mut Rows<'_,C>,kind:&str,v:&RecordValue)->Result<(),ValueError>{let sql_kind=match kind{"neuron"=>"neuron","input-slider"=>"input_slider","input-note"=>"input_note","input-image"=>"input_image","variable"=>"variable","output-preview"=>"output_preview","output-action"=>"output_action","output-export"=>"output_export","cluster"=>"cluster",_=>return Err(invalid())};r.cells(24)?;r.texts([sql_kind,text(field(v,0)?)?])?;match kind{
"neuron"=>{r.cells(24)?;r.text(text(field(v,1)?)?)?;for(id,dir)in[(3,"input"),(4,"output")]{for name in list(field(v,id)?)?{r.cells(24)?;r.texts([dir,text(name)?])?;}}},
"input-slider"=>{r.cells(8)?;r.text(text(field(v,1)?)?)?;float_record(r,v,&[2,3,4,5])?;},
"variable"=>{r.cells(8)?;r.text(text(field(v,1)?)?)?;r.text(text(field(v,2)?)?)?;},
"input-note"|"input-image"|"output-action"|"output-export"=>{r.cells(8)?;r.text(text(field(v,1)?)?)?;},
"output-preview"=>{r.cells(16)?;for path in list(field(v,2)?)?{r.cells(24)?;r.text(text(path)?)?;}},"cluster"=>{r.cells(24)?;r.text(text(field(v,1)?)?)?;},_=>return Err(invalid())}Ok(())}
fn owned_gui<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&FlowUi)->Result<(),ValueError>{r.cells(8)?;for v in [v.camera.x,v.camera.y,v.camera.zoom]{r.float(v)?;}for(name,n)in v.nodes.iter(){r.cells(24)?;r.text(name)?;r.float(n.layout.x)?;r.float(n.layout.y)?;match &n.chrome{NodeChrome::Plain{..}=>{r.text("plain")?;r.cells(16)?;},NodeChrome::Slider{label,min,max,step,value}=>{r.text("slider")?;r.cells(8)?;r.text(label)?;for v in [min,max,step,value]{r.float(*v)?;}},NodeChrome::Note{text}=>{r.text("note")?;r.cells(8)?;r.text(text)?;},NodeChrome::Image{src}=>{r.text("image")?;r.cells(8)?;r.text(src)?;},NodeChrome::Variable{name,schema}=>{r.text("variable")?;r.cells(8)?;r.texts([name.as_str(),schema.as_str()])?;}}}for p in &v.previews{r.cells(32)?;r.texts([p.id.as_str(),p.mode.as_str()])?;if let Some(s)=&p.source{r.texts([s.neuron.as_str(),s.channel.as_str()])?;}if let Some(v)=&p.layout{r.float(v.x)?;r.float(v.y)?;}for path in p.expanded.iter(){r.cells(24)?;r.text(path)?;}}Ok(())}
fn borrowed_gui<C:NativeSchemaControl>(r:&mut Rows<'_,C>,v:&DslValue)->Result<(),ValueError>{r.cells(8)?;dynamic_floats(r,member(v,"camera")?,&["x","y","zoom"])?;for(name,n)in object(member(v,"nodes")?)?{r.cells(24)?;r.text(name)?;dynamic_floats(r,member(n,"layout")?,&["x","y"])?;let c=member(n,"chrome")?;let kind=dynamic_text(member(c,"kind")?)?;r.text(kind)?;match kind{"plain"=>r.cells(16)?,"slider"=>{r.cells(8)?;r.text(dynamic_text(member(c,"label")?)?)?;dynamic_floats(r,c,&["min","max","step","value"])?;},"note"|"image"=>{r.cells(8)?;r.text(dynamic_text(member(c,if kind=="note"{"text"}else{"src"})?)?)?;},"variable"=>{r.cells(8)?;r.text(dynamic_text(member(c,"name")?)?)?;r.text(dynamic_text(member(c,"schema")?)?)?;},_=>return Err(invalid())}}for p in array(member(v,"previews")?)?{r.cells(32)?;r.text(dynamic_text(member(p,"id")?)?)?;r.text(dynamic_text(member(p,"mode")?)?)?;let s=member(p,"source")?;if !matches!(s,DslValue::Null){r.text(dynamic_text(member(s,"neuron")?)?)?;r.text(dynamic_text(member(s,"channel")?)?)?;}let layout=member(p,"layout")?;if !matches!(layout,DslValue::Null){dynamic_floats(r,layout,&["x","y"])?;}for path in array(member(p,"expanded")?)?{r.cells(24)?;r.text(dynamic_text(path)?)?;}}Ok(())}
`;
const nativePath=join(root,"🦀️.rs"),nativeBefore=readFileSync(nativePath,"utf8");let nativeAfter=nativeBefore.replaceAll("let maximum=control.limits().max_rows;","let maximum=control.limits().max_rows;let maximum_bytes=control.limits().max_value_bytes;").replace("rows::owned(document,maximum,c)?","rows::owned(document,maximum,maximum_bytes,c)?").replace("rows::borrowed(record,maximum,c)?","rows::borrowed(record,maximum,maximum_bytes,c)?");
assert.notEqual(nativeAfter,nativeBefore);
const encodeStart=nativeAfter.indexOf("fn encode_record_native("),decodeStart=nativeAfter.indexOf("fn decode_record_native("),publicStart=nativeAfter.indexOf("pub(crate) fn encode(");
assert(encodeStart>=0&&decodeStart>encodeStart&&publicStart>decodeStart);
nativeAfter=nativeAfter.slice(0,encodeStart)+String.raw`fn encode_record_native(encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,construct:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::RecordValue,ValueError>,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};
 let Some(prefix)=CONTROLLED_PACK_PREFIX.filter(|_|matches!(encoding,SnapshotEncoding::Binary))else{return store::encode_sqlite_snapshot_record_native(encoding,CONTROLLED_ENVELOPE_ID,ControlledSnapshotDsl::__dsl_spec_producer(),construct,control)};
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
 let body_limit=limits.max_file_bytes.checked_sub(prefix.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation file cannot contain its declared discriminator"))?;
 control.allocation_stage(SqliteSnapshotPhase::EncodeNative,|remaining,checkpoint|{
  let mut progress=|p:semio_framework_value::native_encoding::NativeEncodeProgress|checkpoint(p.completed,p.total);
  let mut native=NativeEncodeControl::new(remaining,&mut progress);
  let result=(||{
   let spec=ControlledSnapshotDsl::__dsl_spec_producer().encode(&mut native)?;
   let record=semio_framework_dsl_record::native_encoding::EncodedRecord::from_record(construct(&mut native)?);
   let mut options=store::PackEncodeOptions::default();options.limits.max_file_len=body_limit as u64;
   let body=pack::record::encode_document_controlled(&spec,record.as_record(),&options,&mut native).map_err(pack_error)?;
   let length=prefix.len().checked_add(body.len()).filter(|n|*n<=limits.max_file_bytes).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation snapshot output exceeds file byte limit"))?;
   let mut output=native.allocate_vec(length)?;native.begin_stage(length)?;output.extend_from_slice(prefix);native.advance(prefix.len())?;for chunk in body.chunks(65536){output.extend_from_slice(chunk);native.advance(chunk.len())?;}
   Ok(store::io_schema::IoPayload::Binary(output))
  })();(result,native.owned_bytes())
 })?
}
fn decode_record_native(construct:impl FnOnce(&semio_framework_dsl_record::RecordValue,&mut NativeDecodeControl<'_>)->Result<ControlledSnapshot,ValueError>,payload:&store::io_schema::IoPayload,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<ControlledSnapshot,ValueError>{
 use semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotPhase;
 let(store::io_schema::IoPayload::Binary(bytes),Some(prefix))=(payload,CONTROLLED_PACK_PREFIX)else{return store::decode_sqlite_snapshot_record_native(payload,CONTROLLED_ENVELOPE_ID,ControlledSnapshotDsl::__dsl_spec_producer(),construct,control)};
 let limits=control.limits();control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,bytes.len())?;
 if bytes.len()>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Generation snapshot input exceeds file byte limit"))}
 let body=bytes.strip_prefix(prefix).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Generation pack discriminator mismatch"))?;
 control.allocation_stage(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let mut progress=|p:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(p.completed,p.total);
  let mut native=NativeDecodeControl::new(remaining,&mut progress);
  let result=(||{let spec=ControlledSnapshotDsl::__dsl_spec_producer().decode(&mut native)?;let(record,_)=pack::record::decode_document_controlled(body,&spec,&store::PackDecodeOptions::default(),&mut native).map_err(pack_error)?;construct(&record,&mut native)})();
  (result,native.owned_bytes())
 })?
}
`+nativeAfter.slice(publicStart);
const reconstructionPath=join(root,"../📥️reconstruction/🦀️.rs"),reconstructionBefore=readFileSync(reconstructionPath,"utf8");
const reconstructionAfter=reconstructionBefore.replace("artifact::{FloatRow,Reconstruction}","artifact::FloatRow")+String.raw`
struct Reconstruction<'c,'p>{control:&'c mut SqliteSnapshotControl<'p>,units:usize}
impl<'c,'p> Reconstruction<'c,'p>{
 fn new(control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;Ok(Self{control,units:0})}
 fn checkpoint(&mut self)->Result<(),ValueError>{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.units,0)}
 fn scalar(&mut self)->Result<(),ValueError>{self.units=self.units.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Generation relationship traversal overflow"))?;self.checkpoint()}
 fn text(&mut self,text:&str)->Result<String,ValueError>{self.control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint|{let mut progress=|p:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(p.completed,p.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining,&mut progress);let result=native.copy_text(text);(result,native.owned_bytes())})?}
 fn blob(&mut self,bytes:&[u8])->Result<Vec<u8>,ValueError>{self.control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,checkpoint|{let mut progress=|p:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(p.completed,p.total);let mut native=semio_framework_value::NativeDecodeControl::new(remaining,&mut progress);let result=native.copy_bytes(bytes);(result,native.owned_bytes())})?}
}
`;
assert.notEqual(reconstructionBefore,reconstructionAfter);
writeFileSync(join(import.meta.dir,"held-pairs.json"),JSON.stringify([{path,before,after},{path:nativePath,before:nativeBefore,after:nativeAfter},{path:reconstructionPath,before:reconstructionBefore,after:reconstructionAfter}],null,2)+"\n");
console.log("[DEBUG] Generation exact typed and borrowed semantic provider held paths=3 production_mutations=0");
