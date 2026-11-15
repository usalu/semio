import {machine,build,summary,events,S,script} from "./harness.ts";
import {readdirSync,readFileSync,writeFileSync} from "node:fs";import {join} from "node:path";
console.log("code:",script.slice(-120));
const m=machine("red"+(process.env.Q3_CODE?"-new":"-old"),"a");
let r=await build(m,{SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"},{args:["-j2","--profile","prototype"]});console.log("cold",r.code);
let n=0;for(const sh of readdirSync(join(m.cache,"e")))for(const k of readdirSync(join(m.cache,"e",sh)))for(const f of readdirSync(join(m.cache,"e",sh,k))){const p=join(m.cache,"e",sh,k,f),j=JSON.parse(readFileSync(p,"utf8"));if(j.crate==="equivalent"){j.outputs=j.outputs.filter((o:any)=>!o.name.endsWith(".rmeta"));writeFileSync(p,JSON.stringify(j));n++;}}
console.log("tampered entries of equivalent:",n);
r=await build(m,{SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on"},{lane:"l2",args:["-j2","--profile","prototype"]});
console.log("rebuild in a fresh lane:",r.code);
console.log(r.output.split("\n").filter(l=>/^error|extern location|Caused by/.test(l)).slice(0,4).map(l=>l.slice(0,330)).join("\n"));
console.log(JSON.stringify(summary(m)));
process.exit(0);
