import {readFileSync} from "node:fs";
const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const files=[...new Set(d.targets.filter((t:any)=>own.has(t.owner)).map((t:any)=>t.file))] as string[];
for (const f of files){ const s=readFileSync("/Users/ueli/Documents/semio/"+f,"utf8"); const r=JSON.stringify(JSON.parse(s),null,2)+"\n"; const r2=JSON.stringify(JSON.parse(s),null,2); console.log(s===r||s===r2?"RT ":"NO ", f);}
