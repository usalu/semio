import {readFileSync,readdirSync,writeFileSync,mkdirSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {REAPI_V1,decodeProtobufV1} from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/📡️wire/📦️protobuf/🟦️.ts";
const base="/Users/Shared/semio/.cache/f/cache-server/v1/main/ac",cas=process.env.HOME+"/.cache/semio/cas/v1/o/";
const out:any[]=[];let total=0;
for(const a of readdirSync(base))for(const b of readdirSync(base+"/"+a)){
 try{
  const r=decodeProtobufV1(REAPI_V1.actionResult,readFileSync(`${base}/${a}/${b}`)) as any;
  const note=JSON.parse(Buffer.from(r.stdout_raw).toString());
  if(note.kind!=="project-inference")continue;
  const doc=note.documents.find((d:any)=>d.name==="record.br");if(!doc)continue;
  const raw=brotliDecompressSync(readFileSync(cas+doc.hash.slice(0,2)+"/"+doc.hash.slice(2)));
  const rec=JSON.parse(raw.toString());
  let img:any;try{img=JSON.parse(rec.image);}catch{continue;}
  out.push({platform:note.platform,producer:note.producer.split(":")[0].slice(-14),t:note.createdMs,digest:note.digest,size:raw.length,root:img.root,name:img.name,rec});total+=raw.length;
 }catch{}
}

const win=new Set(out.filter(o=>o.platform==="win32-x64").map(o=>o.root));
const best=new Map<string,any>();
for(const o of out){if(!win.has(o.root))continue;const k=o.platform+"|"+o.root;const p=best.get(k);if(!p||o.t>p.t)best.set(k,o);}
out.length=0;for(const v of best.values())out.push(v);
console.log(out.length);
const byp:Record<string,number>={};for(const o of out)byp[o.platform]=(byp[o.platform]??0)+1;console.log(byp);
writeFileSync("records.json",JSON.stringify(out));
