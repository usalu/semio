import fs from "node:fs";
const base="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/";
const read=f=>JSON.parse(fs.readFileSync(f,"utf8")),write=(f,j)=>fs.writeFileSync(f,JSON.stringify(j,null,2)+"\n");
const defs=[
["information","📊️information","Data and Information","Daten und Information"],
["objects","🧬️objects","Object Orientation","Objektorientierung"],
["representations","👁️representations","Object Representations","Repräsentationen von Objekten"],
["models","🧩️models","Partial Models","Teilmodelle und Modellübersetzungen"],
["exchange","🔌️exchange","Interfaces and Model Translation","Schnittstellen und Modellübersetzung"],
["consistency","🔗️consistency","Consistency across Models","Konsistenz über Modelle hinweg"]];
const path="🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json",manifest=read(path),combined=manifest.videos.find(v=>v.id==="bim");
if(!combined)throw Error("Missing combined lesson");
const videos=defs.map(([id,folder,en,de],i)=>({id:"bim_"+id,script:"🏛️architecture/🏗️bim/"+folder+"/🎬️clip/🐍️.py",chapters:[combined.chapters[i]],output:"BIM_"+id}));
manifest.videos.splice(manifest.videos.indexOf(combined),1,...videos);write(path,manifest);
const catalog=defs.map(([id,folder,en,de],i)=>({id:"bim_"+id,title:{en,de},icon:folder.split("️")[0]+"️",symbol:"bim",cell:{column:i%3,row:Math.floor(i/3)}}));
const presentation=read(base+"🪟️presentation/🔣️.json");presentation.topics.find(t=>t.id==="bim").videos=catalog;write(base+"🪟️presentation/🔣️.json",presentation);
for(const file of ["🪟️presentation/🧪️tests/🔣️.cases.json","🪟️presentation/🥞️hierarchy/🧪️tests/🔣️.cases.json"]){const c=read(base+file);c.topics.find(t=>t.id==="bim").videos=file.includes("hierarchy")?catalog.map(v=>v.id):catalog.map(({id,title})=>({id,title}));write(base+file,c);}
const c=read(base+"🧪️tests/🔣️.cases.json");c.curriculum.ids.splice(5,1,...videos.map(v=>v.id));c.curriculum.chapter_counts.splice(5,1,1,1,1,1,1,1);c.curriculum.segment_counts.splice(5,1,[1],[1],[1],[1],[1],[1]);c.curriculum.first_segment_ordinals.splice(5,1,114,115,116,117,118,119);write(base+"🧪️tests/🔣️.cases.json",c);
