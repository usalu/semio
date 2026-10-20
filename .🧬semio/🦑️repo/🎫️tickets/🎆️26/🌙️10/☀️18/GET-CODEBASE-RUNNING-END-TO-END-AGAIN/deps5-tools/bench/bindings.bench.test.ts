import {withBuildTestInvocationV1} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {test} from "bun:test";
import {existsSync,readFileSync,lstatSync,readlinkSync} from "node:fs";
import {dirname,join} from "node:path";

const ROOT=process.cwd(),LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",DEPS=join(LIB,"⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"),FW=join(ROOT,"🧰️framework/🔨️modules"),STORE=join(ROOT,".cache/b/d/m");
const MODES=(process.env.DEPS5_MODES??"old,new").split(","),OWNERSHIP=process.env.DEPS5_OWNERSHIP??"/private/tmp/claude-501/deps5-scratch/ownership.json",LIMIT=Number(process.env.DEPS5_BINDINGS??60);

test("deps5 bindings benchmark",async()=>{
 const routing=await import(join(FW,"🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")),leases=await import(join(FW,"🏃️process/🔒️leases/🟦️.ts"));
 const ownership=JSON.parse(readFileSync(OWNERSHIP,"utf8"));
 await withBuildTestInvocationV1(async original=>{
  const control=routing.createIssuedResourceControlV1(original,"deps5:bindings",0);
  const check=(actual:any)=>{routing.requireIssuedResourceControlInvocationV1(actual,original);routing.requireIssuedResourceControlRefinementV1(control,actual);leases.checkResourceLeaseControl(actual);};
  let rechecks=0;const context=Object.freeze({root:STORE,invocation:original,control,check,recheck:async(actual:any)=>{rechecks++;check(actual);}});
  for(const mode of MODES){
   const mod=await import(join("/private/tmp/claude-501","deps5-mirror-"+mode,DEPS)) as any,counters={lstat:0,readFile:0};(globalThis as any).__deps5=counters;
   const plans=await mod.inspectJavascriptDependencyInputPlansReadV1(ROOT,context,control,ownership,"sync");
   const bindings=plans.plans.flatMap((row:any)=>row.plan.bindings.map((b:any)=>({...b,scope:row.scope})));
   const matching=bindings.filter((b:any)=>{try{return lstatSync(b.sourceDirectory).isSymbolicLink()&&readlinkSync(b.sourceDirectory)===b.targetDirectory;}catch{return false;}});
   console.error("[bench] deps5 bindings",mode,JSON.stringify({total:bindings.length,matching:matching.length,scopes:plans.plans.length}));
   const t0=performance.now();counters.lstat=0;rechecks=0;const cpu=process.cpuUsage();
   const source=await mod.readJavascriptDependencyPreservationSourceV1(ROOT,context,control,ownership);
   const passMs=performance.now()-t0,used=process.cpuUsage(cpu);
   console.error("[bench] deps5 preserving-pass",mode,JSON.stringify({ms:Math.round(passMs),cpuMs:Math.round((used.user+used.system)/1000),lstat:counters.lstat,rechecks,rows:source.rows.length}));
   const expected=new Map(source.rows.map((row:any)=>[row.sourceDirectory,row])),parents=new Map(source.parents.map((row:any)=>[row.path,row.identity])),ancestors=new Map(source.ancestors.map((row:any)=>[row.path,row.identity]));
   if(mode==="new"){
    const sample=matching.slice(0,LIMIT),t1=performance.now();counters.lstat=0;
    for(const b of sample)mod.requireJavascriptDependencyEndpointFenceV1(b.sourceDirectory,expected,parents,ancestors);
    console.error("[bench] deps5 endpoint-fence",mode,JSON.stringify({bindings:sample.length,msPer:Math.round((performance.now()-t1)/Math.max(1,sample.length)*100)/100,lstatPer:Math.round(counters.lstat/Math.max(1,sample.length))}));
   }
   // publish no-op: bindings already pointing to target
   const sample=matching.slice(0,Math.min(LIMIT,matching.length)),t2=performance.now();counters.lstat=0;let n=0,cps=0;
   for(const b of sample){const previous=source.rows.find((r:any)=>r.sourceDirectory===b.sourceDirectory),snapshot={...source};
    const facts={sourceSha256:source.sha256,generation:"a".repeat(64),readySha256:"b".repeat(64),inputSha256:"c".repeat(64),recipeSha256:"d".repeat(64),fundingSha256:"e".repeat(64)};
    if(!previous||previous.kind!=="link")continue;
    const result=await mod.publishJavascriptDependencyPreservingSourceDescriptorV1(b,previous,snapshot,facts,()=>{cps++;check(control);});
    if(result.changed)throw Error("unexpected change "+b.sourceDirectory);n++;}
   console.error("[bench] deps5 publish-noop",mode,JSON.stringify({bindings:n,msPer:Math.round((performance.now()-t2)/Math.max(1,n)*10)/10,lstatPer:Math.round(counters.lstat/Math.max(1,n)),checkpointsPer:Math.round(cps/Math.max(1,n)),ancestors:source.ancestors.length,parents:source.parents.length}));
  }
 });
},3000000);
