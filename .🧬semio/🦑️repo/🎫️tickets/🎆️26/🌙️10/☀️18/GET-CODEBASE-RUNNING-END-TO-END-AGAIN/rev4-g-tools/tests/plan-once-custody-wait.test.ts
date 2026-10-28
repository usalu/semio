/** 🪞️ Contract: a builder that took over a delta refresh plans it once. The plan (membership frontier walk and the verification of every fact in scope) that decided the refresh is the plan the build executes, also when the refresh is scoped to a closure or to the owners a coverage stamp left open; only a generation that moved since forces a new plan. The graph after the refresh equals the independent Python model of the workspace. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalJsonV1,createGenerationStoreV1} from "../../🟨️.mjs";
import {IDENTITY,IMPLEMENTATION,createCompute,createHolds,createMembership,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-plan-once-")),sleep=(ms:number)=>new Promise(resolve=>setTimeout(resolve,ms));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));

function setup(dir:string){
 const counters={walks:0,rows:0,passes:0},holds=createHolds(dir),membership=createMembership(dir,counters),compute=createCompute(dir,[]);
 const counted=async(rows:readonly {kind:string;path:string;fact:unknown}[],signal?:AbortSignal)=>{counters.rows+=rows.length;return holds(rows as never,signal);};
 const store=createGenerationStoreV1({root:join(dir,".gen"),holds:counted,resources:{facts:"repository-graph-facts:plan-once",builder:"repository-graph-build:plan-once"}});
 const progress=(event:{phase?:string;done:number})=>{if(event.phase==='facts'&&event.done===0)counters.passes++;};
 return{counters,store,base:{identity:IDENTITY,implementation:IMPLEMENTATION,membership,compute,progress}};
}


test("[rev4] a file of the closure edited while the refresh waits for the builder custody is built into the generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(8));
  const{counters,store,base}=setup(dir);
  await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now()});
  await sleep(20);
  await writeFile(join(dir,"pkg/p3/src/index.ts"),"import \"@syn/p4\";\nexport const edited=1;\n");
  let waited=false;
  const builder=async(body:()=>Promise<unknown>)=>{
   if(!waited){waited=true;await sleep(20);await writeFile(join(dir,"pkg/p5/src/index.ts"),"import \"@syn/p6\";\nexport const edited=2;\n");}
   return body();
  };
  const result=await store.refreshClosure({...base,roots:["p0"],requestedAt:Date.now(),builder} as never);
  console.log("[rev4] computed",result.computed,"walks",counters.walks);
  const graph=await store.assemble(result.current,{});const{generation:_g,membership:_m,...rest}=graph;
  expect(normalized(rest),"generation equals the disk after the wait").toEqual(pythonGraph(dir));
 }finally{rmSync(dir,{recursive:true,force:true});}
});
