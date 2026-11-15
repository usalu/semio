import {interfaceFileFactsV1} from "./iface-b/🟦️.ts";
import {readFileSync,writeFileSync} from "node:fs";
const files=JSON.parse(readFileSync("rsfiles.json","utf8")) as string[];
const out:string[]=[];
for(const f of files){let b;try{b=readFileSync("/Users/Shared/semio/"+f);}catch{continue;}const x=interfaceFileFactsV1(b);if(x.splice||x.void)out.push(f);}
writeFileSync("splice.json",JSON.stringify(out));console.log(out.length);
