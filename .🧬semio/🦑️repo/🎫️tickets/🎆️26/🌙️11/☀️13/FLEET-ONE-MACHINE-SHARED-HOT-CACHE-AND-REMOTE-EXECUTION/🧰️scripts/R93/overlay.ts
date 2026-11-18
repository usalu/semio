import{plugin}from"bun";import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio",S=process.env.R93_STAGE!;
const map=new Map<string,string>();
for(const l of readFileSync(S+"/index.txt","utf8").split("\n").filter(Boolean)){const[i,rel]=l.split(" ");map.set(R+"/"+rel,S+"/"+i+".new");}
const filter=new RegExp([...map.keys()].map(p=>p.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|"),"u");
plugin({name:"r93-overlay",setup(b){b.onLoad({filter},args=>({contents:readFileSync(map.get(args.path)!,"utf8"),loader:args.path.endsWith(".ts")?"ts":"js"}));}});
