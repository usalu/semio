import fs from"node:fs";
import path from"node:path";
import{createHash}from"node:crypto";
import{readNxAuthorizerMemoV1,replayNxAuthorizerMemoV1,nxAuthorizerMemoImplementationV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts";
const file="/home/deck/.cache/semio-repo/source-memo/5f78bd319d83b5ec8a1d/39b11b90e467a391dbbd.json",first=fs.statSync(file,{bigint:true}),raw=fs.readFileSync(file),memo=await readNxAuthorizerMemoV1(file);
if(!memo)throw Error("Existing canonical memo unavailable");
const current=nxAuthorizerMemoImplementationV1(),matches=memo.implementation===current;let rejectedBeforeReplay:unknown="not-probed-matching-implementation";
if(!matches){const result=await replayNxAuthorizerMemoV1(process.cwd(),memo,AbortSignal.timeout(5000));if(result!==undefined)throw Error("Mismatched implementation memo accepted");rejectedBeforeReplay=true;}
const tail=fs.statSync(file,{bigint:true}),digest=createHash("sha256").update(raw).digest("hex"),result={at:new Date().toISOString(),file,bytes:raw.length,sha256:digest,stable:[first.dev,first.ino,first.size,first.mtimeNs,first.ctimeNs].join(":")===[tail.dev,tail.ino,tail.size,tail.mtimeNs,tail.ctimeNs].join(":"),memoImplementation:memo.implementation,currentImplementation:current,matches,rejectedBeforeReplay,entry:memo.entry,sourceCount:memo.manifest.sources.length,sourceExtensionCounts:memo.manifest.sources.reduce((a:Record<string,number>,row)=>(a[path.extname(row.path)]=(a[path.extname(row.path)]??0)+1,a),{}),sourceFields:[...new Set(memo.manifest.sources.flatMap(Object.keys))],observationCount:memo.observations.length,importsCount:Object.keys(memo.imports).length,topLevelFields:Object.keys(memo),sourceBytesStored:false,executed:false};
fs.writeFileSync(path.join(import.meta.dir,"🗑️generated/source-memo-audit.json"),JSON.stringify(result,null,2)+"\n");console.log(JSON.stringify(result));
