const {run}=require("./native-current-validation.cjs"),{readOrdinaryEvidenceV1}=require("./native-evidence.cjs"),crypto=require("node:crypto"),fs=require("node:fs"),path=require("node:path");
const prefix=process.argv[2]||"native-layout";if(!/^[a-z0-9-]+$/.test(prefix))throw Error("Bounded evidence label required");
const argv=["--repo=/workspaces/semio","--format=json","search","linux","--limit=8"];
const searchSchema=JSON.parse(fs.readFileSync(path.join(process.cwd(),"🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/🧬️schema/🔣️.json"),"utf8")),storage=searchSchema.$defs.StorageProtocol.const,snapshotPolicy=searchSchema.$defs.SnapshotCacheProtocol.const,validateShards=new(require("ajv"))({strict:true}).compile({...searchSchema.$defs.ShardManifest,$defs:{Shard:searchSchema.$defs.Shard}});
const cache=path.join("/workspaces/semio/.🧬semio/🦑️repo/⚡️cache",crypto.createHash("sha256").update("/workspaces/semio").digest("hex"));
let observedBytes=0,nextProgress=16777216;
async function evidence(file,name,maxBytes,signal,capture=false){
 let previous=0;
 return readOrdinaryEvidenceV1(file,name,{maxBytes,signal,capture,onProgress:({bytes})=>{observedBytes+=bytes-previous;previous=bytes;if(observedBytes>=nextProgress){console.log("[DEBUG] native-evidence-bytes "+observedBytes);nextProgress=observedBytes+16777216}}});
}
async function observe(run){
 const controller=new AbortController(),abort=()=>controller.abort();process.once("SIGINT",abort);
 try{return await run(controller.signal)}finally{process.removeListener("SIGINT",abort)}
}
async function snapshot(){return observe(async signal=>{
 const directory=path.join(cache,"tree/snapshot"),rows=[];
 for(const name of ["meta.json","payload.json.gz"]){
  const file=path.join(directory,name);let exists=true;
  try{await fs.promises.lstat(file)}catch(error){if(error.code!=="ENOENT")throw error;exists=false}
  if(!exists){rows.push({name,absent:true});continue}
  rows.push(await evidence(file,name,name==="meta.json"?snapshotPolicy.maximumMetadataBytes:snapshotPolicy.maximumCompressedBytes,signal));
 }
 return rows;
})}
async function indexState(expectedCount){return observe(async signal=>{
 const directory=path.join(cache,"index.search"),manifestEvidence=await evidence(path.join(directory,"manifest.json"),"manifest.json",storage.maximumManifestBytes,signal,true),{content,...manifestRow}=manifestEvidence,manifest=JSON.parse(content.toString("utf8"));
 if(!validateShards(manifest)||manifest.shards.some((row,i)=>i>0&&manifest.shards[i-1].name>=row.name)||manifest.shards.reduce((sum,row)=>sum+row.documents,0)!==manifest.documents)throw Error("Closed sorted shard manifest required");
 if(expectedCount!==undefined&&(manifest.shards.length!==expectedCount||(expectedCount===64&&manifest.shards.some((row,i)=>row.name!=="s"+String(i).padStart(2,"0")))))throw Error("Full workload shard coverage mismatch");
 const rows=[await evidence(path.join(directory,"meta.json"),"meta.json",storage.maximumManifestBytes,signal),manifestRow];
 for(const shard of manifest.shards){
  const name=shard.name+"/events.jsonl",row=await evidence(path.join(directory,name),name,storage.maximumShardBytes,signal);
  if(row.bytes!==String(shard.bytes)||row.sha256!==shard.sha256)throw Error("Shard bytes differ from admitted manifest: "+shard.name);
  rows.push(row);
 }
 return rows;
})}
(async()=>{
 const before=await snapshot(),cancelled=await run(prefix+"-late-cancel",argv,"","identity");
 if(!cancelled.cancelled||cancelled.forced||cancelled.status!==130||cancelled.signal!==null||cancelled.stdout.trim()!=="")throw Error("Cooperative identity-stage cancellation required; no completion query launched");
 const afterCancel=await snapshot();if(JSON.stringify(before)!==JSON.stringify(afterCancel))throw Error("Cancelled discovery published a snapshot");
 const cold=await run(prefix+"-cold",argv);if(cold.status!==0||cold.signal||cold.forced)throw Error("Cold search failed; no completion retry launched");
 const afterCold=await snapshot(),indexAfterCold=await indexState(64);
 console.log("[DEBUG] physical-cold-generation "+JSON.stringify({afterCold,indexAfterCold}));
 const first=JSON.parse(cold.stdout);if(!first||typeof first!=="object")throw Error("Structured tree response required");
 const warm=await run(prefix+"-warm",argv);if(warm.status!==0||warm.signal||warm.forced)throw Error("Warm search failed");
 const afterWarm=await snapshot(),indexAfterWarm=await indexState(64),reused=JSON.stringify(afterCold)===JSON.stringify(afterWarm),indexReused=JSON.stringify(indexAfterCold)===JSON.stringify(indexAfterWarm);
 console.log("[DEBUG] physical-warm-generation "+JSON.stringify({afterWarm,indexAfterWarm,reused,indexReused}));
 if(!reused||!indexReused)throw Error("Warm physical generation not reused");
 const same=JSON.stringify(first)===JSON.stringify(JSON.parse(warm.stdout));if(!same)throw Error("Cold and warm tree responses differ");
 console.log("[DEBUG] layout-native-acceptance "+JSON.stringify({same,reused,indexReused,before,afterCancel,afterCold,afterWarm,coldMilliseconds:cold.elapsedMilliseconds,warmMilliseconds:warm.elapsedMilliseconds,stdoutBytes:Buffer.byteLength(cold.stdout),sha256:crypto.createHash("sha256").update(cold.stdout).digest("hex"),lateCancellationMilliseconds:cancelled.elapsedMilliseconds,settledAfterCancelMilliseconds:cancelled.settledAfterCancelMilliseconds}));
})().catch(error=>{console.error("[DEBUG] "+error.stack);process.exitCode=error.name==="AbortError"?130:1});
