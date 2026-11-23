import {readFileSync,readdirSync,statSync,existsSync,writeFileSync,mkdirSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [worktree,store,since,out,...lostFiles]=process.argv.slice(2),frozen=Date.parse(process.env.VIEW_FROZEN_AT??"9999-01-01T00:00:00Z");
const rows=lostFiles.map(file=>{const rel=file.replace(/\\/g,"/").split("/.cache/f/lost/")[1],cut=rel.indexOf("/");return {lost:file,dir:rel.slice(0,cut),path:rel.slice(cut+1)};});
const wanted=new Set(rows.map(row=>row.path)),history=new Map();
for(const origin of readdirSync(join(store,"v1/origins"))){
 const segments=join(store,"v1/origins",origin,"segments");
 if(!existsSync(segments))continue;
 for(const name of readdirSync(segments).filter(n=>n.endsWith(".jsonl"))){
  const file=join(segments,name);
  if(statSync(file).mtimeMs<Date.parse(since))continue;
  for(const line of readFileSync(file,"utf8").split("\n")){
   if(!line||![...wanted].some(path=>line.includes(path)))continue;
   let event;try{event=JSON.parse(line);}catch{continue;}
   if(wanted.has(event.path)){const list=history.get(event.path)??[];list.push(event);history.set(event.path,list);}
  }
 }
}
const git=(...args)=>spawnSync("git",args,{cwd:worktree,maxBuffer:1<<28});
for(const row of rows){
 const mtime=Math.min(statSync(row.lost).mtimeMs,frozen),before=(history.get(row.path)??[]).filter(e=>Number(e.hlc.split("-")[0])<=mtime).sort((a,b)=>a.hlc<b.hlc?1:-1)[0];
 const commit=git("log","-1",`--before=${new Date(mtime).toISOString()}`,"--format=%H %ct","--",row.path).stdout.toString().trim().split(" ");
 let base,source;
 if(before!==undefined&&before.operation==="put"&&(commit[0]===""||Number(before.hlc.split("-")[0])>=Number(commit[1])*1000)){base=readFileSync(join(store,"v1/blobs",before.sha256.slice(0,2),before.sha256));source=`fleet ${before.hlc}`;}
 else if(commit[0]!==""){base=git("show",`${commit[0]}:${row.path}`).stdout;source=`git ${commit[0].slice(0,12)}`;}
 else{console.log(`no-base | ${row.path}`);continue;}
 const slot=join(out,createHash("sha256").update(row.lost).digest("hex").slice(0,12));
 mkdirSync(slot,{recursive:true});
 writeFileSync(join(slot,"local"),readFileSync(row.lost));writeFileSync(join(slot,"base"),base);writeFileSync(join(slot,"current"),readFileSync(join(worktree,row.path)));
 const merged=spawnSync("git",["merge-file","-p","--diff3","-L","local","-L","base","-L","current",join(slot,"local"),join(slot,"base"),join(slot,"current")],{maxBuffer:1<<28});
 writeFileSync(join(slot,"merged"),merged.stdout);
 console.log(`${merged.status===0?"clean":`conflicts(${merged.status})`} ${source} ${slot} | ${row.path}`);
}
