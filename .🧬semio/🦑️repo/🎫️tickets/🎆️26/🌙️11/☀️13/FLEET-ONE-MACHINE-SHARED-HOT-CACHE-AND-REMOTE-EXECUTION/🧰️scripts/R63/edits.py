#!/usr/bin/env python3
"""R63 staged edits: applies anchored hunks to scratch copies (stage/<id>.new built from stage/<id>.base). Fails if an anchor is absent or ambiguous."""
import sys,os,json
S=os.path.dirname(os.path.abspath(__file__))+"/stage"
idx={l.split()[0]:l for l in open(S+"/index.txt",encoding="utf8").read().splitlines()}
def sub(fid,old,new,count=1):
    p=S+"/"+fid+".new";t=open(p,encoding="utf8").read()
    if t.count(old)!=count:sys.exit("anchor count %d != %d in %s: %s"%(t.count(old),count,fid,old[:80]))
    open(p,"w",encoding="utf8").write(t.replace(old,new))
for f in os.listdir(S):
    if f.endswith(".new"):
        import shutil;shutil.copyfile(S+"/"+f[:-4]+".base",S+"/"+f)
PS="f01c27";PSD="b3200b";SCH="c80235";WF="df6298";CK="8d7fe5";PF="b9990c";PN="bf80a8"
# plan-stage: ledger
sub(PS,'''/** 🏊️ How many worker processes of `workerMegabytes` a plan''','''const farms=new Map();
const priorityOf=kind=>P.farmPriority[kind]??0;
const reservedMegabytes=kind=>{let total=0;for(const row of farms.values())if(priorityOf(row.kind)>=priorityOf(kind))total+=row.megabytes;return total;};

/** 🏊️ How many worker processes of `workerMegabytes` a plan''')
sub(PS,'''export function planFarmWorkersV1({workerMegabytes,residentMegabytes=process.memoryUsage().rss/MEGABYTE,environment=process.env,hostMegabytes=hostMemoryMegabytesV1(),policy=PLAN_STAGE_POLICY_V1}){''','''export function planFarmWorkersV1({workerMegabytes,residentMegabytes=process.memoryUsage().rss/MEGABYTE,environment=process.env,hostMegabytes=hostMemoryMegabytesV1(),policy=PLAN_STAGE_POLICY_V1,kind,counted=kind}){''')
sub(PS,'''Math.floor((hostMegabytes*policy.treeShare-residentMegabytes)/workerMegabytes)));
}''','''Math.floor((hostMegabytes*policy.treeShare-(kind===undefined?residentMegabytes:Math.max(residentMegabytes,policy.parentMegabytes))-(counted===undefined?0:reservedMegabytes(counted)))/(kind===undefined?workerMegabytes:Math.max(workerMegabytes,policy.farmWorkerPeakMegabytes[kind]??0)))));
}

/**
 * 🧮️ The one memory plan of the process tree: every kind of child process a plan starts (`project-farm`, `walk-farm`) takes its workers from the same `treeShare` of the physical memory, after what the process holds itself and after what the farms of the same or a higher priority (`farmPriority`) already reserved, so the sum of all farms and the parent stays within the share that the slice guard judges. A farm that does not fit asks the lower priority farms to give their children back (`yield`) before it takes less. Returns the workers granted and `release()` (idempotent), which the farm calls when its children are gone; nothing is recorded where the plan has no limit.
 * @see ./🧬️schema/🔣️.json
 */
export function reserveFarmV1({kind,workerMegabytes,wanted=Infinity,residentMegabytes,environment=process.env,hostMegabytes,policy=PLAN_STAGE_POLICY_V1,yield:onYield}){
 const cost=Math.max(workerMegabytes,policy.farmWorkerPeakMegabytes[kind]??0),roomOf=counted=>planFarmWorkersV1({workerMegabytes,environment,policy,kind,counted,...residentMegabytes===undefined?{}:{residentMegabytes},...hostMegabytes===undefined?{}:{hostMegabytes}});
 const workers=Math.min(wanted,roomOf(kind));
 if(Math.min(wanted,roomOf("*"))<workers)for(const[token,row]of[...farms])if(priorityOf(row.kind)<priorityOf(kind)){farms.delete(token);try{row.yield?.();}catch{}}
 const token=Symbol(kind);
 if(workers>0&&planStageLimitBytesV1(environment,hostMegabytes??hostMemoryMegabytesV1(),policy)!==Infinity)farms.set(token,{kind,megabytes:workers*cost,yield:onYield});
 return Object.freeze({workers,release(){farms.delete(token);}});
}

/** 📋️ What the farms of this process hold in the plan right now: one row per reservation. */
export const farmReservationsV1=()=>[...farms.values()].map(row=>({kind:row.kind,megabytes:row.megabytes}));''')
sub(PSD,'''policy?:PlanStagePolicyV1}):number;''','''policy?:PlanStagePolicyV1;kind?:string}):number;
/** 🧮️ Reserves workers of one kind of farm in the one memory plan of the process tree. */
export function reserveFarmV1(input:{kind:string;workerMegabytes:number;wanted?:number;residentMegabytes?:number;environment?:Readonly<Record<string,string|undefined>>;hostMegabytes?:number;policy?:PlanStagePolicyV1;yield?:()=>void}):Readonly<{workers:number;release():void}>;
export function farmReservationsV1():{kind:string;megabytes:number}[];''')
# schema: treeShare = the plan limit share of the slice guard, priorities
p=S+"/"+SCH+".new";t=open(p,encoding="utf8").read()
t=t.replace('''        "treeShare": 0.5,
      "treeShare": 0.5,
''','''        "treeShare": 0.4,
        "farmPriority": {
          "project-farm": 2,
          "walk-farm": 1
        },
        "parentMegabytes": 3600,
        "farmWorkerPeakMegabytes": {
          "project-farm": 2200
        },
''',1)
t=t.replace("EN: The worker processes a plan starts (the project farm) count with their processes, not with the managed bytes of the plan: together with the plan they stay within treeShare of the physical memory, below the ceiling of the slice guard.","EN: The worker processes a plan starts (the project farm, the walk farm) count with their processes, not with the managed bytes of the plan: together with the plan they stay within treeShare of the physical memory, which is the plan limit of the slice guard (hostCapBasisPoints of the host policy), below its ceiling; all farms of a process draw from this one share, a farm of a higher priority (farmPriority) asks the lower ones to give their children back. EN: A parent that plans farms counts for at least parentMegabytes (the slice worker measured 3.9 GB when the inference started, 3.1 GB when it was sized) and a project farm worker costs farmWorkerPeakMegabytes (measured: three workers took 6.3 GB around a 3.9 GB parent, budgeted 1536 MB each by the grant). DE: Alle Farmen eines Prozesses ziehen aus diesem einen Anteil, eine Farm mit höherer Priorität (farmPriority) lässt die niedrigeren ihre Kinder zurückgeben; ein Elternprozess zählt mindestens parentMegabytes und ein Projekt-Farm-Arbeiter kostet farmWorkerPeakMegabytes.")
open(p,"w",encoding="utf8").write(t)
# walk farm
sub(WF,'import{planFarmWorkersV1}from"../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";','import{planFarmWorkersV1,reserveFarmV1}from"../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";')
sub(WF,'''@param {{root:string;size:number;module?:string;spawn?:typeof nodeSpawn;policy?:typeof WALK_FARM_V1}} options
 */
export function createWalkFarmV1({root,size,module,spawn=nodeSpawn,policy=WALK_FARM_V1}){''','''@param {{root:string;size:number;module?:string;spawn?:typeof nodeSpawn;policy?:typeof WALK_FARM_V1;lease?:{release():void}}} options
 */
export function createWalkFarmV1({root,size,module,spawn=nodeSpawn,policy=WALK_FARM_V1,lease}){''')
sub(WF,'''  close(){closed=true;for(const child of[...children])''','''  close(){closed=true;lease?.release();for(const child of[...children])''')
sub(WF,''' return planFarmWorkersV1({workerMegabytes:policy.childBytes/1048576,environment,...hostMegabytes''',''' return planFarmWorkersV1({kind:"walk-farm",workerMegabytes:policy.childBytes/1048576,environment,...hostMegabytes''')
sub(WF,'''let capacitySource=''','''/** 🧮️ Takes the walk children of a farm from the one memory plan of the process tree (`reserveFarmV1`, kind `walk-farm`): `wanted` children, fewer where the project farm, the cut or the share leave less; `yield` is called when the project farm needs the memory back. The farm hands the lease to `createWalkFarmV1`, which releases it at its close.
 * @param {number} wanted @param {{yield?:()=>void;environment?:Record<string,string|undefined>;hostMegabytes?:number;residentMegabytes?:number;policy?:typeof WALK_FARM_V1}} [options] */
export function reserveWalkFarmV1(wanted,{yield:onYield,environment=process.env,hostMegabytes,residentMegabytes,policy=WALK_FARM_V1}={}){
 return reserveFarmV1({kind:"walk-farm",workerMegabytes:policy.childBytes/1048576,wanted,environment,yield:onYield,...hostMegabytes===undefined?{}:{hostMegabytes},...residentMegabytes===undefined?{}:{residentMegabytes}});
}

let capacitySource=''')
sub(WF,'''  const size=options.size??Math.min(walkFarmSizeV1(capacitySource()),walkFarmCapV1());
  if(size<2){held.calls=-Infinity;return undefined;}
  held.farm=createWalkFarmV1({root:workspaceRoot,size,''','''  const lease=options.size===undefined?reserveWalkFarmV1(walkFarmSizeV1(capacitySource()),{yield:()=>closeRecipeFarm(key)}):undefined,size=options.size??lease.workers;
  if(size<2){lease?.release();held.calls=-Infinity;return undefined;}
  held.farm=createWalkFarmV1({root:workspaceRoot,size,...lease?{lease}:{},''')
