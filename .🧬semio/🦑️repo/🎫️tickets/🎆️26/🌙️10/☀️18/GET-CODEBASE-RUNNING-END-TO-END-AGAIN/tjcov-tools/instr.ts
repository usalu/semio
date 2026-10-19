import {readFileSync,writeFileSync} from "node:fs";
const D=process.env.SRC!,OUT=process.env.OUT!;
let s=readFileSync(D+"/🟦️.ts","utf8");
s=s.replace('from "./🧬️schema/🔣️.json"','from "'+D+'/🧬️schema/🔣️.json"').replace('from "./🔣️.json"','from "'+D+'/🔣️.json"');
s=s.replace("const contains=(body:string,...required:readonly string[])=>required.every(value=>body.includes(spelling(value)));","const contains=(body:string,...required:readonly string[])=>required.every(value=>{const ok=body.includes(spelling(value));if(!ok){(globalThis as any).__miss=((globalThis as any).__miss??[]);(globalThis as any).__miss.push(value)}return ok});");
const out=s.split("\n").map((l,i)=>l.replace(/return false/g,()=>"return (globalThis.__f="+(i+1)+",false)"));
writeFileSync(OUT,out.join("\n"));
