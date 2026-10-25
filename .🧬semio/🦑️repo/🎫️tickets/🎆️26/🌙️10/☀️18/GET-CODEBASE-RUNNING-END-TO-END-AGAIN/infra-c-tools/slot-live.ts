import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const root=process.cwd(),mode=process.argv[2]??"real",limitSeconds=Number(process.argv[3]??"1800"),out=process.argv[4];
const lib=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"),process_=join(root,"🧰️framework/🔨️modules/🏃️process");
const bazel=join(lib,"🔧️tools/🏗️bazel"),sessions=await import(join(bazel,"🧵️sessions/🟦️.ts")),port_=await import(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"));
const capacity=await import(join(lib,"🌡️host-capacity/🟦️.ts")),hostGate=await import(join(process_,"🏗️build/🌡️host-capacity/🟦️.ts"));
const log=(line:string)=>{const text=`${new Date().toISOString().slice(11,19)} ${line}`;console.log(text);if(out)writeFileSync(out,text+"\n",{flag:"a"});};
const spareNow=async():Promise<number>=>{const probe=capacity.createCapacityControlV1();try{return (await hostGate.readHostCapacityStatusV1({directory:hostGate.hostCapacityDirectoryV1(process.env),policy:capacity.BUILD_HOST_CAPACITY_POLICY_V1,control:probe.control})).evaluation.headroomMegabytes;}finally{probe.dispose();}};
const base=mkdtempSync(join(realpathSync(tmpdir()),"slot-live-")),directories={leases:join(base,"leases"),server:join(base,"p/server"),workspaces:join(base,"p/workspaces")};
for(const dir of Object.values(directories))mkdirSync(dir,{recursive:true});
const plan={slots:3,services:4},holders:{kill:()=>void;alive:()=>boolean}[]=[];
const holder=(slot:string)=>new Promise<void>((resolve,reject)=>{
 const script=join(base,`holder-${slot}.ts`);
 writeFileSync(script,`import {bazelSlotPortV1} from ${JSON.stringify(join(bazel,"🧵️sessions/🔌️port/🟦️.ts"))};const port=bazelSlotPortV1(${JSON.stringify(directories)});const claim=await port.claim(${JSON.stringify(slot)},new AbortController().signal);console.log(claim?"held":"busy");if(claim){await new Promise(done=>process.stdin.on("close",done).resume());claim();}`);
 const child=spawn(process.execPath,[script],{stdio:["pipe","pipe","inherit"]});holders.push({kill:()=>child.kill("SIGKILL"),alive:()=>child.exitCode===null&&child.signalCode===null});child.on("exit",(code,signal)=>log(`holder ${slot} exited code=${code} signal=${signal}`));
 child.stdout.setEncoding("utf8");child.stdout.on("data",text=>text.includes("held")?resolve():reject(Error("busy")));child.on("error",reject);
});
const rules=mode==="lenient"?{...sessions.BAZEL_SESSION_POLICY_V1,servers:{...sessions.BAZEL_SESSION_POLICY_V1.servers,growthSpareMegabytes:-1000000}}:sessions.BAZEL_SESSION_POLICY_V1;
let exit=1;
try{
 for(const slot of ["s0","s1","s2"])await holder(slot);
 log(`mode=${mode}: s0..s2 held by three real processes; lease directory ${directories.leases}; growth margin ${rules.servers.growthSpareMegabytes} MB, spare now ${await spareNow()} MB`);
 const port=port_.bazelSlotPortV1(directories,{write:line=>log("port: "+line),spare:spareNow});
 const controller=new AbortController(),started=Date.now();
 let result:Awaited<ReturnType<typeof sessions.acquireBazelSessionV1>>|undefined,failure:unknown;
 const waiting=sessions.acquireBazelSessionV1(port,plan,{language:"en",key:"00000000000000a1",signal:controller.signal,timeoutMilliseconds:limitSeconds*1000},rules).then((session:typeof result)=>{result=session;},(error:unknown)=>{failure=error;});
 let probes=0,gateFree=0,lastSpare=0;
 while(result===undefined&&failure===undefined&&Date.now()-started<limitSeconds*1000+5000){
  await new Promise(done=>setTimeout(done,2000));probes++;
  const probe=AbortSignal.timeout(1500);let free=false;try{const release=await port.gate(probe,"build");release();free=true;gateFree++;}catch{}
  const waiters=readdirSync(directories.server).filter(name=>name.startsWith("waiter-")).length;
  if(probes%5===1||!free){lastSpare=await spareNow();log(`probe ${probes}: gate free for others=${free}, waiter files=${waiters}, queued build waiters=${port.queued("build")}, spare ${lastSpare} MB`);}
 }
 await waiting;
 if(result){
  log(`RESULT: got slot ${result.slot} (${result.mode}) after ${Math.round((Date.now()-started)/1000)} s; gate was free for every probe: ${gateFree}/${probes}; waiter files left: ${readdirSync(directories.server).filter(name=>name.startsWith("waiter-")).length}`);
  exit=["s3","s4","s5","s6","s7","s8","s9","s10","s11"].includes(result.slot)&&gateFree===probes?0:2;await result.release();
 }else log(`RESULT: no slot after ${Math.round((Date.now()-started)/1000)} s (${String(failure)}); gate was free for every probe: ${gateFree}/${probes}; last spare ${lastSpare} MB`);
 if(!result&&gateFree===probes)exit=3;
}finally{
 for(const row of holders)row.kill();
 await new Promise(done=>setTimeout(done,500));
 if(existsSync(base))rmSync(base,{recursive:true,force:true});
}
process.exit(exit);
