/** Benchmark of the cross-pass taken memo against a full take of every owner, on real files (1310 owners, a repair read of five passes with three edits between passes).
 * Run: SEMIO_TEST_ARTIFACT_DIR=$HOME/Documents/semio/.cache/semio-test-artifacts repo tools exec --light bun -- ./<this file>
 * The "full" column observes every owner afresh (stat, read, sha, listing); "verify" is the owner answered from its retained rows by a row check (the in-process image path that exists today); "memo" is the replay of the new taken memo plus the take of the stale owners. */
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
import {createTakenMemoV1} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs";

const OWNERS=Number(process.env.OWNERS??1310),FILES=Number(process.env.FILES??24),PASSES=5,EDITS=3,SHARED=6;
type Owner={name:string;directory:string;files:string[]};

const root=await mkdtemp(join(tmpdir(),"pass-reuse-bench-"));
try{
 await mkdir(join(root,"shared"),{recursive:true});
 for(let at=0;at<SHARED;at++)await writeFile(join(root,"shared/s"+at+".txt"),"shared "+at);
 const owners:Owner[]=[];
 for(let at=0;at<OWNERS;at++){
  const directory="owner-"+at;await mkdir(join(root,directory),{recursive:true});
  const files:string[]=[];for(let file=0;file<FILES;file++){files.push(directory+"/f"+file+".txt");await writeFile(join(root,files.at(-1)!),"owner "+at+" file "+file+" ".repeat(200));}
  for(let shared=0;shared<SHARED;shared++)files.push("shared/s"+shared+".txt");
  owners.push({name:directory,directory,files});
 }
 const read=(owner:Owner)=>observation.captureObservedSourceInputsV1(async()=>{
  for(const file of owner.files)observation.observedFs.readFileSync(join(root,file),"utf8");
  observation.observedFs.readdirSync(join(root,owner.directory));
  return {name:owner.name,files:owner.files.length};
 },undefined,false);
 const memo=createTakenMemoV1({holds:rows=>observation.observationRowsHoldV1(rows as never),retain:rows=>observation.retainObservedRowsV1(rows as never)});
 const retained=new Map<string,{text:string;rows:readonly unknown[]}>();
 const clock=async<T>(run:()=>Promise<T>)=>{const started=performance.now(),cpu=process.cpuUsage(),value=await run(),used=process.cpuUsage(cpu);return{value,ms:Math.round(performance.now()-started),cpu:Math.round((used.user+used.system)/1000)};};

 const full=()=>observation.captureObservedSourceInputsV1(async()=>{let taken=0;for(const owner of owners){const own=await read(owner);observation.retainObservedRowsV1(own.observations);retained.set(owner.name,{text:JSON.stringify(own.value),rows:own.observations});taken++;}return taken;},undefined,false);
 const verify=()=>observation.captureObservedSourceInputsV1(async()=>{let taken=0;for(const owner of owners){const kept=retained.get(owner.name)!;if(observation.observationRowsHoldV1(kept.rows as never)){observation.retainObservedRowsV1(kept.rows as never);JSON.parse(kept.text);}else{const own=await read(owner);observation.retainObservedRowsV1(own.observations);retained.set(owner.name,{text:JSON.stringify(own.value),rows:own.observations});taken++;}}return taken;},undefined,false);
 const reuse=()=>observation.captureObservedSourceInputsV1(async()=>{
  const reused=new Set((await memo.replay(owners.map(owner=>({name:owner.name,key:"job"})))).map(row=>row.name));let taken=0;
  for(const owner of owners){if(reused.has(owner.name))continue;const own=await read(owner);observation.retainObservedRowsV1(own.observations);memo.remember(owner.name,{key:"job",project:own.value,rows:own.observations});taken++;}
  return taken;
 },undefined,false);

 const rows:Record<string,number[]>={full:[],verify:[],memo:[]},cpu:Record<string,number[]>={full:[],verify:[],memo:[]},taken:Record<string,number[]>={full:[],verify:[],memo:[]};
 const edit=async(pass:number)=>{for(let each=0;each<EDITS;each++){const at=(pass*97+each*389)%OWNERS;await writeFile(join(root,owners[at]!.files[each]!),"edited "+pass+" "+each);}};
 await full();await verify();await reuse();
 for(let pass=2;pass<=PASSES;pass++){
  await edit(pass);
  for(const[name,run]of[["full",full],["verify",verify],["memo",reuse]] as const){const result=await clock(run);rows[name]!.push(result.ms);cpu[name]!.push(result.cpu);taken[name]!.push(result.value.value as number);}
 }
 const sum=(list:number[])=>list.reduce((total,value)=>total+value,0);
 console.log(JSON.stringify({owners:OWNERS,rowsPerOwner:FILES+SHARED+1,passes:PASSES,editsPerPass:EDITS,wallMs:Object.fromEntries(Object.entries(rows).map(([key,list])=>[key,{perPass:list,total:sum(list)}])),cpuMs:Object.fromEntries(Object.entries(cpu).map(([key,list])=>[key,sum(list)])),ownersTaken:Object.fromEntries(Object.entries(taken).map(([key,list])=>[key,list]))},null,1));
}finally{await rm(root,{recursive:true,force:true});}
