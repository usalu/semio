import {readFileSync,writeFileSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env),f=remoteCacheFetchV1(env,{root}),base=`${s.endpoint}/${s.instance}`;
const rows=JSON.parse(readFileSync(root+"/.cache/f/cache-server/fleet/divergences.json","utf8")).rows.filter(r=>r.atMs>1794786360000);
const prov=readFileSync(root+"/.cache/f/cache-server/fleet/provenance.jsonl","utf8").split("\n").filter(Boolean).slice(-400000).map(l=>{try{return JSON.parse(l)}catch{return null}}).filter(Boolean);
const blob=async h=>{const r=await f(`${base}/cas/${h}`);return r.status===200?new Uint8Array(await r.arrayBuffer()):undefined;};
const out=[];
for(const r of rows.slice(0,Number(process.argv[2]??6))){
  const a=await f(`${base}/ac/${r.key}`);const kept=decodeProtobufV1(REAPI_V1.actionResult,new Uint8Array(await a.arrayBuffer()));
  const files=Object.fromEntries((kept.output_files??[]).map(x=>[x.path,x.digest.hash]));
  const cand=prov.filter(p=>p.k==="cas"&&p.o===r.refusedOrigin&&p.t>r.atMs-900000&&p.t<=r.atMs+1000);
  const keptRecord=files["record.br"]?JSON.parse(Buffer.from(brotliDecompressSync(await blob(files["record.br"]))).toString()):undefined;
  const info={key:r.key.slice(0,12),kept:r.keptOrigin,refused:r.refusedOrigin,keptFiles:Object.keys(files).length,candidates:cand.length};
  for(const c of cand){const b=await blob(c.y);if(!b)continue;let j;try{j=JSON.parse(Buffer.from(brotliDecompressSync(b)).toString());}catch{continue;}
    if(j&&keptRecord&&j.image===keptRecord.image&&typeof j.version==="number"&&("rows" in j)&&c.y!==files["record.br"]){info.refusedBlob=c.y.slice(0,12);info.sameDigest=j.digest===keptRecord.digest;info.sameImage=j.image===keptRecord.image;
      const ks=JSON.stringify(keptRecord),rs=JSON.stringify(j);info.keptBytes=ks.length;info.refusedBytes=rs.length;
      let i=0;while(i<ks.length&&ks[i]===rs[i])i++;info.firstDiff=i;info.keptCtx=ks.slice(Math.max(0,i-120),i+160);info.refusedCtx=rs.slice(Math.max(0,i-120),i+160);
      writeFileSync(`div-${r.key.slice(0,8)}-kept.json`,ks);writeFileSync(`div-${r.key.slice(0,8)}-refused.json`,rs);}}
  out.push(info);console.log(JSON.stringify(info,null,0).slice(0,1500));
}
