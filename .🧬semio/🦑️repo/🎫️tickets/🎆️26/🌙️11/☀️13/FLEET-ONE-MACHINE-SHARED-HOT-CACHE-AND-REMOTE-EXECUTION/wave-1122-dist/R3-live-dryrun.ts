import {stat} from "node:fs/promises";
import {join} from "node:path";
import {casRootV1,createCasStoreV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {scanRemoteCacheDirectoryV1,planRemoteCacheBlobBoundV1,judgeRemoteCacheStalenessV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🧹️collector/🟦️.ts";
import {scanRemoteCacheMetaV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🏷️retention/🟦️.ts";
import {openTreeDigestsV1,stalenessReportV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🏷️retention/🍂️staleness/🟦️.ts";
import {remoteCacheBlobBoundV1} from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";

const workspace="/Users/ueli/Documents/semio",root=join(workspace,".cache/b/p/m/fleet/cache-server/v1"),host=createCasStoreV1({root:casRootV1(process.env)});
const index=await scanRemoteCacheDirectoryV1(root,{concurrency:4}),metas=await scanRemoteCacheMetaV1(root,{has:key=>index.has(key)},undefined,8);
const now=Date.now(),tree=await openTreeDigestsV1(workspace);
const verdicts=await judgeRemoteCacheStalenessV1(index,metas,{now,graceMilliseconds:600000,tree});
const report=stalenessReportV1({mode:"dry-run",now,verdicts,retired:0,refused:{},examples:3});
const kinds:Record<string,number>={};for(const key of index.keys()){const kind=key.split("/")[1]!;kinds[kind]=(kinds[kind]??0)+1;}
const sizes=new Map<string,number>();const sizeOf=(hash:string)=>sizes.get(hash)??0;
for(const [key,meta] of metas)if(key.split("/")[1]!=="cas")for(const hash of meta.refs)if(!sizes.has(hash))sizes.set(hash,await stat(host.objectPath(hash)).then(info=>info.size,()=>0));
const bound=(await remoteCacheBlobBoundV1(host,50))!;
const rows=[...metas].filter(([key])=>key.split("/")[1]!=="cas"&&index.has(key)).map(([key,meta])=>{const state=verdicts.get(key)?.state;return {key,accessed:index.get(key)!.accessed,refs:meta.refs,tier:state==="unreached"?0:state===undefined||state==="undeclared"?1:2,guarded:now-meta.storedAt<600000};});
const plan=planRemoteCacheBlobBoundV1(rows,sizeOf,bound);
const byTier:Record<number,number>={};for(const key of plan.keys){const t=rows.find(row=>row.key===key)!.tier;byTier[t]=(byTier[t]??0)+1;}
const gib=(n:number)=>Math.round(n/1073741824*100)/100;
console.log(JSON.stringify({entries:index.size,byKind:kinds,sidecars:metas.size,report:{...report,examples:undefined},referencedBytesGiB:gib(plan.before),boundGiB:gib(bound),targetGiB:gib(Math.floor(bound*0.9)),afterGiB:gib(plan.after),wouldEvictEntries:plan.keys.length,wouldEvictByTier:byTier,guardedEntries:rows.filter(row=>row.guarded).length},null,1));
