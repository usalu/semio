#!/usr/bin/env python3
# anchored patch of the process area: stretch travels once (contention), remote lanes in the bun file runner, remote port in the suite runner.
# usage: apply-process.py <process-root>   (the folder holding 🧪️testing/ and 🧭️routing/)
import sys,os
R=sys.argv[1]
MEM={}; NEW={}
def rep(path,old,new,count=1):
    p=os.path.join(R,path)
    if p not in MEM: MEM[p]=open(p,encoding="utf-8").read()
    assert MEM[p].count(old)==count,(path,old[:60],MEM[p].count(old))
    MEM[p]=MEM[p].replace(old,new)
def flush():
    if os.environ.get("DRY")=="1": print("dry run: all anchors found"); return
    for p,v in list(MEM.items())+list(NEW.items()):
        os.makedirs(os.path.dirname(p),exist_ok=True); open(p+".tmp","w",encoding="utf-8").write(v); os.replace(p+".tmp",p)
C="🧪️testing/🎚️budget/⏲️contention/"
rep(C+"🟦️.ts","""  tickMilliseconds: number;
  stretch:""","""  tickMilliseconds: number;
  inheritance?: Readonly<{ environment: string }>;
  stretch:""")
rep(C+"🟦️.ts","""  const needed = Math.ceil(milliseconds * budgetStretchV1(ratio ?? (launchRatio ??= sharedHostContentionV1()().ratio), policy)), ceiling""","""  const needed = Math.ceil(milliseconds * budgetStretchV1(ratio ?? launchContentionRatioV1(policy), policy)), ceiling""")
rep(C+"🟦️.ts","""/** 🧮️ The funded milliseconds of `stretchedBudgetV1`;""","""/** 🧬️ The stretch a parent run handed down (basis points in the environment variable the policy names), or `undefined`: a nested process never funds a deadline with less stretch than the run that started it, and the stretch of a run tree is measured once, by the host that executes its root. */
export function inheritedStretchRatioV1(environment: Readonly<Record<string, string | undefined>> = process.env, policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): number | undefined {
  const name = policy.inheritance?.environment, text = name === undefined ? undefined : environment[name], value = text === undefined || !/^\\d{1,7}$/u.test(text) ? Number.NaN : Number(text);
  return Number.isSafeInteger(value) && value >= policy.stretch.minimumBasisPoints && value <= policy.stretch.maximumBasisPoints ? value / 10000 : undefined;
}

/** 🚀️ The contention ratio this process funds every deadline with: its own first measurement or the one inherited from its parent run, whichever is larger, fixed at the first call. */
function launchContentionRatioV1(policy: ContentionPolicyV1): number {
  return launchRatio ??= Math.max(inheritedStretchRatioV1(process.env, policy) ?? 0, sharedHostContentionV1()().ratio);
}

/** 🧮️ The stretch every deadline of this process is funded with, in basis points (what a remote run is told as its minimum and what children inherit). */
export function launchStretchBasisPointsV1(policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): number {
  return Math.round(budgetStretchV1(launchContentionRatioV1(policy), policy) * 10000);
}

/** 📤️ The environment that hands a stretch (basis points) to a process, so that it funds its deadlines with at least that stretch; empty when the policy names no variable. */
export function stretchEnvironmentV1(basisPoints: number, policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): Readonly<Record<string, string>> {
  const name = policy.inheritance?.environment;
  return name === undefined ? {} : { [name]: String(Math.round(basisPoints)) };
}

/** 📤️ The environment a child process of a run receives so that it funds its deadlines with the stretch of this process and never measures a second one smaller. */
export function budgetStretchEnvironmentV1(policy: ContentionPolicyV1 = CONTENTION_POLICY_V1): Readonly<Record<string, string>> {
  return stretchEnvironmentV1(launchStretchBasisPointsV1(policy), policy);
}

/** 🧮️ The funded milliseconds of `stretchedBudgetV1`;""")
rep(C+"🔣️.json",'''  "version": 1,
  "tickMilliseconds": 1000,''','''  "version": 1,
  "tickMilliseconds": 1000,
  "inheritance": {
    "environment": "SEMIO_TEST_BUDGET_STRETCH_BASIS_POINTS"
  },''')
rep(C+"🧬️schema/🔣️.json",'''    "tickMilliseconds": { "type": "integer", "minimum": 1, "maximum": 60000 },
''','''    "tickMilliseconds": { "type": "integer", "minimum": 1, "maximum": 60000 },
    "inheritance": {
      "type": "object",
      "additionalProperties": false,
      "required": ["environment"],
      "properties": { "environment": { "type": "string", "pattern": "^[A-Z][A-Z0-9_]{2,63}$" } }
    },
''')

# public entry re-exports
rep("📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts","export { budgetStretchV1, describeClampedBudgetV1,","export { budgetStretchEnvironmentV1, budgetStretchV1, describeClampedBudgetV1, inheritedStretchRatioV1, launchStretchBasisPointsV1,")
rep("📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts","STRETCHED_BUDGET_CEILING_MILLISECONDS_V1, stretchedBudgetMillisecondsV1,","STRETCHED_BUDGET_CEILING_MILLISECONDS_V1, stretchEnvironmentV1, stretchedBudgetMillisecondsV1,")

