/** 🧱️ Contract: a look job answers only when its slowest owner is done, so an owner whose look computes the assembly authority of a native or release owner (minutes under load) is a batch of its own and never holds five plain owners; with no such owner the batches are the plain ones, every name is in exactly one batch in its lane's order, and an independent python implementation batches every fixture case the same.
 * @see ../../🟨️.mjs `lookBatchesV1` */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import fixture from"./🔣️.json";
import{createLookQueueV1,lookBatchesV1}from"../../🟨️.mjs";

test("the fixture cases batch as declared and keep every name once in lane order",()=>{
 for(const row of fixture.cases){
  const got=lookBatchesV1(row.lanes,row.size,name=>row.isolated.includes(name));
  expect(got,row.name).toEqual(row.expected);
  expect(got.flat().sort(),row.name).toEqual(row.lanes.flat().sort());
 }
});
test("an independent python implementation batches every case the same",()=>{
 const ran=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(ran.status,ran.stderr).toBe(0);
 const said=JSON.parse(ran.stdout);
 for(const row of fixture.cases)expect(said[row.name],row.name).toEqual(row.expected);
});
test("the queue builds its jobs from the isolated batches and the read passes the native test",async()=>{
 const jobs:string[][]=[],names=["p1","p2","p3","nat","p4","p5","p6","p7","p8","p9","p10","p11"];
 const queue=createLookQueueV1({farm:{width:()=>1,observe:async(job:any)=>{jobs.push(job.owners.map((owner:any)=>owner.name));return{looks:job.owners.map((owner:any)=>({name:owner.name}))};}},jobOf:(name:string)=>({name}),refuse:(name:string,reason:string)=>Error(name+reason)});
 queue.start([names],name=>name==="nat");
 await Promise.all(names.map(name=>queue.ask(name)));
 expect(jobs.filter(job=>job.includes("nat"))).toEqual([["nat"]]);
 expect(jobs.flat().sort()).toEqual([...names].sort());
 expect(Math.max(...jobs.map(job=>job.length))).toBe(2);
 const source=readFileSync(new URL("../../../🟨️.mjs",import.meta.url),"utf8");
 expect(source).toContain("lookQueue?.start([...lanes,split.heavy],name=>");
 expect(source).toContain('"Cargo.toml"))||Object.values(owner.json.targets??{}).some(target=>target.metadata?.semio?.release)');
});
