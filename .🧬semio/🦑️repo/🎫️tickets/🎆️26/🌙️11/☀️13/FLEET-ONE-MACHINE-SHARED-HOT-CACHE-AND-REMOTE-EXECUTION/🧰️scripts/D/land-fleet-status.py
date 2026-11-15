import os,sys
# Anchored edits that add the fleet section to `repo fleet status`. usage: land-fleet-status.py <fleet-status dir> [--revert]
root=sys.argv[1];revert="--revert" in sys.argv
def read(p): return open(os.path.join(root,p),encoding="utf8").read()
def write(p,s): open(os.path.join(root,p),"w",encoding="utf8").write(s)
edits=[]
def edit(path,old,new): edits.append((path,old,new))

MACHINE='''{
   "type": "object",
   "additionalProperties": false,
   "required": ["origin", "platform", "pressure", "load", "cores", "running", "queued", "headroom", "ageSeconds", "worker", "offloadedIn", "offloadedOut", "refused", "cacheHit", "cacheMiss"],
   "properties": {
    "origin": {"type": "string"},
    "platform": {"type": "string"},
    "pressure": {"enum": ["normal", "warn", "critical"]},
    "load": {"anyOf": [{"type": "number", "minimum": 0}, {"type": "null"}]},
    "cores": {"type": "integer", "minimum": 1},
    "running": {"type": "integer", "minimum": 0},
    "queued": {"type": "integer", "minimum": 0},
    "headroom": {"type": "boolean"},
    "ageSeconds": {"type": "integer", "minimum": 0},
    "worker": {"enum": ["absent", "idle", "busy", "refusing"]},
    "offloadedIn": {"type": "integer", "minimum": 0},
    "offloadedOut": {"type": "integer", "minimum": 0},
    "refused": {"type": "integer", "minimum": 0},
    "cacheHit": {"type": "integer", "minimum": 0},
    "cacheMiss": {"type": "integer", "minimum": 0}
   }
  }'''
COUNTS='''{"type": "object", "additionalProperties": false, "required": ["hits", "misses", "puts"], "properties": {"hits": {"type": "integer", "minimum": 0}, "misses": {"type": "integer", "minimum": 0}, "puts": {"type": "integer", "minimum": 0}}}'''
CLAIMS='''{"type": "object", "additionalProperties": false, "required": ["active", "waiting", "oldestSeconds"], "properties": {"active": {"type": "integer", "minimum": 0}, "waiting": {"type": "integer", "minimum": 0}, "oldestSeconds": {"type": "integer", "minimum": 0}}}'''
edit("🧬️schema/🔣️.json",'      "generation",\n      "scheduler"\n     ]','      "generation",\n      "scheduler",\n      "fleet"\n     ]')
edit("🧬️schema/🔣️.json",'    "scheduler": {\n     "$ref": "#/definitions/SchedulerFact"\n    }\n   }\n  },','    "scheduler": {\n     "$ref": "#/definitions/SchedulerFact"\n    },\n    "fleet": {\n     "$ref": "#/definitions/FleetFact"\n    }\n   }\n  },')
edit("🧬️schema/🔣️.json",'    "scheduler": {\n     "$ref": "#/definitions/SchedulerSection"\n    }\n   }\n  },','    "scheduler": {\n     "$ref": "#/definitions/SchedulerSection"\n    },\n    "fleet": {\n     "$ref": "#/definitions/FleetSection"\n    }\n   }\n  },')
edit("🧬️schema/🔣️.json",'''    "scheduler.agent": {
     "en": "  agent {agent}: {running} running, {queued} queued, {served} served ({percent}%)",''','''    "fleet": {
     "en": "Fleet of machines: {machines} reporting, {queued} queued and {executing} executing at the scheduler; {offloadedIn} units offloaded, {offloadedOut} requested, {refused} refused, {declined} declined, {retried} rescheduled",
     "de": "Maschinenflotte: {machines} melden sich, {queued} warten und {executing} laufen beim Scheduler; {offloadedIn} Einheiten ausgelagert, {offloadedOut} angefragt, {refused} abgelehnt, {declined} verweigert, {retried} neu geplant"
    },
    "fleet.machine": {
     "en": "  {origin} {platform}: pressure {pressure}, load {load}/{cores} cores, {running} running, {queued} queued, worker {worker}{headroom}, in {offloadedIn} out {offloadedOut} refused {refused}, units {cacheHit} hit {cacheMiss} miss, report {age}",
     "de": "  {origin} {platform}: Druck {pressure}, Last {load}/{cores} Kerne, {running} laufen, {queued} warten, Worker {worker}{headroom}, ein {offloadedIn} aus {offloadedOut} abgelehnt {refused}, Einheiten {cacheHit} Treffer {cacheMiss} Fehlgriffe, Meldung {age}"
    },
    "fleet.noheadroom": {
     "en": " (no headroom)",
     "de": " (kein Spielraum)"
    },
    "fleet.cache": {
     "en": "  cache server: {hits} hit, {misses} miss, {puts} stored",
     "de": "  Cache-Server: {hits} Treffer, {misses} Fehlgriffe, {puts} gespeichert"
    },
    "fleet.claims": {
     "en": "  claims: {active} active, {waiting} waiting, oldest {oldest}",
     "de": "  Beanspruchungen: {active} aktiv, {waiting} wartend, älteste {oldest}"
    },
    "scheduler.agent": {
     "en": "  agent {agent}: {running} running, {queued} queued, {served} served ({percent}%)",''')
