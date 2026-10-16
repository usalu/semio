const fs=require("fs");const s=JSON.parse(fs.readFileSync(process.argv[2],"utf8"));
const depth=+process.argv[3]||6; const limit=+process.argv[4]||12;
const parent=/(🏪️store|🎭️actor|🪟️member|🎬️media)/;
s.sccs.forEach((c,i)=>{ if(c.size<2)return; 
 const g={};for(const m of c.members){const k=m.split("/").slice(0,depth).join("/");g[k]=(g[k]||0)+1;}
 const p=c.members.filter(m=>parent.test(m)).length;
 console.log(`#${i} size=${c.size} edges=${c.edges} parentOwnedMembers=${p}`);
 if(c.size>=limit) Object.entries(g).sort((a,b)=>b[1]-a[1]).forEach(([k,n])=>console.log("   ",n,k.replace("🧰️framework/","F/").replace("🛍️products/","P/")));
 else c.members.forEach(m=>console.log("    -",m.replace("🧰️framework/","F/").replace("🛍️products/","P/")));
});
