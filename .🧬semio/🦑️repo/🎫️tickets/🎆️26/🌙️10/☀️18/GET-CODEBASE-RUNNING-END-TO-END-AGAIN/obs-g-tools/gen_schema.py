#!/usr/bin/env python3
"""[obs-g] writes the closed schema of the fleet status module (policy, facts, report, EN+DE messages). usage: gen_schema.py <schema file>"""
import json,sys
def obj(props,required=None,extra=False):
    return {"type":"object","additionalProperties":extra,"required":sorted(required if required is not None else props.keys()),"properties":props}
I={"type":"integer","minimum":0}
S={"type":"integer"}
T={"type":"string"}
B={"type":"boolean"}
def nul(x): return {"anyOf":[x,{"type":"null"}]}
def arr(x,maximum=None):
    a={"type":"array","items":x}
    if maximum is not None: a["maxItems"]=maximum
    return a
def ref(n): return {"$ref":"#/definitions/"+n}
def en(*v): return {"enum":list(v)}
def opt(props,optional): return obj(props,[k for k in props if k not in optional])
Priority=en("interactive","agent","background")
HolderState=en("legacy","progressing","silent","stuck","stale")
Standing=opt({"state":HolderState,"priority":Priority,"phase":T,"done":I,"total":I,"etaMinutes":I,"etaOverdue":B,"implementationDigest":T,"implementationAgeSeconds":I,"parentPid":I,"chain":arr(I),"silentSeconds":I,"staleIds":arr(T)},["silentSeconds","staleIds","priority","phase","done","total","etaMinutes","etaOverdue","implementationDigest","implementationAgeSeconds","parentPid","chain"])
Mode=en("shared","exclusive")
Holder=opt({"pid":I,"resource":T,"mode":Mode,"class":T,"heldSeconds":I,"command":T,"session":T,"agent":T,"standing":ref("StandingFact")},["command","session","agent","standing"])
Waiter=opt({"pid":I,"resource":T,"mode":Mode,"class":T,"waitedSeconds":I,"position":I,"blockers":arr(T),"command":T,"session":T,"agent":T,"priority":Priority,"historyKey":T},["position","command","session","agent","priority","historyKey"])
Grant=opt({"id":T,"operation":T,"pid":I,"jobs":I,"reservedMegabytes":I,"observedMegabytes":nul(I),"heldSeconds":I,"phase":T,"lent":B,"session":T,"agent":T},["phase","lent","session","agent"])
CapWaiter=opt({"id":T,"operation":T,"pid":I,"needMegabytes":I,"waitedSeconds":I,"reason":T,"reasonText":T,"position":I,"session":T,"agent":T},["session","agent"])
Source=opt({"id":en("leases","capacity","bazel","storage","sealed","generation"),"ok":B,"milliseconds":I,"error":T},["error"])
Pressure=en("normal","warn","critical")
HostFact=obj({"platform":T,"cores":nul(I),"totalMegabytes":I,"availableMegabytes":I,"pressure":Pressure,"swapUsedMegabytes":nul(I),"load":nul({"type":"number","minimum":0})})
Facts=opt({
 "version":{"const":1},"atMs":I,"workspace":T,
 "host":HostFact,
 "leases":obj({"holders":arr(ref("HolderFact")),"waiters":arr(ref("WaiterFact")),"cycles":arr(T)}),
 "capacity":obj({"floorMegabytes":I,"headroomMegabytes":S,"capacityMegabytes":I,"grants":arr(ref("GrantFact")),"waiters":arr(ref("CapacityWaiterFact"))}),
 "activeMilliseconds":{"type":"object","additionalProperties":arr(I),"description":"per operation: durations of earlier runs in milliseconds, oldest first"},
 "history":{"type":"object","additionalProperties":arr(I),"description":"per lease history key: durations of earlier holds in milliseconds, oldest first"},
 "bazel":obj({"slotPrefix":T,"slots":arr(opt({"slot":T,"pid":nul(I),"key":T,"usedAt":I,"session":T,"agent":T,"treeMegabytes":nul(I)},["key","usedAt","session","agent"]))}),
 "storage":obj({"hostFreeBytes":nul(I),"hostFreeFloorBytes":I,"images":arr(opt({"id":T,"kind":en("partition","control"),"state":en("mounted","detached","directory","absent"),"usedBytes":nul(I),"capacityBytes":nul(I),"budgetBytes":nul(I),"source":en("statfs","recorded","walked","unknown")},[])),"overAreas":arr(T),"machine":obj({"totalBytes":I,"effectiveTotalBytes":I,"usedBytes":I,"footprintBytes":I,"overBytes":I,"evictBytes":I,"blockedBytes":I,"source":en("argument","environment","file","default")})}),
 "sealed":obj({"mode":en("sealed","live","promote"),"entries":arr(opt({"entry":T,"digest":T,"files":I,"bytes":I,"promotedAtMs":I,"fresh":nul(B),"changed":I,"blocked":obj({"atMs":I,"rows":I,"first":T},["atMs","rows"]),"last":obj({"atMs":I,"added":I,"removed":I,"changed":I})},["blocked","last"]))}),
 "generation":obj({"number":I,"built":obj({"kind":en("cold","delta","refresh"),"dirty":I}),"owners":I,"implementation":T,"publishedAtMs":nul(I),"verified":nul(obj({"atMs":I,"covered":I,"count":I}))}),
 "sources":arr(ref("SourceFact")),
},["host","leases","capacity","activeMilliseconds","history","bazel","storage","sealed","generation"])
Policy=obj({"version":{"const":1},
 "runs":obj({"whoCharacters":I,"whatCharacters":I,"commandCharacters":I,"resourceCharacters":I,"exclusiveShown":I,"maximumRows":I,"parentWalk":I}),
 "queues":obj({"maximumLeaseQueues":I,"maximumWaitersShown":I}),
 "readers":obj({"timeoutMilliseconds":I}),
 "graph":obj({"builderResources":arr(T),"claimResources":arr(T),"slotResources":arr(T)}),
 "findings":obj({"maximum":I,"underReservedBasisPoints":I,"capacityWaitSeconds":I,"leaseWaitSeconds":I,"imageFullBasisPoints":I,"imageShownBasisPoints":I}),
 "digestCharacters":I})
