import {readFileSync} from "node:fs";
for(const f of process.argv.slice(2)){try{Bun.TOML.parse(readFileSync(f,"utf8"));}catch(e){console.log("BAD",f,String(e.message).slice(0,100));}}
