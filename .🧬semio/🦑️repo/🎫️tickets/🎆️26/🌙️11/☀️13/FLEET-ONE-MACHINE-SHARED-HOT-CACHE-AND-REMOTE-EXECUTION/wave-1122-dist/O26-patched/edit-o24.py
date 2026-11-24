import json,sys,os
root=sys.argv[1]
here=os.path.dirname(os.path.abspath(__file__))
NB=os.path.join(root,"🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass")
JS=os.path.join(NB,"🧵️job-slots")
DS=os.path.join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts")
def rd(p):return open(p,encoding="utf8").read()
def wr(p,t):open(p+".o24tmp","w",encoding="utf8").write(t);os.replace(p+".o24tmp",p)
def once(t,old,new,marker=None):
    if (marker or new) in t:return t
    assert t.count(old)==1,(old[:80],t.count(old))
    return t.replace(old,new)
runner=rd(os.path.join(here,"runner.part.ts")).rstrip("\n")+"\n"
fleet=rd(os.path.join(here,"fleet.part.ts"))
p=os.path.join(JS,"🟦️.ts");t=rd(p)
end='\n if(failure)throw failure.error;\n}\n'
if "FleetJobReasonV1" not in t:
    a=t.index('/** 🎫️ What a running pass holds here');b=t.index('\n',t.index('export type PassJobSlotV1'))
    t=t[:a]+'/** 🎫️ What a running pass holds here: a `local` slot (a jobserver token and, beyond the first, a memory reservation of its predicted peak) or a `fleet` one (only the small local footprint of a pass whose units and final run on workers: the cargo process, the rustc wrappers and the input shipping). */\nexport type PassJobSlotV1="local"|"fleet";'+t[b:]
    t=once(t,'definition:"ComponentJobsHost"|','definition:"FleetJobAdmitHost"|"FleetJobAdmitDecision"|"ComponentJobsHost"|')
    a=t.index('const pause=(milliseconds');b=t.index(end,a)+len(end)
    t=t[:a]+runner+t[b:]
    t=t.rstrip("\n")+"\n"+fleet
else:
    a=t.index('/** 🌐️ Why a pass may or may not start on a fleet slot');b=t.index(end,a)+len(end)
    t=t[:a]+runner+t[b:]
    a=t.index('\n/** 🌐️ What one decision to start a pass on a fleet slot reads')
    t=t[:a].rstrip("\n")+"\n"+fleet
