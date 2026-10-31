from pathlib import Path
p=Path("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts")
s=p.read_text()
anchor="/** 🏗️ Runs all installers, export fixups and durable ready phases before its first authenticated Source transition. */"
helper='''/** 📖️ Qualifies immutable inputs outside the actual reader frame and returns values only after that frame drains and qualifies again. */
async function withRepositoryDependencyProducerArtifactReadV1<T>(parent:NativeProtectedMaintenanceParentV1,control:ResourceLeaseControl,artifactPhase:ReturnType<typeof createRepositoryDependencyProducerArtifactPhaseV1>,root:string,invocation:ScriptInvocation,checkpoint:()=>void,body:(context:JavascriptDependencyReadContextV1)=>Promise<T>,sourceOnly=false):Promise<T>{
 await artifactPhase.recheck(control);let value:T|undefined,failed=false,failure:unknown;
 try{value=await dependencyMaintenanceNative.withNativePlatformProtectedMaintenanceReadV1(parent,control,async(physical,current)=>{let active=true;const check=(phase:ResourceLeaseControl)=>{if(!active)throw Error("Funded producer Source reader closed");checkpoint();requireIssuedResourceControlRefinementV1(current,phase);if(!sourceOnly)physical.check();},context:JavascriptDependencyReadContextV1=Object.freeze({root,control:current,invocation,check,recheck:async(phase:ResourceLeaseControl)=>{check(phase);physical.check();check(phase);}});try{if(physical.root!==root||physical.associations.length)throw Error("Exact funded producer leaf required");physical.check();const result=await body(context);physical.check();return result;}finally{active=false;}});}catch(error){failed=true;failure=error;}
 try{await artifactPhase.recheck(control);}catch(error){if(failed&&!Object.is(failure,error))throw new AggregateError([failure,error],"Producer reader and completed input validation failed");throw failed?failure:error;}if(failed)throw failure;return value as T;
}

'''
assert s.count(anchor)==1
s=s.replace(anchor,helper+anchor)
needle="read=<T>(body:(context:JavascriptDependencyReadContextV1)=>Promise<T>,sourceOnly=false)=>dependencyMaintenanceNative.withNativePlatformProtectedMaintenanceReadV1"
assert s.count(needle)==1
start=s.index(needle)
end=s.index("\n",start)
old=s[start:end]
assert old.endswith("finally{active=false;}});")
s=s[:start]+"read=<T>(body:(context:JavascriptDependencyReadContextV1)=>Promise<T>,sourceOnly=false)=>withRepositoryDependencyProducerArtifactReadV1(parent,actual,artifactPhase,root,invocation,checkpoint,body,sourceOnly);"+s[end:]
p.write_text(s)
