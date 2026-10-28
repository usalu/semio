// [graphlat-g] scratch-generation surgery: lists the consumers of the rows of a path, or makes the stored rows of the given paths stale (bogus digest/sha/analysis fact), so a REAL-holds REAL-derive delta build re-infers exactly their consumers.
// usage: bun stale-rows.mjs <scratch-gen-dir> list <path>   |   bun stale-rows.mjs <scratch-gen-dir> stale <path>[,<path>...]
import{readFileSync,writeFileSync}from"node:fs";import{join}from"node:path";import{createHash}from"node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/";
const{canonicalJsonV1}=await import(L+"🧮️fact-codec/🟨️.mjs"),{unpackFactIndexV1,packFactIndexV1,GENERATION_POLICY_V1,factConsumersV1}=await import(L+"🟨️.mjs");
const[D,mode,arg]=process.argv.slice(2),sha=b=>createHash("sha256").update(b).digest("hex");
const pointer=JSON.parse(readFileSync(join(D,"current.json"),"utf8")),manifest=JSON.parse(readFileSync(join(D,"m",pointer.manifest+".json"),"utf8"));
const index=unpackFactIndexV1(readFileSync(join(D,"x",manifest.factIndex+".facts.br")),GENERATION_POLICY_V1.maximumIndexBytes);
const ROOT="/Users/ueli/Documents/semio/",paths=new Set(arg.split(",").map(p=>p.startsWith("/")?p:ROOT+p));
if(mode==="scan"){
 const per=new Map();for(let c=0;c<index.classes.length;c++)for(const r of index.classes[c].r){const row=index.rows[r];if(row.kind!=="file"&&row.kind!=="analysis")continue;const e=per.get(row.path)??per.set(row.path,{file:new Set(),analysis:new Set()}).get(row.path);for(const o of index.classes[c].o)e[row.kind].add(index.owners[o]);}
 const want=Number(arg);for(const[path,e]of per)if(e.file.size>0&&e.file.size<=want&&e.analysis.size>0&&e.analysis.size<=want)console.log(path.replace(ROOT,""),"file",[...e.file].join(","),"analysis",[...e.analysis].join(","));process.exit(0);
}
if(mode==="list"){
 const owners=new Map();for(let c=0;c<index.classes.length;c++)for(const r of index.classes[c].r){const row=index.rows[r];if(!paths.has(row.path))continue;for(const o of index.classes[c].o){const key=row.kind;const set=owners.get(key)??owners.set(key,new Set()).get(key);set.add(index.owners[o]);}}
 for(const[kind,set]of owners)console.log(kind,set.size,[...set].slice(0,8).join(" "));process.exit(0);
}
let changed=0;
index.rows=index.rows.map(row=>{if(!paths.has(row.path))return row;changed++;const fact=row.fact;const bogus=typeof fact==="string"?sha("graphlat:"+fact):fact&&typeof fact==="object"?{...fact,...(fact.sha256?{sha256:sha("graphlat:"+fact.sha256)}:{}),...(fact.size!==undefined?{size:fact.size+1}:{}),...(fact.revision?{revision:sha("graphlat:"+fact.revision)}:{})}:fact;return{...row,fact:bogus};});
const packed=packFactIndexV1(index,GENERATION_POLICY_V1.maximumIndexBytes),indexDigest=sha(packed.bytes);writeFileSync(join(D,"x",indexDigest+".facts.br"),packed.bytes);manifest.factIndex=indexDigest;
const bytes=Buffer.from(canonicalJsonV1(manifest)),digest=sha(bytes);writeFileSync(join(D,"m",digest+".json"),bytes);writeFileSync(join(D,"current.json"),JSON.stringify({...pointer,manifest:digest}));
console.log(JSON.stringify({changedRows:changed,manifest:digest}));
