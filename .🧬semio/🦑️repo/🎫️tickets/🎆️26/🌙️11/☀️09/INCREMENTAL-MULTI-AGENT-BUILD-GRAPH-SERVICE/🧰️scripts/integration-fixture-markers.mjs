import {readFileSync} from "node:fs";
import {graphSourceV1} from "/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧪️tests/📖️source/🟦️.ts";
const src=graphSourceV1();
for(const f of process.argv.slice(2)){
 const j=JSON.parse(readFileSync(f,"utf8"));
 const rows=[...(Array.isArray(j.laws)?j.laws:[]),...Object.entries(j.functions??{}).map(([n,r])=>({name:n,...r}))];
 for(const r of rows){if(!r.start)continue;const s=src.indexOf(r.start),e=s<0?-1:src.indexOf(r.end,s+r.start.length);if(s<0||e<0||src.indexOf(r.start,s+1)>=0)console.log(f.split("/").slice(-3,-1).join("/"),"|",r.name,"| start",s,"end",e,"| endmarker:",r.end?.slice(0,60));}
}
