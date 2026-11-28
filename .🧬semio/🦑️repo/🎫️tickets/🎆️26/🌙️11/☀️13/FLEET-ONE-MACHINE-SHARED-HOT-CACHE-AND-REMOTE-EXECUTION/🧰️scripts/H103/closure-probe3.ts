import {sidecarImportsV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts";
import {join,dirname,resolve} from "node:path";import {fileURLToPath} from "node:url";import {existsSync,readFileSync} from "node:fs";
const B=fileURLToPath(new URL("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",import.meta.url));
const ents=process.argv.slice(2).map(e=>join(B,e)),seen=new Set<string>(),st=[...ents];
while(st.length){const p=st.pop()!;if(seen.has(p)||!existsSync(p))continue;seen.add(p);if(!/\.(ts|mjs|js)$/.test(p))continue;
 for(const r of sidecarImportsV1(readFileSync(p,"utf8"))){if(r.dynamic)continue;st.push(resolve(dirname(p),r.specifier));}}
const files=[...seen].map(f=>f.startsWith(B)?f.slice(B.length):"OUT:"+f);
const by=new Map<string,number>();for(const f of files){const k=f.split("/")[0];by.set(k,(by.get(k)??0)+1);}
console.log(files.length);for(const [k,v] of [...by].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(v,k);
const out=[...seen].filter(f=>!f.startsWith(B));const g=new Map<string,number>();for(const f of out){const k=f.replace(/.*semio\//,"").split("/").slice(0,6).join("/");g.set(k,(g.get(k)??0)+1);}
for(const [k,v] of [...g].sort((a,b)=>b[1]-a[1]).slice(0,8))console.log("OUT",v,k);
