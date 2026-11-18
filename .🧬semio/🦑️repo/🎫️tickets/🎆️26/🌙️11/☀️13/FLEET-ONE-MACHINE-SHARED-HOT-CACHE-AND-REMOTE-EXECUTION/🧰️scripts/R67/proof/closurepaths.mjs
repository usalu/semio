import {readFileSync,writeFileSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio",B=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {remoteCacheFetchV1}=await import(B+"/🌐️remote-cache/🛰️fleet/🟦️.ts");const {graphFleetSettingsV1}=await import(B+"/🪪️graph-state/🧶️nx/🟦️.ts");
const {REAPI_V1,decodeProtobufV1}=await import(B+"/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts");
const {unpackBundleV1}=await import(B+"/🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts");
const env={...process.env};for(const k of Object.keys(env))if(k.startsWith("SEMIO_REMOTE_CACHE"))delete env[k];
const s=graphFleetSettingsV1(root,env),f=remoteCacheFetchV1(env,{root}),base=`${s.endpoint}/${s.instance}`;
const keys=process.argv.slice(2);const paths=new Set();
for(const key of keys){const a=await f(`${base}/ac/${key}`);const res=decodeProtobufV1(REAPI_V1.actionResult,new Uint8Array(await a.arrayBuffer()));
 for(const x of res.output_files??[]){if(!x.path.startsWith("closure-"))continue;const b=await f(`${base}/cas/${x.digest.hash}`);const doc=JSON.parse(Buffer.from(brotliDecompressSync(new Uint8Array(await b.arrayBuffer()))).toString());
  const un=unpackBundleV1(doc);for(const layer of un.layers??[])for(const r of layer)if(r.path)paths.add(r.path);for(const r of un.rows??[])if(r.path)paths.add(r.path);}}
writeFileSync("closure-paths.txt",[...paths].sort().join("\n"));console.log(paths.size);
