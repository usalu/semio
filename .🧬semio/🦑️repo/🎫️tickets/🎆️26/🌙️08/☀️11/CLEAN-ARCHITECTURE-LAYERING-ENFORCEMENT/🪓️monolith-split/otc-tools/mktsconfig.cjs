const fs=require("fs");const R="/Users/ueli/Documents/semio/";
const F=R+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/";
const B="🧰️framework/🛍️products/";
const own=p=>{ if(!/\.(ts|tsx)$/.test(p)) return false; if(p.includes("/📺️renderer/")||p.includes("/📚️library/")||p.includes("/🏪️store/👷️worker/")||p.includes("/🎬️media/")||p.includes("🧶️bundles")) return false;
  return p.startsWith(B+"💻️os/🔨️modules/")||p.startsWith(B+"💻️os/🎚️config/")||p.startsWith(B+"🦑️repo/🔨️modules/💻️client/"); };
const set=new Set();
for(const t of ["os","repo","config"]){const j=JSON.parse(fs.readFileSync(F+`🗑️generated/otc-before-${t}.json`,"utf8"));
  for(const v of j.summary.violations) if(v.rule.name==="no-circular"){ for(const c of v.cycle) if(own(c.name)) set.add(c.name); if(own(v.from)) set.add(v.from);} }
const extra=process.argv.slice(2);
for(const e of extra) set.add(e);
const files=[...set].filter(p=>fs.existsSync(R+p)).sort().map(p=>R+p);
const cfg={extends:R+"🧰️framework/🛍️products/💻️os/tsconfig.json",compilerOptions:{noEmit:true,incremental:false,tsBuildInfoFile:null},include:[],files};
fs.writeFileSync(F+"otc-tools/tsconfig.otc.json",JSON.stringify(cfg,null,1));
console.log("files",files.length);
