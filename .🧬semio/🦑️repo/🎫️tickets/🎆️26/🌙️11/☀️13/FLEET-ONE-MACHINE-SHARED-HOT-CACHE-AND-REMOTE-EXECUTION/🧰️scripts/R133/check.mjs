import {readFileSync} from "node:fs";
import {files,patchOnlyFiles,transform,FRAMEWORK,PATCHES} from "./seam.mjs";
let n=0,patched=0;for(const f of [...files(),...patchOnlyFiles()]){const t=readFileSync(f,"utf8");let o;try{o=transform(f,t);}catch(e){console.log("ERR",f,e.message);continue;}if(o!==t){n++;}}
for(const[s]of PATCHES){const f=FRAMEWORK+"/"+s;try{readFileSync(f);}catch{console.log("MISSING",s);}}
console.log("files changed",n);
for(const[s,from]of PATCHES){const{MATCHED}=await import("./seam.mjs");if(!MATCHED.has(s+from))console.log("UNMATCHED PATCH",s.slice(-60),from.slice(0,60));}