# settler
sub(CK,'import{WALK_FARM_V1,bindWalkFarmCapacityV1,createWalkFarmV1,partIdOfV1,walkFarmCapV1,walkFarmOffV1,walkFarmSizeV1}from"./🏭️walk-farm/🟨️.mjs";','import{WALK_FARM_V1,bindWalkFarmCapacityV1,createWalkFarmV1,partIdOfV1,reserveWalkFarmV1,walkFarmOffV1,walkFarmSizeV1}from"./🏭️walk-farm/🟨️.mjs";')
sub(CK,'''  const size=farmOptions?.size??Math.min(walkFarmSizeV1(capacityOfGrantV1(currentGrantV1)()),walkFarmCapV1());
  if(size<2){farmFailures=WALK_FARM_V1.failuresToStop;return undefined;}
  farm=createWalkFarmV1({root:farmRoot,size,''','''  const lease=farmOptions?.size===undefined?reserveWalkFarmV1(walkFarmSizeV1(capacityOfGrantV1(currentGrantV1)()),{yield:()=>farmClosed()}):undefined,size=farmOptions?.size??lease.workers;
  if(size<2){lease?.release();farmFailures=WALK_FARM_V1.failuresToStop;return undefined;}
  farm=createWalkFarmV1({root:farmRoot,size,...lease?{lease}:{},''')
