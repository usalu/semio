import{expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";

/** 🧮️ Law: the farm never runs two jobs whose known footprint adds up to more than the plan limit. A job carries `knownMegabytes` (the cost table of the owner) or the farm has seen its footprint on this machine; a job that would take the running ones past the limit waits until they ended (a heavy owner runs alone), a job of unknown cost is admitted as before, so no try is cut by its own lanes (two owners of 5 GB beside 6.5 GB made every first try of a 16 GB worker fail). */
const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"..","🧯️cap","🧫️fixtures","🟨️.mjs")).href;
const open=(limitMegabytes:number,footprint=1)=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:3,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",policy:{...GRAPH_FARM_POLICY_V1,footprintMilliseconds:25,holdRecheckMilliseconds:20},footprintOf:async()=>footprint*1048576,limitMegabytes}as never);
const run=async(limit:number,known:(number|undefined)[])=>{
 const dir=mkdtempSync(join(tmpdir(),"farm-admission-")),pool=open(limit);
 try{
  const answers:any[]=await Promise.all(known.map((value,n)=>pool.run({n,name:"owner-"+n,megabytes:[1],delay:400,dir,...value===undefined?{}:{knownMegabytes:value}})));
  return answers.map(answer=>answer.reply as{n:number;start:number;end:number});
 }finally{await pool.close();rmSync(dir,{recursive:true,force:true});}
};
const overlaps=(rows:{start:number;end:number}[])=>{let count=0;for(const a of rows)for(const b of rows)if(a!==b&&a.start<b.end&&b.start<a.end)count++;return count/2;};

test("two owners of 600 MB known footprint beside a limit of 1000 MB never run together, 400 MB owners do, and owners of unknown cost are admitted as before",async()=>{
 const heavy=await run(1000,[600,600,600]);
 expect(heavy.length).toBe(3);expect(overlaps(heavy),"600 + 600 is over 1000: they run one after the other").toBe(0);
 const light=await run(1000,[400,400]);
 expect(overlaps(light),"400 + 400 fits").toBe(1);
 const unknown=await run(1000,[undefined,undefined,undefined]);
 expect(overlaps(unknown),"no known cost: no waiting").toBeGreaterThan(0);
 const alone=await run(1000,[5000,100,100]);
 expect(overlaps(alone.filter(row=>row.n===0).concat(alone.filter(row=>row.n!==0))).valueOf(),"a job above the limit by itself still runs, alone").toBeLessThanOrEqual(1);
 const first=alone.find(row=>row.n===0)!;
 for(const other of alone)if(other.n!==0)expect(other.start>=first.end||other.end<=first.start,"owner 0 runs alone").toBe(true);
},120000);

test("an owner whose footprint the farm has measured counts with it afterwards, without any declared cost",async()=>{
 const dir=mkdtempSync(join(tmpdir(),"farm-admission-")),pool=open(1000,700);
 try{
  await pool.run({n:0,name:"measured",megabytes:[1],delay:150,dir});
  const answers:any[]=await Promise.all([1,2].map(n=>pool.run({n,name:"measured",megabytes:[1],delay:300,dir})));
  const rows=answers.map(answer=>answer.reply as{start:number;end:number});
  expect(overlaps(rows),"700 + 700 is over 1000").toBe(0);
 }finally{await pool.close();rmSync(dir,{recursive:true,force:true});}
},120000);
