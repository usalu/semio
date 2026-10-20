
/** 🧪️ A caller-provided Bazel host layout: binary, output base, install base, disk cache and generated workspace, all ASCII, plus an optional bun wrapper for the action launcher. */
export interface BazelScratchHostV1{bazel:string;workspace:string;outputBase:string;installBase:string;diskCache:string;environment:NodeJS.ProcessEnv;options?:string[];bun?:string}

async function prepareBazelScratchV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,host:BazelScratchHostV1){
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
 const plan=planBazelTasksV1(graph,request),workspace=host.workspace;if(Object.values(plan.tasks).some(task=>task.definition.continuous))throw new Error("Scratch Bazel host refuses continuous tasks");
 await mkdir(workspace,{recursive:true,mode:0o700});const implementation=await publishBazelImplementationV1(root,workspace,invocation);await generateBazelWorkspaceV1(root,graph,request,plan,workspace,invocation,phaseControl,implementation,()=>{});
 if(host.bun){const build=join(workspace,"BUILD.bazel");await writeFile(build,(await readFile(build,"utf8")).replaceAll(`bun = ${JSON.stringify(process.execPath)}`,`bun = ${JSON.stringify(host.bun)}`));}
 const emitted=bazelActionsV1(graph,plan,request.skipCache===true),workers=planWorkersV1(workerPolicy,currentWorkerHostV1(),{...(request.parallel?{jobs:request.parallel}:{}),...(request.workers===undefined?{}:{workers:request.workers})}),settings=bazelWorkerSettingsV1(workers),startup=[`--output_base=${host.outputBase}`,`--max_idle_secs=${policy.serverIdleSeconds}`,`--install_base=${host.installBase}`,"--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(root,".bazelrc")}`],options=["--lockfile_mode=off","--incompatible_strict_action_env",`--disk_cache=${host.diskCache}`,"--incompatible_autoload_externally=",...settings.flags,...(request.bail===true?[]:["--keep_going"]),...(host.options??[])];
 const environment:NodeJS.ProcessEnv={...host.environment,SEMIO_BAZEL_SELECTION_CHAIN:"scratch",SEMIO_BAZEL_WORKER_PLAN:JSON.stringify(workers),...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};
 await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
 const bin=resolve((await captureOwnedCommandV1(host.bazel,[...startup,"info","bazel-bin",...options.filter(option=>!settings.flags.includes(option)&&!option.startsWith("--profile=")&&!option.startsWith("--execution_log"))],workspace,{signal:phaseControl.signal,control:phaseControl,env:environment})).trim().split("\n").at(-1)!);
 if(!bin.includes(sep+"execroot"+sep))throw new Error("Original Bazel execution root required");
 const births=new Map<string,{digest:string;parameters:Readonly<Record<string,readonly (readonly string[])[]>>}>();for(const action of emitted.values()){if(action.kind==="noop")continue;const task=plan.tasks[action.id]!,source=await readFile(join(workspace,task.label+".json"));births.set(task.id,{digest:hash(source),parameters:action.kind==="launch"?{launcher:[[join(bin,task.label)]],launch:[action.restore.map(label=>join(bin,label+".snapshot"))]}:{action:[[join(bin,task.label+".snapshot"),...action.restore.map(label=>join(bin,label+".snapshot"))]]}});}
 const controller=new AbortController(),control=await bazelCancellationControl(invocation,controller.signal,births);Object.assign(environment,{SEMIO_BAZEL_CONTROL_URL:control.url});
 await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
 return {plan,workspace,implementation,emitted,workers,settings,startup,options,environment,bin,controller,control};
}

/** 🧪️ Runs the production native owner (workspace generation, control channel, action births, snapshot restores) against a scratch host without storage images, leases or module setup, with the worker plan and flags of the production owner. */
export async function runBazelTasksScratchV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,host:BazelScratchHostV1):Promise<{code:number;workers:WorkerPlanV1;flags:readonly string[]}>{
 const state=await prepareBazelScratchV1(root,graph,request,invocation,phaseControl,host);
 await withBazelTaskCleanupV1(async()=>{
  await runOwnedCommand(host.bazel,[...state.startup,"build",...state.options,...state.plan.selected.map(id=>"//:"+state.plan.tasks[id]!.label)],state.workspace,"bazel:build",compilerAdmissionBudgetV1(phaseControl),{signal:state.control.commandSignal,env:state.environment});
  for(const id of state.plan.selected){const action=state.emitted.get(id)!;if(action.kind==="task")await restoreBazelSnapshotV1(root,join(state.bin,action.label+".snapshot"),invocation,phaseControl);}
 },async()=>{state.controller.abort(Error("Bazel selected task closure completed"));await state.control.stop();});
 return {code:0,workers:state.workers,flags:state.settings.flags};
}

/** 🧪️ Starts a persistent worker by hand with the argument shape Bazel uses, sends one work request, then a protocol cancel request, and reports how the worker answered. */
export async function probeBazelWorkerCancelScratchV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,host:BazelScratchHostV1,probe:{task:string;afterMilliseconds:number;leafMarker:string}):Promise<{responses:string[];cancelledAfterMilliseconds:number;leafAliveBefore:boolean;leafAliveAfter:boolean}>{
 const state=await prepareBazelScratchV1(root,graph,request,invocation,phaseControl,host),task=state.plan.tasks[probe.task]!,action=state.emitted.get(probe.task)!;
 const alive=()=>Bun.spawnSync(["pgrep","-f",probe.leafMarker]).exitCode===0;
 try{
  const worker=Bun.spawn([process.execPath,join(state.workspace,"📜️script.ts"),state.implementation.snapshot,state.implementation.sha256,"work-action","--persistent_worker"],{cwd:state.workspace,env:{PATH:process.env.PATH??"",BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},stdin:"pipe",stdout:"pipe",stderr:"inherit"});
  worker.stdin.write(JSON.stringify({arguments:[join(state.workspace,task.label+".json"),join(state.bin,task.label+".snapshot"),...action.restore.map(label=>join(state.bin,label+".snapshot"))],inputs:[]})+"\n");await worker.stdin.flush();
  await Bun.sleep(probe.afterMilliseconds);const leafAliveBefore=alive(),sent=performance.now();
  worker.stdin.write('{"cancel":true}\n');await worker.stdin.flush();
  const responses:string[]=[],reader=worker.stdout.getReader(),decoder=new TextDecoder();let buffer="";
  const deadline=setTimeout(()=>worker.kill(),30000);
  while(!responses.length){const row=await reader.read();if(row.done)break;buffer+=decoder.decode(row.value);for(let end=buffer.indexOf("\n");end>=0;end=buffer.indexOf("\n")){responses.push(buffer.slice(0,end));buffer=buffer.slice(end+1);}}
  const cancelledAfterMilliseconds=Math.round(performance.now()-sent);clearTimeout(deadline);await Bun.sleep(500);
  const leafAliveAfter=alive();worker.stdin.end();worker.kill();await worker.exited;
  return {responses,cancelledAfterMilliseconds,leafAliveBefore,leafAliveAfter};
 }finally{state.controller.abort(Error("Bazel selected task closure completed"));await state.control.stop();}
}
