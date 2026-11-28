import {remoteExecutionSidecarWatchV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🟦️.ts";
const root=process.argv[3]??"";
const role=process.argv[2] as any;
const files=[...remoteExecutionSidecarWatchV1(role).closure().files.keys()];
const lib=files[0].split("🏗️build/")[0]+"🏗️build/";
const by=new Map<string,number>();
for(const f of files){const rel=f.startsWith(lib)?f.slice(lib.length):"OUTSIDE:"+f.replace(/.*semio\//,"");const key=rel.split("/").slice(0,rel.startsWith("OUTSIDE")?0:1).join("/")||rel.split("/").slice(0,8).join("/");by.set(key,(by.get(key)??0)+1);}
console.log(role,files.length);
for(const [k,v] of [...by].sort((a,b)=>b[1]-a[1]))console.log(String(v).padStart(5),k);
