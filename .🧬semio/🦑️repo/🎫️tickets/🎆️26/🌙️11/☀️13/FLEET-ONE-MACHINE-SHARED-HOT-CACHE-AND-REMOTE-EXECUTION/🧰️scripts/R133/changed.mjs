import {readFileSync} from "node:fs";
import {files,transform,FRAMEWORK} from "./seam.mjs";
for(const f of files()){const t=readFileSync(f,"utf8");if(transform(f,t)!==t)console.log(f.replace(FRAMEWORK+"/",""));}
