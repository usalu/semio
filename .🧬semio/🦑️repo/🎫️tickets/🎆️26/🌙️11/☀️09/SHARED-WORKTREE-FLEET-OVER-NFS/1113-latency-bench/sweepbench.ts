import {lstatSync} from "node:fs";
import {join} from "node:path";
const [module,worktree]=process.argv.slice(2);
const replica=await import(`${module}/🔁️replica/🟦️.ts`),boundary=await import(`${module}/🚧️boundary/🟦️.ts`),tiers=await import(`${module}/🎣️tiers/🟦️.ts`);
const book=new replica.ReplicaBookV1(),table=new tiers.DirectoryTableV1(book);
const ignore=boundary.compileIgnoreV1("ignored/\n*.log\n");
await boundary.scanSharedV1(worktree,ignore,undefined,()=>{},(path:string,mtime:number)=>{table.set(path,mtime);});
console.log("dirs",table.size);
const absolute=(root:string,path:string):string=>join(root,...path.split("/"));
const pass=(label:string,body:(id:number,name:string)=>void)=>{
 for(let round=0;round<3;round++){
  const c0=process.cpuUsage(),t0=performance.now();
  table.begin();let count=0;
  for(let id=table.next();id>=0;id=table.next()){body(id,table.nameAt(id));count++;}
  const c=process.cpuUsage(c0),w=performance.now()-t0;
  console.log(label,"wall ms",w.toFixed(0),"cpu ms",((c.user+c.system)/1000).toFixed(0),"per dir us",(w*1000/count).toFixed(2));
 }
};
pass("A absolute()+lstatSync+try",(id,name)=>{try{lstatSync(name===""?worktree:absolute(worktree,name)).mtimeMs!==table.mtimeAt(id);}catch{}});
const prefix=worktree+"/";
pass("B concat+lstatSync+try",(id,name)=>{try{lstatSync(name===""?worktree:prefix+name).mtimeMs!==table.mtimeAt(id);}catch{}});
pass("C concat+lstatSync throwIfNoEntry:false",(id,name)=>{const info=lstatSync(name===""?worktree:prefix+name,{throwIfNoEntry:false});info===undefined||info.mtimeMs!==table.mtimeAt(id);});
pass("D name decode only",(id,name)=>{});
