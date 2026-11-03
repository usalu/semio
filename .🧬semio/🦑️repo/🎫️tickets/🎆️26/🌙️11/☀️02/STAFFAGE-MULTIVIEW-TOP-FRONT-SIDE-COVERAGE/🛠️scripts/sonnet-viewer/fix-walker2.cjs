const fs=require("fs");
const path="C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🚶️reference-walker/🔣️.json";
const j=JSON.parse(fs.readFileSync(path,"utf8"));
const s=j.symbols.find((x)=>x.id==="figure-block").geometry.elevation[0];
Object.assign(s,{ x:-0.185, y:-0.96, w:0.37, h:1.73 });
fs.writeFileSync(path,JSON.stringify(j,null,2)+"\n");