# bun file runner
B="🧪️testing/🟨️bun/📁️files/🟦️.ts"
rep(B,"""/** 🔐️ Captures closed primitive policy fields""","""/** 🌐️ Hands files to other machines while the local lanes run the rest: `width` files are offered at once (each by its own lane), `take` answers the terminal result of a file that ran elsewhere or `undefined` when it must run here, and `measure` hears how long a file took here. A file `take` keeps here goes to the local lanes, which never wait for a file that is still out. */
export interface OwnedBunFilesRemoteV1{readonly width:number;readonly take:(file:string,index:number,signal:AbortSignal)=>Promise<OwnedBunFileResultV1|undefined>;readonly measure?:(file:string,seconds:number)=>void|Promise<void>}

/** 🔐️ Captures closed primitive policy fields""")
rep(B,"readonly memory?:TestFileMemoryV1;readonly onFileResult:","readonly memory?:TestFileMemoryV1;readonly remote?:OwnedBunFilesRemoteV1;readonly onFileResult:")
old_start="""    const results=new Array<OwnedBunFileResultV1>(files.length),started=performance.now();const granted=childCapabilitiesV1(invocation.capabilities,selection);let next=0,total=0,failed=false,failure:unknown;
    const worker=async()=>{while(!failed&&next<files.length){const index=next++,file=files[index];try{if(file===undefined)throw Error("Exact admitted Bun file required");
     checkResourceLeaseControl(phase);const run="""
new_start="""    const results=new Array<OwnedBunFileResultV1>(files.length),started=performance.now(),remote=options.remote;const granted=childCapabilitiesV1(invocation.capabilities,selection);let next=0,total=0,failed=false,failure:unknown,remoteLanes=0;
    if(remote!==undefined&&(!Number.isInteger(remote.width)||remote.width<1||typeof remote.take!=="function"))throw Error("Actual owned Bun remote port required");
    const retry:number[]=[],waiting:(()=>void)[]=[],wake=():void=>{for(const resume of waiting.splice(0))resume();},fail=(error:unknown):void=>{if(!failed){failed=true;failure=error;cancellation.abort(error);}wake();};
    const settle=async(index:number,result:OwnedBunFileResultV1):Promise<void>=>{
     total+=Buffer.byteLength(result.capture.stdout)+Buffer.byteLength(result.capture.stderr);if(total>policy.maximumTotalOutputBytes)throw Error("Owned Bun cohort output limit exceeded");results[index]=result;
     await withRequestedIssuedResourceControlV1(phase,0,actual=>onFileResult(result,index,actual));
     await phase.publish({resource:"bun:test:file:"+index,mode:"shared",elapsedMs:performance.now()-started});await phase.yieldContinuation();checkResourceLeaseControl(phase);
    };
    const local=async(index:number):Promise<void>=>{const file=files[index];if(file===undefined)throw Error("Exact admitted Bun file required");
     checkResourceLeaseControl(phase);const began=performance.now(),run="""
assert True
rep(B,old_start,new_start)
old_end="""     total+=Buffer.byteLength(capture.stdout)+Buffer.byteLength(capture.stderr);if(total>policy.maximumTotalOutputBytes)throw Error("Owned Bun cohort output limit exceeded");const result=Object.freeze({file,capture,...(memory?{memory}:{})});results[index]=result;
     await withRequestedIssuedResourceControlV1(phase,0,actual=>onFileResult(result,index,actual));
     await phase.publish({resource:"bun:test:file:"+index,mode:"shared",elapsedMs:performance.now()-started});await phase.yieldContinuation();checkResourceLeaseControl(phase);
    }catch(error){if(!failed){failed=true;failure=error;cancellation.abort(error);}}}};
    await Promise.all(Array.from({length:Math.min(files.length,availableParallelism(),policy.maximumParallelFiles)},worker));if(failed)throw failure;return Object.freeze(results);"""
new_end="""     if(remote?.measure!==undefined)void Promise.resolve().then(()=>remote.measure!(file,Math.round((performance.now()-began)/100)/10)).catch(()=>undefined);
     await settle(index,Object.freeze({file,capture,...(memory?{memory}:{})}));
    };
    const worker=async():Promise<void>=>{for(;;){
     if(failed)return;let index:number|undefined;
     if(remote===undefined)index=next<files.length?next++:undefined;
     else{index=retry.shift();if(index===undefined){if(remoteLanes===0)return;await new Promise<void>(resume=>{waiting.push(resume);});continue;}}
     if(index===undefined)return;
     try{await local(index);}catch(error){fail(error);}
    }};
    const lane=async():Promise<void>=>{try{for(;;){
     if(failed)return;const index=next++,file=files[index];if(file===undefined)return;let result:OwnedBunFileResultV1|undefined;
     try{checkResourceLeaseControl(phase);result=await remote!.take(file,index,cancellation.signal);}catch{if(failed||cancellation.signal.aborted)return;result=undefined;}
     try{if(result===undefined||result.file!==file){retry.push(index);wake();}else await settle(index,Object.freeze({...result}));}catch(error){fail(error);}
    }}finally{remoteLanes--;wake();}};
    remoteLanes=remote===undefined?0:Math.min(remote.width,files.length);const lanes=remoteLanes;
    await Promise.all([...Array.from({length:Math.min(files.length,availableParallelism(),policy.maximumParallelFiles)},worker),...Array.from({length:lanes},lane)]);if(failed)throw failure;return Object.freeze(results);"""
