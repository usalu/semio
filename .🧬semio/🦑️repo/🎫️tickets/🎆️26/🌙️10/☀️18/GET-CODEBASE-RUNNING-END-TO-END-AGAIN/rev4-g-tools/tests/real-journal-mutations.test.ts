import {afterAll,beforeAll,expect,test} from "bun:test";
import {appendFileSync,linkSync,mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,truncateSync,utimesSync,writeFileSync,readdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readChangeStateV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {openVouchV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🪪️vouch/🟦️.ts";
import {startChangeJournalServiceV1} from "../🟦️.ts";

const watcher=process.env.SEMIO_REPO_WATCH_BINARY??"";
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"rev4j-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));

test.skipIf(!watcher)("[rev4] REAL journal: mutations a vouch must see",async()=>{
 const root=join(sandbox,"ws"),directory=join(sandbox,"journal"),outside=join(sandbox,"outside");
 mkdirSync(join(root,"pkg"),{recursive:true});mkdirSync(join(root,"real"),{recursive:true});mkdirSync(outside,{recursive:true});
 writeFileSync(join(root,"pkg/a.json"),"{\"a\":1}");writeFileSync(join(root,"pkg/B.ts"),"export const b=1;");writeFileSync(join(root,"pkg/same.txt"),"0123456789");
 writeFileSync(join(root,"pkg/h1.txt"),"hard");linkSync(join(root,"pkg/h1.txt"),join(root,"pkg/h2.txt"));
 writeFileSync(join(root,"real/x.json"),"{}");symlinkSync(join(root,"real"),join(root,"alias"));
 mkdirSync(join(outside,"moved/inner"),{recursive:true});writeFileSync(join(outside,"moved/inner/package.json"),"{}");
 const environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
 const client=createChangeJournalClientV1({directory,root,environment,start:()=>startChangeJournalServiceV1({directory,root,environment})});
 try{
  const paths=["pkg/a.json","pkg/B.ts","pkg/same.txt","pkg/h1.txt","pkg/h2.txt","alias/x.json","real/x.json","moved","moved/inner","moved/inner/package.json","pkg","real","alias","tmp/new.json"];
  let first;
  for(let attempt=0;attempt<6;attempt++){first=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"c",key:"k"});if(first.mode==="cursor")break;await new Promise(r=>setTimeout(r,800));}
  expect(first!.mode).toBe("cursor");
  await first!.settled(paths);await first!.seal();
  writeFileSync(join(root,"pkg/a.json.tmp"),"{\"a\":2}");renameSync(join(root,"pkg/a.json.tmp"),join(root,"pkg/a.json"));
  renameSync(join(root,"pkg/B.ts"),join(root,"pkg/b.ts"));
  const before=Date.now()/1000-100;writeFileSync(join(root,"pkg/same.txt"),"9876543210");utimesSync(join(root,"pkg/same.txt"),before,before);
  appendFileSync(join(root,"pkg/h2.txt"),"!");
  writeFileSync(join(root,"real/x.json"),"{\"changed\":true}");
  renameSync(join(outside,"moved"),join(root,"moved"));
  mkdirSync(join(root,"tmp"));writeFileSync(join(root,"tmp/new.json"),"{}");
  const second=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"c",key:"k"});
  expect(second.mode).toBe("journal");
  const kept=[...second.skippable(paths)].sort();
  console.log("[rev4] vouched after mutations:",JSON.stringify(kept));
  for(const must of ["pkg/a.json","pkg/B.ts","pkg/same.txt","pkg/h2.txt","real/x.json","moved","moved/inner","moved/inner/package.json","tmp/new.json"])expect(kept,must).not.toContain(must);
  expect(kept,"hard link alias h1 (same inode as the written h2)").not.toContain("pkg/h1.txt");
  expect(kept,"path through a symlinked directory whose target changed").not.toContain("alias/x.json");
 }finally{const pid=readChangeStateV1(directory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}}
},120000);

test.skipIf(!watcher)("[rev4] REAL journal: edit immediately before the fence is never lost (50 rounds)",async()=>{
 const root=join(sandbox,"ws2"),directory=join(sandbox,"journal2");
 mkdirSync(join(root,"d"),{recursive:true});
 const environment={...process.env,SEMIO_REPO_WATCH_BINARY:watcher,SEMIO_REPO_CHANGES:"on"};
 const client=createChangeJournalClientV1({directory,root,environment,start:()=>startChangeJournalServiceV1({directory,root,environment})});
 try{
  for(let i=0;i<20;i++)writeFileSync(join(root,"d/f"+i+".json"),"0");
  const names=Array.from({length:20},(_,i)=>"d/f"+i+".json");
  let first;
  for(let attempt=0;attempt<6;attempt++){first=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"c",key:"k"});if(first.mode==="cursor")break;await new Promise(r=>setTimeout(r,800));}
  expect(first!.mode).toBe("cursor");await first!.settled(names);await first!.seal();
  let missed=0;
  for(let round=0;round<50;round++){
   const target=names[round%20]!;writeFileSync(join(root,target),String(round+1));
   const next=await openVouchV1({client,directory:join(directory,"vouch"),consumer:"c",key:"k"});
   if(next.mode!=="journal"){continue;}
   if(next.skippable([target]).has(target))missed++;
   await next.seal();
  }
  expect(missed).toBe(0);
 }finally{const pid=readChangeStateV1(directory)?.pid;if(pid)try{process.kill(pid,"SIGTERM");}catch{}}
},120000);
