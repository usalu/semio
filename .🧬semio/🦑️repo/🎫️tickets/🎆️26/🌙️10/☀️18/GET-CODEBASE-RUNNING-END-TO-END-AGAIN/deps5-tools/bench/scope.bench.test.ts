import {withBuildTestInvocationV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {test} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";

const ROOT=process.cwd(),LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",DEPS=join(LIB,"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"),FW=join(ROOT,"🧰️framework/🔨️modules"),STORE=join(ROOT,".cache/b/d/m");
const MODES=(process.env.DEPS5_MODES??"old,new").split(","),OWNERSHIP=process.env.DEPS5_OWNERSHIP??"/private/tmp/claude-501/deps5-scratch/ownership.json";

test("deps5 scope read benchmark",async()=>{
 const routing=await import(join(FW,"🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),leases=await import(join(FW,"🏃️process/🔒️leases/🟦️.ts")),ownership=JSON.parse(readFileSync(OWNERSHIP,"utf8"));
 await withBuildTestInvocationV1(async original=>{
  const control=routing.createIssuedResourceControlV1(original,"deps5:scope",0);
  const check=(actual:any)=>{routing.requireIssuedResourceControlInvocationV1(actual,original);routing.requireIssuedResourceControlRefinementV1(control,actual);leases.checkResourceLeaseControl(actual);};
  const context=Object.freeze({root:STORE,invocation:original,control,check,recheck:async(actual:any)=>{check(actual);}});
  for(const mode of MODES){
   const mod=await import(join("/private/tmp/claude-501","deps5-mirror-"+mode,DEPS)) as any,counters={lstat:0,readFile:0};(globalThis as any).__deps5=counters;
   const scopes=ownership.scopes.map((row:any)=>row.manifest==="package.json"?".":row.manifest.replace(/\/package\.json$/u,""));
   const rows:unknown[]=[];let total=0;
   for(const scope of scopes){counters.lstat=0;counters.readFile=0;const t=performance.now(),cpu=process.cpuUsage();let status="ok";try{await mod.readJavascriptDependencySourceOwnerScopeReadV1(ROOT,context,control,ownership,scope);}catch(error){status=String((error as Error).message).slice(0,90);}
    const ms=performance.now()-t,used=process.cpuUsage(cpu);total+=ms;rows.push({scope,ms:Math.round(ms),cpuMs:Math.round((used.user+used.system)/1000),lstat:counters.lstat,readFile:counters.readFile,status});}
   console.error("[bench] deps5 scope-reads",mode,JSON.stringify({totalMs:Math.round(total),rows}));
  }
 });
},3000000);