# project farm
sub(PF,'import { choosePoolV1,','import { reserveFarmV1 } from "../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";\nimport { choosePoolV1,')
sub(PF,'''    hub = createHubV1(key);
    HUBS.set(key, hub);''','''    const lease = reserveFarmV1({ kind: "project-farm", workerMegabytes: policy.workerMegabytes, wanted: plan.workers, environment });
    if (lease.workers < 2) { lease.release(); say("serial", { projects, reason: "memory" }); return undefined; }
    plan.workers = Math.min(plan.workers, lease.workers);
    hub = createHubV1(key);
    hub.lease = lease;
    HUBS.set(key, hub);''')
sub(PF,'''  hub.closing = true;
  if (HUBS.get(hub.key) === hub) HUBS.delete(hub.key);''','''  hub.closing = true;
  hub.lease?.release();
  if (HUBS.get(hub.key) === hub) HUBS.delete(hub.key);''')
sub(PN,'maximumWorkers:planFarmWorkersV1({workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes})','maximumWorkers:planFarmWorkersV1({kind:"project-farm",workerMegabytes:GRAPH_FARM_POLICY_V1.workerMegabytes})')

# tests
PT="5131bb";WT="7cc254";CT="ef86ae"
sub(PT,'"hostShare","treeShare","minimumMegabytes"','"hostShare","treeShare","farmPriority","parentMegabytes","farmWorkerPeakMegabytes","minimumMegabytes"')
sub(PT,'treeShare:{type:"number",exclusiveMinimum:0,maximum:1},','treeShare:{type:"number",exclusiveMinimum:0,maximum:1},farmPriority:{type:"object",additionalProperties:{type:"integer"}},parentMegabytes:{type:"integer",minimum:1},farmWorkerPeakMegabytes:{type:"object",additionalProperties:{type:"integer",minimum:1}},')
sub(WT,'"16 GB worker: half the host for the whole tree, 3000 MB held, 768 MiB a child").toBe(Math.floor((16384*0.5-3000)/(WALK_FARM_V1.childBytes/1048576)))','"16 GB worker: the plan limit share of the host for the whole tree, 3000 MB held, 768 MiB a child").toBe(Math.floor((16384*0.4-Math.max(3000,PLAN_STAGE_POLICY_V1.parentMegabytes))/(WALK_FARM_V1.childBytes/1048576)))')
sub(WT,'expect(walkFarmCapV1({...base,hostMegabytes:8192,residentMegabytes:3000})).toBe(1);','expect(walkFarmCapV1({...base,hostMegabytes:8192,residentMegabytes:3000}),"8 GB worker: the parent alone holds the share, no child").toBe(0);')
sub(CT,'hostMegabytes:16384,environment:{SEMIO_GRAPH_CUT_CONCURRENCY:"64"}})).toBe(4);','hostMegabytes:16384,environment:{SEMIO_GRAPH_CUT_CONCURRENCY:"64"}})).toBe(3);')
sub(CT,'hostMegabytes:16384,environment:{}})).toBe(4);','hostMegabytes:16384,environment:{}})).toBe(3);')

sub(WT,'import{CLOSURE_SETTLE_V1,createClosureSettlerV1}from"../../🟨️.mjs";','import{CLOSURE_SETTLE_V1,createClosureSettlerV1}from"../../🟨️.mjs";\nimport{PLAN_STAGE_POLICY_V1}from"../../../../../🧠️memory/🪜️plan-stage/🟨️.mjs";')
sub(PSD,'readonly recentSamples:number}','readonly recentSamples:number;readonly farmPriority:Readonly<Record<string,number>>;readonly parentMegabytes:number;readonly farmWorkerPeakMegabytes:Readonly<Record<string,number>>}')
sub(PSD,'policy?:PlanStagePolicyV1;kind?:string}):number;','policy?:PlanStagePolicyV1;kind?:string;counted?:string}):number;')
print("staged ok")
