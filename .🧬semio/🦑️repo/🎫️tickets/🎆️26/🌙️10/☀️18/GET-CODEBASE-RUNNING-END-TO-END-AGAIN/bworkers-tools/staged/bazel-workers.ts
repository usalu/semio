import {WORKER_ACTION_KINDS_V1,type WorkerActionKindV1,type WorkerMappingV1,type WorkerPlanV1} from "../../../👷️workers/🟦️.ts";

/** 🚀️ The mnemonic of actions that start a service; they never run in workers. */
export const BAZEL_LAUNCHER_MNEMONIC_V1="RepoLauncher";

const MNEMONICS:Readonly<Record<WorkerActionKindV1,string>>=Object.freeze({test:"RepoTest",check:"RepoCheck",generate:"RepoGenerate",build:"RepoBuild",script:"RepoScript",service:BAZEL_LAUNCHER_MNEMONIC_V1});

/** 🏷️ Names the Bazel action mnemonic of one neutral action kind; Bazel keys its worker pools, strategies and instance limits by mnemonic. */
export function bazelWorkerMnemonicV1(kind:WorkerActionKindV1):string{return MNEMONICS[kind];}

/** 🧰️ What the plan becomes in Bazel: command flags and a report of how exactly each contract field is honoured. */
export type BazelWorkerSettingsV1=Readonly<{flags:readonly string[];mapping:readonly WorkerMappingV1[]}>;

/** 🗺️ Maps a resolved worker plan to Bazel: `--jobs`, one `--strategy` per mnemonic (workers first, sandboxed and local as fallbacks, because host-state actions only run locally), `--worker_max_instances`, multiplexing, `--worker_sandboxing`, cancellation, strict flagfiles and the total worker memory limit with pool shrinking; the per-worker memory bound is kept by the runtime itself. Service launchers never use workers. */
export function bazelWorkerSettingsV1(plan:WorkerPlanV1):BazelWorkerSettingsV1{
 const flags:string[]=[`--jobs=${plan.localJobs}`],mapping:WorkerMappingV1[]=[
  {field:"protocol",support:"exact",setting:plan.enabled?"execution requirement requires-worker-protocol=json with --experimental_worker_strict_flagfiles":"execution requirement requires-worker-protocol=json"},
  {field:"localJobs",support:"exact",setting:`--jobs=${plan.localJobs}`},
  {field:"resetAfterRequests",support:"exact",setting:`worker runtime: garbage collection every ${plan.resetAfterRequests} requests`},
  {field:"sandboxing",support:"exact",setting:plan.enabled&&plan.sandboxing?"--worker_sandboxing":"workers run unsandboxed"},
  {field:"cancellation",support:"exact",setting:plan.enabled&&plan.cancellation?"--experimental_worker_cancellation with execution requirement supports-worker-cancellation":"cancellation requests are not sent"},
  {field:"memory.totalMegabytes",support:"exact",setting:`--experimental_total_worker_memory_limit_mb=${plan.memory.totalMegabytes} with --experimental_shrink_worker_pool`},
  {field:"memory.perWorkerMegabytes",support:"approximate",setting:`worker runtime: garbage collection when the resident set exceeds ${plan.memory.perWorkerMegabytes} MB after a request`}
 ];
 let multiplexed=false;
 for(const kind of WORKER_ACTION_KINDS_V1){
  const row=plan.kinds[kind],name=bazelWorkerMnemonicV1(kind),field=(key:string)=>`kinds.${kind}.${key}`;
  if(kind==="service"){mapping.push(...["workers","maxInstances","multiplex","inProcess"].map(key=>({field:field(key),support:"exact" as const,setting:"launcher actions declare no worker support"})));continue;}
  const strategy=`--strategy=${name}=${row.workers?"worker,sandboxed,local":"sandboxed,local"}`;
  flags.push(strategy);mapping.push({field:field("workers"),support:"exact",setting:strategy});
  if(!row.workers){mapping.push(...["maxInstances","multiplex","inProcess"].map(key=>({field:field(key),support:"exact" as const,setting:"no worker pool for this mnemonic"})));continue;}
  const instances=`--worker_max_instances=${name}=${row.maxInstances}`;flags.push(instances);mapping.push({field:field("maxInstances"),support:"exact",setting:instances});
  if(row.multiplex){multiplexed=true;const multiplex=`--worker_max_multiplex_instances=${name}=${plan.localJobs}`;flags.push(multiplex);mapping.push({field:field("multiplex"),support:"exact",setting:`${multiplex} with --worker_multiplex`});}
  else mapping.push({field:field("multiplex"),support:"exact",setting:"execution requirement supports-multiplex-workers is not declared; --noworker_multiplex unless another kind multiplexes"});
  mapping.push({field:field("inProcess"),support:"exact",setting:row.inProcess?"worker runtime runs the action body in the worker process":"worker runtime starts one child process per request"});
 }
 if(plan.enabled){
  flags.push(multiplexed?"--worker_multiplex":"--noworker_multiplex");
  if(plan.sandboxing)flags.push("--worker_sandboxing");
  if(plan.cancellation)flags.push("--experimental_worker_cancellation");
  flags.push("--experimental_worker_strict_flagfiles",`--experimental_total_worker_memory_limit_mb=${plan.memory.totalMegabytes}`,"--experimental_shrink_worker_pool");
 }
 return Object.freeze({flags:Object.freeze(flags),mapping:Object.freeze(mapping)});
}
