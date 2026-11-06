import{readFileSync,writeFileSync,renameSync}from"node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/📤️turn-output/🦀️.rs",old=readFileSync(file,"utf8");let next=old;
function replace(a:string,b:string){if(!next.includes(a))throw Error("missing "+a);next=next.replace(a,b)}
replace("input:Option<semio_framework_actor::RetainedTurnInput>,binding:Option<OriginalSourceBinding>,capture:","input:Option<semio_framework_actor::RetainedTurnInput>,binding:Option<OriginalSourceBinding>,home_address:usize,capture:");
replace("Self{input:None,binding:None,capture:","Self{input:None,binding:None,home_address:0,capture:");
replace("pub fn encoded(&self)->Option<&[u8]>{self.encoder.output_is_ready(&self.encoded)","pub fn encoded(&self)->Option<&[u8]>{(self.home_address==self.destination()&&self.encoder.output_is_ready(&self.encoded))");
replace("self.input.is_none()&&self.binding.is_none()&&self.capture.terminal_is_empty()","self.input.is_none()&&self.binding.is_none()&&self.home_address==0&&self.capture.terminal_is_empty()");
replace("+size_of::<Option<OriginalSourceBinding>>()}\n fn destination", "+size_of::<Option<OriginalSourceBinding>>()+size_of::<usize>()}\n fn destination");
replace("if self.binding.is_some_and(|binding|!binding.matches(cx))", "if self.home_address!=0&&self.home_address!=self.destination()||self.binding.is_some_and(|binding|!binding.matches(cx))");
replace("self.capture.settle_return(cx)}", "{self.capture.qualify_sized(cx,self.destination(),Self::capture_bytes())?;self.capture.settle_return(cx)}}");
replace("self.input=Some(input);self.binding=Some(binding);", "self.input=Some(input);self.binding=Some(binding);self.home_address=destination;");
replace("if !self.capture.terminal_is_empty(){self.capture.settle_return(cx)?;return Ok(false)}", "if !self.capture.terminal_is_empty(){if self.capture.is_admitted(){self.capture.qualify_sized(cx,self.destination(),Self::capture_bytes())?}self.capture.settle_return(cx)?;return Ok(false)}");
replace("+size_of::<Option<OriginalSourceBinding>>(),depth:1", "+size_of::<Option<OriginalSourceBinding>>()+size_of::<usize>(),depth:1");
replace("self.input=None;self.binding=None;RetainedWorkOutcome", "self.input=None;self.binding=None;self.home_address=0;RetainedWorkOutcome");
if(readFileSync(file,"utf8")!==old)throw Error("concurrent output edit");const temp=import.meta.dir+"/🗑️generated/output-address.tmp";writeFileSync(temp,next);renameSync(temp,file);console.log("Bound complete retained output lifetime to actual prepaid destination");