RunImpl=obj({"state":HolderState,"digest":T,"ageSeconds":I,"freshness":en("current","older","unknown")},["state","freshness"])
RunMem=obj({"admittedMegabytes":nul(I),"observedMegabytes":nul(I),"deltaMegabytes":nul(S),"jobs":I},["admittedMegabytes","observedMegabytes","deltaMegabytes","jobs"])
Run=opt({"pid":I,"pids":arr(I),"state":en("running","blocked","waiting"),"agent":T,"session":T,"command":T,"operation":T,"phase":T,"done":I,"total":I,"etaSeconds":I,"etaOverdue":B,"ageSeconds":I,"priority":Priority,"implementation":ref("RunImplementation"),"memory":ref("RunMemory"),"waitingFor":obj({"kind":en("lease","capacity"),"subject":T,"waitedSeconds":I,"estimatedWaitSeconds":nul(I)}),"exclusive":arr(T),"lent":B},["agent","session","command","operation","phase","done","total","etaSeconds","etaOverdue","priority","implementation","waitingFor","lent"])
LeaseQueue=obj({"resource":T,"class":T,"holders":arr(opt({"pid":I,"mode":Mode,"heldSeconds":I,"etaSeconds":nul(I),"state":HolderState},["state"])),"waiters":arr(opt({"pid":I,"mode":Mode,"position":nul(I),"waitedSeconds":I,"priority":Priority,"estimatedWaitSeconds":nul(I)},["priority"]))})
Report=obj({
 "version":{"const":1},"atMs":I,"workspace":T,
 "summary":obj({"running":I,"blocked":I,"waiting":I,"alerts":I,"warnings":I}),
 "memory":opt({"totalMegabytes":I,"availableMegabytes":I,"floorMegabytes":I,"spareMegabytes":S,"pressure":Pressure,"swapUsedMegabytes":nul(I),"load":nul({"type":"number","minimum":0}),"cores":nul(I),"admittedMegabytes":I,"observedMegabytes":I,"unobservedGrants":I,"overReservedMegabytes":I,"underReservedMegabytes":I,"grants":I,"waiters":I},[]),
 "runs":arr(ref("Run")),
 "queues":obj({
  "graph":obj({"builder":nul(opt({"pid":I,"phase":T,"done":I,"total":I,"heldSeconds":I,"state":HolderState,"etaSeconds":nul(I)},["phase","done","total"])),"computers":I,"slots":I,"waiting":I,"longestWaitSeconds":I,"estimatedWaitSeconds":nul(I)}),
  "capacity":obj({"floorMegabytes":I,"spareMegabytes":S,"grants":I,"waiters":arr(obj({"id":T,"pid":I,"operation":T,"needMegabytes":I,"waitedSeconds":I,"reason":T,"reasonText":T,"position":I,"estimatedWaitSeconds":nul(I)}))}),
  "leases":arr(ref("LeaseQueue")),"leasesOmitted":I}),
 "bazel":obj({"slots":I,"live":I,"held":I,"rssMegabytes":I,"servers":arr(opt({"slot":T,"live":B,"held":B,"pid":nul(I),"holderPid":nul(I),"treeMegabytes":nul(I),"key":T,"usedAt":I,"agent":T},["key","usedAt","agent"]))}),
 "storage":obj({"hostFreeBytes":nul(I),"hostFreeFloorBytes":I,"machine":Facts["properties"]["storage"]["properties"]["machine"],"overAreas":arr(T),"images":arr(opt({"id":T,"kind":en("partition","control"),"state":en("mounted","detached","directory","absent"),"usedBytes":nul(I),"capacityBytes":nul(I),"budgetBytes":nul(I),"usedBasisPoints":nul(I),"source":en("statfs","recorded","walked","unknown")},[]))}),
 "sealed":obj({"mode":en("sealed","live","promote"),"drifted":I,"blocked":I,"entries":arr(opt({"entry":T,"digest":T,"files":I,"bytes":I,"promotedAtMs":I,"ageSeconds":I,"fresh":nul(B),"changed":I,"blocked":Facts["properties"]["sealed"]["properties"]["entries"]["items"]["properties"]["blocked"],"last":Facts["properties"]["sealed"]["properties"]["entries"]["items"]["properties"]["last"]},["blocked","last"]))}),
 "generation":nul(obj({"number":I,"built":Facts["properties"]["generation"]["properties"]["built"],"owners":I,"implementation":T,"publishedAtMs":nul(I),"ageSeconds":nul(I),"verified":nul(obj({"atMs":I,"ageSeconds":I,"covered":I,"count":I,"basisPoints":I}))})),
 "findings":arr(ref("Finding")),
 "sources":arr(ref("SourceFact")),
})
Finding=opt({"id":T,"severity":en("alert","warn","info"),"subject":T,"values":{"type":"object","additionalProperties":{"anyOf":[{"type":"string"},{"type":"number"}]}},"text":T},[])
def msg(en_,de_): return {"en":en_,"de":de_}
M={
 "header":msg("Fleet {time}: {running} running, {blocked} blocked, {waiting} waiting; {alerts} alert(s), {warnings} warning(s)","Flotte {time}: {running} laufen, {blocked} blockiert, {waiting} warten; {alerts} Alarm(e), {warnings} Warnung(en)"),
 "memory":msg("Memory: {admitted} MB admitted, {observed} MB real over {grants} run(s); host {available} of {total} MB free, {spare} MB above the {floor} MB floor; pressure {pressure}, swap {swap} MB, load {load}","Speicher: {admitted} MB zugesagt, {observed} MB real in {grants} Lauf/Läufen; Rechner {available} von {total} MB frei, {spare} MB über der Untergrenze von {floor} MB; Druck {pressure}, Swap {swap} MB, Last {load}"),
 "runsHeader":msg("Runs ({count}, {shown} shown)","Läufe ({count}, {shown} angezeigt)"),
 "runsNone":msg("Runs: none registered","Läufe: keine registriert"),
 "run":msg("  {pid} {state} {who} {what}{progress}{eta} · {age} · {priority} · code {code} · memory {memory}{waiting}","  {pid} {state} {who} {what}{progress}{eta} · {age} · {priority} · Code {code} · Speicher {memory}{waiting}"),
 "state.running":msg("running","läuft"),"state.blocked":msg("blocked","blockiert"),"state.waiting":msg("waiting","wartet"),
 "code.current":msg("current","aktuell"),"code.older":msg("older","älter"),"code.unknown":msg("unknown","unbekannt"),
 "priority.interactive":msg("interactive","interaktiv"),"priority.agent":msg("agent","Agent"),"priority.background":msg("background","Hintergrund"),"priority.unknown":msg("-","-"),
 "progress":msg(" {phase}{counter}"," {phase}{counter}"),
 "eta":msg(" · about {eta} left"," · noch ca. {eta}"),"etaOverdue":msg(" · longer than usual"," · länger als üblich"),
 "memoryPair":msg("{admitted}/{observed} MB","{admitted}/{observed} MB"),"memoryNone":msg("-","-"),
 "waiting.lease":msg(" · waits {waited} for {subject}{estimate}"," · wartet seit {waited} auf {subject}{estimate}"),
 "waiting.capacity":msg(" · waits {waited} for memory ({subject}){estimate}"," · wartet seit {waited} auf Speicher ({subject}){estimate}"),
 "estimate":msg(", about {estimate} more",", noch ca. {estimate}"),
 "queuesHeader":msg("Queues","Warteschlangen"),
 "queue.graph":msg("  Graph: builder pid {pid} {phase}{counter}, held {held}{eta}; {computers} computer(s), {slots} slot(s), {waiting} waiting{longest}{estimate}","  Graph: Builder pid {pid} {phase}{counter}, seit {held}{eta}; {computers} Rechner, {slots} Slot(s), {waiting} wartend{longest}{estimate}"),
 "queue.graphIdle":msg("  Graph: no builder; {computers} computer(s), {waiting} waiting{longest}","  Graph: kein Builder; {computers} Rechner, {waiting} wartend{longest}"),
 "queue.longest":msg(" (longest {waited})"," (längste {waited})"),
 "queue.capacity":msg("  Capacity: {grants} granted, {waiters} waiting, {spare} MB spare above the {floor} MB floor","  Kapazität: {grants} zugesagt, {waiters} wartend, {spare} MB frei über der Untergrenze von {floor} MB"),
 "queue.capacityWaiter":msg("    #{position} pid {pid} {operation} needs {need} MB, waited {waited}, {reason}{estimate}","    #{position} pid {pid} {operation} braucht {need} MB, wartet seit {waited}, {reason}{estimate}"),
 "queue.lease":msg("  Lease {resource} [{class}]: {holders}","  Lease {resource} [{class}]: {holders}"),
 "queue.leaseHolder":msg("pid {pid} {mode} {held}{eta}","pid {pid} {mode} seit {held}{eta}"),
 "queue.leaseNoHolder":msg("no holder","kein Halter"),
 "queue.leaseWaiter":msg("    #{position} pid {pid} {mode} waited {waited}{estimate}","    #{position} pid {pid} {mode} wartet seit {waited}{estimate}"),
 "queue.leasesOmitted":msg("  … {count} more lease queue(s)","  … {count} weitere Lease-Warteschlange(n)"),
 "queue.none":msg("  nothing queued","  nichts in Warteschlangen"),
 "bazel":msg("Bazel: {live} server(s) live ({rss} MB), {held} of {slots} slot(s) held","Bazel: {live} Server aktiv ({rss} MB), {held} von {slots} Slot(s) belegt"),
 "bazelServer":msg("  {slot} pid {pid} {rss} MB {state}","  {slot} pid {pid} {rss} MB {state}"),
 "bazel.held":msg("held by pid {holder}","belegt von pid {holder}"),"bazel.idle":msg("idle","untätig"),
 "storage":msg("Storage: host free {free} GiB (floor {floor} GiB); machine budget {used} of {effective} GiB used, {over} GiB over, {evict} GiB to evict ({source})","Speicher auf Platte: Rechner frei {free} GiB (Untergrenze {floor} GiB); Rechnerbudget {used} von {effective} GiB belegt, {over} GiB darüber, {evict} GiB zu räumen ({source})"),
 "storageImage":msg("  {id} {used} of {capacity} GiB ({percent}%)","  {id} {used} von {capacity} GiB ({percent}%)"),
 "sealed":msg("Implementation ({mode}): {count} snapshot(s), {drifted} drifted, {blocked} blocked","Implementierung ({mode}): {count} Abbild(er), {drifted} abgewichen, {blocked} blockiert"),
 "sealedEntry":msg("  {entry} {digest} promoted {age} ago, {files} files, {freshness}","  {entry} {digest} vor {age} übernommen, {files} Dateien, {freshness}"),
 "fresh.true":msg("fresh","frisch"),"fresh.false":msg("live tree has moved on ({changed} change(s))","Arbeitsbaum ist weitergezogen ({changed} Änderung(en))"),"fresh.null":msg("freshness unknown","Frische unbekannt"),
 "generation":msg("Generation {number}: built {kind} {age} ago ({dirty} dirty owner(s)); verified {covered} of {count} owners ({percent}%) {verified} ago","Generation {number}: {kind} gebaut vor {age} ({dirty} geänderte Besitzer); geprüft {covered} von {count} Besitzern ({percent}%) vor {verified}"),
 "generationNone":msg("Generation: none published","Generation: keine veröffentlicht"),
 "findingsHeader":msg("Findings","Befunde"),
 "finding":msg("  [{severity}] {text}","  [{severity}] {text}"),
 "severity.alert":msg("alert","Alarm"),"severity.warn":msg("warning","Warnung"),"severity.info":msg("info","Info"),
 "sources":msg("Sources: {count} readers answered in {milliseconds} ms","Quellen: {count} Leser antworteten in {milliseconds} ms"),
 "sourcesFailed":msg("Sources: {failed} of {count} failed: {list}","Quellen: {failed} von {count} fehlgeschlagen: {list}"),
 "unavailable":msg("{section}: unavailable ({reason})","{section}: nicht verfügbar ({reason})"),
 "unit.s":msg("{n} s","{n} s"),"unit.min":msg("{n} min","{n} min"),"unit.h":msg("{n} h","{n} h"),
 "finding.cycle":msg("Deadlock cycle among leases: {subject}","Verklemmungszyklus bei Leases: {subject}"),
 "finding.holder-stuck":msg("pid {pid} holds {resource} without progress for {minutes} min","pid {pid} hält {resource} seit {minutes} min ohne Fortschritt"),
 "finding.holder-stale":msg("pid {pid} holds {resource} with older code than {ids}","pid {pid} hält {resource} mit älterem Code ({ids})"),
 "finding.memory-pressure":msg("Host memory pressure is {pressure}","Der Speicherdruck des Rechners ist {pressure}"),
 "finding.under-reserved":msg("pid {pid} uses {observed} MB but only {reserved} MB are admitted","pid {pid} nutzt {observed} MB, zugesagt sind aber nur {reserved} MB"),
 "finding.capacity-wait":msg("pid {pid} waits {waited} for memory ({operation}, needs {need} MB)","pid {pid} wartet seit {waited} auf Speicher ({operation}, braucht {need} MB)"),
 "finding.lease-wait":msg("pid {pid} waits {waited} for {resource}","pid {pid} wartet seit {waited} auf {resource}"),
 "finding.disk-low":msg("Host disk space {free} GiB is below the {floor} GiB floor","Freier Plattenplatz {free} GiB liegt unter der Untergrenze von {floor} GiB"),
 "finding.disk-over-budget":msg("The caches exceed the machine budget by {over} GiB","Die Caches überschreiten das Rechnerbudget um {over} GiB"),
 "finding.image-full":msg("Image {id} is {percent}% full","Abbild {id} ist zu {percent}% voll"),
 "finding.sealed-blocked":msg("The implementation snapshot {entry} is blocked: {first}","Das Implementierungsabbild {entry} ist blockiert: {first}"),
 "finding.sealed-drifted":msg("The live tree changed after snapshot {entry} ({changed} change(s))","Der Arbeitsbaum änderte sich nach dem Abbild {entry} ({changed} Änderung(en))"),
 "finding.source-failed":msg("The {section} reader failed: {reason}","Der Leser {section} schlug fehl: {reason}"),
 "error.syntax":msg("Usage: repo fleet status [--json] [--limit=<n>]","Verwendung: repo fleet status [--json] [--limit=<n>]"),
 "error.timeout":msg("The reader did not answer within {seconds} s","Der Leser antwortete nicht innerhalb von {seconds} s"),
 "error.limit":msg("--limit requires an integer from 1 to {maximum}","--limit verlangt eine ganze Zahl von 1 bis {maximum}"),
 "error.invalid":msg("The fleet report broke its own schema: {details}","Der Flottenbericht verletzt sein eigenes Schema: {details}"),
}
Policy_const={"version":1,
 "runs":{"whoCharacters":12,"whatCharacters":24,"commandCharacters":80,"resourceCharacters":60,"exclusiveShown":3,"maximumRows":200,"parentWalk":16},
 "queues":{"maximumLeaseQueues":12,"maximumWaitersShown":6},
 "readers":{"timeoutMilliseconds":5000},
 "graph":{"builderResources":["build-graph-state:","repository-graph-facts:"],"claimResources":["graph-claim:"],"slotResources":["graph-claim-slot:"]},
 "findings":{"maximum":30,"underReservedBasisPoints":15000,"capacityWaitSeconds":600,"leaseWaitSeconds":900,"imageFullBasisPoints":9000,"imageShownBasisPoints":8000},
 "digestCharacters":12}
Corpus=obj({"version":{"const":1},"cases":arr(obj({"id":T,"language":en("en","de"),"facts":ref("Facts"),"report":ref("Report")}))})
schema={"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://semio.tech/framework/build/fleet-status/v1","title":"Fleet status: one read-only picture of the shared workspace (runs, queues, memory, Bazel servers, storage, sealed implementation, graph generation)",
 "definitions":{"Policy":{"const":Policy_const},"PolicySchema":Policy,"Messages":{"const":M},
  "StandingFact":Standing,"HolderFact":Holder,"WaiterFact":Waiter,"GrantFact":Grant,"CapacityWaiterFact":CapWaiter,"SourceFact":Source,"Facts":Facts,
  "RunImplementation":RunImpl,"RunMemory":RunMem,"Run":Run,"LeaseQueue":LeaseQueue,"Finding":Finding,"Report":Report,"Corpus":Corpus}}
open(sys.argv[1],"w",encoding="utf8").write(json.dumps(schema,ensure_ascii=False,indent=1)+"\n")
print("written",sys.argv[1],len(json.dumps(schema)),"bytes")
