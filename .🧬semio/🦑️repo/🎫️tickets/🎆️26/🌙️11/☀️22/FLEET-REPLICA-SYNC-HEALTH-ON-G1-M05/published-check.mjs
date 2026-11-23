import {readFileSync,readdirSync,statSync,existsSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
const [worktree,store,since,list]=process.argv.slice(2),paths=readFileSync(list,"utf8").split("\n").map(line=>line.trim()).filter(Boolean),wanted=new Set(paths),latest=new Map();
for(const origin of readdirSync(join(store,"v1/origins"))){
 const segments=join(store,"v1/origins",origin,"segments");
 if(!existsSync(segments))continue;
 for(const name of readdirSync(segments).filter(n=>n.endsWith(".jsonl"))){
  const file=join(segments,name);
  if(statSync(file).mtimeMs<Date.parse(since))continue;
  for(const line of readFileSync(file,"utf8").split("\n")){
   if(!line||!paths.some(path=>line.includes(path)))continue;
   let event;try{event=JSON.parse(line);}catch{continue;}
   if(!wanted.has(event.path))continue;
   const seen=latest.get(event.path);
   if(seen===undefined||seen.hlc<event.hlc)latest.set(event.path,event);
  }
 }
}
for(const path of paths){
 const event=latest.get(path),target=join(worktree,path),local=existsSync(target)?createHash("sha256").update(readFileSync(target)).digest("hex"):"absent";
 console.log(`${event===undefined?"unpublished":event.sha256===local?"published":"differs"} ${event?.hlc??"-"} | ${path}`);
}
