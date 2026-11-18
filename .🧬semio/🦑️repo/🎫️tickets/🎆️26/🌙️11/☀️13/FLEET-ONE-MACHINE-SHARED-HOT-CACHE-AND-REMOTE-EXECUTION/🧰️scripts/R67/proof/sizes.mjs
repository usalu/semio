import {readFileSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env),f=remoteCacheFetchV1(env,{root}),base=`${s.endpoint}/${s.instance}`;
const rows=JSON.parse(readFileSync(root+"/.cache/f/cache-server/fleet/divergences.json","utf8")).rows.filter(r=>r.atMs>1794786360000);
const keys=[...new Set(rows.map(r=>r.key))].slice(0,Number(process.argv[2]??12));
const all=[];
for(const key of keys){const a=await f(`${base}/ac/${key}`);if(a.status!==200)continue;const res=decodeProtobufV1(REAPI_V1.actionResult,new Uint8Array(await a.arrayBuffer()));
 const files=(res.output_files??[]).map(x=>({p:x.path,h:x.digest.hash,z:Number(x.digest.size_bytes)}));let raw=0;const docs=[];
 for(const x of files){if(!x.p.endsWith(".br"))continue;const b=await f(`${base}/cas/${x.h}`);if(b.status!==200)continue;const u=brotliDecompressSync(new Uint8Array(await b.arrayBuffer()));raw+=u.length;docs.push([x.p.slice(0,16),u.length]);}
 const rec=docs.find(d=>d[0].startsWith("record"));const clos=docs.filter(d=>d[0].startsWith("closure"));
 all.push({key:key.slice(0,8),files:files.length,rawMB:+(raw/1048576).toFixed(2),zMB:+(files.reduce((s,x)=>s+x.z,0)/1048576).toFixed(2),record:rec?.[1],closures:clos.length,maxClosure:Math.max(0,...clos.map(c=>c[1])),sumClosure:clos.reduce((s,c)=>s+c[1],0)});console.log(JSON.stringify(all.at(-1)));}
