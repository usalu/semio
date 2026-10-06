const root="/Users/ueli/Documents/semio";
const ticket=`${root}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O`;
const input=`${ticket}/📥️inputs/chart-paged-original-producer`;
const codec="🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/📦️codec/🦀️.rs";
const test="🧰️framework/🛍️products/📓️print/🧪️tests/🧬️chart-mutations/🦀️.rs";
const fixture="🧰️framework/🛍️products/📓️print/🧫️fixtures/🧬️chart-mutations/📦️operation-pages.json";
const command=process.argv[2];
const pairs:{path:string,before:string,after:string}[]=[];
async function pair(path:string,after:string){pairs.push({path,before:await Bun.file(`${root}/${path}`).exists()?await Bun.file(`${root}/${path}`).text():"",after});}
if(command==="demand"){
 await pair(fixture,JSON.stringify({path:["title"],word:"x",payloadBytes:8194,header:[1,1],metadataBytes:512,allocationBytes:65536,maximumCloseSteps:8322,maximumCloseItems:1,maximumCloseBytes:4096},null,2)+"\n");
 const before=await Bun.file(`${root}/${test}`).text();
 await pair(test,before+"\n"+await Bun.file(`${input}/law.rs`).text());
}else if(command==="fixture-join"){
 const before=await Bun.file(`${root}/${test}`).text();
 if(!before.includes("let mut cancel=|work|work<1024;")||!before.includes("ValueRefusalKind::Cancelled"))throw Error("exact owning fixture prerequisite guard");
 await pair(test,before.replace("let mut cancel=|work|work<1024;","let mut cancel=|work:semio_framework_value::NativeEncodeProgress|work.completed<1024;").replace("ValueRefusalKind::Cancelled","ValueRefusalKind::Canceled"));
}else if(command==="progress-type-join"){
 const before=await Bun.file(`${root}/${test}`).text();
 const old="work:semio_framework_value::NativeEncodeProgress";
 if(before.split(old).length!==2)throw Error("exact progress authority guard");
 await pair(test,before.replace(old,"work:semio_framework_value::native_encoding::NativeEncodeProgress"));
}else if(command==="typed-refusal-join"){
 const before=await Bun.file(`${root}/${test}`).text();
 const old="    let mut short=options.clone();short.limits.max_file_len-=1;";
 if(before.split(old).length!==2)throw Error("exact refusal source guard");
 let after=before.replace(old,"    let kind=|error:protocol::ProtocolError|match error{protocol::ProtocolError::Pack(protocol::PackError::Refusal(refusal))=>refusal.kind(),error=>panic!(\"expected genuine typed Pack refusal, got {error:?}\")};\n"+old);
 for(const policy of ["short","options"]){const old=`operation.encode_op_into(&${policy},&mut prefix,&mut encoding).unwrap_err().kind()`;if(after.split(old).length!==2)throw Error("exact refusal category guard");after=after.replace(old,`kind(operation.encode_op_into(&${policy},&mut prefix,&mut encoding).unwrap_err())`);}
 await pair(test,after);
}else if(command==="stage-provider"||command==="provider"){
 const old=JSON.parse(await Bun.file(`${ticket}/📥️inputs/operation-byte-pages/chart-paged-current-source-owner-held-pair.json`).text()).pairs[0];
 const before=await Bun.file(`${root}/${codec}`).text();
 if(before!==old.before)throw Error("current Chart ordinary codec differs from exact audited source");
 let after=old.after.replace(" fn encode_op_into(","}\nimpl ChangeChartValue{\n /// ✍️ Streams the original authored chart fields through the caller's complete operation policy.\n pub fn encode_op_into(").replace(" fn decode_op_span("," /// 🫳️ Decodes the same immutable source and verifies canonical bytes under caller controls.\n pub fn decode_op_span(");
 after=after.replace("  let record=self.__dsl_to_record_controlled(control).map_err(protocol::PackRefusal::from)?;\n","").replace("pack::record::encode_record_body_into(&spec,&record,options,output,control)?;","pack::record::encode_projected_record_body_into(&spec,self,options,output,control)?;");
 await pair(codec,after);
}else throw Error("exact demand, stage-provider or provider command required");
await Bun.write(`${input}/${command}-guarded-pairs.json`,JSON.stringify({state:command==="stage-provider"?"HeldOriginalProducer":"ExactGuardedFamily",pairs},null,2)+"\n");
if(command!=="stage-provider"){
 for(const p of pairs){const file=Bun.file(`${root}/${p.path}`);const actual=await file.exists()?await file.text():"";if(actual!==p.before)throw Error(`fresh guard ${p.path}`);}
 for(const p of pairs)await Bun.write(`${root}/${p.path}`,p.after);
}
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length,productionWrites:command==="stage-provider"?0:pairs.length}));
