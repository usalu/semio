import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../..");const root=resolve(ticket,"../../../../../../..");const input=import.meta.dir;
const base="✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch";
const command=process.argv[2];const pairs:{path:string,before:string,after:string}[]=[];
async function pair(path:string,after:string){const file=Bun.file(`${root}/${path}`);pairs.push({path,before:await file.exists()?await file.text():"",after});}
if(command==="stage-provider"||command==="provider"){
 const path=`${base}/🦀️.rs`;const before=await Bun.file(`${root}/${path}`).text();const anchor="impl OpBinary for SnapshotPatch {";
 if(before.split(anchor).length!==2||before.includes("mod borrowed_operation_source"))throw Error("exact original patch codec anchor");
 await pair(path,before.replace(anchor,'#[path = "📦️codec/🫳️borrowed/🦀️.rs"]\nmod borrowed_operation_source;\n\n'+anchor));
 await pair(`${base}/📦️codec/🫳️borrowed/🦀️.rs`,await Bun.file(`${input}/producer.rs`).text());
}else if(command==="fixture-preflight-join"){
 const path="✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🧪️tests/🔬️unit/🦀️.rs";
 const before=await Bun.file(`${root}/${path}`).text();
 const original="mutation: &u8, _description: Option<&str>, _lane: fixture_store::HistoryLane";
 if(before.split(original).length!==2)throw Error("exact unused probe preflight argument");
 let after=before.replace(original,"mutation: &u8, _lane: fixture_store::HistoryLane");
 const oldCall="factory.preflight(&mutation, None, fixture_store::HistoryLane::Document)";
 if(after.includes(oldCall))after=after.replace(oldCall,"factory.preflight(&mutation, fixture_store::HistoryLane::Document)");
 await pair(path,after);
}else if(command==="allocation-policy-demand"){
 const path=`${base}/🧪️tests/🦀️.rs`;const before=await Bun.file(`${root}/${path}`).text();if(before.includes("honors_paid_allocation_policy"))throw Error("own allocation demand already mounted");
 await pair(path,before+"\n"+await Bun.file(`${input}/allocation-law.rs`).text());
 const fixture=`${base}/🧫️fixtures/📦️operation-source.json`;const data=await Bun.file(`${root}/${fixture}`).json();data.refusedAllocationBytes=1024;await pair(fixture,JSON.stringify(data,null,2)+"\n");
}else if(command==="demand"){
 const path=`${base}/🧪️tests/🦀️.rs`;const before=await Bun.file(`${root}/${path}`).text();if(before.includes("paged_snapshot_patch_original_json_source_"))throw Error("own demand already mounted");
 await pair(path,before+"\n"+await Bun.file(`${input}/law.rs`).text());
 await pair(`${base}/🧫️fixtures/📦️operation-source.json`,await Bun.file(`${input}/fixture.json`).text());
}else throw Error("exact stage-provider, provider or demand required");
await Bun.write(`${input}/${command}-guarded-pairs.json`,JSON.stringify({state:command==="stage-provider"?"Held":"ExactGuardedSourceFamily",pairs},null,2)+"\n");
if(command!=="stage-provider"){for(const pair of pairs){const file=Bun.file(`${root}/${pair.path}`);if((await file.exists()?await file.text():"")!==pair.before)throw Error(`fresh guard ${pair.path}`);}for(const pair of pairs)await Bun.write(`${root}/${pair.path}`,pair.after);}
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length,productionWrites:command==="stage-provider"?0:pairs.length}));
