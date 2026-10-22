import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
const require=createRequire(import.meta.url),iarna=require("@iarna/toml");
const norm=v=>JSON.parse(JSON.stringify(v,(k,x)=>typeof x==="bigint"?Number(x):x));
let ok=0,bad=0;
for(const f of process.argv.slice(2)){const src=readFileSync(f,"utf8");let a,b;try{a=norm(iarna.parse(src));}catch(e){a="ERR";}try{b=norm(Bun.TOML.parse(src));}catch(e){b="ERR";}
 if(JSON.stringify(a)===JSON.stringify(b))ok++;else{bad++;if(bad<6)console.log("DIFF",f,a==="ERR"?"iarna-err":b==="ERR"?"bun-err":"value");}}
console.log({ok,bad});
