import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {availableParallelism} from "node:os";
import {existsSync,mkdirSync,mkdtempSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";

const root=(()=>{for(let path=import.meta.dir;;path=dirname(path)){if(existsSync(join(path,".git")))return path;if(dirname(path)===path)throw Error("repository root not found");}})(),framework=join(root,"🧰️framework"),modules=join(framework,"🔨️modules/🏃️process"),library=join(framework,"🛍️products/🦑️repo/🔨️modules/📚️library");
const {withBuildTestInvocationV1}=await import(join(modules,"🧪️testing/📥️invocation/🟦️.ts")),capacity=await import(join(modules,"📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts")),{withCountedBunCapacity}=await import(join(library,"⚡️caching/📦️bundle/🟦️.ts"));
const orderPolicy=(await import(join(modules,"🔒️leases/🧭️order/🔣️policy.json"),{with:{type:"json"}})).default;
const output=process.env.SEMIO_TEST_ARTIFACT_DIR!,results=process.env.SLOT_D_OUT!,targets=Number(process.env.SLOT_D_TARGETS??"3"),reps=Number(process.env.SLOT_D_REPS??"3"),entry=join(root,"node_modules/typescript/lib/typescript.js");
mkdirSync(output,{recursive:true});mkdirSync(results,{recursive:true});
const distinctClaimants=join(output,"lease-order-policy-live.json");writeFileSync(distinctClaimants,JSON.stringify({...orderPolicy,fairness:{...orderPolicy.fairness,reentrantShared:false}}));process.env.SEMIO_LEASE_ORDER_POLICY=distinctClaimants;process.env.SEMIO_LEASE_ORDER="off";
const bundle=(environment:Readonly<Record<string,string|undefined>>,outfile:string)=>new Promise<void>((resolve,reject)=>{
 const child=spawn(process.execPath,["build",entry,"--minify","--target=bun",`--outfile=${outfile}`],{env:environment as NodeJS.ProcessEnv,stdio:["ignore","ignore","inherit"]});child.on("error",reject);child.on("exit",code=>code===0?resolve():reject(Error("bun build exited "+code)));
});
const sweep=(spans:readonly {start:number;end:number}[])=>spans.reduce((peak,span)=>Math.max(peak,spans.filter(other=>other.start<span.end&&span.start<other.end).length),0);
async function measure(mode:"counted"|"whole-host"):Promise<{mode:string;wallMs:number;peak:number;spans:{target:number;start:number;end:number;parallelism:number}[]}>{
 return withBuildTestInvocationV1(async original=>{
  const directory=mkdtempSync(join(output,"live-"+mode+"-")),spans:{target:number;start:number;end:number;parallelism:number}[]=[],begun=Date.now();
  const control=(name:string)=>capacity.scriptCompilerAdmissionControlV1(original,"slot-d:"+name,1_800_000);
  const work=async(target:number,environment:Readonly<Record<string,string|undefined>>,parallelism:number)=>{const start=Date.now();for(let rep=0;rep<reps;rep++)await bundle(environment,join(directory,`out-${target}-${rep}.js`));spans.push({target,start:start-begun,end:Date.now()-begun,parallelism});};
  const base={...process.env,LANG:"en_US.UTF-8",SEMIO_COMPILER_CAPACITY_HELD:undefined};
  await Promise.all(Array.from({length:targets},(_,target)=>mode==="counted"
   ?withCountedBunCapacity(directory,control(mode+target),environment=>work(target,environment,Number(environment.SEMIO_COMPILER_PARALLELISM)),base)
   :capacity.withCompilerCapacityScopeV1({request:{kind:"whole-host"},directory,control:control(mode+target),environment:base},facts=>work(target,base,facts.parallelism))));
  const row={mode,wallMs:Date.now()-begun,peak:sweep(spans),spans};writeFileSync(join(results,`bun-overlap-${mode}.json`),JSON.stringify(row,null,1));console.log(`[slot-d] ${mode}: ${targets} bundling targets x ${reps} bundles, wall ${(row.wallMs/1000).toFixed(1)} s, peak concurrency ${row.peak}, slots ${spans.map(span=>span.parallelism).join("/")}`);return row;
 });
}
test("three bundling targets overlap inside the counted Bun slot and finish earlier than behind the old exclusive whole-host request",async()=>{
 const cores=availableParallelism();if(cores<4)return;
 const counted=await measure("counted"),whole=await measure("whole-host");
 const oracle=spawnSync("python3",["-I","-c","import json,sys\nrows=json.load(sys.stdin)\nout={}\nfor r in rows:\n    s=sorted(r['spans'],key=lambda x:x['start'])\n    out[r['mode']]={'peak':max(sum(1 for o in s if o['start']<x['end'] and x['start']<o['end']) for x in s),'wall':r['wallMs']}\nprint(json.dumps(out))"],{input:JSON.stringify([counted,whole]),encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);const verdict=JSON.parse(oracle.stdout);
 console.log(`[slot-d] oracle ${oracle.stdout.trim()} cores=${cores}`);
 expect(verdict["counted"].peak).toBe(counted.peak);expect(verdict["whole-host"].peak).toBe(whole.peak);
 expect(whole.peak,"the old whole-host request never overlaps").toBe(1);expect(counted.peak,"the counted slot overlaps").toBeGreaterThanOrEqual(2);
 console.log(`[slot-d] wall before (whole-host) ${(whole.wallMs/1000).toFixed(1)} s, after (counted) ${(counted.wallMs/1000).toFixed(1)} s`);
 expect(counted.wallMs).toBeLessThan(whole.wallMs);
},1_800_000);
