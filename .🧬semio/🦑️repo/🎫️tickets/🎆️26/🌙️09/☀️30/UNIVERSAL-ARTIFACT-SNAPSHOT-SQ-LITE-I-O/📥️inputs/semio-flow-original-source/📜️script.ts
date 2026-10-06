import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../..");const root=resolve(ticket,"../../../../../../..");const input=import.meta.dir;
const base="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations";
const command=process.argv[2];const pairs:{path:string,before:string,after:string}[]=[];
async function pair(path:string,after:string){const file=Bun.file(`${root}/${path}`);pairs.push({path,before:await file.exists()?await file.text():"",after});}
if(command==="stage-provider"||command==="provider"){
 const path=`${base}/🦀️.rs`;const before=await Bun.file(`${root}/${path}`).text();const anchor="impl OpBinary for SemioFlowMutation {";
 if(before.split(anchor).length!==2||before.includes("mod borrowed_operation_source"))throw Error("exact original Flow codec anchor");
 await pair(path,before.replace(anchor,'#[path = "📦️codec/🫳️borrowed/🦀️.rs"]\nmod borrowed_operation_source;\n\n'+anchor));
 await pair(`${base}/📦️codec/🫳️borrowed/🦀️.rs`,await Bun.file(`${input}/producer.rs`).text());
}else if(command==="demand"){
 const path=`${base}/🧪️tests/🔬️unit/🦀️.rs`;const before=await Bun.file(`${root}/${path}`).text();if(before.includes("paged_flow_original_source_"))throw Error("own demand already mounted");
 await pair(path,before+"\n"+await Bun.file(`${input}/law.rs`).text());
 await pair(`${base}/🧫️fixtures/📦️operation-source.json`,await Bun.file(`${input}/fixture.json`).text());
}else throw Error("exact stage-provider, provider or demand required");
await Bun.write(`${input}/${command}-guarded-pairs.json`,JSON.stringify({state:command==="stage-provider"?"Held":"ExactGuardedSourceFamily",pairs},null,2)+"\n");
if(command!=="stage-provider"){for(const pair of pairs){const file=Bun.file(`${root}/${pair.path}`);if((await file.exists()?await file.text():"")!==pair.before)throw Error(`fresh guard ${pair.path}`);}for(const pair of pairs)await Bun.write(`${root}/${pair.path}`,pair.after);}
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length,productionWrites:command==="stage-provider"?0:pairs.length}));
