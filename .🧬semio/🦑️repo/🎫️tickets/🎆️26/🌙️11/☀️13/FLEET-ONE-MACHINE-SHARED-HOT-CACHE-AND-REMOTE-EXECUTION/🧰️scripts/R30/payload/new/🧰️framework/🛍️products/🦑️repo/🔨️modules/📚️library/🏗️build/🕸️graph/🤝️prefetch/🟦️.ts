import type {BuildGraphPartStubV1} from "../📋️model/🟦️.ts";
import {pinnedAuthorityReadV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import * as observationV1 from "../../👁️observation/🟨️.mjs";
import {createGraphPhaseClockV1,writeGraphPhaseRowV1} from "../⏱️phases/🟨️.mjs";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {withRepositoryBuildGraphSelectionV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {readBuildOwnerRegistryV1,type BuildOwnerRegistryV1} from "../🚪️admission/🟦️.ts";
import {lstat} from "node:fs/promises";
import {join,isAbsolute} from "node:path";
import {createHash} from "node:crypto";
import {checkScriptInvocation,requireIssuedResourceControlInvocationV1,type ScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease,acquireFreshResourceLeaseV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {GENERATION_POLICY_V1,readGenerationPointerV1} from "../🕰️generations/🟨️.mjs";
import {createClaimRegistryV1,createQueuedClaimLeasePortV1,prepareStaleProjectsV1,renderClaimMessageV1,slotCountV1,type ClaimMessageKeyV1} from "../🕰️generations/🧩️claims/🟨️.mjs";
import {totalmem} from "node:os";
import {createPlanStageGuardV1} from "../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import {refuseBuildV1} from "../../📖️texts/🟦️.ts";
import {environmentDigestV1,environmentPartsV1} from "../🕰️generations/🌍️environment/🟨️.mjs";
import {diffEnvironmentV1,edgeConsumersV1} from "../🕰️generations/🧵️edge-reads/🟨️.mjs";
import type {EnvironmentPartsV1,EnvironmentStoredPartsV1} from "../🕰️generations/🧵️edge-reads/🟨️.mjs";
import {requireBuildOwnerRegistryAuthorityV1,javascriptManagerV1,GRAPH_LIMITS_V1} from "../🔐️authority/🟦️.ts";
import {generationScopeV1,generationFilesV1,generationCustodyV1} from "../🛑️custody/🟦️.ts";


/** 🧩️ The stale projects of a pending refresh: the generation they were planned against, their names, their roots and the implementation digest of the planner. */
export type GenerationPrefetchRequestV1={readonly generation:number;readonly dirty:readonly string[];readonly roots:Readonly<Record<string,string>>;readonly structural:boolean;readonly implementation:string};


/** 🌐️ Fills the project records of the owners whose roots are in `roots` without assembling anything, as the plan stage `inference` of the plan guard (the slice service names the stage and operation of a plan that grows past its memory; the worker farm it starts is bounded by the same budget, `planFarmWorkersV1`): the inference the build runs for them with the options it runs it with (so the records are the ones the build finds warm), without the test and policy contributions, the edges and the environment. */
export async function prefetchBuildGraphProjectsV1(root:string,registry:BuildOwnerRegistryV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,files:readonly string[],roots:ReadonlySet<string>,hooks:OwnerHooksV1={}):Promise<number> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);registry=readBuildOwnerRegistryV1(registry);
  requireBuildOwnerRegistryAuthorityV1(registry,JSON.parse(pinnedAuthorityReadV1(join(root,"📋️build-owners.json"),()=>observationV1.observedFs.readFileSync(join(root,"📋️build-owners.json"),"utf8"))));
  const owner=await import("../📥️inference/🟨️.mjs");checkResourceLeaseControl(phaseControl);
  const context={workspaceRoot:root,invocation,phaseControl,registry,setupShare:setupShareV1(invocation)};
  const stages=createPlanStageGuardV1({refuse:detail=>refuseBuildV1("unserved","graph.plan-too-heavy",detail)});
  const captured=await stages.run(()=>observationV1.captureObservedSourceInputsV1(async()=>{
    stages.mark("inference");
    const options={analyzeLockfile:false,catalogProviders:registry.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})),snapshot:true,ownerRoots:roots,onOwnerObservations:(name:string)=>{hooks.done?.(name);},...hooks.skip===undefined?{}:{skipOwner:hooks.skip},...hooks.rank===undefined?{}:{ownerRank:hooks.rank}};
    return (await owner.discoverBuildProjectsV1(files.filter(file=>file.endsWith("📋️project.json")||file.endsWith("Cargo.toml")||javascriptManagerV1.lockFileNames().includes(file)||file.endsWith(".patch")),options,context)).length;
  },undefined,false));
  return captured.value;
}


