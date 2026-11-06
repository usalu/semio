import{readFileSync,writeFileSync,renameSync}from"node:fs";
const ticket=import.meta.dir;
function edit(p:string,fn:(s:string)=>string){const before=readFileSync(p,"utf8"),after=fn(before);const temp=ticket+"/🗑️generated/quoted-"+process.pid;if(before===after)throw Error("No change "+p);writeFileSync(temp,after);if(readFileSync(p,"utf8")!==before)throw Error("Concurrent edit "+p);renameSync(temp,p);}
edit("🧰️framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs",s=>s.replace("Number(semio_framework_value::Number),String", "Number(semio_framework_value::Number),QuotedUnsigned(u128),String").replace("JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,_=>", "JsonWriteNode::Number(value)=>measured.number(value,&mut control)?,JsonWriteNode::QuotedUnsigned(value)=>JsonWriteOutput::quoted_unsigned(&mut measured,value,&mut control)?,_=>").replace(" fn number(&mut self,number:",` fn quoted_unsigned(&mut self,mut value:u128,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ValueError>{
  let mut bytes=[b'"';41];let mut start=40;
  loop{start-=1;bytes[start]=b'0'+(value%10)as u8;value/=10;if value==0{break}}
  start-=1;bytes[start]=b'"';let text=std::str::from_utf8(&bytes[start..]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"quoted unsigned scalar is not ASCII"))?;
  self.raw(text,control)
 }
 fn number(&mut self,number:`).replace("JsonWriteNode::Number(value) => writer.number(value, control)?,","JsonWriteNode::Number(value) => writer.number(value, control)?,\n                    JsonWriteNode::QuotedUnsigned(value) => writer.quoted_unsigned(value, control)?,"));
edit("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/📦️packages/🦀️rust/Cargo.toml",s=>s.replace("semio-os-kernel = { workspace = true }\n", "").replace("[dependencies]\n", "[dependencies]\nsemio-os-kernel = { workspace = true }\nsemio-framework-kernel-identity = { path = \"../../../../../../../../🔨️modules/🎠️kernel/🪪️identity/📦️packages/🦀️rust\" }\n"));
console.log("Installed bounded quoted unsigned scalar and Poll dependencies");
