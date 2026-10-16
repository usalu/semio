const fs=require("fs");const G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/";
const phase=process.argv[2];
const B="🧰️framework/🛍️products/";
const trees={"os/modules (all)":B+"💻️os/🔨️modules/","os/modules renderer":B+"💻️os/🔨️modules/📺️renderer/","repo/modules (all)":B+"🦑️repo/🔨️modules/","repo/modules library":B+"🦑️repo/🔨️modules/📚️library/","os/config":B+"💻️os/🎚️config/"};
const seen=new Map();
for(const t of ["os","repo","config"]){const f=G+`otc-${phase}-${t}.json`;if(!fs.existsSync(f))continue;const j=JSON.parse(fs.readFileSync(f,"utf8"));
 for(const v of j.summary.violations) if(v.rule.name==="no-circular") seen.set(v.from+" -> "+v.to,v);}
const isTest=p=>p.includes("/🧪️tests/")||p.includes("/🧪️test/");
console.log(`phase=${phase} distinct no-circular violations over the three cruises: ${seen.size}`);
for(const [n,p] of Object.entries(trees)){const vs=[...seen.values()].filter(v=>v.from.startsWith(p));console.log(`${n.padEnd(24)} total=${vs.length} prod=${vs.filter(v=>!isTest(v.from)).length}`);}
const own=[...seen.values()].filter(v=>{const f=v.from;return (f.startsWith(trees["os/modules (all)"])&&!f.startsWith(trees["os/modules renderer"]))||(f.startsWith(trees["repo/modules (all)"])&&!f.startsWith(trees["repo/modules library"]))||f.startsWith(trees["os/config"]);});
console.log(`in-scope (os/modules minus renderer, repo/modules minus library, os/config): ${own.length}`);
if(process.argv[3]==="list")own.forEach(v=>console.log(v.from+" -> "+v.to));