/** 🎛️ What a continuous inference of a read that waits for the fleet consults while it runs: `skip(name)` before each project (the fleet delivered it, or the wait ended), `rank(name)` the order of the projects (lower first) and `done(name)` after each project is finished here. */
export type OwnerHooksV1={readonly skip?:(name:string)=>boolean;readonly rank?:(name:string)=>number;readonly done?:(name:string)=>void};

export type OfferedStateV1={readonly names:Set<string>;readonly loaded:Set<string>};
/** 🧺️ The names one read has offered and the directories their file was read from; a fresh state is a restarted read. */
export const createOfferedStateV1=():OfferedStateV1=>({names:new Set(),loaded:new Set()});
const processOfferedV1=createOfferedStateV1();
const loadOfferedV1=async(state:OfferedStateV1,environment:NodeJS.ProcessEnv):Promise<void>=>{
  const{steeringPathsV1,readOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs"),directory=steeringPathsV1(environment)?.directory;
  if(directory===undefined||state.loaded.has(directory))return;
  state.loaded.add(directory);for(const name of readOfferedV1(environment))state.names.add(name);
};
/** 📝️ Notes the projects a wave of this read looked up itself as offered: the lookup of a hit offers its record and an inference uploads it, so the offer pass has nothing left to do for them, now or after a restart of the read. */
export async function noteOfferedV1(names:readonly string[],environment:NodeJS.ProcessEnv=process.env,state:OfferedStateV1=processOfferedV1):Promise<void>{
  const{writeOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs");
  await loadOfferedV1(state,environment);for(const name of names)state.names.add(name);writeOfferedV1(environment,names);
}
/** 📤️ Offers the records of the projects other machines asked for and that are fresh in this tree to the fleet cache: a fresh project is not inferred by the read (its record holds), so nothing would upload it; one pass over exactly these projects looks their records up (a hit offers it) and infers nothing. A call offers at most `limit` of them (one wave of the read, at least the policy wave), never a project a wave of this read looked up itself and never one an earlier read offered (`noteOfferedV1`). `offer` runs the pass for the roots (the graph server's read gives `prefetchBuildGraphProjectsV1`); a failing pass is told and never fails the read. Answers the projects offered. */
export async function offerFreshPriorityV1(input:{stubs:Readonly<Record<string,{root:string}>>;offer(roots:ReadonlySet<string>):Promise<unknown>;environment?:NodeJS.ProcessEnv;state?:OfferedStateV1;limit?:number;say?(line:string):void}):Promise<number>{
  const environment=input.environment??process.env,state=input.state??processOfferedV1,{freshPriorityV1,writeOfferedV1}=await import("../🕰️generations/🧭️steering/🟨️.mjs");
  await loadOfferedV1(state,environment);
  const names=freshPriorityV1(environment,state.names,input.limit,name=>input.stubs[name]!==undefined);
  if(names.length===0)return 0;
  try{await input.offer(new Set(names.map(name=>input.stubs[name]!.root)));for(const name of names)state.names.add(name);writeOfferedV1(environment,names);return names.length;}
  catch(error){input.say?.("graph: offering "+names.length+" fresh projects to the fleet failed ("+String((error as Error)?.message??error).replace(/\s+/gu," ").slice(0,160)+")");return 0;}
}


/** 🔏️ The content stamp of every stale owner: a digest of the identity (path, size, modification time) of each captured file below its root, so a done claim covers an owner only for the content it was made for and an edit after the prefetch opens the owner again; an owner without a root, or whose files cannot be read, gets `-`, which only equals itself. */
export async function ownerContentStampsV1(root:string,files:readonly string[],roots:Readonly<Record<string,string>>,owners:readonly string[]):Promise<Readonly<Record<string,string>>>{
  const{lstat}=await import("node:fs/promises"),byRoot=new Map<string,string[]>(),rows=new Map<string,string[]>(),failed=new Set<string>();
  for(const name of owners){const base=roots[name];if(base===undefined)continue;const key=base==="."?"":base.replace(/\/+$/u,"");byRoot.set(key,[...byRoot.get(key)??[],name]);}
  for(const file of files){
    let owner:string[]|undefined,cut=file.length;
    for(;;){owner=byRoot.get(file.slice(0,cut));if(owner||cut<=0)break;cut=file.lastIndexOf("/",cut-1);if(cut<0){owner=byRoot.get("");break;}}
    if(!owner)continue;
    let row:string;
    try{const info=await lstat(isAbsolute(file)?file:join(root,file));row=file+"\0"+info.size+"\0"+Math.floor(info.mtimeMs);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT"){for(const name of owner)failed.add(name);continue;}row=file+"\0gone";}
    for(const name of owner){const list=rows.get(name)??rows.set(name,[]).get(name)!;if(list.length>=GRAPH_LIMITS_V1.ownerRowListMaximum)failed.add(name);else list.push(row);}
  }
  return Object.fromEntries(owners.map(name=>[name,roots[name]===undefined||failed.has(name)?"-":createHash("sha256").update((rows.get(name)??[]).sort().join("\n")).digest("hex").slice(0,16)]));
}


/** 🤝️ The one continuous inference of a read that waits for the fleet: the governed `prefetch` of exactly `names` (one claim over all of them, one plan stage `inference`, one worker farm bounded by the memory of the plan, `planFarmWorkersV1`, kept alive and reused for the whole run), in the order of `names`, consulting `skip` before each project (what the fleet delivered meanwhile is dropped) and telling `done` after each; `release` ends every worker process afterwards, also when it fails, and returns the host's capacity grants. @see ../../🪪️graph-state/🙋️wanted/📖️read/🟦️.ts */
export async function inferRunV1(input:{request:GenerationPrefetchRequestV1;names:readonly string[];skip(name:string):boolean;done(name:string):void;prefetch(request:GenerationPrefetchRequestV1,hooks:OwnerHooksV1):Promise<unknown>;release():Promise<unknown>}):Promise<void>{
  const roots=Object.fromEntries(input.names.flatMap(name=>input.request.roots[name]===undefined?[]:[[name,input.request.roots[name]!]])),rank=new Map(input.names.map((name,at)=>[name,at]));
  try{await input.prefetch({...input.request,dirty:[...input.names],roots},{skip:input.skip,rank:name=>rank.get(name)??Infinity,done:input.done});}finally{await input.release().catch(()=>undefined);}
}


/** 📖️ The fleet-first decision of a graph read with stale projects, taken before any memory is reserved for inference, by the memory of each project: the costs recorded on this machine and published by the serving machines (`🪪️graph-state/🙋️wanted/💰️cost`) say which projects are too heavy for it, and those are asked from the fleet and waited for however few they are. The admitted memory is half of what the host has (a declared smaller machine counts as declared), never more than the host has less a floor and never lowered by the free memory of the moment (`fleetAdmittedMegabytesV1`), never to the grant of the slice pool load that asks (a 2048 MB booking left a 16 GB worker a grant of 0 MB per project and a wave of 0 projects); it sets the size of a wave, while the grant per project stays the share of the machine until not even the smallest wave could ever fit (`fleetGrantMegabytesV1`); neither the wave nor the solo grant looks at the free memory of the moment (a worker with 464 MB spare while test runs held memory refused with exit 75: the capacity gate queues the inference with progress until the memory is there): a machine admitted less than a serving machine's headroom publishes the projects it cannot infer, waits (progress, bound, cancellation) for the fleet's answer and refuses with the designed `timeout` class (exit status 75) when more light projects remain than it can infer; the too heavy projects the fleet did not deliver are inferred here one at a time and alone with `soloGrantV1` (the result's `solo`, which `graphOwnerEnforcementV1` of the inference derives the same way and runs after the light ones), and only one that does not fit even that is refused; while the request is outstanding `infer` (the governed prefetch of the host) infers the light remainder here at the same time, lightest first and without what the fleet delivers meanwhile (`fleetReadV1`); the generation restore that follows carries every served project. `SEMIO_GRAPH_FLEET_READ=off`, `SEMIO_GRAPH_FLEET_WAIT_MILLISECONDS=<n>` (a shorter bound than the policy's, for proofs) and every fault of the fleet itself end in the local read. See `🪪️graph-state/🙋️wanted/📖️read`. */
export async function fleetReadStaleGenerationV1(input:{root:string;request:GenerationPrefetchRequestV1;invocation:ScriptInvocation;language:"en"|"de";environment?:NodeJS.ProcessEnv;hostMegabytes?:()=>number;infer?:(run:Readonly<{names:readonly string[];skip(name:string):boolean;done(name:string):void}>)=>Promise<unknown>}):Promise<unknown> {
  const environment=input.environment??process.env;
  if(environment.SEMIO_GRAPH_FLEET_READ==="off")return;
  try{
    const{fleetReadV1}=await import("../../🪪️graph-state/🙋️wanted/📖️read/🟦️.ts"),{graphFleetSettingsV1}=await import("../../🪪️graph-state/🧶️nx/🟦️.ts"),{graphFleetClientV1,graphPlatformV1}=await Promise.all([import("../../🪪️graph-state/🛰️exchange/🟦️.ts"),import("../../🪪️graph-state/🟦️.ts")]).then(([exchange,state])=>({graphFleetClientV1:exchange.graphFleetClientV1,graphPlatformV1:state.graphPlatformV1})),{remoteCacheFetchV1}=await import("../../🌐️remote-cache/🛰️fleet/🟦️.ts"),{remoteExecutionOriginV1}=await import("../../🌐️remote-execution/🌡️load/🟦️.ts"),memory=await import("../../🧠️memory/🟨️.mjs"),cost=await import("../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts");
    const{fleetAdmittedMegabytesV1,fleetGrantMegabytesV1}=await import("../../🪪️graph-state/🙋️wanted/🟦️.ts"),machine=input.hostMegabytes===undefined?memory.fleetHostReadingV1():undefined,reading={hostMegabytes:machine===undefined?input.hostMegabytes!():machine.hostMegabytes},admittedMegabytes=fleetAdmittedMegabytesV1(reading),grantMegabytes=fleetGrantMegabytesV1(reading);
    const{wantedPolicyV1}=await import("../../🪪️graph-state/🙋️wanted/🟦️.ts"),waited=Number(environment.SEMIO_GRAPH_FLEET_WAIT_MILLISECONDS),bound=Number.isInteger(waited)&&waited>0?waited:undefined,settings=graphFleetSettingsV1(input.root,environment),client=settings===undefined?undefined:graphFleetClientV1(settings,{fetch:remoteCacheFetchV1(environment,{root:input.root})});
    const context=cost.ownerCostContextV1(),fleetCosts=client===undefined||admittedMegabytes>=wantedPolicyV1.serveHeadroomMegabytes?undefined:await Promise.race([cost.readOwnerCostsV1({client,ceilingMegabytes:context.ceilingMegabytes}).catch(()=>undefined),new Promise<undefined>(resolve=>setTimeout(resolve,10_000).unref())]),table=cost.mergeOwnerCostTablesV1([cost.readOwnerCostFileV1(cost.ownerCostPathV1(input.root),context.ceilingMegabytes),fleetCosts]);
    return await fleetReadV1({stale:input.request.dirty,admittedMegabytes,grantMegabytes,...machine===undefined?{}:{readings:{host:machine.hostMegabytes,physical:machine.physicalMegabytes,limit:machine.limitMegabytes===null?(input.language=="en"?"none":"keine"):machine.limitMegabytes+" MB",available:machine.availableMegabytes===null?(input.language=="en"?"unknown":"unbekannt"):machine.availableMegabytes+" MB"}},soloGrant:cost.soloGrantV1(reading,wantedPolicyV1),client,estimate:name=>cost.estimateOwnerCostV1(table,name,input.request.roots[name]==="."?"root:language:cargo":undefined,undefined,context),requester:remoteExecutionOriginV1(input.root,environment),platform:graphPlatformV1(),graphKey:input.request.implementation,language:input.language,say:line=>{process.stderr.write(line+"\n");},signal:input.invocation.control.signal,...input.infer===undefined?{}:{infer:input.infer},...bound===undefined?{}:{policy:{...wantedPolicyV1,waitMilliseconds:bound}}});
  }catch(error){
    if(input.invocation.control.signal.aborted||(error as {name?:string;code?:string}).name==="RepoError"&&(error as {code?:string}).code==="unserved")throw error;
    process.stderr.write("graph: the fleet-first read failed ("+String((error as Error)?.message??error).replace(/\s+/gu," ").slice(0,200)+"), reading locally\n");
  }
}


/** 🎟️ The compute slots of a graph rebuild: the configured count (the memory share of the claim registry, or the forced `SEMIO_GRAPH_COMPUTERS`), lowered to what rebuilds measurably took on earlier runs of this host so the slots are admitted by the memory they use; a store that cannot be read keeps the configured count. */
async function rebuildSlotsV1():Promise<number>{
  const configured=slotCountV1({memoryMegabytes:Math.floor(totalmem()/1048576),environment:process.env});
  if(process.env.SEMIO_GRAPH_COMPUTERS!==undefined)return configured;
  try{return Math.min(configured,(await import("../../🌡️host-capacity/🟦️.ts")).buildRebuildSlotsV1(process.env));}catch{return configured;}
}


/** 🤝️ Prepares the stale projects of a pending refresh together with the commands that wait for the same graph, before the exclusive build (the caller has asked for the capacity of the rebuild; the learned size is booked by `book` once a chunk is claimed, not while the run waits for a compute slot): claims a chunk of them, fills their project records in chunks that release the heap in between, joins what a peer computes, and never fails the command (the build computes what is left and reports a real failure). */
export async function prefetchStaleGenerationV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;registry:BuildOwnerRegistryV1;files:()=>readonly string[];request:GenerationPrefetchRequestV1;release:()=>Promise<void>;language:"en"|"de";book:()=>Promise<unknown>;hooks?:OwnerHooksV1;chunk?:number}):Promise<void> {
  const{root,invocation,phaseControl,registry,request}=input;
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  const say=(key:ClaimMessageKeyV1,fields:Record<string,string|number>)=>{process.stderr.write(renderClaimMessageV1(key,fields,input.language)+"\n");};
  try{await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const files=generationFilesV1(store),directory=join(store.root,"generations"),clock=createGraphPhaseClockV1({publish:resource=>control.publish({resource,mode:"shared",elapsedMs:0}),write:writeGraphPhaseRowV1});
    const claims=createClaimRegistryV1({root:directory,fs:files as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never,directory:join(root,".cache/b/l/m/data"),control})});
    let begun=false,booked=false;
    try{
      await prepareStaleProjectsV1({registry:claims,base:request.generation,implementation:request.implementation,wanted:request.dirty,stamps:await ownerContentStampsV1(root,input.files(),request.roots,request.dirty),slots:await rebuildSlotsV1(),signal:invocation.control.signal,say,...input.chunk===undefined?{}:{chunk:input.chunk},
        served:async()=>{const bytes=await files.read(join(directory,GENERATION_POLICY_V1.pointer)) as Uint8Array|undefined,pointer=bytes?readGenerationPointerV1(bytes):undefined;return pointer&&pointer.generation!==request.generation?pointer.generation:undefined;},
        progress:async({done,total})=>{if(!begun){begun=true;await clock.begin("prefetch",total);}await clock.advance("prefetch",done);},
        run:async owners=>{
          checkScriptInvocation(invocation);
          if(!booked){booked=true;await input.book();}
          const roots=new Set(owners.map(name=>request.roots[name]).filter((value):value is string=>value!==undefined));
          try{if(roots.size)await prefetchBuildGraphProjectsV1(root,registry,invocation,control,input.files(),roots,input.hooks);}finally{await input.release();}
        }});
    }finally{if(begun)await clock.end("prefetch").catch(()=>undefined);}
  },{optionalCaches:false,shared:true});}
  catch(error){if(invocation.control.signal.aborted)throw error;say("failed",{reason:String((error as Error)?.message??error).replace(/\s+/gu," ").slice(0,GRAPH_LIMITS_V1.reasonCharacters)});}
}


