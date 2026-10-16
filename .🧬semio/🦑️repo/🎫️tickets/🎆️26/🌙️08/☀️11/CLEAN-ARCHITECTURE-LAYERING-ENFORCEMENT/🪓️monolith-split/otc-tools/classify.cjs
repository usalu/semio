const fs=require("fs");const G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/";
const B="🧰️framework/🛍️products/";
const phase=process.argv[2];
const seen=new Map();
for(const t of ["os","repo","config"]){const j=JSON.parse(fs.readFileSync(G+`otc-${phase}-${t}.json`,"utf8"));
 for(const v of j.summary.violations) if(v.rule.name==="no-circular") seen.set(v.from+" -> "+v.to,v);}
const tree=f=>f.startsWith(B+"💻️os/🔨️modules/📺️renderer/")?"os/modules renderer (rhu)":f.startsWith(B+"💻️os/🔨️modules/")?"os/modules":f.startsWith(B+"🦑️repo/🔨️modules/📚️library/")?"repo/modules library (parent)":f.startsWith(B+"🦑️repo/🔨️modules/")?"repo/modules":f.startsWith(B+"💻️os/🎚️config/")?"os/config":"outside";
const parent=n=>/(\/🏪️store\/|\/🎭️actor\/|\/🪟️member\/|\/🎬️media\/)/.test(n);
const gen=n=>n.includes("🧶️bundles");
const rows={};
for(const v of seen.values()){const tr=tree(v.from);if(tr==="outside")continue;
 const names=[v.from,v.to,...v.cycle.map(c=>c.name)];
 const cat=names.some(gen)?"generated bundle":names.some(parent)?"parent-owned (store/actor/member/media)":"cuttable";
 rows[tr]??={total:0,"parent-owned (store/actor/member/media)":0,"generated bundle":0,cuttable:0};rows[tr].total++;rows[tr][cat]++;}
console.log(phase);for(const [k,v] of Object.entries(rows).sort())console.log(k.padEnd(34),JSON.stringify(v));
