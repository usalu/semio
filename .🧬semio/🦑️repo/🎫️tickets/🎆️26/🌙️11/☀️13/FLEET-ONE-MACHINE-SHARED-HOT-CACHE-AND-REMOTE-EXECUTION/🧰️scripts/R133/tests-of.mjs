import {readFileSync,existsSync,readdirSync,statSync} from "node:fs";
import {dirname,join} from "node:path";
const FW="/Users/ueli/Documents/semio/🧰️framework/";
const out=new Set();
for(const line of readFileSync(process.argv[2],"utf8").split("\n").filter(Boolean)){
 const abs=line.startsWith("/")?line:FW+line;let d=dirname(abs);
 while(d.length>FW.length){const t=join(d,"🧪️tests");if(existsSync(t)){for(const n of readdirSync(t)){const p=join(t,n);if(n==="🟦️.ts")out.add(p);}break;}d=dirname(d);}
}
console.log([...out].sort().join("\n"));
