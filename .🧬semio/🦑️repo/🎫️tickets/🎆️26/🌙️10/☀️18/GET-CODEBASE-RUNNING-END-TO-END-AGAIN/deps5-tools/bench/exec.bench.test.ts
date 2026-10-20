import {withBuildTestInvocationV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {test,expect} from "bun:test";
import {createRequire} from "node:module";
import {existsSync,readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";

const ROOT=process.cwd(),LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",DEPS=join(LIB,"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"),FW=join(ROOT,"🧰️framework/🔨️modules"),STORE=join(ROOT,".cache/b/d/m");
const MODES=(process.env.DEPS5_MODES??"old,new").split(","),OWNERSHIP=process.env.DEPS5_OWNERSHIP??"/private/tmp/claude-501/deps5-scratch/ownership.json";

test("deps5 execution benchmark",async()=>{
 const routing=await import(join(FW,"🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),leases=await import(join(FW,"🏃️process/🔒️leases/🟦️.ts"));
 if(!existsSync(OWNERSHIP)){const cjs=createRequire(import.meta.url)(join(ROOT,LIB,"🗂️workspaces/🟦️bun/🟨️.cjs"));const abort=new AbortController();const scopes=await cjs.withCurrentBunWorkspaceOwnershipV1(ROOT,{kind:"all"},{signal:abort.signal,advance:async()=>{}},async(custody:any)=>JSON.parse(JSON.stringify(custody.scopes)));writeFileSync(OWNERSHIP,JSON.stringify({version:1,scopes}));}
 const ownership=JSON.parse(readFileSync(OWNERSHIP,"utf8")),out:Record<string,unknown>={};
 await withBuildTestInvocationV1(async original=>{
  const control=routing.createIssuedResourceControlV1(original,"deps5:exec",0);
  const check=(actual:any)=>{routing.requireIssuedResourceControlInvocationV1(actual,original);routing.requireIssuedResourceControlRefinementV1(control,actual);leases.checkResourceLeaseControl(actual);};
  const context=Object.freeze({root:STORE,invocation:original,control,check,recheck:async(actual:any)=>{check(actual);}});
  for(const mode of MODES){
   const mod=await import(join("/private/tmp/claude-501","deps5-mirror-"+mode,DEPS)) as any,counters={lstat:0,readFile:0};(globalThis as any).__deps5=counters;
   const t0=performance.now(),plans=await mod.inspectJavascriptDependencyInputPlansReadV1(ROOT,context,control,ownership,"sync"),plansMs=performance.now()-t0;
   const input={version:1,owners:ownership,requested:plans.plans.map((row:any)=>row.scope),generations:plans.plans.map((row:any)=>({scope:row.scope,generation:row.plan.generation}))};
   const row:Record<string,unknown>={plansMs:Math.round(plansMs),scopes:plans.plans.length,missingGenerations:plans.plans.filter((r:any)=>!existsSync(join(STORE,"j",r.plan.generation.slice(0,20)))).length};
   counters.lstat=0;counters.readFile=0;const cpu=process.cpuUsage(),t=performance.now();
   const exec=await mod.readJavascriptDependencySourceOwnerExecutionV1(ROOT,context,control,input);
   row.execMs=Math.round(performance.now()-t);const used=process.cpuUsage(cpu);row.execCpuMs=Math.round((used.user+used.system)/1000);row.lstat=counters.lstat;row.readFile=counters.readFile;row.execSha256=exec.sha256.slice(0,16);row.relations=exec.relations.length;row.scopesDigest=JSON.stringify(exec.scopes).length;
   out[mode]=row;console.error("[bench] deps5 exec",mode,JSON.stringify(row));
  }
 });
 if(out.old&&out.new)expect((out.new as any).execSha256).toBe((out.old as any).execSha256);
},3000000);
