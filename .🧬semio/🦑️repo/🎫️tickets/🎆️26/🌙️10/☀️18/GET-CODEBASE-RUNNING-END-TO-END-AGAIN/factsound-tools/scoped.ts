/** 📏️ Measures, on a sealed memo record of the live tree, the relevance of the whole generation (base) against the relevance of the owners the record's pin covers (+ @env + derived supports): rows, rechecked rows, stale rows by the engine's own check, CPU. Usage: bun scoped.ts <memo dir> <record key prefix> <root> */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [directory,prefix,root]=process.argv.slice(2) as [string,string,string];
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {observationsHold}=await import(L+"/👁️observation/🟨️.mjs");
const generation=await import(L+"/🕸️graph/🕰️generations/🟨️.mjs");
const {scopeWithDerivedV1}=await import(L+"/🕸️graph/🕰️generations/✂️cutoff/🟨️.mjs");
const {decodeVerifiedScopeV1}=await import(L+"/🕸️graph/🕰️generations/🔭️coverage/🟨️.mjs");
const {buildGraphMemoRelevanceV1}=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/📤️relevance/🟦️.ts");
const {memoExcluderV1}=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🟦️.ts");
const {readChangeStateV1}=await import(root+"/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts");
const rss=()=>Math.round(process.memoryUsage().rss/1048576),cpu=()=>{const usage=process.cpuUsage();return (usage.user+usage.system)/1e6;};
const record=JSON.parse(readFileSync(join(directory,prefix+".json"),"utf8")),pin=JSON.parse(record.pin);
const engine=generation.createGenerationStoreV1({root:join(root,".cache/b/g/m/generations"),fs:generation.createNodeFilePortV1(),holds:async()=>true,resources:{facts:"probe"}});
const manifest=await engine.readManifestAt(record.manifest);if(!manifest)throw Error("manifest retired");
const frontier=await engine.readFrontier(manifest.frontier),index=await engine.readIndexView(manifest.factIndex);
const owners=Object.keys(manifest.owners).sort(generation.compareCodePointsV1),covered=decodeVerifiedScopeV1(owners,pin.covered);
const state=readChangeStateV1(join(directory,".."));if(!state)throw Error("no journal state");
const excluded=memoExcluderV1(state.exclude,true),isLink=async()=>false;
for(const mode of ["whole","slice"]){
 const started=cpu(),ids=mode==="whole"?undefined:generation.selectFactClassesV1(index,scopeWithDerivedV1(index,new Set([...covered,generation.GENERATION_POLICY_V1.envOwner]))).ids;
 const rows=function*(){if(ids)for(const id of ids)yield index.rows[id];else for(let at=0;at<index.rows.length;at++)yield index.rows[at];};
 const built=await buildGraphMemoRelevanceV1({root,rows,frontier,excluded,isLink});
 const sealCpu=cpu()-started,checkStarted=cpu();
 let stale=0;const chunks=built.recheck.rows;
 for(let at=0;at<chunks.length;at+=256){const chunk=chunks.slice(at,at+256);if(await observationsHold(chunk,64,new Set(),true))continue;for(const row of chunk)if(!await observationsHold([row],1,new Set(),true))stale++;}
 console.log("[DEBUG]",mode,JSON.stringify({covered:covered.size,selected:ids?.length??index.rows.length,leaves:built.rows,directories:built.directories.size,recheckRows:built.recheck.rows.length,identities:built.recheck.identities.length,stale,unsealable:built.unsealable,sealCpu:+sealCpu.toFixed(2),recheckCpu:+(cpu()-checkStarted).toFixed(2),rss:rss()}));
}
