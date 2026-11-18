import{plugin}from"bun";import{existsSync,readFileSync}from"node:fs";import{dirname,resolve}from"node:path";
const R="/Users/ueli/Documents/semio",map=new Map<string,string>();
for(const l of (process.env.R147_OVER??"").split("\n").filter(Boolean)){const at=l.indexOf("=");map.set(R+"/"+l.slice(0,at),l.slice(at+1));}
if(map.size>0){const filter=new RegExp([...map.keys()].map(p=>p.replace(/[.*+?^${}()|[\]\\]/gu,"\\$&")).join("|"),"u");
plugin({name:"r147-overlay",setup(b){
 b.onResolve({filter:/^(\.{1,2}\/|\/)/u},args=>{const target=args.path.startsWith("/")?args.path:resolve(dirname(args.importer),args.path);return map.has(target)?{path:existsSync(target)?target:map.get(target)!}:undefined;});
 b.onLoad({filter},args=>{if(!map.has(args.path))return undefined;const text=readFileSync(map.get(args.path)!,"utf8");return args.path.endsWith(".json")?{exports:{default:JSON.parse(text),...JSON.parse(text)},loader:"object"}:{contents:text,loader:args.path.endsWith(".ts")?"ts":"js"};});}});}
