import sys
src,dst=sys.argv[1],sys.argv[2]
t=open(src).read()
def call_end(t,i):
    j=t.index('(',i);d=0
    while True:
        c=t[j]
        if c=='(':d+=1
        elif c==')':
            d-=1
            if d==0:return j+1
        elif c in '"\'`':
            q=c;j+=1
            while t[j]!=q:
                if t[j]=='\\':j+=1
                j+=1
        j+=1
def wrap(t,marker,phase,fn_start):
    assert t.count(marker)==1,(marker,t.count(marker))
    i=t.index(marker);k=i+marker.index(fn_start);e=call_end(t,k)
    return t[:k]+'measureV1("'+phase+'",()=>'+t[k:e]+')'+t[e:]
def sub(t,a,b):
    assert t.count(a)==1,(a[:60],t.count(a));return t.replace(a,b)
t=sub(t,'import {requireCargoSelectedPathEnvelopeV1','import{createPhaseLedgerV1,phaseLedgerV1,phaseLedgerFileSinkV1,phaseLedgerTaskV1,phaseTableLinesV1,withPhaseLedgerV1,readCargoTimingsV1,cargoTimingsLinesV1,readUnitEventsV1,unitEventRowsV1}from"./🧮️phase-ledger/🟦️.ts";\nimport {requireCargoSelectedPathEnvelopeV1')
t=wrap(t,'const planning=await captureCargoCompilationGraphV1(','plan:graph','captureCargoCompilationGraphV1(')
t=wrap(t,'await requireCargoSelectedPathEnvelopeControlledV1(policy.buildDirectory,planning.units','plan:envelope','requireCargoSelectedPathEnvelopeControlledV1(')
t=wrap(t,'await admitCargoSourceBoundariesV1(planning,','plan:sources','admitCargoSourceBoundariesV1(')
t=wrap(t,'const recipe=await cargoArtifactTargetRecipeV1(','plan:recipe','cargoArtifactTargetRecipeV1(')
t=wrap(t,'const facts=await captureCargoArtifactPlanFactsV1(','plan:facts','captureCargoArtifactPlanFactsV1(')
t=wrap(t,'await recipe.recheck()','recheck:recipe','recipe.recheck(')
t=wrap(t,'const current=await captureCargoCompilationGraphV1(','recheck:graph','captureCargoCompilationGraphV1(')
t=wrap(t,'await requireCargoSelectedPathEnvelopeControlledV1(policy.buildDirectory,current.units','recheck:envelope','requireCargoSelectedPathEnvelopeControlledV1(')
t=wrap(t,'await admitCargoSourceBoundariesV1(current,','recheck:sources','admitCargoSourceBoundariesV1(')
t=wrap(t,'const observed=await captureCargoArtifactPlanFactsV1(','recheck:facts','captureCargoArtifactPlanFactsV1(')
t=wrap(t,'lease=await acquireCargoTargetRecipeV1(','lease:target','acquireCargoTargetRecipeV1(')
t=wrap(t,'await stageArtifacts(','stage','stageArtifacts(')
t=wrap(t,'await ensureCargoLockCurrentV1(','lock-current','ensureCargoLockCurrentV1(')
t=sub(t,'await consume(capture);','await measureV1("consume",()=>consume(capture));')
t=sub(t,'await withCapturedCargoArtifactsTaskV1(capture,async view=>{','await measureV1("consume:source-digests",()=>withCapturedCargoArtifactsTaskV1(capture,async view=>{')
t=sub(t,'await recheck();\n          });\n          await measureV1("consume"','await recheck();\n          }));\n          await measureV1("consume"')
t=sub(t,'const receipt=await observeCompleted();await withCompletedCargoArtifactCaptureV1(owner,files,stagedNames,receipt,options,completedCompilation);','const receipt=await measureV1("provenance",()=>observeCompleted());await measureV1("custody",()=>withCompletedCargoArtifactCaptureV1(owner,files,stagedNames,receipt,options,completedCompilation));')
t=sub(t,'await withWriter(async()=>withCompilerCapacityScopeV1(','const endWait=phaseLedgerV1().begin("wait:capacity");await withWriter(async()=>withCompilerCapacityScopeV1(')
t=sub(t,'async(facts,control)=>{\n  const environment:Record<string,string|undefined>','async(facts,control)=>{endWait();\n  const environment:Record<string,string|undefined>')
t=sub(t,'const child = spawn(','const endCargo=phaseLedgerV1().begin("cargo"),cargoStartedMs=Date.now();const child = spawn(')
t=sub(t,'...cargoArgs, "--message-format=json-render-diagnostics", ...compilerArgs], {','...cargoArgs, ...cargoTimingsArgumentsV1(options.command,environment), "--message-format=json-render-diagnostics", ...compilerArgs], {')
t=sub(t,'const compilerStatus = await status;','const compilerStatus = await status;endCargo();recordCargoRunV1(targetDirectory,environment,cargoStartedMs);')
t=sub(t,'      stopProgress();\n','      endCargo();stopProgress();\n')
a='/** 📋️ Owns captured planning and bounded Source for exactly one received compiler operation. */\nexport async function withCargoArtifactPlanV1<T>('
new='''const measureV1=<T>(phase:string,body:()=>Promise<T>):Promise<T>=>phaseLedgerV1().run(phase,body);
const cargoTimingsArgumentsV1=(command:string|undefined,environment:Readonly<Record<string,string|undefined>>):string[]=>(command===undefined||command==="build")&&environment.SEMIO_CARGO_TIMINGS!=="off"?["--timings"]:[];

/** 🧮️ Adds what the run itself says to the ledger: Cargo's own timing report of the run (start delay, front end, code generation, critical path, slow units) and the unit events of the lane (restores, lookups, compiles, publications, waits).
 * @see ./🧮️phase-ledger/🟦️.ts */
function recordCargoRunV1(targetDirectory:string,environment:Readonly<Record<string,string|undefined>>,startedMs:number):void{
 const ledger=phaseLedgerV1(),end=ledger.begin("ledger");
 try{
  const task=ledger.table().task,timings=readCargoTimingsV1(targetDirectory,startedMs-1000);
  if(timings){
   for(const line of cargoTimingsLinesV1(task,timings))process.stderr.write(line+"\\n");
   for(const[phase,seconds,count]of[["cargo:start",timings.startSeconds,0],["cargo:frontend",timings.frontendSeconds,0],["cargo:codegen",timings.codegenSeconds,0],["cargo:unsplit",timings.unsplitSeconds,0],["cargo:critical",timings.criticalSeconds,timings.criticalUnits],["cargo:units",0,timings.units]]as const)ledger.add({phase,seconds,count,bytes:null,within:true});
   for(const unit of timings.slow)ledger.add({phase:"slow:"+unit.name.toLowerCase().replace(/[^a-z0-9:-]+/gu,"-").slice(0,50),seconds:unit.seconds,count:1,bytes:null,within:true});
  }
  const root=environment.SEMIO_UNIT_CACHE_ROOT;
  if(root)for(const row of unitEventRowsV1(readUnitEventsV1(root),environment.SEMIO_UNIT_CACHE_LANE??"d",startedMs,Date.now()))ledger.add(row);
 }catch{}finally{end();}
}

/** 📋️ Owns captured planning and bounded Source for exactly one received compiler operation, and prints the per-phase time ledger of the task at its end (stderr, and one file line per ledger for the front of the run).
 * @see ./🧮️phase-ledger/🟦️.ts */
export async function withCargoArtifactPlanV1<T>(manifest:string,args:string[],policy:CargoArtifactBuildPolicyV1,options:CargoArtifactBuildOptionsV1,body:(plan:CargoArtifactPlanV1,control:ResourceLeaseControl)=>Promise<T>):Promise<T>{
 const ledger=createPhaseLedgerV1({task:phaseLedgerTaskV1(options.environment??process.env,relative(policy.cwd,manifest).split(sep).join("/")),sink:phaseLedgerFileSinkV1(policy.cwd)});
 try{return await withPhaseLedgerV1(ledger,()=>withCargoArtifactPlanLedgeredV1(manifest,args,policy,options,body));}
 finally{for(const line of phaseTableLinesV1(ledger.finish()))process.stderr.write(line+"\\n");}
}

async function withCargoArtifactPlanLedgeredV1<T>('''
t=sub(t,a,new)
t=sub(t,'Promise<T>{\n  const storage=requireNativeStorageReservationV1(options.storage,options.control);requireIssuedResourceControlV1','Promise<T>{\n  const endSetup=phaseLedgerV1().begin("setup");const storage=requireNativeStorageReservationV1(options.storage,options.control);requireIssuedResourceControlV1')
t=sub(t,'return withCargoArtifactPlanInputsV1({manifest,cwd:policy.cwd,arguments:[...prefix,...args],environment:options.environment??process.env,control},options.withCargoInputs,async actual=>{\n','return withCargoArtifactPlanInputsV1({manifest,cwd:policy.cwd,arguments:[...prefix,...args],environment:options.environment??process.env,control},options.withCargoInputs,async actual=>{endSetup();\n')
open(dst,'w').write(t)
print("ok",len(t))
