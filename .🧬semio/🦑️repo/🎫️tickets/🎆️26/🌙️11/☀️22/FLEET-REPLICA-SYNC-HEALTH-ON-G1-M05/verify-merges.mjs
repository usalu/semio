import {readFileSync,readdirSync,statSync,existsSync,writeFileSync,renameSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [worktree,store,since,report,mode]=process.argv.slice(2),lostRoot=join(worktree,".cache/f/lost");
const sha=bytes=>createHash("sha256").update(bytes).digest("hex");
const rows=readFileSync(report,"utf8").split("\n").filter(line=>/^(merged|conflict)/.test(line)).map(line=>{const [verdict,dir,path]=line.split(" | ");return {slot:verdict.split(" ")[1],kind:verdict.split(" ")[0],dir,path};});
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
   if(!wanted.has(event.path))continue;
   const list=history.get(event.path)??[];list.push(event);history.set(event.path,list);
  }
 }
}
const git=(...args)=>spawnSync("git",args,{cwd:worktree,maxBuffer:1<<28});
for(const row of rows){
 const lostFile=join(lostRoot,row.dir,row.path),mtime=Math.min(statSync(lostFile).mtimeMs,Date.parse(process.env.VIEW_FROZEN_AT??"9999-01-01T00:00:00Z")),before=(history.get(row.path)??[]).filter(e=>Number(e.hlc.split("-")[0])<=mtime).sort((a,b)=>a.hlc<b.hlc?1:-1)[0];
 const commit=git("log","-1",`--before=${new Date(mtime).toISOString()}`,"--format=%H %ct","--",row.path).stdout.toString().trim().split(" ");
 let base,source;
 if(before!==undefined&&(commit[0]===""||Number(before.hlc.split("-")[0])>=Number(commit[1])*1000)){
  if(before.operation!=="put"){console.log(`base-not-put ${before.operation} | ${row.path}`);continue;}
  base=readFileSync(join(store,"v1/blobs",before.sha256.slice(0,2),before.sha256));source=`fleet ${before.hlc}`;
 }else if(commit[0]!==""){base=git("show",`${commit[0]}:${row.path}`).stdout;source=`git ${commit[0].slice(0,12)}`;}
 else{console.log(`no-base | ${row.path}`);continue;}
 const slot=row.slot,used=readFileSync(join(slot,"ancestor"));
 if(sha(used)===sha(base)){console.log(`ancestor-ok ${row.kind} ${source} | ${row.path}`);continue;}
 writeFileSync(join(slot,"base"),base);
 const merged=spawnSync("git",["merge-file","-p","--diff3","-L","local","-L","base","-L","current",join(slot,"local"),join(slot,"base"),join(slot,"current")],{maxBuffer:1<<28});
 writeFileSync(join(slot,"remerged"),merged.stdout);
 const target=join(worktree,row.path),now=existsSync(target)?readFileSync(target):Buffer.alloc(0),written=row.kind==="merged"?readFileSync(join(slot,"merged")):readFileSync(join(slot,"current"));
 let action;
 if(merged.status!==0)action=`reconflict(${merged.status})`;
 else if(sha(merged.stdout)===sha(written))action="same-result";
 else if(sha(now)!==sha(written))action="changed-meanwhile-untouched";
 else if(mode!=="write")action="would-fix";
 else{const tmp=join(slot,"fix.tmp");writeFileSync(tmp,merged.stdout);renameSync(tmp,target);action="fixed";}
 console.log(`${action} ${row.kind} ${source} ${slot} | ${row.path}`);
}
