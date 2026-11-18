import{plugin}from"bun";import{readFileSync}from"node:fs";
const R="/Users/ueli/Documents/semio",map=new Map<string,string>();
for(const l of (process.env.R129_OVER??"").split("\n").filter(Boolean)){const at=l.indexOf("=");map.set(R+"/"+l.slice(0,at),l.slice(at+1));}
if(map.size>0){const filter=new RegExp([...map.keys()].map(p=>p.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|"),"u");
plugin({name:"r129-overlay",setup(b){b.onLoad({filter},args=>{const text=readFileSync(map.get(args.path)!,"utf8");return args.path.endsWith(".json")?{exports:{default:JSON.parse(text),...JSON.parse(text)},loader:"object"}:{contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};});}});}
