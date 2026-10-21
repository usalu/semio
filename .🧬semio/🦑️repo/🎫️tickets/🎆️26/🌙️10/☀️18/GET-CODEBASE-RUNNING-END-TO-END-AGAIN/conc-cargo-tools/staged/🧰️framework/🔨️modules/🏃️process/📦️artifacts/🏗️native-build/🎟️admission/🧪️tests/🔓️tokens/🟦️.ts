import{test,expect}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join,resolve}from"node:path";
import{fileURLToPath}from"node:url";
import{startAdmissionBrokerV1}from"../../🟦️.ts";
import{createJobserverV1,defaultJobserverStyleV1}from"../../../🎰️jobserver/🟦️.ts";

/** 🔓️ N scheduler processes share K jobserver tokens with a Cargo that hoards every token it can read, while the broker admits only one unit at a time, so all but the first unit are refused. Handing a token back on a refusal lets Cargo spend it while the unit can never reacquire it: no unit finishes. Keeping the token finishes all N. `SEMIO_TEST_SCHEDULER` names a prebuilt scheduler (the old one proves the test fails). */
const manifest=resolve(fileURLToPath(new URL("../../📦️packages/🦀️rust/Cargo.toml",import.meta.url)));
const reserveBytes=6*2**30,pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
async function schedulerBinary(scratch:string):Promise<string|undefined>{
 if(process.env.SEMIO_TEST_SCHEDULER)return process.env.SEMIO_TEST_SCHEDULER;
 if(process.platform==="win32"||!existsSync(manifest))return undefined;
 const built=spawnSync("cargo",["build","--release","--manifest-path",manifest,"--bins"],{cwd:scratch,env:{...process.env,CARGO_TARGET_DIR:join(scratch,"target")},encoding:"utf8"});
 if(built.status!==0)throw Error("Scheduler build failed: "+built.stderr);
 return join(scratch,"target","release","semio-build-scheduler");
}

async function drive(scheduler:string,scratch:string,units:number,tokens:number,hoard:number,boundMs:number):Promise<{finished:number;admitted:number;deadlocked:boolean}>{
 const rustc=join(scratch,"fake-rustc.sh"),sources=join(scratch,"src"),jobs=mkdtempSync(join(scratch,"j-"));mkdirSync(sources,{recursive:true});
 writeFileSync(rustc,"#!/bin/sh\nsleep 0.1\nexit 0\n");chmodSync(rustc,0o755);
 const broker=await startAdmissionBrokerV1({physicalBytes:reserveBytes+100*2**20,history:new Map()});broker.registerPass({id:"p",profile:"dev",target:"host"});
 const pool=await createJobserverV1({version:1,tokens,style:defaultJobserverStyleV1()},jobs),children:ReturnType<typeof spawn>[]=[];
 let finished=0;
 const all=new Promise<void>(done=>{
  void(async()=>{
   for(let index=0;index<units;index++){
    const slot=await pool.acquire();
    const child=spawn(scheduler,[rustc,"--crate-name","u"+index,"--crate-type","lib","--emit=link","-C","extra-filename=-"+index,"--out-dir",join(scratch,"out"),join(sources,"s"+index+".rs")],{env:{...process.env,...pool.environment(),SEMIO_BUILD_BROKER:broker.endpoint,SEMIO_BUILD_PASS:"p",CARGO_PKG_NAME:"p"+index},stdio:"ignore"});
    children.push(child);child.on("close",()=>{slot.release();if(++finished===units)done();});
   }
   for(let extra=0;extra<hoard;extra++)pool.acquire().then(()=>undefined,()=>undefined);
  })();
 });
 let deadlocked=false;
 await Promise.race([all,pause(boundMs).then(()=>{deadlocked=true;})]);
 const admitted=broker.units().length;
 for(const child of children)child.kill("SIGKILL");
 await broker.close();pool.close();
 return{finished,admitted,deadlocked};
}

for(const row of[{units:8,tokens:2},{units:12,tokens:3},{units:6,tokens:2}])test(`${row.units} refused-then-admitted units, ${row.tokens} tokens and a token-hoarding Cargo finish`,async()=>{
 if(process.platform==="win32")return;
 const scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir(),"tk-")),scheduler=await schedulerBinary(scratch);
 if(!scheduler)return;
 const result=await drive(scheduler,scratch,row.units,row.tokens,row.units,12000);
 expect(result.deadlocked,`${result.finished}/${row.units} finished`).toBe(false);expect(result.finished).toBe(row.units);expect(result.admitted).toBe(row.units);
},60000);

test("without a hoarding Cargo the same workload finishes too (the hoard is what the hand-back could not survive)",async()=>{
 if(process.platform==="win32")return;
 const scratch=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir(),"tk-")),scheduler=await schedulerBinary(scratch);
 if(!scheduler)return;
 const result=await drive(scheduler,scratch,10,3,0,12000);expect(result.deadlocked).toBe(false);expect(result.finished).toBe(10);
},60000);
