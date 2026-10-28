"""Anchored patches of the cache-gc command and launcher: the Nx toolkit store delegate, the delegate hook and the background launch of the collector."""
RETENTION = "../../../⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention"
CONTROL = "import(\"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts\").ResourceLeaseControl"
INVOCATION = "import(\"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts\").ScriptInvocation"
ROUTING = "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"
OPENING = "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening"

FUNCTION = """/** 🧶️ Bounds the Nx toolkit generations of the tools image: the newest few stay, the current activation and every generation a live reader holds always stay, the rest is retired oldest first (a limited number per run) inside the whole-image exclusive scope, which never waits for running commands unless the image is pressed by the machine budget or its free space; a dry run only plans. */
async function nxToolingReclaim(root:string,dryRun:boolean,language:CacheLanguageV1,phaseControl:@CONTROL@,invocation:@INVOCATION@,machine:MachinePlanV1|null):Promise<string>{
 const{runNxToolingStoreBoundV1,censusNxToolingStoreV1,formatNxToolingStoreBoundV1}=await import("@R@/🎚️store-bound/🏃️run/🟦️.ts"),{probeNxToolingHeldV1}=await import("@R@/🎚️store-bound/🏃️run/🔌️host/🟦️.ts"),{nxToolingStoreBoundPolicyV1}=await import("@R@/🎚️store-bound/🟦️.ts"),opening=await import("@O@/🟦️.ts"),routing=await import("@ROUTING@");
 const area=cacheLedgerV1().areas.find(row=>row.id==="nx-tooling")!,policy=nxToolingStoreBoundPolicyV1(),mount=join(root,budgetPolicy.partitions.tools.directory,"m"),leaseDirectory=opening.repositoryLeaseDirectoryV1(root),signal=phaseControl.signal;
 const progress=(stage:string)=>routing.advanceScriptInvocation(invocation,"cache:"+stage,"running"),free=async():Promise<number|null>=>{try{const row=statfsSync(mount);return row.bavail*row.bsize;}catch{return null;}};
 let pressure:boolean|undefined;
 const pressed=async():Promise<boolean>=>pressure??=((await free())??Infinity)<area.reclaim!.softAvailableBytes||(machine?.stores.find(store=>store.id==="tools")?.excessBytes??0)>0;
 const retire=async(identities:readonly string[],onIdentity:(identity:string,index:number,total:number)=>Promise<void>)=>{
  const{withRepositoryCacheStoreMaintenanceV1,withRepositoryCacheStoreQueuedMaintenanceV1}=await import("@O@/🏪️store/🟦️.ts"),{retireQuiescentManagedToolingGenerationsV1}=await import("@R@/📥️physical/🟦️.ts"),request={version:1 as const,mode:"count-bound" as const,identities},wait=(await pressed())?policy.maximumWaitUnderPressureMs:policy.maximumWaitMs;
  const scope=(store:Parameters<typeof retireQuiescentManagedToolingGenerationsV1>[1],actual:typeof phaseControl)=>retireQuiescentManagedToolingGenerationsV1(root,store,actual,request,onIdentity);
  try{const result=await(wait>0?withRepositoryCacheStoreQueuedMaintenanceV1(root,"tools",invocation,phaseControl,{version:1,mode:"queued",maximumWaitMilliseconds:wait},scope):withRepositoryCacheStoreMaintenanceV1(root,"tools",invocation,phaseControl,scope));return{retired:result.retired,held:result.held,kept:result.kept};}
  catch(error){if(String((error as Error).message)==="Quiescent pre-pin cache storage admission required")return "busy" as const;throw error;}
 };
 const ports={signal,now:()=>Date.now(),progress,census:()=>censusNxToolingStoreV1(root,mount,{signal,progress}),held:(identity:string)=>probeNxToolingHeldV1(leaseDirectory,identity,signal),pressure:pressed,freeBytes:free,...dryRun?{}:{retire}};
 return formatNxToolingStoreBoundV1(language,await runNxToolingStoreBoundV1(dryRun,ports,policy)).join("\\n");
}

"""
for key, value in {"@CONTROL@": CONTROL, "@INVOCATION@": INVOCATION, "@R@": RETENTION, "@O@": OPENING, "@ROUTING@": ROUTING}.items():
    FUNCTION = FUNCTION.replace(key, value)

COMMAND_PATCHES = [
 ('import {cacheRunnerPortsV1,collectCacheV1,machineHostFreeBytesV1,machineUsesV1,reportCacheLedgerV1,type CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";',
  'import {cacheRunnerPortsV1,collectCacheV1,machineHostFreeBytesV1,machineUsesV1,reportCacheLedgerV1,type CacheRunnerPortsV1} from "../🏃️runner/🟦️.ts";\nimport type {MachinePlanV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts";'),
 ('/** 🏗️ Binds the command to the running build CLI', FUNCTION + '/** 🏗️ Binds the command to the running build CLI'),
 ('  delegate:(owner,id,dryRun,language)=>{\n',
  '  delegate:(owner,id,dryRun,language,context)=>{\n   if(owner==="nx-tooling"&&id==="nx-tooling")return nxToolingReclaim(root,dryRun,language,phaseControl,invocation,context?.machine??null).catch(error=>{if(invocation.control.signal.aborted)throw error;return "failed: "+String((error as Error).message);});\n'),
]
