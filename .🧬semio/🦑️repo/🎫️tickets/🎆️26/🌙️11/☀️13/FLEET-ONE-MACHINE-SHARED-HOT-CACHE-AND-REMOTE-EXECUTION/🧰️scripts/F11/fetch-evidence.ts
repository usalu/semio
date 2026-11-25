import {brotliDecompressSync} from "node:zlib";
import {writeFileSync,mkdirSync} from "node:fs";
import {remoteCacheFetchV1,resolveAuthorisedRemoteCacheV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
const root="/Users/ueli/Documents/semio",[id,out]=[process.argv[2]!,process.argv[3]!];
const found=await resolveAuthorisedRemoteCacheV1({...process.env,SEMIO_WORKSPACE_ROOT:root} as never,{root}),r=found.resolution!;
const call=remoteCacheFetchV1(r.environment,{root}),ep=r.contract.endpoint;
const list=(await (await call(ep+"/evidence")).json()) as {evidence:any[]},row=list.evidence.find(e=>e.id===id);
mkdirSync(out,{recursive:true});
for(const f of row.files){if(!f.stored||f.path==="")continue;const res=await call(`${ep}/evidence/${id}/${f.name}`);const b=new Uint8Array(await res.arrayBuffer());let t:Uint8Array=b;try{t=brotliDecompressSync(b);}catch{}writeFileSync(`${out}/${f.side}-${f.path.replace(/\W/g,"_")}.json`,t);}
console.log("ok",row.files.length);
