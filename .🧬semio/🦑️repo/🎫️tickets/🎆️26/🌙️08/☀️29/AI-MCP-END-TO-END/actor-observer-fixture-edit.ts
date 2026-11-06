import{readFileSync,writeFileSync,renameSync}from"node:fs";
const root="🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/";
for(const name of["🦀️.rs","🛬️decoder/🦀️.rs"]){const file=root+name;let source=readFileSync(file,"utf8");if(!source.includes("physical_spent"))throw Error(file);source=source.replaceAll("physical_spent","fresh_spent");const temp=import.meta.dirname+"/🗑️generated/actor-observer-fixture.rs";writeFileSync(temp,source);renameSync(temp,file);}
