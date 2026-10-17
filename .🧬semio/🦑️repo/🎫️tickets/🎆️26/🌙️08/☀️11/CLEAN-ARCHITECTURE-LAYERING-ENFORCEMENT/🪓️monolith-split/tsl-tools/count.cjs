const fs=require("fs");const G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/";
const phase=process.argv[2];
const B="🧰️framework/🛍️products/💻️os/";
const j=JSON.parse(fs.readFileSync(G+`tsl-cyc-${phase}-os.json`,"utf8"));
const vs=j.summary.violations.filter(v=>v.rule.name==="no-circular");
const trees={"os/modules all":B+"🔨️modules/","os/modules w/o renderer":null,"os/modules renderer":B+"🔨️modules/📺️renderer/","os/modules store/worker tree":B+"🔨️modules/🏪️store/👷️worker/","os/config":B+"🎚️config/"};
const seen=new Map();for(const v of vs)seen.set(v.from+" -> "+v.to,v);
const all=[...seen.values()];
const inT=(v,p)=>v.from.startsWith(p);
console.log(`phase=${phase} no-circular violations total=${all.length}`);
for(const [n,p] of Object.entries(trees)){
 const f=p?all.filter(v=>inT(v,p)):all.filter(v=>inT(v,B+"🔨️modules/")&&!inT(v,B+"🔨️modules/📺️renderer/"));
 console.log(n.padEnd(30),f.length);}
const watch=["backbone-parity","👷️worker/🟦️.ts","📥️installation","🎬️media/","rendermedia","authored-extension-installation-identity"];
for(const w of watch){const f=all.filter(v=>v.from.includes(w)||v.to.includes(w));console.log(`touching ${w}: ${f.length}`);if(process.argv[3]==="list")f.forEach(v=>console.log("   "+v.from.replace(B,"")+" -> "+v.to.replace(B,"")));}
