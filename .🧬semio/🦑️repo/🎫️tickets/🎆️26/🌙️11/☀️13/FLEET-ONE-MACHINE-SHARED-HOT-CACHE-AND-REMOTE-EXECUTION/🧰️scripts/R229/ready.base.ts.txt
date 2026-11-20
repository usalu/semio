import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{readFreeSpaceV1,type FreeSpaceOptionsV1,type FreeSpaceUnknownReasonV1}from"../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts";
import{createHash}from"node:crypto";
import{spawn,spawnSync}from"node:child_process";
import{closeSync,existsSync,mkdirSync,openSync,readFileSync,readSync,renameSync,rmSync,statSync,writeFileSync,writeSync}from"node:fs";
import{availableParallelism,homedir,hostname,setPriority,tmpdir}from"node:os";
import{delimiter,join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{fleetHostReadingV1}from"../../../🧠️memory/🟨️.mjs";
import{remoteExecutionOriginV1}from"../../🌡️load/🟦️.ts";
import type{RemoteExecutionSidecarContextV1}from"../🧰️shared/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{readyDeadlineV1,readyForgetTreeV1,readyStallTrackerV1,readyStalledV1,readyIdentityV1,readyKillTreeV1,readyKillPlanV1,readyLivenessV1,readyTreeCpuV1,readyLeaseVerdictV1,readyHolderRestartV1,readyStaleStopV1,readyStagePhaseV1,type ReadyPhaseV1,readyHolderWaitTrackerV1,readyHolderWaitOfV1,readyHolderCustodyV1,readyBackgroundReaderV1,readyForegroundBusyV1,type ReadyHolderWaitV1,type ReadyReadRecordV1,readyScaledTreePolicyV1,readyStallRestartV1,readyStepWatchV1,readyStepOverdueSecondsV1,type ReadyStepV1,readySemanticsV1,readySemanticsChangedV1,readyProcessStartSyncV1,readyProgressKeyV1,readyReadTableV1,readyReadTreeV1,readyTreeMembersV1,readyWriteTreeV1,READY_TREE_POLICY_V1,type ReadyKnownV1,type ReadyProcessV1}from"./🌲️process-tree/🟦️.ts";
import{CAS_RESERVE_FACTOR_V1}from"../../../🗄️cas/🧬️schema/🟦️.ts";
import{localMachineSpeedFactorV1,scaledMillisecondsV1}from"../../../🐢️speed/🟨️.mjs";
import{inspectTypescriptCompilerV1}from"../../../🚀️bootstrap/🔷️typescript-compiler/🟨️.mjs";
import{READY_CONTAINMENT_POLICY_V1,readyContainmentLineV1,readyContainmentStatusV1,readyHeirsV1,readyKillRecorderV1,readyOwnerTokenV1,readyReapOrphansV1,readyRegistryEntriesV1,readyRegistryRootV1,readySharedPidsV1,readySampleDelayV1,readyStopV1,readyStoppedWindowsV1,readySweepRostersV1,type ReadyKillReasonV1,type ReadyContainmentResultV1,type ReadyContainmentV1,type ReadyPersonV1,type ReadyReapedV1,type ReadyWindowV1}from"./🌲️process-tree/🧹️containment/🟦️.ts";

/** 🚦️ The numbers of the readiness loop (see `./🧬️schema/🔣️.json`). */
export const READY_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
/** 🪜️ The stages in order: what each runs and what it depends on. */
export const READY_STAGES_V1=schema.definitions.Stages.const as readonly ReadyStageV1[];
/** 🎯️ The declared probes of the `workspace` stage: one target of a handful of projects of different kinds, so readiness reads their closure and never the whole workspace. */
export const READY_PROBES_V1=schema.definitions.Probes.const as Readonly<{version:1;target:string;projects:readonly Readonly<{name:string;kind:string}>[]}>;
/** 🧬️ The schema of `ready.json`. */
export const READY_STATUS_SCHEMA_V1=schema.definitions.Status;
/** 🏔️ Whether the readiness loop of `origin` is busy (see `readyForegroundBusyV1`), read from its `ready.json` in `store`; a store that is not there or a file that cannot be read says no, so a machine without a readiness loop (the group server) is never held back. */
export function readyForegroundBusyOfStoreV1(store:string|undefined,origin:string,nowMs:number=Date.now()):boolean{
 if(store===undefined||store==="")return false;
 try{return readyForegroundBusyV1(JSON.parse(readFileSync(join(store,"v1","origins",origin,READY_POLICY_V1.statusName),"utf8")),nowMs);}catch{return false;}
}
const P=READY_POLICY_V1,ZERO="0".repeat(64),sha=(text:string|Uint8Array):string=>createHash("sha256").update(text).digest("hex");

export type ReadyStageV1=Readonly<{name:string;kind:"bootstrap"|"repo";arguments:readonly string[];timeoutMilliseconds:number;inputs:readonly string[];bootstrapInput:boolean}>;
export type ReadyPlatformV1="linux"|"macos"|"windows";
export type ReadyCodeV1="human-running"|"lease-held"|"memory-low"|"stage-failed"|"tooling-compiler"|"stage-timeout"|"stage-stalled"|"stage-stall-exhausted"|"stage-code-changed"|"stage-code-stale"|"stage-step-overdue"|"stage-holder-silent"|"store-full"|"disk-short"|"disk-unmeasured"|"disk-collecting"|"disk-consumers"|"loop-failed"|"spawn-failed"|"interrupted"|"server";
export type ReadyReasonCodeV1="first-run"|"inputs-changed"|"previous-failed"|"stalled"|"code-changed"|"code-stale"|"step-overdue"|"holder-silent"|"adopted"|"binary-missing";
export type ReadyReasonV1=Readonly<{code:ReadyReasonCodeV1;en:string;de:string}>;
export type ReadyRefusalV1=Readonly<{code:ReadyCodeV1;en:string;de:string}>;
export type ReadyStateNameV1="ready"|"running"|"waiting"|"backoff"|"observing"|"server";
export type ReadyStatusV1={version:1;origin:string;host:string;platform:ReadyPlatformV1;pid:number;state:ReadyStateNameV1;digest:string;ready:boolean;stage:string|null;stageIndex:number;stageCount:number;attempts:number;exit:number|null;refusal:ReadyRefusalV1|null;reason:ReadyReasonV1|null;containment?:ReadyContainmentV1|null;peakMegabytes:number|null;tail:string[];nextAttemptMs:number|null;readyMs:number|null;startedMs:number;updatedMs:number};
export type ReadyTimingV1=Readonly<{stage:string;startedMs:number;endedMs:number;seconds:number;exit:number|null;outcome:"ok"|"stage-failed"|"stage-timeout"|"stalled"|"spawn-failed"|"interrupted";peakMegabytes:number|null;growthMegabytes?:number|null;attempts:number;digest:string}>;
export type ReadyFailureV1=Readonly<{digest:string;stage:string;attempts:number;exit:number|null;signal?:string|null;code:ReadyCodeV1;nextAttemptMs:number;atMs:number;tail:readonly string[]}>;
export type ReadyLocalStateV1={version:1;stages:Record<string,{digest:string;doneMs:number}>;failure:ReadyFailureV1|null;attempts:number;peakMegabytes:number|null;readyMs:number|null;adopted:boolean};

/** 🖥️ The platform name of the fleet for a Node platform. */
export function readyPlatformV1(platform:NodeJS.Platform=process.platform):ReadyPlatformV1{return platform==="darwin"?"macos":platform==="win32"?"windows":"linux";}

/** 💡️ Why a stage is (re)started, with its text in English and German. */
export function readyReasonV1(code:ReadyReasonCodeV1,stage:string):ReadyReasonV1{
 const fill=(language:"en"|"de"):string=>(schema.definitions.Reasons.const[language] as Record<string,string>)[code]!.replace("{stage}",stage);
 return Object.freeze({code,en:fill("en"),de:fill("de")});
}

/** 🪦️ How a stage process ended in words of both languages: an exit status, the signal that ended it, or neither (the process vanished without leaving an exit file, so something outside the supervisor ended its wrapper). A pure function.
 * @see ./🧪️tests/🪦️ending */
export function readyEndingV1(exit:number|null,signal:string|null|undefined):Readonly<{en:string;de:string}>{
 if(exit!==null)return{en:`exit status ${exit}`,de:`Exit-Status ${exit}`};
 if(signal!==null&&signal!==undefined)return{en:`no exit status, ended by signal ${signal}`,de:`kein Exit-Status, durch das Signal ${signal} beendet`};
 return{en:"no exit status and no signal: the stage process vanished, the last lines of its log say how",de:"kein Exit-Status und kein Signal: der Stufenprozess verschwand, die letzten Zeilen seines Protokolls sagen wie"};
}

/** 🗣️ The refusal with its text in English and German. */
export function readyRefusalV1(code:ReadyCodeV1,fields:Readonly<Record<string,string|number|Readonly<{en:string;de:string}>>>={}):ReadyRefusalV1{
 const fill=(language:"en"|"de"):string=>(schema.definitions.Texts.const[language] as Record<string,string>)[code]!.replace(/\{(\w+)\}/gu,(_all,name:string)=>{const value=fields[name];return String(value!==null&&typeof value==="object"?value[language]:value??"");});
 return Object.freeze({code,en:fill("en"),de:fill("de")});
}

/** #️⃣ The digest of what readiness depends on: per stage the sha256 over its name, the platform, the digest of the stage before it and the content hash of each of its inputs (`absent` for a missing file), so a stage depends only on its own inputs, so an edit that touches the inputs of one stage never makes another stale (the stages run in a fixed order anyway); the digest of the machine is the sha256 over the stage digests. `hash` answers the sha256 of a file below the worktree or `undefined`. */
export function readyDigestsV1(input:Readonly<{platform:ReadyPlatformV1;hash:(path:string)=>string|undefined;stages?:readonly ReadyStageV1[]}>):Readonly<{digest:string;stages:readonly Readonly<{name:string;digest:string}>[]}>{
 const bootstrap=(input.platform==="windows"?P.bootstrap.windows:P.bootstrap.unix).join("/");
 const stages=(input.stages??READY_STAGES_V1).map(stage=>{
  const paths=[...stage.inputs,...stage.bootstrapInput?[bootstrap]:[]];
  const digest=sha(`${stage.name}\n${input.platform}\n${ZERO}\n`+paths.map(path=>`${path}\0${input.hash(path)??"absent"}\n`).join(""));
  return{name:stage.name,digest};
 });
 return Object.freeze({digest:sha(stages.map(stage=>stage.digest).join("\n")),stages:Object.freeze(stages)});
}

/** 🧪️ The hash of one input as readiness sees it: the tool table counts by what it provisions (tool ids, pins, locations, setup and steps, never summaries or texts), `package.json` by `packageManager`, `engines` and `workspaces`, every other file by its bytes, so ordinary landings (a new text, a new script, a new dependency of the build) never make a stage stale. */
export function readyInputHashV1(path:string,content:string):string{
 try{
  if(path.endsWith("🧰️tool-setup/🔣️.json")){const doc=JSON.parse(content) as {groups:unknown;tools:{id:string;pin:unknown;locate:unknown;setup:unknown}[];steps:{id:string;kind:string;configuration:unknown;platforms:unknown;timeoutSeconds:unknown}[]};return sha(JSON.stringify({groups:doc.groups,tools:doc.tools.map(row=>[row.id,row.pin,row.locate,row.setup]),steps:doc.steps.map(row=>[row.id,row.kind,row.configuration,row.platforms,row.timeoutSeconds])}));}
  if(path==="package.json"){const doc=JSON.parse(content) as {packageManager?:unknown;engines?:unknown;workspaces?:unknown};return sha(JSON.stringify({packageManager:doc.packageManager??null,engines:doc.engines??null,workspaces:doc.workspaces??null}));}
 }catch{}
 return sha(content);
}

/** ⏳️ The wait after the `attempts`-th failure in a row: it doubles from the initial wait to the maximum, grows by up to the jitter share (`unit` in [0,1), fixed per machine so the fleet does not retry in step) and never exceeds the maximum. */
export function readyBackoffV1(attempts:number,unit:number):number{
 const base=Math.min(P.backoffMaximumMilliseconds,P.backoffInitialMilliseconds*2**Math.max(0,attempts-1));
 return Math.min(P.backoffMaximumMilliseconds,Math.floor(base*(10000+Math.floor(unit*P.backoffJitterBasisPoints))/10000));
}

/** 💽️ The disk or quota that a failed stage reported as full, from the lines of its log: the errno (`EDQUOT` or `ENOSPC`) and the directory the lease store or a database could not create files in (see `storeFullPattern` of the schema; the message comes from the SQLite open explanation); `null` when no line says so. */
export function readyStoreFullV1(lines:readonly string[]):Readonly<{errno:string;path:string}>|null{
 const expression=new RegExp(P.storeFullPattern,"u");
 for(let index=lines.length-1;index>=0;index--){const found=expression.exec(lines[index]!);if(found!==null)return Object.freeze({errno:found[1]!,path:found[2]!});}
 return null;
}

/** 📦️ What a filesystem says it has free at a path: bytes available to the user and free inodes, `null` for a fact the platform does not report (Windows reports no inodes, a network share may report nothing). `freeBytes` is `null` exactly when the volume could not be measured, and then `unknown` says why: an unknown measurement is never read as zero bytes free. */
export type ReadyFilesystemFactsV1=Readonly<{freeBytes:number|null;freeInodes:number|null;unknown?:Readonly<{reason:FreeSpaceUnknownReasonV1;detail:string}>}>;
/** 🕳️ A path whose free space could not be measured, with the reason and a short detail. */
export type ReadyDiskUnknownV1=Readonly<{path:string;reason:string;detail:string}>;
/** 💽️ The first resource a stage lacks before it starts: the directory, `space` or `inodes`, what the stage needs and what is free (megabytes for `space`). */
export type ReadyDiskShortV1=Readonly<{path:string;resource:"space"|"inodes";needed:number;available:number}>;

/** 🧾️ One consumer a short disk names: the writer, its root, the bytes it holds and the exact command that frees it. */
export type ReadyDiskConsumerV1=Readonly<{id:string;root:string;bytes:number;command:string}>;
/** 🧹️ What the collection for a waiting stage answered: whether the need fits now and, when it does not, the largest consumers that remain. */
export type ReadyDiskRecoveryV1=Readonly<{fits:boolean;consumers:readonly ReadyDiskConsumerV1[]}>;
/** 📐️ What a stage needs free, in megabytes and inodes, and where the number comes from: the declared estimate (`diskRequirements`) until the machine has measured the stage, then the largest net growth of free space over the last successful runs times the safety factor of the workspace scale family (read from the cas budget, not copied), never below the floor. */
export function readyDiskNeedV1(stage:string,runs:readonly ReadyTimingV1[]=[]):Readonly<{megabytes:number;inodes:number;source:"declared"|"measured"}>{
 const declared=(P.diskRequirements as Record<string,{megabytes:number;inodes:number}>)[stage]??P.diskDefaultRequirement;
 const grown=runs.filter(run=>run.stage===stage&&run.outcome==="ok"&&typeof run.growthMegabytes==="number").slice(-P.diskGrowth.keptRuns).map(run=>run.growthMegabytes as number);
 if(grown.length===0)return Object.freeze({megabytes:declared.megabytes,inodes:declared.inodes,source:"declared" as const});
 return Object.freeze({megabytes:Math.max(P.diskGrowth.floorMegabytes,Math.ceil(Math.max(...grown)*CAS_RESERVE_FACTOR_V1)),inodes:declared.inodes,source:"measured" as const});
}

/** 🔎️ The facts of the filesystem that holds `path` (its nearest existing ancestor when it is not there yet), from the one free-space question of the build system (`readFreeSpaceV1`: `statfs` on POSIX, `GetDiskFreeSpaceExW` on Windows, where the runtime's `statfs` answers a row of zeros). A volume that cannot be measured has `freeBytes` `null` and the reason; a filesystem that reports no inode table (NTFS, btrfs, some network shares) has `freeInodes` `null`, never 0. Disk quotas of a user are not visible to `statfs` (only project quotas of a directory are), so a quota shortage still shows as the write error that the stage classifier names.
 * @see ../../../../../../../../🔨️modules/🏃️process/🆓️free-space/🟦️.ts
 * @see https://man7.org/linux/man-pages/man2/statfs.2.html */
export function readyFilesystemFactsV1(path:string,options?:Omit<FreeSpaceOptionsV1,"nearest">):ReadyFilesystemFactsV1{
 const facts=readFreeSpaceV1(path,{nearest:true,...options});
 return facts.known?Object.freeze({freeBytes:facts.availableBytes,freeInodes:facts.freeInodes}):Object.freeze({freeBytes:null,freeInodes:null,unknown:Object.freeze({reason:facts.reason,detail:facts.detail})});
}

/** 💽️ Whether the paths a stage writes to hold what it needs (`diskRequirements` of the schema, megabytes and inodes): the first resource that is short, `null` when all hold or nothing is known. Checked before the stage starts, so a full disk or an exhausted inode table is a typed refusal that names path, need and what is free, and not the raw database error the stage dies with half an hour later. A pure function of the facts provider.
 * @see ./🧪️tests/🔁️resume/🟦️.ts */
export function readyDiskShortV1(input:Readonly<{stage:string;paths:readonly string[];facts(path:string):ReadyFilesystemFactsV1|null;runs?:readonly ReadyTimingV1[]}>):ReadyDiskShortV1|null{
 const need=readyDiskNeedV1(input.stage,input.runs);
 for(const path of input.paths){
  const facts=input.facts(path);if(facts===null)continue;
  const bytes=facts.freeBytes!==null&&Number.isFinite(facts.freeBytes)&&facts.freeBytes>=0?facts.freeBytes:null,inodes=facts.freeInodes!==null&&Number.isFinite(facts.freeInodes)&&facts.freeInodes>=0?facts.freeInodes:null;
  if(bytes!==null&&bytes<need.megabytes*1048576)return Object.freeze({path,resource:"space" as const,needed:need.megabytes,available:Math.floor(bytes/1048576)});
  if(inodes!==null&&inodes<need.inodes)return Object.freeze({path,resource:"inodes" as const,needed:need.inodes,available:inodes});
 }
 return null;
}

/** 🕳️ The paths a stage writes to whose free bytes could not be measured (the provider answered nothing, or `freeBytes` is `null`, not finite or negative): the stage starts anyway with a warning, because "unknown" is not "full"; the gate of the stage and the write error itself stay. A pure function of the facts provider. */
export function readyDiskUnknownV1(input:Readonly<{paths:readonly string[];facts(path:string):ReadyFilesystemFactsV1|null}>):readonly ReadyDiskUnknownV1[]{
 const rows:ReadyDiskUnknownV1[]=[];
 for(const path of input.paths){
  const facts=input.facts(path);
  if(facts===null)rows.push(Object.freeze({path,reason:"unreadable",detail:"-"}));
  else if(facts.freeBytes===null||!Number.isFinite(facts.freeBytes)||facts.freeBytes<0)rows.push(Object.freeze({path,reason:facts.unknown?.reason??"incoherent-row",detail:facts.unknown?.detail??String(facts.freeBytes)}));
 }
 return Object.freeze(rows);
}

/** 🗣️ The warning of a stage that starts although a path could not be measured: English and German. */
export function readyDiskUnmeasuredV1(stage:string,row:ReadyDiskUnknownV1):ReadyRefusalV1{
 return readyRefusalV1("disk-unmeasured",{stage,path:row.path,reason:row.reason,detail:row.detail});
}

/** 🗣️ The refusal of a stage that lacks disk: English and German with the unit of the resource. */
export function readyDiskRefusalV1(stage:string,short:ReadyDiskShortV1,consumers?:readonly ReadyDiskConsumerV1[]):ReadyRefusalV1{
 const fields={stage,path:short.path,resource:short.resource,unit:short.resource==="space"?"MB":"inodes",needed:short.needed,available:short.available};
 return consumers===undefined?readyRefusalV1("disk-short",fields):readyRefusalV1("disk-consumers",{...fields,consumers:consumers.map(row=>`${row.root} (${Math.round(row.bytes/1048576)} MB): \`${row.command}\``).join("; ")||"-"});
}
/** 🧹️ The refusal of a stage whose need is being freed right now by the collectors of the registry. */
export function readyDiskCollectingV1(stage:string,short:ReadyDiskShortV1):ReadyRefusalV1{
 return readyRefusalV1("disk-collecting",{stage,path:short.path,unit:"MB",needed:short.needed,available:short.available});
}

/** 🩹️ The tail of a failed stage with the cause named: a database that "unable to open database file" while a path is out of space or inodes is a full disk, so the tail gets the line the classifier knows (`storeFullPattern`) for that path. */
export function readyTailWithDiskCauseV1(tail:readonly string[],short:ReadyDiskShortV1|null):readonly string[]{
 return short!==null&&tail.some(line=>line.includes(P.sqliteOpenPattern))?[...tail,`No space left (ENOSPC) in ${short.path}: the disk`]:tail;
}

/** 🧮️ How many jobs a stage's build tools may run at once: one fewer than the cores, and no more than the free memory pays for at `megabytesPerJob` each, at least one. */
export function readyJobsV1(input:Readonly<{cores:number;availableMegabytes:number|null}>):number{
 const byCores=Math.max(1,Math.floor(input.cores)-1);
 return input.availableMegabytes===null?byCores:Math.max(1,Math.min(byCores,Math.floor(input.availableMegabytes/P.megabytesPerJob)));
}

/** 🌳️ The resident megabytes of a process and all its descendants from `pid,ppid,rss(KB)` rows. */
export function readyTreeMegabytesV1(rows:string,root:number):number{
 const parent=new Map<number,number>(),size=new Map<number,number>();
 for(const line of rows.split("\n")){const [pid,ppid,rss]=line.trim().split(/\s+/u).map(Number);if(Number.isInteger(pid)&&Number.isInteger(ppid)){parent.set(pid!,ppid!);size.set(pid!,Number.isFinite(rss)?rss!:0);}}
 let kilobytes=0;
 for(const [pid,rss] of size){for(let at:number|undefined=pid,depth=0;at!==undefined&&depth<64;at=parent.get(at),depth++)if(at===root){kilobytes+=rss;break;}}
 return Math.round(kilobytes/1024);
}

export type ReadyDecisionInputV1=Readonly<{server:boolean;digest:string;stages:readonly Readonly<{name:string;digest:string}>[];done:Readonly<Record<string,string>>;marker:boolean;repo:boolean;human:boolean;lease:"free"|"own"|"other";memoryOk:boolean;failure:Readonly<{digest:string;nextAttemptMs:number}>|null;now:number;restartPending?:boolean}>;
export type ReadyDecisionV1=Readonly<{action:"server"|"idle"|"mark"|"observe"|"wait"|"backoff"|"run";stageIndex:number;code:ReadyCodeV1|null;waitMilliseconds:number}>;

/** ⚖️ The whole decision of one turn: the group server never; every stage current with the marker and the `repo` binary there is ready and does nothing; current stages without the marker only write it; without the binary the run starts again at the first stage; a person running the setup or another runner holding the lease is observed; a recorded failure of the same digest waits for its back-off (a changed input is a new cause and retries at once); too little free memory waits; otherwise the first stale stage runs. */
export function readyDecideV1(input:ReadyDecisionInputV1):ReadyDecisionV1{
 const act=(action:ReadyDecisionV1["action"],stageIndex:number,code:ReadyCodeV1|null,waitMilliseconds:number):ReadyDecisionV1=>Object.freeze({action,stageIndex,code,waitMilliseconds});
 if(input.server)return act("server",-1,"server",P.idlePollMilliseconds);
 const stale=input.stages.findIndex(stage=>input.done[stage.name]!==stage.digest);
 if(stale<0&&input.repo)return input.marker?act("idle",-1,null,P.idlePollMilliseconds):act("mark",-1,null,0);
 const index=stale<0||!input.repo?0:stale;
 if(input.restartPending===true)return act("wait",index,null,1000);
 if(input.human)return act("observe",index,"human-running",P.observePollMilliseconds);
 if(input.lease==="other")return act("observe",index,"lease-held",P.observePollMilliseconds);
 if(input.failure!==null&&input.failure.digest===input.digest&&input.failure.nextAttemptMs>input.now)return act("backoff",index,null,input.failure.nextAttemptMs-input.now);
 if(!input.memoryOk)return act("wait",index,"memory-low",P.memoryPollMilliseconds);
 return act("run",index,null,0);
}

/** 🔒️ The one lease per machine in `<worktree>/.cache/f/ready/lease.json`: created exclusively, renewed by its holder, taken over only when its wall time is older than the stale limit or its process is gone, and the takeover itself goes through an exclusive directory so two takers cannot both win. */
export function createReadyLeaseV1(input:Readonly<{directory:string;owner:string;pid?:number;startMs?:number;now?:()=>number;alive?:(pid:number)=>boolean;startOf?:(pid:number)=>number|null|undefined}>):Readonly<{acquire():"own"|"other";renew():boolean;release():void;peek():"free"|"own"|"other";holder():Readonly<{pid:number;owner:string;ageMilliseconds:number}>|undefined}>{
 const now=input.now??Date.now,pid=input.pid??process.pid,file=join(input.directory,P.leaseName),takeover=join(input.directory,"takeover"),startMs=input.startMs??Math.round(Date.now()-process.uptime()*1000);
 const alive=input.alive??((other:number):boolean=>{try{process.kill(other,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}});
 const startOf=input.startOf??((other:number)=>readyProcessStartSyncV1(other));
 const read=():{owner:string;pid:number;startMs?:number;wallMs:number}|undefined=>{try{return JSON.parse(readFileSync(file,"utf8"));}catch{return undefined;}};
 const verdict=(lease:{pid:number;startMs?:number;wallMs:number}|undefined)=>{
  if(lease===undefined)return readyLeaseVerdictV1({lease,nowMs:now(),staleMs:P.leaseStaleMilliseconds,process:"gone"});
  const stale=now()-lease.wallMs>P.leaseStaleMilliseconds,running=stale?"alive":alive(lease.pid)?"alive":"gone";
  return readyLeaseVerdictV1({lease,nowMs:now(),staleMs:P.leaseStaleMilliseconds,process:running,processStartMs:running==="alive"&&!stale&&lease.startMs!==undefined?startOf(lease.pid):null});
 };
 const stale=(lease:{pid:number;startMs?:number;wallMs:number}|undefined):boolean=>verdict(lease).state==="free";
 const record=():string=>JSON.stringify({owner:input.owner,pid,startMs,wallMs:now()});
 const create=():boolean=>{try{writeFileSync(file,record(),{flag:"wx"});return true;}catch{return false;}};
 return Object.freeze({
  acquire(){
   mkdirSync(input.directory,{recursive:true});
   if(create())return"own";
   const held=read();
   if(held?.owner===input.owner){this.renew();return"own";}
   if(!stale(held))return"other";
   try{if(now()-statSync(takeover).mtimeMs>30000)rmSync(takeover,{recursive:true,force:true});}catch{}
   try{mkdirSync(takeover);}catch{return"other";}
   try{
    if(!stale(read()))return"other";
    rmSync(file,{force:true});
    return create()?"own":"other";
   }finally{rmSync(takeover,{recursive:true,force:true});}
  },
  renew(){if(read()?.owner!==input.owner)return false;try{const temporary=`${file}.${pid}.tmp`;writeFileSync(temporary,record());renameSync(temporary,file);return true;}catch{return false;}},
  release(){if(read()?.owner===input.owner)rmSync(file,{force:true});},
  peek(){const held=read();return stale(held)?"free":held!.owner===input.owner?"own":"other";},
  holder(){const held=read();return held===undefined||stale(held)?undefined:{pid:held.pid,owner:held.owner,ageMilliseconds:now()-held.wallMs};},
 });
}

/** 🧑️ The evidence of one turn about a person's setup: how long ago `human.json` was touched, the `{pid,startedMs}` the person's wrapper wrote into it (`null` for the empty file of an older wrapper), whether that process lives, whether an older wrapper's command is in the process list, and the file age by its creation time. */
export type ReadyHumanEvidenceV1=Readonly<{now:number;touchedAgeMilliseconds:number;birthAgeMilliseconds:number;record:Readonly<{pid:number;startedMs:number}>|null;pidAlive:boolean;legacyProcess:boolean}>;

/** 🧑️ Whether a person is running the setup, decided on evidence only: the marker must be touched within the fresh limit AND its owner must still be there (the recorded process lives and started less than the maximum ago; an empty marker of an older wrapper counts only while that wrapper's command is in the process list), so a marker left by a dead or hung process expires by itself instead of blocking the machine. */
export function readyHumanVerdictV1(evidence:ReadyHumanEvidenceV1):boolean{
 if(evidence.touchedAgeMilliseconds>=P.humanFreshMilliseconds)return false;
 if(evidence.record===null)return evidence.legacyProcess&&evidence.birthAgeMilliseconds<P.humanMaximumMilliseconds;
 return evidence.pidAlive&&evidence.now-evidence.record.startedMs<P.humanMaximumMilliseconds;
}

/** 🧾️ The `{pid,startedMs}` of a `human.json` written by the wrapper of a person (schema `Human`), `null` when the text is empty or not that record. */
export function readyHumanRecordV1(text:string):Readonly<{pid:number;startedMs:number}>|null{
 try{const row=JSON.parse(text) as {version?:unknown;pid?:unknown;startedMs?:unknown};return row.version===1&&Number.isSafeInteger(row.pid)&&(row.pid as number)>0&&Number.isSafeInteger(row.startedMs)&&(row.startedMs as number)>0?{pid:row.pid as number,startedMs:row.startedMs as number}:null;}catch{return null;}
}

/** 🧑️ Reads the evidence from `<directory>/human.json` and the process table (`ps` on Linux and macOS; an empty marker has no owner to find on Windows, so it never counts there) and decides with `readyHumanVerdictV1`. */
export function readyHumanActiveV1(directory:string,now:number=Date.now(),probe:Readonly<{alive?(pid:number):boolean;commands?():readonly string[]}>={}):boolean{
 try{
  const file=join(directory,P.humanName),stat=statSync(file),record=readyHumanRecordV1(readFileSync(file,"utf8"));
  const commands=probe.commands??(():readonly string[]=>{if(process.platform==="win32")return[];const rows=spawnSync("ps",["-A","-o","args="],{encoding:"utf8"});return rows.status===0?rows.stdout.split("\n"):[];});
  return readyHumanVerdictV1({now,touchedAgeMilliseconds:now-stat.mtimeMs,birthAgeMilliseconds:now-stat.birthtimeMs,record,pidAlive:record!==null&&(probe.alive??alive)(record.pid),legacyProcess:record===null&&now-stat.mtimeMs<P.humanFreshMilliseconds&&commands().some(line=>line.includes(P.humanLegacyCommand))});
 }catch{return false;}
}

export const emptyReadyStateV1=():ReadyLocalStateV1=>({version:1,stages:{},failure:null,attempts:0,peakMegabytes:null,readyMs:null,adopted:false});

/** 📊️ The status file `<store>/v1/origins/<origin>/ready.json`: replaced atomically at most every few seconds and at once when the state changes; validated against the schema, never throws (a store that cannot be written costs nothing). */
export function createReadyStatusV1(input:Readonly<{store:string|undefined;origin:string;platform?:ReadyPlatformV1;now?:()=>number;rename?:(from:string,to:string)=>void}>):Readonly<{update(patch:Partial<ReadyStatusV1>):void;current():ReadyStatusV1}>{
 const now=input.now??Date.now,startedMs=now();
 let doc:ReadyStatusV1={version:1,origin:input.origin,host:hostname(),platform:input.platform??readyPlatformV1(),pid:process.pid,state:"waiting",digest:ZERO,ready:false,stage:null,stageIndex:0,stageCount:READY_STAGES_V1.length,attempts:0,exit:null,refusal:null,reason:null,peakMegabytes:null,tail:[],nextAttemptMs:null,readyMs:null,startedMs,updatedMs:startedMs},written=0,timer:ReturnType<typeof setTimeout>|undefined;
 const write=():void=>{
  timer=undefined;written=now();
  if(input.store===undefined||input.store==="")return;
  try{
   const out={...doc,updatedMs:now()};
   if(validateJsonSchemaSubset(READY_STATUS_SCHEMA_V1,out,schema).length)return;
   const directory=join(input.store,"v1","origins",input.origin),file=join(directory,P.statusName),temporary=`${file}.${process.pid}.tmp`;
   const text=JSON.stringify(out);mkdirSync(directory,{recursive:true});writeFileSync(temporary,text);try{(input.rename??renameSync)(temporary,file);}catch{try{writeFileSync(file,text);}catch{}try{rmSync(temporary,{force:true});}catch{}}
  }catch{}
 };
 return Object.freeze({
  update(patch){
   const changed=patch.state!==undefined&&patch.state!==doc.state||patch.stage!==undefined&&patch.stage!==doc.stage||patch.ready!==undefined&&patch.ready!==doc.ready;
   doc={...doc,...patch};
   if(changed||now()-written>=P.statusIntervalMilliseconds){if(timer!==undefined)clearTimeout(timer);write();}
   else if(timer===undefined){timer=setTimeout(write,P.statusIntervalMilliseconds);timer.unref?.();}
  },
  current:()=>doc,
 });
}

export type ReadyRunResultV1=Readonly<{containment?:ReadyContainmentResultV1;signal?:string|null;stepOverdue?:Readonly<ReadyStepV1&{limit:number;quiet:number}>;holderSilent?:Readonly<{pid:number;command:string;seconds:number}>;exit:number|null;timedOut:boolean;spawnFailed:boolean;cancelled:boolean;stalled:boolean;codeChanged?:string;codeStale?:boolean;detached:boolean;startedMs:number;peakMegabytes:number|null;tail:readonly string[];failure?:readonly string[]}>;
export type ReadyPortsV1=Readonly<{
 server:boolean;stages?:readonly ReadyStageV1[];now():number;sleep(milliseconds:number,signal:AbortSignal):Promise<void>;jitter:number;cores:number;
 digests():Promise<Readonly<{digest:string;stages:readonly Readonly<{name:string;digest:string}>[]}>>;
 state:Readonly<{read():ReadyLocalStateV1;write(state:ReadyLocalStateV1):void}>;
 marker:Readonly<{exists():boolean;write():void}>;repoPresent():boolean;human():boolean;
 lease:ReturnType<typeof createReadyLeaseV1>;inflight?():string|undefined;
 memory():Readonly<{availableMegabytes:number|null;ok:boolean}>;disk?(stage:string):ReadyDiskShortV1|null;diskUnknown?(stage:string):readonly ReadyDiskUnknownV1[];recover?(request:Readonly<{stage:string;short:ReadyDiskShortV1;signal:AbortSignal;report(line:string):void}>):Promise<ReadyDiskRecoveryV1>;freeMegabytes?():number|null;compiler?():Readonly<{en:string;de:string}>|null;
 run(stage:ReadyStageV1,options:Readonly<{signal:AbortSignal;jobs:number;onLine(line:string):void;mirror?(stage:string,lines:readonly string[]):void}>):Promise<ReadyRunResultV1>;
 status:Readonly<{update(patch:Partial<ReadyStatusV1>):void}>;
 mirror?(stage:string,lines:readonly string[]):void;timing?(run:ReadyTimingV1):void;
 say(line:string):void;busy?(busy:boolean):void;restartPending?():boolean;
}>;

/** 🔁️ The readiness loop that does not die: it runs the loop (`runReadyLoopV1`) and, when it throws (a store that cannot be written, a port that fails), says so in `ready.json` (refusal `loop-failed`, state `backoff`), waits the back-off of the failures in a row (they start again from one after a run that lasted `loopStableMilliseconds`) and starts the loop again, for as long as the signal does not abort. The state of the stages is on disk, so nothing is lost by the restart.
 * @see ./🧪️tests/🔁️resume/🟦️.ts */
export async function runReadyV1(ports:ReadyPortsV1,signal:AbortSignal):Promise<void>{
 let failures=0;
 while(!signal.aborted){
  const started=ports.now();
  try{await runReadyLoopV1(ports,signal);return;}
  catch(error){
   if(signal.aborted)return;
   failures=ports.now()-started>=P.loopStableMilliseconds?1:failures+1;
   const pause=readyBackoffV1(failures,ports.jitter),message=String((error as Error|undefined)?.message??error).replace(/\s+/gu," ").slice(0,P.tailCharacters),seconds=Math.round(pause/1000);
   try{ports.status.update({state:"backoff",ready:false,refusal:readyRefusalV1("loop-failed",{message,seconds,count:failures}),nextAttemptMs:ports.now()+pause});}catch{}
   try{ports.say(`readiness loop failed (${message}), it starts again in ${seconds} s (try ${failures} in a row) / Bereitschaftsschleife fehlgeschlagen (${message}), sie startet in ${seconds} s neu (Versuch ${failures} in Folge)`);}catch{}
   await ports.sleep(pause,signal);
  }
 }
}

/** 🔁️ The loop of one worker, turn by turn: reads the digest, decides (`readyDecideV1`), runs the first stale stage under the lease with cancellation (the signal stops the child and records nothing as failed), records the result in the local state and in `ready.json`, and writes the marker `.cache/f/native-bootstrap.done` after the last stage. A machine whose marker and `repo` binary exist but that never ran a stage here adopts the marker as the current digest, so a machine a person made ready is not set up again. The group server returns at once and writes nothing. */
async function runReadyLoopV1(ports:ReadyPortsV1,signal:AbortSignal):Promise<void>{
 if(ports.server)return;
 const plan=ports.stages??READY_STAGES_V1,names=plan.map(stage=>stage.name);
 const stalls=new Map<string,number>(),warnedUnmeasured=new Set<string>();
 let renew:ReturnType<typeof setInterval>|undefined,stalledStage:string|undefined,stalledReason:ReadyReasonCodeV1="stalled",lastContainment:ReadyContainmentV1|null=null;
 const leave=():void=>{if(renew!==undefined)clearInterval(renew);renew=undefined;ports.lease.release();};
 try{
  while(!signal.aborted){
   const {digest,stages}=await ports.digests(),stored=ports.state.read(),marker=ports.marker.exists(),repo=ports.repoPresent(),now=ports.now();
   let state=stored,done:Record<string,string>=Object.fromEntries(Object.entries(stored.stages).map(([name,row])=>[name,row.digest]));
   if(Object.keys(done).length===0&&marker&&repo){
    state={...stored,adopted:true,stages:Object.fromEntries(stages.map(stage=>[stage.name,{digest:stage.digest,doneMs:now}])),readyMs:now};
    ports.state.write(state);done=Object.fromEntries(stages.map(stage=>[stage.name,stage.digest]));
   }
   if(done.dependencies!==undefined){const wrong=ports.compiler?.()??null;if(wrong!==null){const at=ports.now(),attempts=state.attempts+1,{dependencies:_dropped,...rest}=state.stages;state={...state,stages:rest,failure:{digest,stage:"dependencies",attempts,exit:null,code:"tooling-compiler",nextAttemptMs:at+readyBackoffV1(attempts,ports.jitter),atMs:at,tail:[wrong.en,wrong.de]},attempts};ports.state.write(state);delete done.dependencies;}}
   const memory=ports.memory(),lease=ports.lease.peek();
   const decision=readyDecideV1({server:false,digest,stages,done,marker,repo,human:ports.human(),lease,memoryOk:memory.ok,failure:state.failure===null?null:{digest:state.failure.digest,nextAttemptMs:state.failure.nextAttemptMs},now,restartPending:ports.restartPending?.()});
   const base:Partial<ReadyStatusV1>={digest,attempts:state.attempts,peakMegabytes:state.peakMegabytes,readyMs:state.readyMs,stageCount:names.length,exit:state.failure?.exit??null,tail:[...state.failure?.tail??[]],containment:lastContainment};
   const full=state.failure===null?null:readyStoreFullV1(state.failure.tail),failure=state.failure!==null&&state.failure.digest===digest?readyRefusalV1(state.failure.code,{stage:state.failure.stage,ending:readyEndingV1(state.failure.exit,state.failure.signal),path:full?.path??"-",errno:full?.errno??"-",count:READY_TREE_POLICY_V1.stallRestartMaximum}):null;
   const stage=decision.stageIndex>=0?names[decision.stageIndex]!:null;
   if(decision.action==="idle"){leave();lastContainment=null;ports.status.update({...base,state:"ready",ready:true,stage:null,stageIndex:0,refusal:null,reason:null,containment:null,nextAttemptMs:null,exit:null,tail:[]});}
   else if(decision.action==="mark"){ports.marker.write();continue;}
   else if(decision.action==="observe"||decision.action==="wait"){
    leave();
    const refusal=decision.code===null?failure:readyRefusalV1(decision.code,{stage:stage??"-",available:memory.availableMegabytes??"?",needed:P.memoryFloorMegabytes});
    ports.status.update({...base,state:decision.action==="observe"?"observing":"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal,nextAttemptMs:null});
   }
   else if(decision.action==="backoff"){
    leave();
    ports.status.update({...base,state:"backoff",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:failure,nextAttemptMs:state.failure!.nextAttemptMs});
   }
   else if(decision.action==="run"){
    const diskStage=plan[decision.stageIndex]!.name;
    let short=ports.inflight?.()===diskStage?null:ports.disk?.(diskStage)??null,remaining:readonly ReadyDiskConsumerV1[]|undefined;
    if(ports.inflight?.()!==diskStage)for(const row of ports.diskUnknown?.(diskStage)??[]){const key=`${diskStage}\0${row.path}\0${row.reason}`;if(!warnedUnmeasured.has(key)){warnedUnmeasured.add(key);const warning=readyDiskUnmeasuredV1(diskStage,row);ports.say(`${warning.en} / ${warning.de}`);}}
    if(short!==null&&short.resource==="space"&&ports.recover!==undefined){
     leave();
     ports.status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyDiskCollectingV1(diskStage,short),nextAttemptMs:null});
     try{
      const outcome=await ports.recover({stage:diskStage,short,signal,report:line=>ports.say(line)});
      if(signal.aborted)break;
      if(outcome.fits)short=ports.disk?.(diskStage)??null;else remaining=outcome.consumers;
     }catch(error){
      if(signal.aborted)break;
      remaining=[];ports.say(`collecting space for stage ${diskStage} failed (${String((error as Error)?.message??error).slice(0,200)}) / Platz für Stufe ${diskStage} sammeln fehlgeschlagen`);
     }
    }
    if(short!==null){
     leave();
     const refusal=readyDiskRefusalV1(diskStage,short,remaining);
     ports.status.update({...base,state:"waiting",ready:false,stage,stageIndex:decision.stageIndex+1,refusal,nextAttemptMs:ports.now()+P.memoryPollMilliseconds});
     ports.say(`${refusal.en} / ${refusal.de}`);
     await ports.sleep(P.memoryPollMilliseconds,signal);
     continue;
    }
    if(ports.lease.acquire()==="other"){ports.status.update({...base,state:"observing",ready:false,stage,stageIndex:decision.stageIndex+1,refusal:readyRefusalV1("lease-held",{holder:ports.lease.holder?.()?.pid??"?"}),nextAttemptMs:null});await ports.sleep(P.observePollMilliseconds,signal);continue;}
    renew??=setInterval(()=>{ports.lease.renew();},P.leaseRenewMilliseconds);renew.unref?.();
    const row=plan[decision.stageIndex]!,jobs=readyJobsV1({cores:ports.cores,availableMegabytes:memory.availableMegabytes});
    const reason=ports.inflight?.()!==undefined?readyReasonV1("adopted",ports.inflight()!):stalledStage===row.name?readyReasonV1(stalledReason,row.name):state.failure!==null&&state.failure.stage===row.name?readyReasonV1("previous-failed",row.name):!repo?readyReasonV1("binary-missing",row.name):stored.stages[row.name]!==undefined?readyReasonV1("inputs-changed",row.name):readyReasonV1("first-run",row.name);
    ports.status.update({...base,state:"running",ready:false,stage:row.name,stageIndex:decision.stageIndex+1,refusal:null,reason,containment:null,nextAttemptMs:null});
    ports.say(`stage ${row.name} starts with ${jobs} job(s) / Stufe ${row.name} startet mit ${jobs} Job(s)`);
    ports.busy?.(true);stalledStage=undefined;
    const freeBefore=ports.inflight?.()!==undefined?null:ports.freeMegabytes?.()??null;
    let result:ReadyRunResultV1;
    try{result=await ports.run(row,{signal,jobs,onLine:line=>{ports.status.update({tail:[line.slice(0,P.tailCharacters)]});},...ports.mirror===undefined?{}:{mirror:(stage,lines)=>ports.mirror!(stage,lines)}});}finally{ports.busy?.(false);}
    if(!result.detached&&!signal.aborted)lastContainment=result.containment===undefined?null:readyContainmentStatusV1({stage:row.name,...result.containment});
    const peak=Math.max(state.peakMegabytes??0,result.peakMegabytes??0)||state.peakMegabytes;
    const wrong=row.name==="dependencies"&&result.exit===0&&!result.timedOut&&!result.spawnFailed?ports.compiler?.()??null:null;
    const stalledOut=result.stalled&&!result.timedOut&&!result.cancelled&&!signal.aborted,outcome=result.exit===0&&!result.timedOut&&!result.spawnFailed&&!(row.name==="native"&&!ports.repoPresent())&&wrong===null?"ok":stalledOut?"stalled":result.timedOut?"stage-timeout":result.spawnFailed?"spawn-failed":result.cancelled?"interrupted":"stage-failed";
    if(!result.detached&&!signal.aborted)ports.timing?.({stage:row.name,startedMs:result.startedMs,endedMs:ports.now(),seconds:Math.max(0,Math.round((ports.now()-result.startedMs)/1000)),exit:result.exit,outcome,peakMegabytes:result.peakMegabytes,growthMegabytes:freeBefore===null?null:((free:number|null)=>free===null?null:Math.max(0,freeBefore-free))(ports.freeMegabytes?.()??null),attempts:state.attempts+(outcome==="ok"?0:1),digest:stages[decision.stageIndex]!.digest.slice(0,12)});
    if(result.detached||signal.aborted){ports.state.write({...state,peakMegabytes:peak});break;}
    if(!stalledOut)stalls.delete(row.name);
    const stallCount=stalledOut&&result.codeChanged===undefined?(stalls.get(row.name)??0)+1:0,stallVerdict=stallCount>0?readyStallRestartV1({consecutive:stallCount}):undefined,stallExhausted=stallVerdict?.action==="fail";
    if(stalledOut&&!stallExhausted){
     const byCode=result.codeChanged!==undefined,stale=result.codeStale===true,silent=result.codeChanged===undefined?result.holderSilent:undefined,over=result.codeChanged===undefined&&silent===undefined?result.stepOverdue:undefined,minutes=Math.round((stale?READY_TREE_POLICY_V1.staleStageMilliseconds:byCode?READY_TREE_POLICY_V1.holderWaitRestartMilliseconds:READY_TREE_POLICY_V1.stallStopMilliseconds)/60000),pause=stallVerdict?.waitMilliseconds??0;
     if(stallCount>0)stalls.set(row.name,stallCount);
     stalledStage=row.name;stalledReason=stale?"code-stale":byCode?"code-changed":silent!==undefined?"holder-silent":over!==undefined?"step-overdue":"stalled";
     ports.mirror?.(`${row.name}-stalled`,result.tail);
     if(stale)ports.say(`stage ${row.name} had run for at least ${minutes} min when its code changed (${result.codeChanged}), its process tree was stopped, it starts again at once with the new code and keeps what it finished / Stufe ${row.name} lief mindestens ${minutes} Min., als sich ihr Code änderte (${result.codeChanged}), ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem neuen Code und behält alles Fertige`);
     else if(byCode)ports.say(`stage ${row.name} waited ${minutes} min for a named holder while its code changed (${result.codeChanged}), its process tree was stopped, it starts again at once with the new code / Stufe ${row.name} wartete ${minutes} Min. auf einen benannten Halter, während sich ihr Code änderte (${result.codeChanged}), ihr Prozessbaum wurde beendet, sie startet sofort neu mit dem neuen Code`);
     else if(silent!==undefined)ports.say(`stage ${row.name} waited for process ${silent.pid} (${silent.command}) that holds the dependency store but has not renewed its registration for ${silent.seconds} s, it may be blocked; its process tree was stopped, the stage starts again in ${Math.round(pause/1000)} s / Stufe ${row.name} wartete auf Prozess ${silent.pid} (${silent.command}), der den Abhängigkeitsspeicher hält, aber seine Anmeldung seit ${silent.seconds} s nicht erneuert hat und womöglich blockiert ist; ihr Prozessbaum wurde beendet, sie startet in ${Math.round(pause/1000)} s neu`);
     else if(over!==undefined)ports.say(`stage ${row.name} ran the step "${over.text}" for ${over.seconds} s although it declares ${over.bound} s (overdue on this machine after ${over.limit} s) and nothing else moved for ${over.quiet} s; its process tree was stopped, the stage starts again with the current code in ${Math.round(pause/1000)} s (stop ${stallCount} of ${READY_TREE_POLICY_V1.stallRestartMaximum}) / Stufe ${row.name} führte den Schritt "${over.text}" seit ${over.seconds} s aus, obwohl er ${over.bound} s erklärt (auf dieser Maschine überfällig nach ${over.limit} s), und nichts anderes bewegte sich seit ${over.quiet} s; ihr Prozessbaum wurde beendet, die Stufe startet in ${Math.round(pause/1000)} s mit dem aktuellen Code neu (Stopp ${stallCount} von ${READY_TREE_POLICY_V1.stallRestartMaximum})`);
     else ports.say(`stage ${row.name} showed nothing new for ${minutes} min, its process tree was stopped, it starts again in ${Math.round(pause/1000)} s (stop ${stallCount} of ${READY_TREE_POLICY_V1.stallRestartMaximum}) / Stufe ${row.name} zeigte ${minutes} Min. nichts Neues, ihr Prozessbaum wurde beendet, sie startet in ${Math.round(pause/1000)} s neu (Stopp ${stallCount} von ${READY_TREE_POLICY_V1.stallRestartMaximum})`);
     ports.status.update({...base,state:"waiting",ready:false,stage:row.name,stageIndex:decision.stageIndex+1,refusal:stale?readyRefusalV1("stage-code-stale",{stage:row.name,minutes,file:result.codeChanged!}):byCode?readyRefusalV1("stage-code-changed",{stage:row.name,minutes,file:result.codeChanged!}):silent!==undefined?readyRefusalV1("stage-holder-silent",{stage:row.name,minutes,pid:silent.pid,command:silent.command,silent:silent.seconds,seconds:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}):over!==undefined?readyRefusalV1("stage-step-overdue",{stage:row.name,step:over.text,seconds:over.seconds,bound:over.bound,limit:over.limit,quiet:over.quiet,pause:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}):readyRefusalV1("stage-stalled",{stage:row.name,minutes,seconds:Math.round(pause/1000),count:stallCount,maximum:READY_TREE_POLICY_V1.stallRestartMaximum}),containment:lastContainment,nextAttemptMs:pause>0?ports.now()+pause:null});
     leave();
     if(pause>0)await ports.sleep(pause,signal);
     continue;
    }
    if(stallExhausted)stalls.delete(row.name);
    const binaryMissing=row.name==="native"&&result.exit===0&&!ports.repoPresent();
    if(result.exit===0&&!result.timedOut&&!result.spawnFailed&&!binaryMissing&&wrong===null){
     const written:ReadyLocalStateV1={...state,stages:{...state.stages,[row.name]:{digest:stages[decision.stageIndex]!.digest,doneMs:ports.now()}},failure:null,attempts:0,peakMegabytes:peak};
     if(decision.stageIndex===names.length-1){ports.marker.write();written.readyMs=ports.now();}
     ports.state.write(written);
     continue;
    }
    const failedTail=wrong!==null?[wrong.en,wrong.de]:readyTailWithDiskCauseV1(result.tail,result.tail.some(line=>line.includes(P.sqliteOpenPattern))?ports.disk?.(row.name)??null:null);
    const attempts=state.attempts+1,code:ReadyCodeV1=wrong!==null?"tooling-compiler":stallExhausted?"stage-stall-exhausted":result.timedOut?"stage-timeout":result.spawnFailed?"spawn-failed":result.cancelled?"interrupted":readyStoreFullV1(failedTail)!==null?"store-full":"stage-failed",at=ports.now();
    const next:ReadyFailureV1={digest,stage:row.name,attempts,exit:result.exit,signal:result.signal??null,code,nextAttemptMs:at+readyBackoffV1(result.exit===P.retryExit?1:attempts,ports.jitter),atMs:at,tail:failedTail.slice(-P.tailLines)};
    ports.state.write({...state,failure:next,attempts,peakMegabytes:peak});
    ports.mirror?.(`${row.name}-failed`,wrong!==null?next.tail:result.failure??next.tail);
    ports.say(`stage ${row.name} failed (${code}, ${readyEndingV1(result.exit,result.signal).en}), next attempt in ${Math.round((next.nextAttemptMs-at)/1000)} s / Stufe ${row.name} fehlgeschlagen (${readyEndingV1(result.exit,result.signal).de}), nächster Versuch in ${Math.round((next.nextAttemptMs-at)/1000)} s`);
    leave();
    continue;
   }
   else leave();
   await ports.sleep(Math.max(250,Math.min(decision.waitMilliseconds,P.idlePollMilliseconds)),signal);
  }
 }finally{leave();}
}

const isServer=(origin:string,group:string|undefined):boolean=>group!==undefined&&group!==""&&origin===`${group}${P.serverSuffix}`;

/** 🏷️ Whether this origin is the group server of its group, which is never set up by this service. */
export function readyIsServerV1(origin:string,group:string|undefined):boolean{return isServer(origin,group);}

function stagePathV1(environment:Readonly<Record<string,string|undefined>>,root:string,home:string):NodeJS.ProcessEnv{
 const windows=process.platform==="win32",entries=[...P.pathDirectories.map(entry=>join(home,...entry.split("/"))),...windows?P.systemPath.windows:P.systemPath.unix,...(environment.PATH??"").split(delimiter)].filter(Boolean);
 const env:NodeJS.ProcessEnv={...environment,PATH:[...new Set(entries)].join(delimiter),HOME:environment.HOME??home,SEMIO_WORKSPACE_ROOT:root,[P.languageEnvironment]:environment[P.languageEnvironment]??"en"};
 for(const name of P.droppedEnvironment)delete env[name];
 if(!windows&&env.LC_ALL===undefined&&(env.LANG===undefined||!/utf-?8/iu.test(env.LANG)))env.LANG=P.locale[readyPlatformV1() as "macos"|"linux"];
 return env;
}

/** 🔎️ The `repo` binary of the account: the first of its tool directory and the launcher directory that holds it (on Windows the launcher lives in `%LOCALAPPDATA%\\semio\\launcher\\bin` and is linked into `~/.local/bin` only later), `undefined` when none does. */
export function readyRepoBinaryV1(home:string,environment:Readonly<Record<string,string|undefined>>=process.env,platform:NodeJS.Platform=process.platform):string|undefined{
 const windows=platform==="win32",name=windows?P.repoNames.windows:P.repoNames.unix;
 const directories=[...(windows?P.repoDirectories.windows:P.repoDirectories.unix).map(entry=>join(home,...entry.split("/"))),...windows&&environment[P.launcherEnvironment]!==undefined?[join(environment[P.launcherEnvironment]!,"semio","launcher","bin")]:[]];
 return directories.map(directory=>join(directory,name)).find(path=>existsSync(path));
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return(error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🧾️ The record of the stage in flight, `<worktree>/.cache/f/ready/running.json`: which stage, which process group, when it started, where its exit file and log are. */
export type ReadyInflightV1=Readonly<{stage:string;pid:number;startedMs:number;exitFile:string;log:string;supervisor?:number}>;

function readInflightV1(file:string):ReadyInflightV1|undefined{try{return JSON.parse(readFileSync(file,"utf8")) as ReadyInflightV1;}catch{return undefined;}}

/** 💾️ Writes a file in the store so a reader never sees half of it: a temporary file renamed over the target; where the rename is refused (Windows with an open reader, a network share) the text is written to the target directly and the temporary file is removed, so a mirror never goes stale and no `.tmp` is left. */
export function readyWriteStoreFileV1(file:string,text:string,rename:(from:string,to:string)=>void=renameSync):void{
 const temporary=`${file}.${process.pid}.tmp`;
 writeFileSync(temporary,text);
 try{rename(temporary,file);}catch{try{writeFileSync(file,text);}catch{}try{rmSync(temporary,{force:true});}catch{}}
}

/** 🧾️ The lines of a refusal and of the plan guard are kept long (`refusalLineCharacters`) in the log tails and mirrors: the number that names the memory and the cause stands at their end. */
const LONG_LINE_V1=/^(The project graph could not be read|Der Projektgraph konnte|graph: plan (stop|stage)|Graph: Plan)/u;

/** 📜️ The whole lines a stage wrote to its log since `offset` (at most the last four mebibytes of it; a half line stays for the next call), with the offset after them: the supervisor sees every line once, however much the stage prints between two looks, so a marker such as the first line of the tasks is never lost to the tail window. */
function logSinceV1(log:string,offset:number):Readonly<{lines:string[];offset:number}>{
 try{
  const size=statSync(log).size,from=size<offset?0:Math.max(offset,size-4194304);
  if(size<=from)return{lines:[],offset:Math.min(offset,size)};
  const buffer=Buffer.alloc(size-from),handle=openSync(log,"r");
  try{readSync(handle,buffer,0,buffer.length,from);}finally{closeSync(handle);}
  const end=buffer.lastIndexOf(10);
  if(end<0)return{lines:[],offset:from};
  return{lines:buffer.subarray(0,end).toString("utf8").split(/\r?\n/u),offset:from+end+1};
 }catch{return{lines:[],offset};}
}

/** 🧷️ The lines that tell why a stage failed: the last `failureWindowBytes` of its log without the machine rows and the heartbeat sentences of `failureNoise` (a flood of them after the failure text, or before it, is no cover), the last `count` of them cut to `width`; a log that has nothing else keeps its own last lines. */
function failureLinesV1(log:string,count:number,width:number):string[]{
 try{
  const size=statSync(log).size,start=Math.max(0,size-P.failureWindowBytes),buffer=Buffer.alloc(size-start),handle=openSync(log,"r");
  try{readSync(handle,buffer,0,buffer.length,start);}finally{closeSync(handle);}
  const all=buffer.toString("utf8").split(/\r?\n/u).filter(line=>line!==""),noise=P.failureNoise.map(pattern=>new RegExp(pattern,"u")),quiet=all.filter(line=>!noise.some(expression=>expression.test(line)));
  return(quiet.length>0?quiet:all).slice(-count).map(line=>LONG_LINE_V1.test(line)?line.slice(0,P.refusalLineCharacters):line.slice(0,width));
 }catch{return[];}
}

/** 🪦️ The line that says a stage process vanished: no exit file, no signal, so its wrapper was ended from outside the supervisor; when it was last seen alive and whether this supervisor had sent a stop signal itself. */
const vanishedLineV1=(pid:number,lastSeenSeconds:number,stopped:boolean):string=>`the stage process ${pid} is gone: it left no exit file and no signal was recorded, so its wrapper was ended without a chance to write the end (a SIGKILL from a person or a program, the system under memory pressure, or a restart of the machine); it was last seen alive ${lastSeenSeconds} s ago and this supervisor ${stopped?"had sent a stop signal to its tree":"sent no signal"} / Der Stufenprozess ${pid} ist weg: er hinterließ keine Exit-Datei und es wurde kein Signal festgehalten, sein Wrapper wurde also beendet, ohne das Ende schreiben zu können (ein SIGKILL einer Person oder eines Programms, das System unter Speicherdruck oder ein Neustart der Maschine); zuletzt lebend gesehen vor ${lastSeenSeconds} s, und dieser Aufseher ${stopped?"hatte ein Stoppsignal an seinen Baum gesandt":"sandte kein Signal"}`;

/** 🪦️ The line that says the process table gave the stage pid another start time twice in a row: the recorded and the seen start, the tolerance and how long the table read took, so a slow table can be told from a reused pid. */
const changedLineV1=(pid:number,recordedMs:number,seenMs:number,firstMs:number,toleranceMs:number,tableMs:number):string=>`process ${pid} stands in the process table with a start time ${Math.round(Math.abs(seenMs-recordedMs)/1000)} s away from the recorded one (recorded ${new Date(recordedMs).toISOString()}, seen ${new Date(seenMs).toISOString()} and before ${new Date(firstMs).toISOString()}, tolerance ${Math.round(toleranceMs/1000)} s, the last table read took ${Math.round(tableMs/1000)} s) in two reads in a row, so it is judged another process that reused the pid and the stage is ended / Prozess ${pid} steht mit einer um ${Math.round(Math.abs(seenMs-recordedMs)/1000)} s abweichenden Startzeit in der Prozesstabelle (festgehalten ${new Date(recordedMs).toISOString()}, gesehen ${new Date(seenMs).toISOString()} und zuvor ${new Date(firstMs).toISOString()}, Toleranz ${Math.round(toleranceMs/1000)} s, das letzte Lesen der Tabelle dauerte ${Math.round(tableMs/1000)} s) in zwei Lesungen hintereinander, er gilt daher als anderer Prozess mit wiederverwendeter pid und die Stufe wird beendet`;

/** 🪦️ The line that names the signal that ended a stage process (the wrapper records it; a bare exit status cannot tell a kill from a failure). */
const exitSignalLineV1=(signal:string):string=>`the stage process was ended by signal ${signal} / Der Stufenprozess wurde durch das Signal ${signal} beendet`;

function logTailV1(log:string,count:number=P.tailLines,width:number=P.tailCharacters):string[]{
 try{
  const size=statSync(log).size,start=Math.max(0,size-P.tailBytes),buffer=Buffer.alloc(size-start),handle=openSync(log,"r");
  try{readSync(handle,buffer,0,buffer.length,start);}finally{closeSync(handle);}
  return buffer.toString("utf8").split(/\r?\n/u).filter(line=>line!=="").slice(-count).map(line=>LONG_LINE_V1.test(line)?line.slice(0,P.refusalLineCharacters):line.slice(0,width));
 }catch{return[];}
}

/** 🧹️ What a stage runner needs to find the orphans of earlier tries: the lease registry folder of the user, the windows of this worker's stopped tries and the working window of a person. */
export type ReadyOrphanPortsV1=Readonly<{registryRoot:string;windows():readonly ReadyWindowV1[];human():ReadyPersonV1|null}>;

/** 🏃️ Runs one stage, or adopts the one already in flight, as a process of its own that outlives this one: a detached wrapper (`./🏃️wrapper/🟦️.ts`) starts `bash <bootstrap> setup` with `SKIP_REPO_BOOTSTRAP=1` (PowerShell with `-SkipRepoBootstrap` on Windows) or the `repo` binary of the account, with the account's tool directories on PATH, the language of the machine, the fleet variables and no cache override, `CARGO_BUILD_JOBS` and `MAKEFLAGS` bounded by `jobs`, its output in `.cache/f/ready/<stage>.log` (the file descriptor belongs to the stage, so no supervisor restart loses a line; the log of the try before stays as `<stage>.previous.log`, and a failure is mirrored with its provenance and without the machine rows, see `failureLinesV1`); the record `running.json` lets a restarted runner (the agent restarts for new code every few minutes, and a stage takes longer) wait for the same process instead of killing it. The runner polls for the exit file, the resident size of the process tree (the peak), the last output lines, the timeout and the cancel request (a file `.cache/f/ready/cancel`, which ends the whole process group: SIGTERM, SIGKILL after the grace). Aborting `signal` only stops waiting (`detached: true`), the stage goes on. */
export async function runReadyStageV1(input:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;stage:ReadyStageV1;signal:AbortSignal;jobs:number;home?:string;onLine(line:string):void;mirror?(stage:string,lines:readonly string[]):void;tree?:typeof READY_TREE_POLICY_V1;orphans?:ReadyOrphanPortsV1;containment?:typeof READY_CONTAINMENT_POLICY_V1;speed?:number}>):Promise<ReadyRunResultV1>{
 const speed=input.speed??localMachineSpeedFactorV1(input.environment),stageTimeout=scaledMillisecondsV1(input.stage.timeoutMilliseconds,speed);
 const windows=process.platform==="win32",directory=machinePathV1(input.root,...P.directory),home=input.home??homedir(),inflightFile=join(directory,P.inflightName),cancelFile=join(directory,P.cancelName);
 const failed=(line:string,spawnFailed:boolean):ReadyRunResultV1=>({exit:null,timedOut:false,spawnFailed,cancelled:false,stalled:false,detached:false,startedMs:Date.now(),peakMegabytes:null,tail:[line]});
 mkdirSync(directory,{recursive:true});
 const contain=input.containment??READY_CONTAINMENT_POLICY_V1,lines:string[]=[],say=(line:string):void=>{lines.push(line);input.onLine(line);},earlier={reaped:[] as ReadyReapedV1[],survivors:[] as ReadyKnownV1[]};
 const ownerOf=(stage:string,startedMs:number):string=>readyOwnerTokenV1(input.root,stage,startedMs);
 const spare=(table:readonly ReadyProcessV1[]):ReadonlySet<number>=>input.orphans===undefined?new Set<number>():readySharedPidsV1(readyRegistryEntriesV1(input.orphans.registryRoot,contain),table,contain);
 const sweepRosters=async():Promise<void>=>{const swept=await readySweepRostersV1({directory,say,policy:contain,owner:ownerOf,spare});earlier.reaped.push(...swept.reaped);earlier.survivors.push(...swept.survivors);};
 const reapRegistry=async(stage:Readonly<{root:ReadyKnownV1;known:readonly ReadyKnownV1[]}>|null,kind:"registry"|"holder"):Promise<ReadyContainmentResultV1>=>input.orphans===undefined?{reaped:[],survivors:[]}:readyReapOrphansV1({registryRoot:input.orphans.registryRoot,root:input.root,windows:input.orphans.windows(),human:input.orphans.human(),stage,stageName:input.stage.name,kind,directory,spare,say,policy:contain,explain:lines=>input.mirror?.(`${input.stage.name}-containment`,lines)});
 let record=readInflightV1(inflightFile);
 if(record!==undefined&&!alive(record.pid)&&!existsSync(record.exitFile)){rmSync(inflightFile,{force:true});await sweepRosters();if(record.stage===input.stage.name)return{...failed("the process of the stage vanished without an exit status / der Prozess der Stufe verschwand ohne Exit-Status",false),tail:logTailV1(record.log),containment:{reaped:earlier.reaped,survivors:earlier.survivors}};record=undefined;}
 if(record===undefined){await sweepRosters();const swept=await reapRegistry(null,"registry");earlier.reaped.push(...swept.reaped);earlier.survivors.push(...swept.survivors);}
 const wait=async(inflight:ReadyInflightV1):Promise<ReadyRunResultV1>=>new Promise<ReadyRunResultV1>(finish=>{
  let peak=0,timedOut=false,cancelled=false,stalled=false,tick=0,last="",killed=false,known:readonly ReadyKnownV1[]=readyReadTreeV1(directory,inflight.stage)?.processes??[],sampling=false,progressKey:string|null=null,progressAt=Date.now(),stallAt=Date.now(),codeFile:string|undefined,codeStale=false,codeCheckedAt=0,phase:ReadyPhaseV1=null,phaseAt=inflight.startedMs,phaseOffset=0,finishing=false,reaping=false,reapedAt=0,nextSample=0,sampledAt=Date.now();
  const treePolicy=input.tree??readyScaledTreePolicyV1(speed),semanticsAtStart=readySemanticsV1(input.root,treePolicy),tracker=readyStallTrackerV1(treePolicy),stepWatch=readyStepWatchV1(treePolicy),holder=readyHolderWaitTrackerV1(treePolicy),holderLines=treePolicy.holderWaitPatterns.map(pattern=>new RegExp(pattern,"u")),left=new Map<number,ReadyKnownV1>(earlier.survivors.map(row=>[row.pid,row])),reaped:ReadyReapedV1[]=[...earlier.reaped];
  const rootOf=():ReadyKnownV1=>({pid:inflight.pid,startMs:inflight.startedMs});
  let overdueStep:Readonly<ReadyStepV1&{limit:number;quiet:number}>|undefined,aliveAt=Date.now(),tableMilliseconds=0,confirming=false,killReason:ReadyKillReasonV1="stall",cpuBefore:ReadonlyMap<number,number>=new Map(),inspecting=false,inspectedAt=0,silentHolder:Readonly<{pid:number;command:string;seconds:number}>|undefined;const yielded=new Set<number>();
  const recorder=(reason:ReadyKillReasonV1)=>readyKillRecorderV1({directory,stage:inflight.stage,owner:ownerOf(inflight.stage,inflight.startedMs),reason,seconds:Math.round(treePolicy.stallStopMilliseconds/1000),say,policy:contain});
  const stopTree=async(signal:"SIGTERM"|"SIGKILL"):Promise<void>=>{const table=await readyReadTableV1(),spared=spare(table),plan=await readyKillTreeV1({root:rootOf(),known,signal,...table.length===0?{}:{table,spare:spared},record:recorder(killReason)});if(process.platform!=="win32"&&plan.includes(inflight.pid)&&(spared.size===0||readyKillPlanV1({table,root:rootOf(),known,self:process.pid}).length===plan.length))try{process.kill(-inflight.pid,signal);}catch{}};
  const kill=(reason:ReadyKillReasonV1):void=>{if(killed)return;killed=true;killReason=reason;void stopTree("SIGTERM");setTimeout(()=>{void stopTree("SIGKILL");},P.graceMilliseconds).unref();};
  const progressEvery=Math.max(1,Math.round(P.progressMilliseconds/P.pollMilliseconds)),sampleEvery=Math.max(1,Math.round(P.samplerMilliseconds/P.pollMilliseconds)),mirrorEvery=Math.max(1,Math.round(P.mirrorMilliseconds/P.pollMilliseconds));
  const settle=async():Promise<void>=>{
   const result=await readyStopV1({root:rootOf(),known,policy:contain,grace:contain.terminateGraceMilliseconds,spare,record:recorder("settle"),...windows?{}:{group:inflight.pid}});
   for(const row of result.survivors)left.set(row.pid,row);
   if(result.survivors.length===0){if(result.verified)readyForgetTreeV1(directory,inflight.stage);return;}
   readyWriteTreeV1(directory,inflight.stage,rootOf(),result.survivors,Date.now());
   for(const row of result.survivors)say(readyContainmentLineV1("survivor",{pid:row.pid,name:row.name??"?",passes:result.passes}));
  };
  const readRecord=():ReadyReadRecordV1|undefined=>{try{const value=JSON.parse(readFileSync(machinePathV1(input.root,...treePolicy.readRecord),"utf8")) as Partial<ReadyReadRecordV1>;return Number.isSafeInteger(value.runner)&&Number.isFinite(value.startedMs)&&(value.reason==="request"||value.reason==="refresh")?{reason:value.reason,runner:value.runner!,read:value.read??null,startedMs:value.startedMs!,status:value.status??null,endedMs:value.endedMs??null}:undefined;}catch{return undefined;}};
  const inspect=async(named:ReadyHolderWaitV1,waiting:number):Promise<void>=>{
   const table=await readyReadTableV1(),entries=input.orphans===undefined?[]:readyRegistryEntriesV1(input.orphans.registryRoot,contain),entry=entries.filter(row=>row.pid===named.pid).sort((left,right)=>right.startedAtMs-left.startedAtMs)[0],verdict=readyHolderCustodyV1({wait:named,entry,table,nowMs:Date.now(),policy:treePolicy});
   if(killed)return;
   silentHolder=verdict.kind==="silent"?{pid:named.pid,command:named.command,seconds:Math.round(verdict.silentMs/1000)}:undefined;
   if(verdict.kind==="progressing"||verdict.kind==="gone"){stallAt=Date.now();progressAt=Date.now();}
   if(verdict.kind!=="progressing"&&verdict.kind!=="silent"||waiting<treePolicy.holderYieldMilliseconds||yielded.has(named.pid))return;
   const reader=readyBackgroundReaderV1({record:readRecord(),holder:named.pid,table,self:process.pid,policy:treePolicy});
   if(reader===undefined)return;
   yielded.add(named.pid);
   const seconds=Math.round(waiting/1000),record=readyKillRecorderV1({directory,stage:inflight.stage,owner:ownerOf(inflight.stage,inflight.startedMs),reason:"yield",seconds,say,policy:contain});
   say(readyContainmentLineV1("yielded-reader",{stage:inflight.stage,seconds,pid:named.pid,name:named.command.slice(0,80),reason:reader.reason}));
   await readyKillTreeV1({root:reader.root,known:[],signal:"SIGTERM",table,spare:new Set<number>(),record});
   setTimeout(()=>{void readyKillTreeV1({root:reader.root,known:[],signal:"SIGKILL",spare:new Set<number>(),record});},P.graceMilliseconds).unref();
  };
  const done=(result:ReadyRunResultV1):void=>{clearInterval(timer);input.signal.removeEventListener("abort",detach);const full=result.detached?result:{...result,containment:{reaped:[...reaped],survivors:[...left.values()]},...silentHolder!==undefined&&result.stalled?{holderSilent:silentHolder}:{},...overdueStep!==undefined&&result.stalled?{stepOverdue:overdueStep}:{}};finish(codeFile===undefined||result.detached?full:{...full,codeChanged:codeFile,...codeStale?{codeStale:true}:{}});};
  const watchedMs=Date.now(),stamp=(milliseconds:number):string=>new Date(milliseconds).toISOString();
  const provenance=():string=>{const started=inflight.supervisor,same=started===process.pid;return same?`stage ${inflight.stage} (process ${inflight.pid}) started ${stamp(inflight.startedMs)} and watched by supervisor ${process.pid} / Stufe ${inflight.stage} (Prozess ${inflight.pid}) gestartet ${stamp(inflight.startedMs)} und von Aufseher ${process.pid} überwacht`:`stage ${inflight.stage} (process ${inflight.pid}) started ${stamp(inflight.startedMs)} by supervisor ${started??"?"}, adopted by supervisor ${process.pid} at ${stamp(watchedMs)} / Stufe ${inflight.stage} (Prozess ${inflight.pid}) gestartet ${stamp(inflight.startedMs)} von Aufseher ${started??"?"}, übernommen von Aufseher ${process.pid} um ${stamp(watchedMs)}`;};
  const ending=(signal:string|null,why?:string):Readonly<{tail:string[];failure:string[]}>=>{const note=signal!==null?[exitSignalLineV1(signal)]:why===undefined?[]:[why];return{tail:[...failureLinesV1(inflight.log,P.tailLines-note.length,P.tailCharacters),...note],failure:[provenance(),...failureLinesV1(inflight.log,P.mirrorLines-1-note.length,P.mirrorLineCharacters),...note]};};
  const detach=():void=>done({exit:null,timedOut:false,spawnFailed:false,cancelled:false,stalled:false,detached:true,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,tail:logTailV1(inflight.log)});
  const timer=setInterval(()=>{
   tick++;
   if(finishing)return;
   if(existsSync(inflight.exitFile)){
    let exit:number|null=null,spawnFailed=false,signal:string|null=null;
    try{const row=JSON.parse(readFileSync(inflight.exitFile,"utf8")) as {exit:number|null;signal?:string|null;spawnFailed:boolean};exit=row.exit;spawnFailed=row.spawnFailed;signal=row.signal??null;}catch{}
    if(exit===0)readyForgetTreeV1(directory,inflight.stage);
    input.mirror?.(inflight.stage,logTailV1(inflight.log,P.mirrorLines,P.mirrorLineCharacters));
    rmSync(inflight.exitFile,{force:true});rmSync(inflightFile,{force:true});
    return done({exit,signal,timedOut,spawnFailed,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...exit===0?{tail:logTailV1(inflight.log)}:ending(signal)});
   }
   if(!alive(inflight.pid)){rmSync(inflightFile,{force:true});finishing=true;const why=vanishedLineV1(inflight.pid,Math.round((Date.now()-aliveAt)/1000),killed);void settle().finally(()=>done({exit:null,signal:null,timedOut,spawnFailed:false,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...ending(null,why)}));return;}
   aliveAt=Date.now();
   if(input.mirror!==undefined&&tick%mirrorEvery===0)input.mirror(inflight.stage,logTailV1(inflight.log,P.mirrorLines,P.mirrorLineCharacters));
   if(tick%progressEvery===0){const lines=logTailV1(inflight.log),line=lines.at(-1)??"",key=readyProgressKeyV1(lines);if(key!==null&&key!==progressKey){progressKey=key;progressAt=Date.now();}if(line!==""&&line!==last){last=line;input.onLine(line);}const recent=logTailV1(inflight.log,60);if(tracker.observe(recent)){stallAt=Date.now();progressAt=Date.now();}const waiting=holder.observe(recent,Date.now()),fed=logSinceV1(inflight.log,phaseOffset),seenPhase=readyStagePhaseV1(fed.lines,phase,treePolicy);phaseOffset=fed.offset;if(!killed&&!inspecting&&waiting>0&&Date.now()-inspectedAt>=treePolicy.holderCheckMilliseconds){const named=readyHolderWaitOfV1(recent,treePolicy);if(named!==undefined){inspecting=true;inspectedAt=Date.now();void inspect(named,waiting).catch(()=>undefined).finally(()=>{inspecting=false;});}}if(seenPhase!==phase){if(phase!==null||seenPhase!=="reading")phaseAt=Date.now();phase=seenPhase;}const watched=stepWatch.observe(recent,Date.now());if(!killed&&phase!=="executing"&&watched.overdue&&watched.step!==undefined){overdueStep={...watched.step,limit:readyStepOverdueSecondsV1(watched.step.bound,treePolicy),quiet:Math.round(watched.quietMs/1000)};stalled=true;kill("overdue");}if(!killed&&!reaping&&input.orphans!==undefined&&Date.now()-reapedAt>=contain.holderReapMilliseconds&&recent.some(line=>holderLines.some(expression=>expression.test(line)))){reaping=true;reapedAt=Date.now();void reapRegistry({root:rootOf(),known},"holder").then(swept=>{reaped.push(...swept.reaped);for(const row of swept.reaped)left.delete(row.pid);for(const row of swept.survivors)left.set(row.pid,row);}).finally(()=>{reaping=false;});}if(!killed&&(waiting>=treePolicy.holderWaitRestartMilliseconds||readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy))&&Date.now()-codeCheckedAt>=treePolicy.codeCheckMilliseconds){codeCheckedAt=Date.now();const holder=readyHolderRestartV1(waiting,true,treePolicy),file=readySemanticsChangedV1(input.root,semanticsAtStart,treePolicy);if(file!==null&&(holder||readyStaleStopV1(phase,Date.now()-phaseAt,treePolicy))){codeFile=file;codeStale=!holder;stalled=true;kill("restart");}}}
   if(!killed&&readyStalledV1(Date.now()-stallAt,treePolicy)){stalled=true;kill("stall");}
   if(Date.now()>=nextSample&&!sampling&&!killed){sampling=true;const tableBegan=Date.now();void readyReadTableV1().then(table=>{sampling=false;tableMilliseconds=Date.now()-tableBegan;nextSample=Date.now()+readySampleDelayV1(Date.now()-inflight.startedMs,contain);if(table.length===0||killed)return;const root=rootOf();if(readyIdentityV1(root,table)==="reused"&&!existsSync(inflight.exitFile)&&!confirming&&!finishing){confirming=true;const first=table.find(row=>row.pid===root.pid)!;void readyReadTableV1().then(second=>{confirming=false;if(finishing||killed||existsSync(inflight.exitFile)||second.length===0||readyIdentityV1(root,second)!=="reused")return;const again=second.find(row=>row.pid===root.pid)!,why=changedLineV1(root.pid,root.startMs,again.startMs,first.startMs,P.toleranceMilliseconds,tableMilliseconds);rmSync(inflightFile,{force:true});readyForgetTreeV1(directory,inflight.stage);finishing=true;void settle().finally(()=>done({exit:null,signal:null,timedOut,spawnFailed:false,cancelled,stalled,detached:false,startedMs:inflight.startedMs,peakMegabytes:peak>0?peak:null,...ending(null,why)}));});return;}known=readyTreeMembersV1(table,root,[...known,...readyHeirsV1({table,previous:[root,...known],sinceMs:sampledAt})]);sampledAt=Date.now();readyWriteTreeV1(directory,inflight.stage,root,known,sampledAt);});}
   if(!windows&&tick%sampleEvery===0){const rows=spawnSync("ps",["-A","-o","pid=,ppid=,rss=,time="],{encoding:"utf8"});if(rows.status===0){peak=Math.max(peak,readyTreeMegabytesV1(rows.stdout,inflight.pid));const cpu=readyTreeCpuV1(rows.stdout,inflight.pid,known.map(member=>member.pid));if(!killed&&readyLivenessV1(cpuBefore,cpu,treePolicy)){stallAt=Date.now();progressAt=Date.now();}cpuBefore=cpu;}}
   {const elapsed=Date.now()-inflight.startedMs;if(elapsed>stageTimeout){const verdict=readyDeadlineV1({elapsedMs:elapsed,softMs:stageTimeout,sinceProgressMs:Date.now()-progressAt,policy:treePolicy});if(verdict==="timeout"||verdict==="stalled"){timedOut=true;kill("timeout");}}}
   if(existsSync(cancelFile)){cancelled=true;rmSync(cancelFile,{force:true});kill("cancel");}
  },P.pollMilliseconds);
  input.signal.addEventListener("abort",detach,{once:true});
  if(input.signal.aborted)detach();
 });
 if(record!==undefined)return record.stage===input.stage.name?wait(record):wait(record).then(result=>result.detached||input.signal.aborted?result:runReadyStageV1(input));
 const env=stagePathV1(input.environment,input.root,home);
 const startedMs=Date.now();
 env.CARGO_BUILD_JOBS=String(input.jobs);env.MAKEFLAGS=`-j${input.jobs}`;env[contain.ownerEnvironment]=ownerOf(input.stage.name,startedMs);
 let command:string,args:string[];
 if(input.stage.kind==="bootstrap"){
  const script=join(input.root,...(windows?P.bootstrap.windows:P.bootstrap.unix));
  if(windows){command="powershell.exe";args=["-NoProfile","-ExecutionPolicy","Bypass","-File",script,...input.stage.arguments,"-SkipRepoBootstrap"];}
  else{command="bash";args=[script,...input.stage.arguments];env.SKIP_REPO_BOOTSTRAP="1";}
 }else{
  const repo=readyRepoBinaryV1(home,input.environment);
  if(repo===undefined)return Promise.resolve(failed(`no repo binary below ${home} / kein repo-Programm unter ${home}`,true));
  command=repo;args=[...input.stage.arguments];
 }
 const log=join(directory,`${input.stage.name}.log`),exitFile=join(directory,`${input.stage.name}.exit`);
 rmSync(exitFile,{force:true});rmSync(cancelFile,{force:true});
 let child:ReturnType<typeof spawn>;
 try{renameSync(log,`${log.slice(0,-4)}${P.previousLogSuffix}`);}catch{}
 const handle=openSync(log,"w");
 if(lines.length>0)writeSync(handle,`${lines.join("\n")}\n`);
 try{child=spawn(process.execPath,[join(import.meta.dir,"🏃️wrapper","🟦️.ts"),exitFile,command,...args],{cwd:input.root,env,stdio:["ignore",handle,handle],detached:true,windowsHide:true});}
 catch(error){closeSync(handle);return Promise.resolve(failed(String((error as Error).message).slice(0,P.tailCharacters),true));}
 closeSync(handle);
 child.on("error",()=>undefined);child.unref();
 if(child.pid===undefined)return Promise.resolve(failed(`${command} could not be started / ${command} konnte nicht gestartet werden`,true));
 const started:ReadyInflightV1={stage:input.stage.name,pid:child.pid,startedMs,exitFile,log,supervisor:process.pid};
 writeFileSync(inflightFile,JSON.stringify(started));
 return wait(started);
}

/** 🔌️ The real ports of a machine: the files below the worktree, the free memory by the memory gate of the graph (`fleetHostReadingV1` against `memoryFloorMegabytes`), the stage runner and `ready.json` in the store of the fleet. */
export function createReadyPortsV1(input:Readonly<{facts?(path:string):ReadyFilesystemFactsV1|null;root:string;environment:Readonly<Record<string,string|undefined>>;origin:string;group:string|undefined;home?:string;stages?:readonly ReadyStageV1[];stages?:readonly ReadyStageV1[];work?:{busy:boolean;restartPending:boolean;wake?:()=>void};say(line:string):void}>):ReadyPortsV1{
 const directory=machinePathV1(input.root,...P.directory),stateFile=join(directory,P.stateName),marker=machinePathV1(input.root,".cache","f",P.markerName),windows=process.platform==="win32";
 const status=createReadyStatusV1({store:input.environment.SEMIO_FLEET_STORE_ROOT,origin:input.origin});
 const hashOf=(path:string):string|undefined=>{try{return readyInputHashV1(path,readFileSync(join(input.root,...path.split("/")),"utf8"));}catch{return undefined;}};
 const recentRuns=():readonly ReadyTimingV1[]=>{try{return(JSON.parse(readFileSync(join(directory,P.timingsName),"utf8")) as {runs:ReadyTimingV1[]}).runs;}catch{return[];}};
 const owner=`${process.pid}:${Math.random().toString(16).slice(2)}`,jitter=parseInt(sha(input.origin).slice(0,8),16)/0x100000000;
 return{
  server:isServer(input.origin,input.group),...input.stages===undefined?{}:{stages:input.stages},now:Date.now,jitter,cores:availableParallelism(),
  sleep:(milliseconds,signal)=>new Promise<void>(done=>{if(signal.aborted)return done();const timer=setTimeout(done,milliseconds);signal.addEventListener("abort",()=>{clearTimeout(timer);done();},{once:true});}),
  digests:async()=>readyDigestsV1({platform:readyPlatformV1(),hash:hashOf,...input.stages===undefined?{}:{stages:input.stages}}),
  state:{
   read:()=>{try{const value=JSON.parse(readFileSync(stateFile,"utf8")) as ReadyLocalStateV1;return value.version===1&&typeof value.stages==="object"?value:emptyReadyStateV1();}catch{return emptyReadyStateV1();}},
   write:state=>{mkdirSync(directory,{recursive:true});const temporary=`${stateFile}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(state));renameSync(temporary,stateFile);},
  },
  marker:{exists:()=>existsSync(marker),write:()=>{mkdirSync(machinePathV1(input.root,".cache","f"),{recursive:true});writeFileSync(marker,new Date().toISOString()+"\n");}},
  repoPresent:()=>readyRepoBinaryV1(input.home??homedir(),input.environment)!==undefined,
  human:()=>readyHumanActiveV1(directory),
  lease:createReadyLeaseV1({directory,owner}),
  inflight:()=>{try{const row=JSON.parse(readFileSync(join(directory,P.inflightName),"utf8")) as {stage:string;pid:number};return alive(row.pid)?row.stage:undefined;}catch{return undefined;}},
   disk:stage=>readyDiskShortV1({stage,paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1,runs:recentRuns()}),
   diskUnknown:()=>readyDiskUnknownV1({paths:[machinePathV1(input.root,".cache"),tmpdir()],facts:input.facts??readyFilesystemFactsV1}),
   freeMegabytes:()=>{const free=[machinePathV1(input.root,".cache"),tmpdir()].map(path=>(input.facts??readyFilesystemFactsV1)(path)?.freeBytes).filter((bytes):bytes is number=>typeof bytes==="number");return free.length===0?null:Math.floor(Math.min(...free)/1048576);},
   recover:async request=>{
    const facts=input.facts??readyFilesystemFactsV1,reading=()=>{const row=facts(request.short.path);return row===null||row.freeBytes===null?undefined:{partition:request.short.path,totalBytes:row.freeBytes,freeBytes:row.freeBytes};};
    const{recoverNeedV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🟦️.ts"),{realDiskWritersV1}=await import("../../../🌡️host-capacity/🧹️disk-recovery/🔌️real/🟦️.ts");
    const writers=realDiskWritersV1({workspace:input.root,home:input.home??homedir(),tmp:tmpdir(),uid:process.getuid?.()??0,environment:input.environment,language:"both",report:request.report,free:()=>reading()?.freeBytes});
    const outcome=await recoverNeedV1({facts:reading,needBytes:request.short.needed*1048576,writers,language:"both",report:request.report,signal:request.signal});
    return{fits:outcome.verdict!=="refuse",consumers:outcome.consumers};
   },
   compiler:()=>{const seen=inspectTypescriptCompilerV1({root:input.root});return seen.ok||(seen.refusal.code==="typescript-compiler-declaration"&&seen.refusal.found===null&&!existsSync(seen.refusal.resolved))?null:{en:seen.refusal.en,de:seen.refusal.de};},
 memory:()=>{const reading=fleetHostReadingV1({environment:input.environment}),free=reading.availableMegabytes;return{availableMegabytes:free,ok:free===null||free>=P.memoryFloorMegabytes};},
  run:(stage,options)=>runReadyStageV1({root:input.root,environment:input.environment,stage,...input.home===undefined?{}:{home:input.home},orphans:{registryRoot:readyRegistryRootV1(input.environment,process.platform,input.home??homedir()),windows:()=>{try{return readyStoppedWindowsV1((JSON.parse(readFileSync(join(directory,P.timingsName),"utf8")) as {runs:ReadyTimingV1[]}).runs);}catch{return[];}},human:()=>{try{return readyHumanRecordV1(readFileSync(join(directory,P.humanName),"utf8"));}catch{return null;}}},...options}),
  status,say:input.say,
  mirror:(stage,lines)=>{
   if(input.environment.SEMIO_FLEET_STORE_ROOT===undefined||input.environment.SEMIO_FLEET_STORE_ROOT==="")return;
   try{
    let text=`${new Date().toISOString()} ${stage}\n${lines.join("\n")}\n`;
    if(text.length>P.mirrorBytes)text=text.slice(text.length-P.mirrorBytes);
    const dir=join(input.environment.SEMIO_FLEET_STORE_ROOT,"v1","origins",input.origin),file=join(dir,`${P.mirrorPrefix}${stage}.txt`);
    mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,text);
   }catch{}
   try{
    const report=machinePathV1(input.root,P.sliceReport.directory,P.sliceReport.name),dir=join(input.environment.SEMIO_FLEET_STORE_ROOT!,"v1","origins",input.origin),target=join(dir,P.sliceReport.name),bytes=readFileSync(report);
    if(bytes.length<=P.sliceReport.fileBytes)readyWriteStoreFileV1(target,bytes.toString("utf8"));
   }catch{}
  },
  timing:run=>{
   try{
    const local=join(directory,P.timingsName);
    let runs:ReadyTimingV1[]=[];
    try{runs=(JSON.parse(readFileSync(local,"utf8")) as {runs:ReadyTimingV1[]}).runs;}catch{}
    runs=[...runs,run].slice(-P.timingsKept);
    const doc={version:1,origin:input.origin,runs,updatedMs:Date.now()};
    mkdirSync(directory,{recursive:true});writeFileSync(local,JSON.stringify(doc));
    const store=input.environment.SEMIO_FLEET_STORE_ROOT;
    if(store!==undefined&&store!==""&&validateJsonSchemaSubset(schema.definitions.Timings,doc,schema).length===0){const dir=join(store,"v1","origins",input.origin),file=join(dir,P.timingsName);mkdirSync(dir,{recursive:true});readyWriteStoreFileV1(file,JSON.stringify(doc));}
   }catch{}
  },
  busy:busy=>{if(input.work===undefined)return;input.work.busy=busy;if(!busy)input.work.wake?.();},
  restartPending:()=>input.work?.restartPending===true,
 };
}

/** 🚦️ Runs the readiness loop of this machine until the signal aborts, at a low priority, with the fleet variables of the replica daemon; the group server returns at once and writes nothing. A restart for new code waits while a stage runs (`work.busy`). */
export async function runReady(context:RemoteExecutionSidecarContextV1):Promise<void>{
 try{setPriority(10);}catch{}
 const origin=remoteExecutionOriginV1(context.root,context.environment);
 let group=context.environment.SEMIO_FLEET_GROUP;
 if(group===undefined||group==="")try{group=(JSON.parse(readFileSync(machinePathV1(context.root,".cache","f","machine.json"),"utf8")) as {groupId?:string}).groupId;}catch{}
 const ports=createReadyPortsV1({root:context.root,environment:context.environment,origin,group,...context.environment.HOME===undefined?{}:{home:context.environment.HOME},...context.work===undefined?{}:{work:context.work},say:context.write});
 if(ports.server){context.write("the group server is never set up by this service / der Gruppenserver wird von diesem Dienst nie eingerichtet");return;}
 await runReadyV1(ports,context.signal);
}