wr(p,t)
p=os.path.join(JS,"🧬️schema/🔣️.json");d=json.loads(rd(p));D=d["definitions"]
D["Policy"]["const"].update({"fleetPollMilliseconds":5000,"fleetPassFootprintMegabytes":512,"fleetFloorMegabytes":512})
D["FleetJobReason"]={"enum":["fleet-fits","fleet-none","fleet-busy","fleet-cap","fleet-pressure","fleet-memory"]}
def integer(lo,hi,null=False):return {"type":["integer","null"] if null else "integer","minimum":lo,"maximum":hi}
D["FleetJobAdmitHost"]={"type":"object","additionalProperties":False,"required":["running","maximum","slots","availableMegabytes","reservedMegabytes","floorMegabytes","pressure","footprintMegabytes"],"properties":{"running":integer(0,1024),"maximum":integer(0,1024),"slots":integer(0,1048576,True),"availableMegabytes":integer(0,1048576,True),"reservedMegabytes":integer(0,4194304),"floorMegabytes":integer(0,1048576),"pressure":{"anyOf":[{"$ref":"#/definitions/Pressure"},{"type":"null"}]},"footprintMegabytes":integer(1,1048576)}}
D["FleetJobAdmitDecision"]={"type":"object","additionalProperties":False,"required":["admit","reason","freeMegabytes"],"properties":{"admit":{"type":"boolean"},"reason":{"$ref":"#/definitions/FleetJobReason"},"freeMegabytes":integer(-4194304,1048576)}}
M=D["Messages"]["const"]
M["en"]["jobs"]="[component-jobs] {operation}: {jobs} component jobs at once = {local} local slot(s) + {remote} fleet slot(s) ({slots} worker slots fit one {peak} MB final at the start and are read again while the build runs; {reason})"
M["de"]["jobs"]="[component-jobs] {operation}: {jobs} Komponenten-Jobs gleichzeitig = {local} lokale Plätze + {remote} Flotten-Plätze ({slots} Worker-Plätze tragen beim Start ein {peak}-MB-Endstück und werden während des Builds neu gelesen; {reason})"
M["en"]["fleet"]={"plan":"[component-jobs] {operation}: {local} local slot(s) and up to {fleet} fleet slot(s); a fleet job reserves {footprint} MB here, the free worker slots are read again every {seconds} s ({slots} fit a final now)","started":"[component-jobs] {pass} starts on a fleet slot beside {running} running job(s): it reserves {footprint} MB here and its final is placed on a worker ({reason})","waitingFleet":"[component-jobs] {pass} waits for a fleet slot ({running} fleet job(s) run here, {slots} worker slot(s) fit a final right now; {reason})","waitingLocal":"[component-jobs] {pass} waits for a local slot: it needs {peak} MB here, {free} MB are free above the {floor} MB floor ({lreason}); no fleet slot ({reason})","waitingEither":"[component-jobs] {pass} waits for a fleet slot or a local slot: no worker slot is free ({reason}) and a local job needs {peak} MB, {free} MB are free above the {floor} MB floor ({lreason})","reasons":{"fleet-fits":"a worker slot is free and the small local footprint fits","fleet-none":"the fleet does not answer","fleet-busy":"every worker slot that fits a final is taken","fleet-cap":"the authored cap of fleet jobs is reached","fleet-pressure":"memory pressure is critical, so not even a fleet job starts","fleet-memory":"the free memory above the floor does not carry another fleet job"}}
M["de"]["fleet"]={"plan":"[component-jobs] {operation}: {local} lokale Plätze und bis zu {fleet} Flotten-Plätze; ein Flotten-Job belegt hier {footprint} MB, freie Worker-Plätze werden alle {seconds} s neu gelesen ({slots} tragen jetzt ein Endstück)","started":"[component-jobs] {pass} startet auf einem Flotten-Platz neben {running} laufenden Job(s): es reserviert hier {footprint} MB, sein Endstück wird auf einem Worker platziert ({reason})","waitingFleet":"[component-jobs] {pass} wartet auf einen Flotten-Platz ({running} Flotten-Job(s) laufen hier, {slots} Worker-Platz/Plätze tragen gerade ein Endstück; {reason})","waitingLocal":"[component-jobs] {pass} wartet auf einen lokalen Platz: es braucht hier {peak} MB, über der Untergrenze von {floor} MB sind {free} MB frei ({lreason}); kein Flotten-Platz ({reason})","waitingEither":"[component-jobs] {pass} wartet auf einen Flotten-Platz oder einen lokalen Platz: kein Worker-Platz ist frei ({reason}), und ein lokaler Job braucht {peak} MB, über der Untergrenze von {floor} MB sind {free} MB frei ({lreason})","reasons":{"fleet-fits":"ein Worker-Platz ist frei und der kleine lokale Anteil passt","fleet-none":"die Flotte antwortet nicht","fleet-busy":"jeder Worker-Platz, der ein Endstück trägt, ist belegt","fleet-cap":"die festgelegte Obergrenze für Flotten-Jobs ist erreicht","fleet-pressure":"der Speicherdruck ist kritisch, daher startet nicht einmal ein Flotten-Job","fleet-memory":"der freie Speicher über der Untergrenze trägt keinen weiteren Flotten-Job"}}
wr(p,json.dumps(d,indent=2,ensure_ascii=False)+"\n")
p=os.path.join(NB,"🟦️.ts");t=rd(p)
t=once(t,"import { componentPassPeakMegabytesV1, describeLocalComponentJobsV1,","import { componentFleetChargeV1, componentJobsLimitV1, componentPassPeakMegabytesV1, describeFleetComponentJobsV1, describeLocalComponentJobsV1,",marker="componentFleetChargeV1, componentJobsLimitV1")
t=once(t,"type ComponentJobsDecisionV1, type LocalJobSampleV1, type PassJobSlotV1 }","type ComponentJobsDecisionV1, type FleetJobEventV1, type LocalJobSampleV1, type PassJobSlotV1 }",marker="type FleetJobEventV1")
t=once(t,"  componentJobs?: (facts: Readonly<{ local: number; unitMegabytes: number; crates: readonly string[]; signal: AbortSignal }>) => Promise<ComponentJobsDecisionV1 | undefined>;\n","  componentJobs?: (facts: Readonly<{ local: number; unitMegabytes: number; crates: readonly string[]; signal: AbortSignal }>) => Promise<ComponentJobsDecisionV1 | undefined>;\n  fleetSlots?: (facts: Readonly<{ crates: readonly string[]; signal: AbortSignal }>) => Promise<number | null | undefined>;\n",marker="  fleetSlots?: (facts")
old="  const { jobs: componentJobsTotal, local: localJobsAllowed } = planComponentJobSlotsV1({ fleetJobs: jobSlots, tokens, wanted: options.localJobs?.maximum ?? 1024 });\n"
new="  const fleetLive = fleetRemote > 0 && options.fleetSlots !== undefined, offloadableCrates = [...new Set(plan.passes.filter(pass => nativeBuildPassOffloadableV1(pass)).flatMap(pass => pass.packages.map(row => row.name.replaceAll(\"-\", \"_\"))))];\n  const { jobs: plannedJobs, local: localJobsAllowed } = planComponentJobSlotsV1({ fleetJobs: fleetLive ? 1 : jobSlots, tokens, wanted: options.localJobs?.maximum ?? 1024 }), componentJobsTotal = fleetLive ? componentJobsLimitV1 : plannedJobs;\n"
t=once(t,old,new,marker="const fleetLive = fleetRemote > 0")
old="    await Promise.all(lanes.map(async (lane) => {\n      await runPassesInJobSlotsV1(lane.passes, { jobs: componentJobsTotal, check, run: runPass, signal, ...local ? { local } : {}, ...componentJobsTotal > 1 ? { fail: stop } : {} });"
new="    const fleetOptions = fleetLive ? { maximum: componentJobsLimitV1 - 1, slots: () => options.fleetSlots!({ crates: offloadableCrates, signal }), ...componentFleetChargeV1, sample: () => sampleLocalMemory(signal), report: (pass: NativeBuildPassV1, event: FleetJobEventV1) => log(describeFleetComponentJobsV1(event.kind, { pass: pass.id, running: event.running, slots: event.slots ?? 0, footprint: event.footprintMegabytes, peak: event.peakMegabytes, free: event.freeMegabytes, floor: event.floorMegabytes, waitsFor: event.waitsFor ?? \"fleet\", reason: event.fleetReason ?? \"fleet-none\", localReason: event.localReason ?? \"local-memory\" })) } : undefined;\n    if (fleetOptions) log(describeFleetComponentJobsV1(\"plan\", { operation: \"component build\", local: localJobsAllowed, fleet: fleetOptions.maximum, footprint: fleetOptions.footprintMegabytes, seconds: Math.round(fleetOptions.pollMilliseconds / 1000), slots: await options.fleetSlots!({ crates: offloadableCrates, signal }).catch(() => undefined) ?? 0 }));\n    await Promise.all(lanes.map(async (lane) => {\n      await runPassesInJobSlotsV1(lane.passes, { jobs: componentJobsTotal, check, run: runPass, signal, ...local ? { local } : {}, ...fleetOptions ? { fleet: fleetOptions } : {}, ...componentJobsTotal > 1 ? { fail: stop } : {} });"
t=once(t,old,new,marker="const fleetOptions = fleetLive")
wr(p,t)
t=rd(DS)
old="fleetRunning:()=>store.environment.SEMIO_UNIT_CACHE_ROOT?"
new="fleetSlots:async facts=>{const root=store.environment.SEMIO_UNIT_CACHE_ROOT,measured=root===undefined?[]:facts.crates.flatMap(crate=>readUnitPeakV1(root,crate)??[]).sort((left,right)=>left-right),peak=measured.length>0?measured[Math.floor(measured.length/2)]!:offloadUnitPeakMegabytesV1(undefined,[\"cdylib\"],\"wasm32-wasip2\",root===undefined?undefined:readFamilyPeakV1(root));return fleetLinkedSlotsV1(store.environment,{signal:facts.signal,peakMegabytes:peak});},fleetRunning:()=>store.environment.SEMIO_UNIT_CACHE_ROOT?"
t=once(t,old,new,marker="fleetSlots:async facts=>")
wr(DS,t)
print("ok",root)
