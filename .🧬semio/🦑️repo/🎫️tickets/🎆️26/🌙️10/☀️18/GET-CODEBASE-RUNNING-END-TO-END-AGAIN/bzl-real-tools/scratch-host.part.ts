
/** 🧪️ A caller-provided Bazel host layout: binary, output base, install base, disk cache and generated workspace, all ASCII. */
export interface BazelScratchHostV1{bazel:string;workspace:string;outputBase:string;installBase:string;diskCache:string;environment:NodeJS.ProcessEnv;options?:string[]}

/** 🧪️ Runs the production native owner (workspace generation, control channel, action births, snapshot restores) against a scratch host without storage images, leases or module setup. */
export async function runBazelTasksScratchV1(root:string,graph:BuildGraphV1,request:BazelTaskRequestV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,host:BazelScratchHostV1):Promise<number>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
 const plan=planBazelTasksV1(graph,request),workspace=host.workspace;if(Object.values(plan.tasks).some(task=>task.definition.continuous))throw new Error("Scratch Bazel host refuses continuous tasks");
 await mkdir(workspace,{recursive:true,mode:0o700});const implementation=await publishBazelImplementationV1(root,workspace,invocation);await generateBazelWorkspaceV1(root,graph,request,plan,workspace,invocation,phaseControl,implementation,()=>{});
 const emitted=bazelActionsV1(graph,plan,request.skipCache===true),startup=[`--output_base=${host.outputBase}`,`--max_idle_secs=${policy.serverIdleSeconds}`,`--install_base=${host.installBase}`,"--host_jvm_args=-Xmx2048m","--nosystem_rc","--nohome_rc",`--bazelrc=${join(root,".bazelrc")}`],options=["--lockfile_mode=off","--incompatible_strict_action_env",`--disk_cache=${host.diskCache}`,"--incompatible_autoload_externally=",`--jobs=${request.parallel??4}`,...(request.bail===true?[]:["--keep_going"]),...(host.options??[])];
 const environment:NodeJS.ProcessEnv={...host.environment,SEMIO_BAZEL_SELECTION_CHAIN:"scratch",...(process.platform==="darwin"?{LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}:{})};
 await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
 const bin=resolve((await captureOwnedCommandV1(host.bazel,[...startup,"info","bazel-bin",...options.filter(option=>!option.startsWith("--jobs="))],workspace,{signal:phaseControl.signal,control:phaseControl,env:environment})).trim().split("\n").at(-1)!);
 if(!bin.includes(sep+"execroot"+sep))throw new Error("Original Bazel execution root required");
 const births=new Map<string,{digest:string;parameters:Readonly<Record<string,readonly (readonly string[])[]>>}>();for(const action of emitted.values()){if(action.kind==="noop")continue;const task=plan.tasks[action.id]!,source=await readFile(join(workspace,task.label+".json"));births.set(task.id,{digest:hash(source),parameters:action.kind==="launch"?{launcher:[[join(bin,task.label)]],launch:[action.restore.map(label=>join(bin,label+".snapshot"))]}:{action:[[join(bin,task.label+".snapshot"),...action.restore.map(label=>join(bin,label+".snapshot"))]]}});}
 const controller=new AbortController(),control=await bazelCancellationControl(invocation,controller.signal,births);Object.assign(environment,{SEMIO_BAZEL_CONTROL_URL:control.url});
 await withBazelTaskCleanupV1(async()=>{
  await writeBazelRuntimeV1(workspace,root,scriptInvocationEnvironment(invocation,environment));
  await runOwnedCommand(host.bazel,[...startup,"build",...options,...plan.selected.map(id=>"//:"+plan.tasks[id]!.label)],workspace,"bazel:build",compilerAdmissionBudgetV1(phaseControl),{signal:control.commandSignal,env:environment});
  for(const id of plan.selected){const action=emitted.get(id)!;if(action.kind==="task")await restoreBazelSnapshotV1(root,join(bin,action.label+".snapshot"),invocation,phaseControl);}
 },async()=>{controller.abort(Error("Bazel selected task closure completed"));await control.stop();});
 return 0;
}
