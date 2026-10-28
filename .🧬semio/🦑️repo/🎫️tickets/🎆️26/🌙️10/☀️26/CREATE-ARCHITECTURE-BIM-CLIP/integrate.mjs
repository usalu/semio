import fs from "node:fs";
const base="🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/",clip="🏛️architecture/🏗️bim/🎬️clip/";
const read=f=>JSON.parse(fs.readFileSync(f,"utf8")),write=(f,j)=>fs.writeFileSync(f,JSON.stringify(j,null,2)+"\n");
const replace=(f,a,b)=>{const s=fs.readFileSync(f,"utf8");if(!s.includes(a))throw Error("Missing anchor "+f+" "+a);fs.writeFileSync(f,s.replaceAll(a,b));};
const cases=read(base+"🧪️tests/🔣️.cases.json");
cases.curriculum.ids.push("bim");cases.curriculum.chapter_counts.push(6);cases.curriculum.segment_counts.push([1,1,1,1,1,1]);cases.curriculum.render_segment_count=119;cases.curriculum.logical_clip_count=32;cases.curriculum.first_segment_ordinals.push(114);cases.partial_reuse.curriculum_unique_sources=111;write(base+"🧪️tests/🔣️.cases.json",cases);
const schema=read(base+"🧬️schema/🔣️.schema.json");schema.properties.videos.minItems=1;delete schema.properties.videos.maxItems;schema.properties.videos.items.properties.script.pattern="^🏛️architecture/(?:[^/]+/)+🎬️clip/🐍️\\.py$";schema.$defs.segment.properties.script.pattern="^🏛️architecture/(?:[^/]+/)+🐍️\\.py$";write(base+"🧬️schema/🔣️.schema.json",schema);
const manifest="🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json",data=read(manifest);
data.videos.push({id:"bim",script:clip+"🐍️.py",chapters:[
["BIM_01_Information","📊️information","Beat1_DataToInformation"],["BIM_02_Objects","🧬️objects","Beat2_ObjectOrientation"],["BIM_03_Representations","👁️representations","Beat3_SameObject"],["BIM_04_Models","🧩️models","Beat4_PartialModels"],["BIM_05_Exchange","🔌️exchange","Beat5_Interfaces"],["BIM_06_Consistency","🔗️consistency","Beat6_Consistency"]
].map(([scene,folder,beat])=>({scene,segments:[{script:clip+folder+"/🐍️.py",scene:beat}]})),output:"Full_BIM"});write(manifest,data);
const video={id:"bim",title:{en:"BIM · Objects and Models",de:"BIM · Objekte und Modelle"},icon:"🏗️",symbol:"bim",cell:{column:0,row:0}},presentation=read(base+"🪟️presentation/🔣️.json");presentation.topics.find(t=>t.id==="bim").videos=[video];write(base+"🪟️presentation/🔣️.json",presentation);
const ps=read(base+"🪟️presentation/🧬️schema.json");ps.$defs.video.properties.symbol.enum.push("bim");write(base+"🪟️presentation/🧬️schema.json",ps);
for(const file of ["🪟️presentation/🧪️tests/🔣️.cases.json","🪟️presentation/🥞️hierarchy/🧪️tests/🔣️.cases.json"]){const c=read(base+file);c.topics.find(t=>t.id==="bim").videos=file.includes("hierarchy")?["bim"]:[{id:video.id,title:video.title}];write(base+file,c);}
const test=base+"🧪️tests/🐍️.py";
replace(test,'len({v["id"] for v in videos}), 5','len({v["id"] for v in videos}), len(CONTRACT["curriculum"]["ids"])');
replace(test,'{"audience": "speaker", "quality":','{"language": "de", "audience": "speaker", "quality":');
replace(test,'media / "speaker"','media / "de" / "speaker"');
replace(test,'output / "speaker"','output / "de" / "speaker"');
replace(test,'output / "speaker/1/v/FixtureChapter.mp4"','output / "de/speaker/1/v/FixtureChapter.mp4"');
replace(test,'runtime.source_narration(segment)','runtime.source_narration(segment, language="de")');