/** 🧺️ What one command's prefetch and build share of the whole-workspace setup of the inference (the native owners, the catalogue and the config bytes behind a validated observation, see `emojiProjectJsonNodes`): one per command, handed along in the inference context, dropped by the final release of the command; `computed` and `reused` count the setups. */
export const setupSharesV1=new WeakMap<object,{readonly cache:Map<string,unknown>;computed:number;reused:number}>();

export function setupShareV1(invocation:object):{readonly cache:Map<string,unknown>;computed:number;reused:number}{
  let share=setupSharesV1.get(invocation);if(!share)setupSharesV1.set(invocation,share={cache:new Map(),computed:0,reused:0});
  return share;
}


/** 💤️ Whether this command may build at once: a slice claim on its stale owners would be held at once and no live peer claim covers a stale owner of its plan, so a prefetch first would compute the owners twice. Any doubt or failure answers false and the prefetch runs as before. */
export async function builderIdleV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;request:GenerationPrefetchRequestV1}):Promise<boolean> {
  const{root,invocation,phaseControl,request}=input;
  try{
    requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
    return await withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
      const claims=createClaimRegistryV1({root:join(store.root,"generations"),fs:generationFilesV1(store) as never,lease:createQueuedClaimLeasePortV1({acquire:acquireQueuedResourceLease as never,tryAcquire:acquireResourceLease as never,acquireFresh:acquireFreshResourceLeaseV1 as never,directory:join(root,".cache/b/l/m/data"),control})});
      return claims.idle({base:request.generation,implementation:request.implementation,wanted:request.dirty,builderFree:()=>generationCustodyV1(root,store,control).idle({owners:request.dirty,structural:request.structural,signal:invocation.control.signal}),signal:invocation.control.signal});
    },{optionalCaches:false,shared:true});
  }catch(error){if(invocation.control.signal.aborted)throw error;return false;}
}


