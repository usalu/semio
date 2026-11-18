import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const {unpackBundleV1}=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env),f=remoteCacheFetchV1(env,{root}),base=`${s.endpoint}/${s.instance}`;
const needles=["🪪️graph-state/🛰️exchange/🟦️.ts","💾️storage/🟦️.ts","owner-pool/🟦️.ts","project-records/🟦️.ts","🚦️ready/🟦️.ts"];
const a=await f(`${base}/ac/${process.argv[2]}`);const res=decodeProtobufV1(REAPI_V1.actionResult,new Uint8Array(await a.arrayBuffer()));
const found={};let docs=0;
for(const x of res.output_files??[]){if(!x.path.startsWith("closure-"))continue;docs++;const b=await f(`${base}/cas/${x.digest.hash}`);const doc=JSON.parse(Buffer.from(brotliDecompressSync(new Uint8Array(await b.arrayBuffer()))).toString());
 const un=unpackBundleV1(doc);const rows=[...(un.rows??[]),...(un.layers??[]).flat()];
 for(const r of rows){if(typeof r.path!=="string")continue;for(const n of needles)if(r.path.endsWith(n)){const k=n+"|"+r.kind;found[k]=(found[k]??0)+1;}}
 if(docs===1)console.log("sample row kinds",[...new Set(rows.map(r=>r.kind))]);}
console.log(docs,JSON.stringify(found));
