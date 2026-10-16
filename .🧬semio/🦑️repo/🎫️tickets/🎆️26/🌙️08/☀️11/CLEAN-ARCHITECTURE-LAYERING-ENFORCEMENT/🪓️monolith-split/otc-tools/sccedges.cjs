const fs=require("fs");
const [,,cruise,sccfile,...ids]=process.argv;
const j=JSON.parse(fs.readFileSync(cruise,"utf8"));const s=JSON.parse(fs.readFileSync(sccfile,"utf8"));
const mod=new Map(j.modules.map(m=>[m.source,m]));
const sh=p=>p.replace("🧰️framework/","F/").replace("🛍️products/","P/").replace("💻️os/🔨️modules/","OSM/").replace("🦑️repo/🔨️modules/","REPOM/").replace("/🟦️.ts","/§").replace("/🟦️.tsx","/§x");
for(const id of ids){const c=s.sccs[+id];const set=new Set(c.members);console.log(`## SCC #${id} size=${c.size}`);
 for(const m of c.members){const mm=mod.get(m);if(!mm)continue;for(const d of mm.dependencies){if(d.resolved&&set.has(d.resolved))console.log(`  ${sh(m)}  ->  ${sh(d.resolved)}  [${d.dependencyTypes.filter(t=>!["local","import"].includes(t)).join(",")}${d.dynamic?",dyn":""}]  "${d.module}"`);}}}
