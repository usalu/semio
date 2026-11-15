import {appendFileSync,mkdirSync,renameSync,rmSync,writeFileSync,existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
const [module,worktree,store,configText,scenariosText,samplesText="3"]=process.argv.slice(2);
const config=JSON.parse(configText!) as Record<string,number|boolean>,scenarios=scenariosText!.split(","),samples=+samplesText;
const replica=await import(`${module}/🔁️replica/🟦️.ts`),journal=await import(`${module}/📒️journal/🟦️.ts`);
let tiers:any;try{tiers=await import(`${module}/🎣️tiers/🟦️.ts`);}catch{}
const options={worktree,storeRoot:store,originId:"b1",groupId:"g1",pollMilliseconds:250,scanSeconds:15,debounceMilliseconds:150,persistMilliseconds:5000,watch:false,...config};
const controller=new AbortController(),events:any[]=[];
const started=performance.now();
const running=replica.runV1(options,controller.signal,(e:any)=>{events.push(e);if(e.event==='error'||e.event==='blocked'||e.event==='refused'||e.event==='unstable'||e.event==='skipped')console.error('EVENT',JSON.stringify(e));});
const head=async()=>{const h=await journal.readHeadV1(store,"b1");return h?`${h.segment}:${h.bytes}`:"";};
const until=async(test:()=>Promise<boolean>|boolean,ms:number)=>{const end=performance.now()+ms;while(performance.now()<end){if(await test())return true;await Bun.sleep(2);}return false;};
await until(()=>events.some(e=>e.event==="published"||e.event==="scan")||performance.now()-started>0,1);
const quiet=async(ms=2500)=>{let last=await head(),at=performance.now();while(performance.now()-at<ms){await Bun.sleep(50);const now=await head();if(now!==last){last=now;at=performance.now();}}};
console.error("settling after start...");
await quiet(4000);
const rnd=(n:number)=>Math.floor(Math.random()*n);
const coldFile=()=>join(worktree,`pkg${rnd(100)}`,`mod${rnd(25)}`,`sub${rnd(20)}`,`file${rnd(2)}.txt`);
const hint=tiers?.writeHintV1??(async()=>{});
const results:Record<string,number[]>={};
const sample=async(name:string,act:()=>Promise<void>|void,limit:number)=>{
 const before=await head();
 await act();
 const t0=performance.now();
 const ok=await until(async()=>(await head())!==before,limit);
 const ms=ok?performance.now()-t0:-1;
 (results[name]??=[]).push(Math.round(ms));
 console.error(name,Math.round(ms));
 await quiet();
};
for(let i=0;i<samples;i++){
 for(const scenario of scenarios){
  const limit=scenario.endsWith("cold")?200_000:30_000;
  if(scenario==="hint-edit-cold"){const f=coldFile();await sample(scenario,async()=>{appendFileSync(f,`edit ${Math.random()}\n`);await hint(worktree,[f]);},limit);}
  else if(scenario==="hint-create"){const f=join(worktree,`pkg${rnd(100)}`,`mod${rnd(25)}`,`new-${Date.now()}.txt`);await sample(scenario,async()=>{writeFileSync(f,"x\n");await hint(worktree,[f]);},limit);}
  else if(scenario==="edit-hot"){const f=coldFile();appendFileSync(f,`prime ${Math.random()}\n`);await hint(worktree,[f]);await quiet(1500);await sample(scenario,()=>{appendFileSync(f,`hot ${Math.random()}\n`);},limit);}
  else if(scenario==="edit-published"){const f=join(worktree,`pkg${rnd(100)}`,`mod${rnd(25)}`,`pub-${Date.now()}.txt`);writeFileSync(f,"first\n");const b=await head();await until(async()=>(await head())!==b,200_000);await quiet(1500);await sample(scenario,()=>{appendFileSync(f,`again ${Math.random()}\n`);},limit);}
  else if(scenario==="create"){const f=join(worktree,`pkg${rnd(100)}`,`mod${rnd(25)}`,`sub${rnd(20)}`,`created-${Date.now()}.txt`);await sample(scenario,()=>{writeFileSync(f,"created\n");},200_000);}
  else if(scenario==="delete"){const f=join(worktree,`pkg${rnd(100)}`,`mod${rnd(25)}`,`sub${rnd(20)}`,`del-${Date.now()}.txt`);writeFileSync(f,"d\n");const b=await head();await until(async()=>(await head())!==b,200_000);await quiet(1500);await sample(scenario,()=>{rmSync(f);},200_000);}
  else if(scenario==="rename-save"){const f=coldFile(),t=`${f}.tmp-${Date.now()}`;await sample(scenario,()=>{writeFileSync(t,`saved ${Math.random()}\n`);renameSync(t,f);},200_000);}
  else if(scenario==="edit-cold"){const f=coldFile();await sample(scenario,()=>{appendFileSync(f,`cold ${Math.random()}\n`);},limit);}
 }
}
controller.abort();await running.catch(()=>{});
const summary=Object.fromEntries(Object.entries(results).map(([k,v])=>{const s=[...v].sort((a,b)=>a-b);return [k,{samples:v,min:s[0],median:s[Math.floor(s.length/2)],max:s.at(-1)}];}));
console.log(JSON.stringify(summary));
