import {readFileSync} from "node:fs";
const R="/Users/ueli/Documents/semio/";
const d = JSON.parse(await Bun.file(import.meta.dir + "/../🗑️generated/cache-rest-census.json").text());
const own = new Set(["🌎️hub","🎓️teaching","🏢️semio-tech","♻️mit-bestand","👴️leutwiler","root"]);
const files=[...new Set(d.targets.filter((t:any)=>own.has(t.owner)).map((t:any)=>t.file))] as string[];
let n=0;
for (const f of files){ const j=JSON.parse(readFileSync(R+f,"utf8")); for (const [name,t] of Object.entries<any>(j.targets??{})) { if (t.cache===false && !t.continuous) { n++; const c=(t.options?.command??JSON.stringify(t.options?.commands??"")).replaceAll("./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","<L>/"); console.log(`${f.replace(/\/📋️project.json$/,"").replace("/📦️packages/🦀️rust","/RS")}\t${name}\t${c.slice(0,110)}`);} } }
console.error(n);
