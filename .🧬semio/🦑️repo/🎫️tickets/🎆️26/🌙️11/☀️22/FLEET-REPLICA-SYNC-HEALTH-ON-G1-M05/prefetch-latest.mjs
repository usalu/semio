import {readFileSync,readdirSync,statSync,existsSync,mkdirSync,writeFileSync,renameSync,utimesSync} from "node:fs";
import {createHash} from "node:crypto";
import {dirname,join} from "node:path";
const [store,worktree,since,mode,...prefixes]=process.argv.slice(2),origins=join(store,"v1","origins"),latest=new Map(),history=new Map();
for(const origin of readdirSync(origins)){
 const segments=join(origins,origin,"segments");
 if(!existsSync(segments))continue;
 for(const name of readdirSync(segments).filter(n=>n.endsWith(".jsonl"))){
  const file=join(segments,name);
  if(statSync(file).mtimeMs<Date.parse(since))continue;
  for(const line of readFileSync(file,"utf8").split("\n")){
   if(!line||!prefixes.some(p=>line.includes(p)))continue;
   const event=JSON.parse(line);
   if(!prefixes.some(p=>event.path.startsWith(p)))continue;
   if(event.operation==="put"){const shas=history.get(event.path)??new Set();shas.add(event.sha256);history.set(event.path,shas);}
   const held=latest.get(event.path);
   if(held===undefined||event.hlc>held.hlc)latest.set(event.path,{...event,origin,segment:name});
  }
 }
}
const sha=bytes=>createHash("sha256").update(bytes).digest("hex");
for(const [path,event] of [...latest].sort()){
 const target=join(worktree,path),here=existsSync(target)?sha(readFileSync(target)):null,stale=here!==null&&(history.get(path)?.has(here)===true||statSync(target).mtimeMs<Date.parse(process.env.PUBLISHED_UNTIL??"1970-01-01T00:00:00Z"));
 let action;
 if(event.operation!=="put")action=`skip-${event.operation}`;
 else if(here===event.sha256)action="same";
 else if(here!==null&&!stale)action="local-edit-untouched";
 else{
  const bytes=readFileSync(join(store,"v1","blobs",event.sha256.slice(0,2),event.sha256));
  if(sha(bytes)!==event.sha256||bytes.length!==event.bytes)action="blob-mismatch";
  else if(mode!=="write")action=stale?"would-refresh":"would-write";
  else{
   mkdirSync("C:/b/prefetch",{recursive:true});
   const tmp=`C:/b/prefetch/${event.sha256}`;
   writeFileSync(tmp,bytes,{mode:event.executable?0o755:0o644});
   const at=new Date(Number(event.hlc.split("-")[0]));utimesSync(tmp,at,at);
   if(here!==null&&sha(readFileSync(target))!==here)action="changed-meanwhile-untouched";
   else{mkdirSync(dirname(target),{recursive:true});renameSync(tmp,target);action=stale?"refreshed":"written";}
  }
 }
 console.log(`${action} ${event.origin} ${event.hlc} ${event.segment} ${path}`);
}