edit("🧬️schema/🔣️.json",'''    "agents": {
     "type": "array",
     "items": {
      "$ref": "#/definitions/SchedulerAgentFact"
     }
    }
   }
  }
 }
}''','''    "agents": {
     "type": "array",
     "items": {
      "$ref": "#/definitions/SchedulerAgentFact"
     }
    }
   }
  },
  "FleetMachine": '''+MACHINE+''',
  "FleetFact": {
   "description": "The machines of the fleet as the scheduler of remote execution sees them, with the counters of the cache server.",
   "type": "object",
   "additionalProperties": false,
   "required": ["scheduler", "expireSeconds", "machines", "queued", "executing", "offloadRejected", "refused", "retried"],
   "properties": {
    "scheduler": {"type": "string"},
    "expireSeconds": {"type": "integer", "minimum": 0},
    "machines": {"type": "array", "items": {"$ref": "#/definitions/FleetMachine"}},
    "queued": {"type": "integer", "minimum": 0},
    "executing": {"type": "integer", "minimum": 0},
    "offloadRejected": {"type": "integer", "minimum": 0},
    "refused": {"type": "integer", "minimum": 0},
    "retried": {"type": "integer", "minimum": 0},
    "cache": '''+COUNTS+''',
    "claims": '''+CLAIMS+'''
   }
  },
  "FleetSection": {
   "type": "object",
   "additionalProperties": false,
   "required": ["scheduler", "machines", "queued", "executing", "offloadedIn", "offloadedOut", "refused", "declined", "retried", "cacheHit", "cacheMiss"],
   "properties": {
    "scheduler": {"type": "string"},
    "machines": {"type": "array", "items": {"$ref": "#/definitions/FleetMachine"}},
    "queued": {"type": "integer", "minimum": 0},
    "executing": {"type": "integer", "minimum": 0},
    "offloadedIn": {"type": "integer", "minimum": 0},
    "offloadedOut": {"type": "integer", "minimum": 0},
    "refused": {"type": "integer", "minimum": 0},
    "declined": {"type": "integer", "minimum": 0},
    "retried": {"type": "integer", "minimum": 0},
    "cacheHit": {"type": "integer", "minimum": 0},
    "cacheMiss": {"type": "integer", "minimum": 0},
    "cache": '''+COUNTS+''',
    "claims": '''+CLAIMS+'''
   }
  }
 }
}''')
edit("🟦️.ts",'"generation"|"scheduler";ok:boolean','"generation"|"scheduler"|"fleet";ok:boolean')
edit("🟦️.ts",' scheduler?:FleetSchedulerFactV1;\n',' scheduler?:FleetSchedulerFactV1;\n fleet?:FleetLoadFactV1;\n')
edit("🟦️.ts",'export type FleetFindingV1=','/** 🌍️ The machines of the fleet as the scheduler of remote execution sees them (load, pressure, queue, worker, offload and unit cache counters) and what the cache server counts. */\nexport type FleetLoadFactV1=Readonly<{scheduler:string;expireSeconds:number;machines:readonly Readonly<{origin:string;platform:string;pressure:"normal"|"warn"|"critical";load:number|null;cores:number;running:number;queued:number;headroom:boolean;ageSeconds:number;worker:"absent"|"idle"|"busy"|"refusing";offloadedIn:number;offloadedOut:number;refused:number;cacheHit:number;cacheMiss:number}>[];queued:number;executing:number;offloadRejected:number;refused:number;retried:number;cache?:Readonly<{hits:number;misses:number;puts:number}>;claims?:Readonly<{active:number;waiting:number;oldestSeconds:number}>}>;\nexport type FleetFindingV1=')
edit("🟦️.ts",'generation:Row|null;scheduler?:Row;findings','generation:Row|null;scheduler?:Row;fleet?:Row;findings')
edit("🟦️.ts",'const natural=(left:string,right:string)','const fleetSection=(fact:FleetLoadFactV1):Row=>({scheduler:fact.scheduler,machines:[...fact.machines].sort((left,right)=>compare(left.origin,right.origin)),queued:fact.queued,executing:fact.executing,offloadedIn:fact.machines.reduce((sum,row)=>sum+row.offloadedIn,0),offloadedOut:fact.machines.reduce((sum,row)=>sum+row.offloadedOut,0),refused:fact.refused,declined:fact.offloadRejected,retried:fact.retried,cacheHit:fact.machines.reduce((sum,row)=>sum+row.cacheHit,0),cacheMiss:fact.machines.reduce((sum,row)=>sum+row.cacheMiss,0),...fact.cache===undefined?{}:{cache:fact.cache},...fact.claims===undefined?{}:{claims:fact.claims}});\n\nconst natural=(left:string,right:string)')
edit("🟦️.ts",'...facts.scheduler===undefined?{}:{scheduler:schedulerSection(facts.scheduler)},findings:kept','...facts.scheduler===undefined?{}:{scheduler:schedulerSection(facts.scheduler)},...facts.fleet===undefined?{}:{fleet:fleetSection(facts.fleet)},findings:kept')
edit("🟦️.ts",' const bazel=report.bazel;\n',''' const fleet=report.fleet;
 if(fleet!==undefined){
  lines.push(say("fleet",{machines:fleet.machines.length,queued:fleet.queued,executing:fleet.executing,offloadedIn:fleet.offloadedIn,offloadedOut:fleet.offloadedOut,refused:fleet.refused,declined:fleet.declined,retried:fleet.retried}));
  for(const machine of fleet.machines as Row[])lines.push(say("fleet.machine",{origin:machine.origin,platform:machine.platform,pressure:machine.pressure,load:machine.load===null?"?":machine.load,cores:machine.cores,running:machine.running,queued:machine.queued,worker:machine.worker,headroom:machine.headroom?"":say("fleet.noheadroom"),offloadedIn:machine.offloadedIn,offloadedOut:machine.offloadedOut,refused:machine.refused,cacheHit:machine.cacheHit,cacheMiss:machine.cacheMiss,age:duration(machine.ageSeconds)}));
  if(fleet.cache!==undefined)lines.push(say("fleet.cache",fleet.cache));
  if(fleet.claims!==undefined)lines.push(say("fleet.claims",{active:fleet.claims.active,waiting:fleet.claims.waiting,oldest:duration(fleet.claims.oldestSeconds)}));
 }
 const bazel=report.bazel;
''')
edit("📥️readers/🟦️.ts",'/** 🏭️ The readers of the real workspace. */\nexport const FLEET_READERS_V1:FleetReadersV1=Object.freeze({leases:readFleetLeasesV1,capacity:readFleetCapacityV1,bazel:readFleetBazelV1,storage:readFleetStorageV1,sealed:readFleetSealedV1,generation:readFleetGenerationV1,scheduler:readFleetSchedulerV1});','''/** 🌍️ Fleet: the machines of the fleet with their load, pressure, queue, worker and offload counters from the scheduler of remote execution, and the counters and claims of the cache server; nothing while no scheduler is published. */
export async function readFleetLoadV1(context:FleetReaderContextV1):Promise<Partial<FleetFactsV1>>{
 const fleet=await readRemoteExecutionFleetFactsV1({root:context.root,environment:context.environment});
 return fleet===undefined?{}:{fleet};
}

/** 🏭️ The readers of the real workspace. */
export const FLEET_READERS_V1:FleetReadersV1=Object.freeze({leases:readFleetLeasesV1,capacity:readFleetCapacityV1,bazel:readFleetBazelV1,storage:readFleetStorageV1,sealed:readFleetSealedV1,generation:readFleetGenerationV1,scheduler:readFleetSchedulerV1,fleet:readFleetLoadV1});''')
edit("📥️readers/🟦️.ts",'import {createFileSchedulerStoreV1,schedulerDirectoryV1} from "../../🗓️scheduler/💾️store/🟦️.ts";','import {createFileSchedulerStoreV1,schedulerDirectoryV1} from "../../🗓️scheduler/💾️store/🟦️.ts";\nimport {readRemoteExecutionFleetFactsV1} from "../../🌐️remote-execution/🚢️status/🟦️.ts";')
if revert: edits=[(p,n,o) for (p,o,n) in reversed(edits)]
for p,o,n in edits:
    if read(p).count(o)!=1: print("ANCHOR",read(p).count(o),p,repr(o[:80]));sys.exit(1)
cache={}
for p,o,n in edits:
    s=cache.get(p) or read(p)
    if s.count(o)!=1: print("ANCHOR after earlier edit",p,repr(o[:80]));sys.exit(1)
    cache[p]=s.replace(o,n)
for p,s in cache.items(): write(p,s)
print("reverted" if revert else "applied",len(edits),"edits in",len(cache),"files")
