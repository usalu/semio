import {readFileSync,readdirSync,statSync,existsSync,writeFileSync,mkdirSync,renameSync,rmSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
const [worktree,store,since,lostSince,mode]=process.argv.slice(2),lostRoot=join(worktree,".cache/f/lost"),work="C:/b/reconcile";
const sha=bytes=>createHash("sha256").update(bytes).digest("hex");
const entries=readdirSync(lostRoot).filter(d=>statSync(join(lostRoot,d)).mtimeMs>=Date.parse(lostSince)).flatMap(d=>{
 const walk=p=>statSync(join(lostRoot,d,p)).isDirectory()?readdirSync(join(lostRoot,d,p)).flatMap(n=>walk(p?`${p}/${n}`:n)):[p];
 return walk("").map(path=>({dir:d,path}));
}).filter(e=>process.env.PATH_FILTER===undefined||process.env.PATH_FILTER.split(",").some(f=>e.path.includes(f)));
const wanted=new Set(entries.map(e=>e.path)),history=new Map();
for(const origin of readdirSync(join(store,"v1/origins"))){
 const segments=join(store,"v1/origins",origin,"segments");
 if(!existsSync(segments))continue;
 for(const name of readdirSync(segments).filter(n=>n.endsWith(".jsonl"))){
  const file=join(segments,name);
  if(statSync(file).mtimeMs<Date.parse(since))continue;
  for(const line of readFileSync(file,"utf8").split("\n")){
   if(!line.includes('"put"'))continue;
   const at=line.indexOf('"path":"'),path=at<0?"":JSON.parse(line).path;
   if(!wanted.has(path))continue;
   const event=JSON.parse(line),list=history.get(path)??[];list.push(event);history.set(path,list);
  }
 }
}
const lines=bytes=>bytes.toString("utf8").replace(/\r/g,"").split("\n");
const distance=(a,b)=>{const x=lines(a),y=new Set(lines(b));let d=0;for(const l of x)if(!y.has(l))d++;const xs=new Set(x);for(const l of lines(b))if(!xs.has(l))d++;return d;};
mkdirSync(work,{recursive:true});
const latestByPath=new Map();
for(const entry of entries){const held=latestByPath.get(entry.path);if(held===undefined||entry.dir>held.dir)latestByPath.set(entry.path,entry);}
for(const entry of entries){
 const lostFile=join(lostRoot,entry.dir,entry.path),local=readFileSync(lostFile),target=join(worktree,entry.path),current=existsSync(target)?readFileSync(target):null,known=history.get(entry.path)??[];
 let verdict;
 if(current!==null&&sha(local)===sha(current))verdict="same-as-current";
 else if(known.some(e=>e.sha256===sha(local)))verdict="already-published";
 else if(latestByPath.get(entry.path)!==entry)verdict="superseded-by-later-lost";
 else if(current===null)verdict="unique-target-gone";
 else{
  const candidates=known.filter(e=>existsSync(join(store,"v1/blobs",e.sha256.slice(0,2),e.sha256))).map(e=>({e,d:distance(readFileSync(join(store,"v1/blobs",e.sha256.slice(0,2),e.sha256)),local)})).sort((a,b)=>a.d-b.d||(a.e.hlc<b.e.hlc?1:-1));
  const ancestor=candidates[0]?.e;
  if(ancestor===undefined)verdict="unique-no-ancestor";
  else{
   const slot=join(work,sha(entry.dir+entry.path).slice(0,12));mkdirSync(slot,{recursive:true});
   writeFileSync(join(slot,"local"),local);writeFileSync(join(slot,"ancestor"),readFileSync(join(store,"v1/blobs",ancestor.sha256.slice(0,2),ancestor.sha256)));writeFileSync(join(slot,"current"),current);
   const merged=spawnSync("git",["merge-file","-p","--diff3","-L","local","-L","ancestor","-L","current",join(slot,"local"),join(slot,"ancestor"),join(slot,"current")],{maxBuffer:1<<28});
   writeFileSync(join(slot,"merged"),merged.stdout);
   if(merged.status!==0)verdict=`conflict(${merged.status}) ${slot}`;
   else if(sha(merged.stdout)===sha(current))verdict="merge-equals-current";
   else if(mode!=="write")verdict=`would-merge ${slot}`;
   else if(sha(readFileSync(target))!==sha(current))verdict="changed-meanwhile-untouched";
   else{const tmp=join(work,`${sha(merged.stdout)}.tmp`);writeFileSync(tmp,merged.stdout);renameSync(tmp,target);verdict=`merged ${slot}`;}
  }
 }
 console.log(`${verdict} | ${entry.dir} | ${entry.path}`);
}