/** 🧭️ What an attached reader looks at: whether a live claim of the builder custody (held or queued) conflicts with the owners of its request, and the generation the pointer names now; a failure answers by throwing and the attachment ends in the local read. */
export async function builderLookV1(input:{root:string;invocation:ScriptInvocation;phaseControl:ResourceLeaseControl;request:GenerationPrefetchRequestV1}):Promise<{held:boolean;generation:number|undefined}> {
  const{root,invocation,phaseControl,request}=input;
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return withRepositoryBuildGraphSelectionV1(root,invocation,phaseControl,generationScopeV1,async(store,control)=>{
    const idle=await generationCustodyV1(root,store,control).idle({owners:request.dirty,structural:request.structural,signal:invocation.control.signal}),bytes=await generationFilesV1(store).read(join(store.root,"generations",GENERATION_POLICY_V1.pointer)) as Uint8Array|undefined;
    return{held:!idle,generation:bytes?readGenerationPointerV1(bytes).generation:undefined};
  },{optionalCaches:false,shared:true});
}


/** 🧵️ The owners of the previous generation that keep their fragment and only get their edges again when the environment moved: every owner of a real project root that is not inferred again, except the `consumers` when they are known (the owners whose recorded lookups an edit of the environment touches, see `edgeConsumersOfPartV1`): the others keep their edges, are `carried` and only name the new environment. A withheld owner has no edges, but its fragment names the environment it was published for, so it is refreshed like the others; tagged owners are rediscovered as a group when the environment moves. */
export function edgeRefreshOwnersV1(input:{cold:boolean;environment:string;previousEnvironment:string|null;roots:ReadonlySet<string>;stubs:Readonly<Record<string,{root:string;kind:string;tag?:string}>>;inferred:ReadonlySet<string>;consumers?:ReadonlySet<string>}):{projects:string[];withheld:string[];carried:string[]} {
  if(input.cold||input.environment===input.previousEnvironment)return{projects:[],withheld:[],carried:[]};
  const kept=Object.entries(input.stubs).filter(([name,stub])=>!stub.tag&&!input.inferred.has(name)&&!input.roots.has(stub.root)),projects=kept.filter(([,stub])=>stub.kind==="project"),carried=(name:string)=>input.consumers!==undefined&&!input.consumers.has(name);
  return{projects:projects.filter(([name])=>!carried(name)).map(([name])=>name),withheld:kept.filter(([,stub])=>stub.kind==="unavailable").map(([name])=>name),carried:projects.filter(([name])=>carried(name)).map(([name])=>name)};
}


