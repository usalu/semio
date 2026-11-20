import{plugin}from"bun";
process.env.BUN_OPTIONS="--preload "+import.meta.path;
import{readFileSync}from"node:fs";
const rows=readFileSync(process.env.R226_MAP!,"utf8").split("\n").filter(Boolean).map(line=>line.split("\t") as [string,string]),map=new Map(rows);
const escaped=[...map.keys()].map(path=>path.replace(/[.*+?^${}()|[\]\\]/g,"\\$&"));
plugin({name:"r226-stage",setup(build){build.onLoad({filter:new RegExp("^(?:"+escaped.join("|")+")$")},args=>{const text=readFileSync(map.get(args.path)!,"utf8");return args.path.endsWith(".json")?{exports:{default:JSON.parse(text)},loader:"object"}:{contents:text,loader:"ts"};});}});
