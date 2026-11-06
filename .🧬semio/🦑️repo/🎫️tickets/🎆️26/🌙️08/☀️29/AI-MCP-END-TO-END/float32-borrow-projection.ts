import{readFileSync,writeFileSync,renameSync}from"node:fs";
const root="🧰️framework/🔨️modules/🎒️pack/🔤️json",tests=root+"/👁️borrow/🧪️tests",ticket=import.meta.dir;
function edit(path:string,change:(s:string)=>string){const old=readFileSync(path,"utf8"),next=change(old);if(next===old)throw Error("unchanged "+path);if(readFileSync(path,"utf8")!==old)throw Error("concurrent write "+path);const temp=ticket+"/🗑️generated/float32-"+Date.now()+".tmp";writeFileSync(temp,next);renameSync(temp,path)}
edit(tests+"/🔣️.json",s=>{const f=JSON.parse(s);f.cases.push({kind:"float32",input:0.1,expected:"0.1"});return JSON.stringify(f)});
edit(tests+"/🧬️.json",s=>{const f=JSON.parse(s);f.properties.cases.items.properties.kind.enum.push("float32");return JSON.stringify(f)});
edit(tests+"/🦀️.rs",s=>s+`
#[test]
fn borrowed_binary32_preserves_its_actual_shortest_precision(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let expected=fixture["cases"].as_array().unwrap().iter().find(|v|v["kind"]=="float32").unwrap()["expected"].as_str().unwrap();case(&0.1f32,expected);
 for value in[-0.0f32,0.0,1.0,-1.25,1e-7,1e13,f32::MIN_POSITIVE,f32::MAX,f32::INFINITY,f32::NAN]{case(&value,&serde_json::to_string(&value).unwrap())}
}
`);
edit(root+"/🦀️.rs",s=>{const a="use semio_framework_pack_json_float::{write_float_to, ScalarText};";if(!s.includes(a))throw Error("float imports moved");return s.replace(a,"use semio_framework_pack_json_float::{write_float_to,write_float32_to, ScalarText};")});
edit(root+"/🎛️controlled-and-borrowed-bridge/🦀️.rs",s=>{const replacements=[
 ["Number(semio_framework_value::Number),QuotedUnsigned(u128)","Number(semio_framework_value::Number),Float32(f32),QuotedUnsigned(u128)"],
 [" fn quoted_unsigned(",` fn float32(&mut self,value:f32,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{control.checkpoint()?;let mut scalar=ScalarText::new();write_float32_to(value,&mut scalar).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"controlled binary32 JSON scalar overflow"))?;self.raw(scalar.text(),control)?;control.checkpoint()}
 fn quoted_unsigned(`],
 ["JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,","JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,JsonWriteNode::Float32(value)=>JsonWriteOutput::float32(&mut measured,value,&mut control)?,"],
 ["JsonWriteNode::Number(value) => writer.number(value, control)?,","JsonWriteNode::Number(value) => writer.number(value, control)?,\n                    JsonWriteNode::Float32(value) => writer.float32(value,control)?,"]
 ];for(const[a,b]of replacements){if(!s.includes(a))throw Error("missing writer seam "+a);s=s.replace(a,b)}return s});
edit(root+"/👁️borrow/🦀️.rs",s=>{const a="JsonWriteNode::Number(Number::Float(f64::from(*self)))";if(!s.includes(a))throw Error("f32 projection moved");return s.replace(a,"JsonWriteNode::Float32(*self)")});
console.log("Authored binary32 wire law and direct native precision projection");