/** 🏛️ The owner table the environment of a generation was made of, read back from its stubs: the owners that are no test or policy owner and not withheld, as `[name, root, native root or null]`. */
export function previousEnvironmentOwnersV1(stubs:Readonly<Record<string,{root:string;kind:string;nativeRoot?:string;tag?:string}>>):[string,string,string|null][] {
  return Object.entries(stubs).filter(([,stub])=>!stub.tag&&stub.kind==="project").map(([name,stub])=>[name,stub.root,stub.nativeRoot??null]);
}


/** 🎯️ The owners that may not keep their edges when the environment moved from the previous generation to `parts`: what the stored parts and the stubs say the previous environment was is checked against its digest (a mismatch, or no stored parts, answers undefined: nothing is known and every kept owner is refreshed), the two environments are diffed and each kept owner's recorded lookups (`reads` of its stub) are matched against the diff. */
export function edgeConsumersOfPartV1(input:{previousEnvironment:string|null;previousParts?:EnvironmentStoredPartsV1;previousOwners:readonly (readonly [string,string,string|null])[];parts:EnvironmentPartsV1;stubs:Readonly<Record<string,BuildGraphPartStubV1>>;inferred:ReadonlySet<string>;roots:ReadonlySet<string>}):ReadonlySet<string>|undefined {
  if(input.previousParts===undefined)return undefined;
  const previous=environmentPartsV1({owners:input.previousOwners,...input.previousParts});
  if(environmentDigestV1(previous)!==input.previousEnvironment)return undefined;
  const rows=new Map(Object.entries(input.stubs).filter(([name,stub])=>!stub.tag&&stub.kind==="project"&&!input.inferred.has(name)&&!input.roots.has(stub.root)).map(([name,stub])=>[name,{root:stub.root,...(stub.nativeRoot?{nativeRoot:stub.nativeRoot}:{}),...(stub.reads?{reads:stub.reads}:{})}])),table=(owners:readonly (readonly [string,string,string|null])[])=>new Map(owners.map(row=>[row[0],[row[1],row[2]] as readonly (string|null)[]]));
  return edgeConsumersV1({diff:diffEnvironmentV1(previous,input.parts),rows,tables:[table(previous.owners),table(input.parts.owners)]});
}
