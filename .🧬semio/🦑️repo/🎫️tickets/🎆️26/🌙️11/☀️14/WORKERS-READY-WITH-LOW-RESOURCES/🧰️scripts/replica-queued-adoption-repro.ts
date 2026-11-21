import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
const source=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica");
const {closeReplicaV1,followOnceV1,openReplicaV1,runV1,seedV1}=await import(join(source,"🟦️.ts"));
const {appendEventsV1,formatHlcV1,storeBlobV1}=await import(join(source,"..","📒️journal","🟦️.ts"));
const {codeDigestV1,readAttestationV1}=await import(join(source,"..","🩺️attest","🟦️.ts"));
const deadline=async(check:()=>Promise<boolean>|boolean)=>{
 const until=Date.now()+8000;
 while(!await check()){
  if(Date.now()>until)throw Error("fixture did not reach the expected replica state");
  await Bun.sleep(15);
 }
};

test("the live replica restart cap carries an unrelated pending event through close and reopen",async()=>{
 const root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"queued-adopt-")),worktree=join(root,"w"),store=join(root,"s"),code=join(root,"c");
 for(const path of [worktree,store,code])mkdirSync(path);
 const module=join(code,"🟦️.ts"),controller=new AbortController(),events:{event:string}[]=[],checks:string[]=[];
 const platformAbsent={present:false,head:null,commit:null,indexTree:null};
 let clock=Date.now(),running:Promise<void>|undefined,done=false;
 try{
  writeFileSync(join(worktree,"owned.txt"),"baseline\n");
  writeFileSync(module,"export const candidate = 1;\n");
  const old=await codeDigestV1(code);
  const options={worktree,storeRoot:store,originId:"g1-m01",groupId:"g1",now:()=>clock,pollMilliseconds:10,persistMilliseconds:0,watch:false,attestMilliseconds:20,restartStableMilliseconds:50,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,codeRoot:code,gitState:async()=>platformAbsent,gitView:async()=>null,selfCheck:async(digest:string)=>{checks.push(digest);return {ok:true as const};},logFiles:[] as string[]};
  await seedV1(options);
  const path="unrelated.txt",payload="replicated eventual payload\n",sha256=createHash("sha256").update(payload).digest("hex");
  const entry={version:1 as const,hlc:formatHlcV1({ms:clock,counter:0},"g1-m07"),operation:"put" as const,path,sha256,bytes:Buffer.byteLength(payload),executable:false,base:null};
  const head=await appendEventsV1(store,"g1-m07",[entry]);
  running=runV1(options,controller.signal,(event:{event:string})=>events.push(event)).finally(()=>{done=true;});
  clock+=100;
  await deadline(async()=>{clock+=20;const row=await readAttestationV1(store,options.originId);return row?.code===old&&row.queue.incoming===1;});
  writeFileSync(module,"export const candidate = 2;\n");
  const next=await codeDigestV1(code);
  for(let round=0;checks.length===0&&round<50;round++){clock+=50;await Bun.sleep(20);}
  expect(old).not.toBe(next);expect(checks).toEqual([next]);
  clock+=20_000;
  await Bun.sleep(100);
  expect(done).toBe(false);expect(events.some(event=>event.event==="restarting")).toBe(false);
  clock+=300_100;
  await deadline(()=>done);
  await running;
  expect(events.some(event=>event.event==="restarting")).toBe(true);
  const reopened=await openReplicaV1(options);
  try{
   expect(reopened.queue.size).toBe(1);
   expect(reopened.queue.get(path)).toMatchObject({operation:"put",sha256,hlc:entry.hlc});
   expect(reopened.cursors["g1-m07"]).toEqual({segment:head.segment,bytes:head.bytes});
   const input=join(root,"source.txt");writeFileSync(input,payload);
   expect((await storeBlobV1(store,"g1-m07",input)).sha256).toBe(sha256);
   clock+=60_000;
   await followOnceV1(reopened);
   expect(reopened.queue.size).toBe(0);
   expect(readFileSync(join(worktree,path),"utf8")).toBe(payload);
  }finally{await closeReplicaV1(reopened);}
 }finally{controller.abort();await running?.catch(()=>undefined);rmSync(root,{recursive:true,force:true});}
},20000);