rep(B,old_end,new_end)

# suite runner
S="🧪️testing/🗂️suites/🏃️run/🟦️.ts"
rep(S,'import{describeClampedBudgetV1,stretchedBudgetV1}from"../../🎚️budget/⏲️contention/🟦️.ts";','import{budgetStretchEnvironmentV1,describeClampedBudgetV1,stretchedBudgetV1}from"../../🎚️budget/⏲️contention/🟦️.ts";')
rep(S,'import{runOwnedBunFilesV1}from"../../🟨️bun/📁️files/🟦️.ts";','import{runOwnedBunFilesV1,type OwnedBunFileResultV1}from"../../🟨️bun/📁️files/🟦️.ts";')
rep(S,"""/** 📋️ Names one run of a suite of a route table:""","""/** 🌐️ What a file that ran on another machine answers: where it ran (`origin`), how long it took, whether it passed, and its output (already prefixed and complete lines). */
export type ProcessTestRemoteRanV1=Readonly<{origin:string;seconds:number;passed:boolean;output:string}>;
/** 🌐️ Offloads the files of a suite to idle machines: `width` files are out at once, `file` answers the run of one checkout-relative file on another machine or `undefined` when it must run here (`budgetMs` is its declared budget; `index` and `total` name it among the files of the run), `measure` hears how long a file took here, and `environment` is what every file process of the run gets so a file never offers its own nested suites again. */
export type ProcessTestRemoteV1=Readonly<{width:number;environment:Readonly<Record<string,string>>;file:(file:string,context:Readonly<{budgetMs:number;index:number;total:number;signal:AbortSignal}>)=>Promise<ProcessTestRemoteRanV1|undefined>;measure?:(file:string,seconds:number)=>void|Promise<void>}>;
/** 📋️ Names one run of a suite of a route table:""")
rep(S,"launchers?:ProcessTestLaunchersV1;workers?:ProcessTestWorkersV1;report?:","launchers?:ProcessTestLaunchersV1;workers?:ProcessTestWorkersV1;remote?:ProcessTestRemoteV1;report?:")
rep(S,"""SEMIO_WORKSPACE_ROOT:checkout,SEMIO_TEST_ARTIFACT_DIR:output,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"},""","""SEMIO_WORKSPACE_ROOT:checkout,SEMIO_TEST_ARTIFACT_DIR:output,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1",...budgetStretchEnvironmentV1(),...(run.remote?.environment??{})},
  ...(run.remote===undefined?{}:{remote:{width:run.remote.width,...(run.remote.measure===undefined?{}:{measure:(file:string,seconds:number)=>run.remote!.measure!(relative(checkout,file).split(sep).join("/"),seconds)}),take:async(file:string,index:number,signal:AbortSignal):Promise<OwnedBunFileResultV1|undefined>=>{
   const ran=await run.remote!.file(relative(checkout,file).split(sep).join("/"),{budgetMs:suite.budgetMs,index,total:bunSources.length,signal});
   return ran===undefined?undefined:Object.freeze({file,capture:Object.freeze({version:1 as const,stdout:"",stderr:ran.output===""||ran.output.endsWith("\\n")?ran.output:ran.output+"\\n",status:ran.passed?0:1,signal:null})});
  }}}),""")
rep(S,""" if(failed.length)throw refuseProcessV1("failed","script.suite-files-failed",{suite:suite.id,count:String(failed.length),files:failed.join(", ")});""",""" if(failed.length){const order=new Map(files.map((file,position)=>[file,position] as const));failed.sort((left,right)=>(order.get(left)??0)-(order.get(right)??0));throw refuseProcessV1("failed","script.suite-files-failed",{suite:suite.id,count:String(failed.length),files:failed.join(", ")});}""")

rep("📦️packages/🟦️typescript/🧪️testing/🗂️suites/🏃️run/🟦️.ts","export type { ProcessTestLaunchersV1 }","export type { ProcessTestLaunchersV1, ProcessTestRemoteRanV1, ProcessTestRemoteV1 }")

# new test (staged beside this script)
NEW[os.path.join(R,"🧪️testing/🟨️bun/📁️files/🧪️tests/🌐️remote/🟦️.ts")]=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"staged/bun-remote-test.ts"),encoding="utf-8").read()
NEW[os.path.join(R,"🧪️testing/🎚️budget/⏲️contention/🧪️tests/🧬️inherit/🟦️.ts")]=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"staged/stretch-inherit-test.ts"),encoding="utf-8").read()
flush()
