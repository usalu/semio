import {createSidecarClosureWatchV1} from "../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts";
import {join} from "node:path";import {fileURLToPath} from "node:url";
const B=fileURLToPath(new URL("../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",import.meta.url));
const entries=["🕸️graph/🕰️host/🟦️.ts","🕸️graph/🛑️custody/🟦️.ts","🕸️graph/📥️inference/🟨️.mjs"].map(e=>join(B,e));
const files=[...createSidecarClosureWatchV1(entries,[],B).closure().files.keys()].map(f=>f.slice(B.length));
const by=new Map<string,number>();for(const f of files){const k=f.startsWith("/")||f.includes("../")?"OUT":f.split("/").slice(0,1).join("/");by.set(k,(by.get(k)??0)+1);}
console.log(files.length);for(const [k,v] of [...by].sort((a,b)=>b[1]-a[1]))console.log(v,k);
console.log(files.filter(f=>/caching|execution|process/.test(f)).slice(0,10).join("\n"));
