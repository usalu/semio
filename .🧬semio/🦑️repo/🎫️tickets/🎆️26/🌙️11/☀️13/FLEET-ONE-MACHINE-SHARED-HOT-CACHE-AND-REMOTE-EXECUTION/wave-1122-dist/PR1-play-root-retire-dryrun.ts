/** 🔍️ Read-only dry-run of the unit-cache retirement for one root: verdicts and the bytes only doomed entries reference. Writes only its digest memo under the ticket's generated folder. */
import {statSync} from "node:fs";
import {join} from "node:path";
import {scanUnitDeclarationsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts";
import {judgeStalenessV1,openTreeDigestsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {casRootV1,createCasStoreV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts";
const [root,workspace,memo]=process.argv.slice(2) as [string,string,string];
const rows=await scanUnitDeclarationsV1(root),tree=await openTreeDigestsV1(workspace,memo);
const verdicts=await judgeStalenessV1(rows.map(r=>({key:r.key,storedAt:r.storedAt,refs:r.refs,declaration:r.declaration})),{now:Date.now(),graceMilliseconds:10*60000,digest:tree.digest});
await tree.flush().catch(()=>undefined);
const cas=createCasStoreV1({root:casRootV1()}),size=(sha:string)=>{try{return statSync(cas.objectPath(sha)).size;}catch{return 0;}};
const states:Record<string,number>={},versions:Record<string,number>={},reasons:Record<string,number>={};
const doomed=new Set<string>(),live=new Set<string>();
for(const r of rows){const v=verdicts.get(r.key)!;states[v.state]=(states[v.state]??0)+1;versions[String(r.declaration?.version??"none")]=(versions[String(r.declaration?.version??"none")]??0)+1;if(v.reason)reasons[v.reason]=(reasons[v.reason]??0)+1;
 if(v.state==="stale"||v.state==="unreached")r.refs.forEach(s=>doomed.add(s));else r.refs.forEach(s=>live.add(s));}
let only=0,all=0;for(const s of doomed){const b=size(s);all+=b;if(!live.has(s))only+=b;}
let held=0;for(const s of new Set(rows.flatMap(r=>r.refs)))held+=size(s);
console.log(JSON.stringify({root,entries:rows.length,declarationVersions:versions,states,reasons,heldGiB:+(held/2**30).toFixed(2),doomedRefsGiB:+(all/2**30).toFixed(2),freedIfRetiredGiB:+(only/2**30).toFixed(2)},null,1));
