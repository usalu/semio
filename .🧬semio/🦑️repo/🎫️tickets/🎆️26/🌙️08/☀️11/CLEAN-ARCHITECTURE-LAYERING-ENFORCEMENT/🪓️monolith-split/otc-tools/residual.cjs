const fs=require("fs");const G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/";
const phase=process.argv[2];const B="🧰️framework/🛍️products/";
const scope=p=>p.startsWith(B+"💻️os/🔨️modules/")&&!p.includes("/📺️renderer/")||p.startsWith(B+"🦑️repo/🔨️modules/")&&!p.includes("/📚️library/")||p.startsWith(B+"💻️os/🎚️config/");
const excl=p=>/(🏪️store|🎭️actor|🪟️member|🎬️media|🧶️bundles)/.test(p);
const seen=new Map();
for(const t of ["os","repo","config"]){const s=JSON.parse(fs.readFileSync(G+`otc-${phase}-${t}.scc.json`));for(const c of s.sccs){const m=c.members.filter(scope);if(!m.length)continue;const key=c.members.slice().sort()[0];seen.set(key,{size:c.size,inScope:m,excluded:c.members.filter(excl)});}}
for(const [k,v] of seen){console.log(`SCC size=${v.size} in-scope-members=${v.inScope.length} parent-owned-or-generated-members=${v.excluded.length}`);v.inScope.forEach(m=>console.log("   ",m.replace(B,"")));}
