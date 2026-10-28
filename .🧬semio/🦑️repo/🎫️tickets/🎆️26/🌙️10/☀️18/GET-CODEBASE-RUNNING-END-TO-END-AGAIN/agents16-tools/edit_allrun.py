"""🧑‍🤝‍🧑 agents16 batch `allrun`: anchored code edits that let three agents run side by side (every anchor must occur exactly once; idempotent: an edit whose result is already present is skipped).
Usage: python3 -I edit_allrun.py <root> [--check]
1 decision: an agent session with no running jobs gets its first job whenever memory fits (our booked cores never starve a session); an `elastic` request (the project farm) is decided after every other request, never makes one wait and never becomes a barrier.
2 decision: an upgrade whose memory fits but whose cores our other runs book is admitted with the jobs the cores leave (one under the session rule) and keeps its whole memory.
2b gate: a nested invocation that inherited a quiet grant hands its body the grant as it is after the upgrade (not the quiet row's one job).
3 upgrade: EN/DE `upgradedReduced` when the growth was admitted with fewer jobs (`reduced`).
4 estimates + door: a reservation backed by the class's own measured peaks takes no ramp slot.
5 door: the farm request is elastic, carries the agent session and observes nothing of its own (its workers are descendants of the run whose grant observes them); giving it back when its pool is refused is batch `farmfree` (edit_farmfree.py), because the farm module is a hot file of another lane.
"""
import os, sys

HC = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
EDITS = [
 (HC + "📏️decision/🟦️.ts",
  "/** 🎫️ What one operation asks for (`Request` of the schema); `envelopePerJobMegabytes`",
  "/** 🎫️ What one operation asks for (`Request` of the schema); `elastic` marks a supplementary request of a run that already holds a grant and goes on without it (the project farm): it is decided after every other request and never makes one wait; `envelopePerJobMegabytes`"),
 (HC + "📏️decision/🟦️.ts",
  "rampSlot?:boolean;quiet?:boolean}>;\n/** ⏳️",
  "rampSlot?:boolean;quiet?:boolean;elastic?:boolean}>;\n/** ⏳️"),
 (HC + "📏️decision/🟦️.ts",
  "then by arrival, then by id; grow requests of running grants come after every other request. */",
  "then by arrival, then by id; grow requests of running grants and elastic requests come after every other request. */"),
 (HC + "📏️decision/🟦️.ts",
  "const rank=(waiter:HostWaiterV1)=>waiter.grow===true?2:",
  "const rank=(waiter:HostWaiterV1)=>waiter.grow===true||waiter.request.elastic===true?2:"),
 (HC + "📏️decision/🟦️.ts",
  "it never shrinks a reservation, never becomes a barrier and never takes the partial path of an upgrade. */",
  "it never shrinks a reservation, never becomes a barrier and never takes the partial path of an upgrade. An elastic request (`elastic`, the workers a running build asks for beside its own grant and does without) is decided after every other request like a grow request: it waits `queue-order` while any other request waits or a barrier holds, takes only what fits now and never becomes a barrier head or an override. Our own booked cores never keep an agent session from its first job: a request whose session holds no other live grant (and was given none earlier in the same evaluation) is admitted with its minimum jobs whenever its memory fits and no soft stop applies, even when the cores of our grants already reach `load.perCoreCeiling` per core (while they stay below that ceiling plus one core per core of the host), so every agent progresses and the ceiling bounds everything beyond one job per session. An upgrade whose whole request fits the memory but not the cores our other grants book is admitted (`reduced`) with the most jobs those cores leave once the minimum jobs of every fresh or quiet request served after it are set aside (so a growth never takes the cores a later small request needs, with or without an agent session), or with one job under the session rule, and keeps the memory of its whole request (a run's memory does not shrink with its jobs; the grower adds jobs when cores free up), instead of waiting for another run to end. */"),
 (HC + "📏️decision/🟦️.ts",
  "quiet=request.quiet===true,grows=waiter.grow===true&&waiter.upgrade===true,",
  "quiet=request.quiet===true,grows=waiter.grow===true&&waiter.upgrade===true,elastic=request.elastic===true,"),
 (HC + "📏️decision/🟦️.ts",
  " for(const waiter of orderHostWaitersV1(aged,active)){\n",
  " const ordered=orderHostWaitersV1(aged,active);\n for(const [at,waiter] of ordered.entries()){\n"),
 (HC + "📏️decision/🟦️.ts",
  "heads:{bypasses:number}[]=[];let blocked=false,",
  "heads:{bypasses:number}[]=[],sessions=new Set<string>();let blocked=false,"),
 (HC + "📏️decision/🟦️.ts",
  "const refuse=(reason:HostReasonV1):void=>{if(!grows)waits=true;",
  "const refuse=(reason:HostReasonV1):void=>{if(!grows&&!elastic)waits=true;"),
 (HC + "📏️decision/🟦️.ts",
  "admitted++;rampers=freeRampers+(request.rampSlot===true?1:0);",
  "admitted++;if(waiter.session!==undefined)sessions.add(waiter.session);rampers=freeRampers+(request.rampSlot===true?1:0);"),
 (HC + "📏️decision/🟦️.ts",
  "  if(blocked&&!(quiet&&quietLeft>0)){refuse(\"queue-order\");continue;}",
  "  if(elastic&&(blocked||waits)||blocked&&!(quiet&&quietLeft>0)){refuse(\"queue-order\");continue;}"),
 (HC + "📏️decision/🟦️.ts",
  "  const memoryFits=fits(request.minimumJobs);\n",
  "  const memoryFits=fits(request.minimumJobs),first=!elastic&&waiter.session!==undefined&&own<ceiling+host.cores&&!sessions.has(waiter.session)&&active.every(grant=>grant.id===request.id||grant.session!==waiter.session);\n"),
 (HC + "📏️decision/🟦️.ts",
  "if(jobs===0&&pause===undefined&&memoryFits&&(avoids||own-",
  "if(jobs===0&&pause===undefined&&memoryFits&&(avoids||first&&replaced===undefined||own-"),
 (HC + "📏️decision/🟦️.ts",
  "  if(jobs>0){admit(jobs,jobs<request.preferredJobs?\"reduced\":\"fits\");continue;}\n",
  "  if(jobs>0){admit(jobs,jobs<request.preferredJobs?\"reduced\":\"fits\");continue;}\n  if(pause===undefined&&memoryFits&&replaced!==undefined){const cores=Math.min(request.preferredJobs,Math.floor((ceiling-own+replaced.cpu-ordered.slice(at+1).filter(later=>later.upgrade!==true&&later.grow!==true&&later.request.elastic!==true).reduce((sum,later)=>sum+later.request.minimumJobs*later.request.cpuPerJob,0))/request.cpuPerJob));if(cores>=1||first){admit(Math.max(1,cores),\"reduced\",need(request.minimumJobs));continue;}}\n"),
 (HC + "📏️decision/🟦️.ts",
  "  else if(reason!==\"memory\"&&active.length-(replaced?1:0)===0",
  "  else if(reason!==\"memory\"&&!elastic&&active.length-(replaced?1:0)===0"),
 (HC + "📏️decision/🟦️.ts",
  "   if((reason===\"memory\"||reason===\"load\")&&!quiet){",
  "   if((reason===\"memory\"||reason===\"load\")&&!quiet&&!elastic){"),
 (HC + "📏️decision/🟦️.ts",
  "|\"upgradedPartly\"|\"upgradeWithdrawn\"|",
  "|\"upgradedPartly\"|\"upgradedReduced\"|\"upgradeWithdrawn\"|"),
 (HC + "🧬️schema/🔣️.json",
  "        \"upgradedPartly\",\n        \"upgradeWithdrawn\",",
  "        \"upgradedPartly\",\n        \"upgradedReduced\",\n        \"upgradeWithdrawn\","),
 (HC + "🧬️schema/🔣️.json",
  "        \"upgradedPartly\": {\n          \"type\": \"string\"\n        },\n",
  "        \"upgradedPartly\": {\n          \"type\": \"string\"\n        },\n        \"upgradedReduced\": {\n          \"type\": \"string\"\n        },\n"),
 (HC + "🧬️schema/🔣️.json",
  "        \"rampSlot\": {\n          \"type\": \"boolean\"\n        },\n        \"quiet\": {\n          \"type\": \"boolean\"\n        }\n      }\n    },\n    \"Waiter\": {",
  "        \"rampSlot\": {\n          \"type\": \"boolean\"\n        },\n        \"quiet\": {\n          \"type\": \"boolean\"\n        },\n        \"elastic\": {\n          \"type\": \"boolean\"\n        }\n      }\n    },\n    \"Waiter\": {"),
 (HC + "🧬️schema/🔣️.json",
  "the run goes on with {jobs} of {preferred} job(s)\",\n",
  "the run goes on with {jobs} of {preferred} job(s)\",\n          \"upgradedReduced\": \"[capacity] {operation}: reservation grown from {from} MB to {to} MB with {jobs} of {preferred} job(s) after {seconds}s: the host cannot carry more right now ({available} MB available, {floor} MB floor); the run grows to more jobs once memory and cores free up\",\n"),
 (HC + "🧬️schema/🔣️.json",
  "der Lauf geht mit {jobs} von {preferred} Job(s) weiter\",\n",
  "der Lauf geht mit {jobs} von {preferred} Job(s) weiter\",\n          \"upgradedReduced\": \"[capacity] {operation}: Reservierung nach {seconds} s von {from} MB auf {to} MB mit {jobs} von {preferred} Job(s) vergroessert: der Rechner traegt gerade nicht mehr ({available} MB verfuegbar, Untergrenze {floor} MB); der Lauf waechst auf mehr Jobs, sobald Speicher und Kerne frei werden\",\n"),
 (HC + "🎫️gate/🟦️.ts",
  "  const grant=inherited as HostGrantV1;\n  if(grant.quiet===true)await(await import(\"../🔼️upgrade/🟦️.ts\")).requestHostCapacityUpgradeV1(environment,options.control);\n",
  "  let grant=inherited as HostGrantV1;\n  if(grant.quiet===true){await(await import(\"../🔼️upgrade/🟦️.ts\")).requestHostCapacityUpgradeV1(environment,options.control);const current=readJson(inheritedPath);if(isHostCapacityV1(\"Grant\",current)&&(current as HostGrantV1).id===grant.id)grant=current as HostGrantV1;}\n"),
 (HC + "🎫️gate/🟦️.ts",
  "/** 🚪️ Admits one operation against the whole machine and runs its body under the grant: it waits in a fair machine-wide queue with progress lines and cancellation, reserves memory and cores while the body runs and always gives them back. */",
  "/** 🚪️ Admits one operation against the whole machine and runs its body under the grant: it waits in a fair machine-wide queue with progress lines and cancellation, reserves memory and cores while the body runs and always gives them back. A nested invocation that inherits a quiet grant first has it grown and then hands its body the grant as it is after the growth (its jobs, cores and plan), never the quiet row's single job. */"),
 (HC + "🔼️upgrade/🟦️.ts",
  "      const wanted=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes));\n      write(mine.reservedMegabytes<wanted?",
  "      const wanted=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,Math.max(0,host.totalMegabytes-evaluation.floorMegabytes));\n      write(mine.reason===\"reduced\"?formatHostCapacityV1(language,\"upgradedReduced\",{operation:request.operation,from:state.reservedMegabytes,to:mine.reservedMegabytes,jobs:mine.jobs,preferred:request.preferredJobs,seconds:Math.round((at-started)/1000),available:host.availableMegabytes,floor:evaluation.floorMegabytes}):mine.reservedMegabytes<wanted?"),
 (HC + "🔼️upgrade/🟦️.ts",
  "Progress lines come in the language of the run;",
  "A growth whose memory fits but whose cores our other runs hold is admitted with the jobs those cores leave (at least one for an agent session with nothing else running), keeping the memory of the whole request (`upgradedReduced`), and the grower of the holder adds jobs once cores free up, instead of the run waiting for another run to end. Progress lines come in the language of the run;"),
 (HC + "📐️estimates/🟦️.ts",
  "/** ⏱️ Gives a scaled request the time it needs to reach its peak and, when its reservation was shrunk below the default (fixed block or per-job share), a ramp slot: only `memory.maximumRamping` such reservations may grow towards their peak at once, so a wrong estimate overshoots with a bounded number of runs. */\nexport function rampHostRequestV1(base:HostRequestV1,scaled:HostRequestV1,estimate:HostEstimateV1):HostRequestV1{\n return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({...scaled,rampMilliseconds:estimate.rampMilliseconds,rampSlot:scaled.fixedMegabytes<base.fixedMegabytes||scaled.perJobMegabytes<base.perJobMegabytes}));",
  "/** ⏱️ Gives a scaled request the time it needs to reach its peak and, when its reservation was shrunk below the default (fixed block or per-job share) on evidence that is not the class's own measured peaks (`measured`: `hostClassBoundV1` answers `measured`), a ramp slot: only `memory.maximumRamping` such reservations may grow towards their peak at once, so a wrong estimate overshoots with a bounded number of runs; a class sized by the peaks its own runs measured is no guess and ramps without a slot, so concurrent agents are not capped at the slot count. */\nexport function rampHostRequestV1(base:HostRequestV1,scaled:HostRequestV1,estimate:HostEstimateV1,measured:boolean=false):HostRequestV1{\n return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({...scaled,rampMilliseconds:estimate.rampMilliseconds,rampSlot:!measured&&(scaled.fixedMegabytes<base.fixedMegabytes||scaled.perJobMegabytes<base.perJobMegabytes)}));"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "import {adoptUpgradedHostPlanV1,hostProfileV1,",
  "import {adoptUpgradedHostPlanV1,hostClassBoundV1,hostProfileV1,"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "estimate.ratioBasisPoints,capacity,estimate.floorMegabytes),request=rampHostRequestV1(base,scaled,estimate),growth=estimate.growRatioBasisPoints<estimate.ratioBasisPoints?rampHostRequestV1(base,scaleHostRequestV1(policy,{...base,priority:request.priority},estimate.growRatioBasisPoints,capacity,estimate.floorMegabytes),estimate):request;",
  "estimate.ratioBasisPoints,capacity,estimate.floorMegabytes),measured=hostClassBoundV1(policy,learned,ask.lineage,now()).source===\"measured\",request=rampHostRequestV1(base,scaled,estimate,measured),growth=estimate.growRatioBasisPoints<estimate.ratioBasisPoints?rampHostRequestV1(base,scaleHostRequestV1(policy,{...base,priority:request.priority},estimate.growRatioBasisPoints,capacity,estimate.floorMegabytes),estimate,measured):request;"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "/** 🏊️ Asks the machine-wide gate for the worker processes of the graph inference farm beside the grant the invocation already holds: a fresh request in the fair queue, answered when the gate admits it, never given up on a timer.",
  "/** 🏊️ Asks the machine-wide gate for the worker processes of the graph inference farm beside the grant the invocation already holds: an elastic request of the invocation's agent session (decided after every other request, it never makes another run wait and never becomes a barrier, because the farm infers in the parent without it), answered when the gate admits it, never given up on a timer; its grant observes nothing of its own, since its workers are descendants of the run whose grant already observes them."),
 (LIB + "🌡️host-capacity/🟦️.ts",
  " const request=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:\"exec\",preferredJobs:input.preferredJobs,kinds:[\"build\"]}),megabytes=request?.perJobMegabytes??policy.kinds.build.megabytes;",
  " const sized=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:\"exec\",preferredJobs:input.preferredJobs,kinds:[\"build\"]}),request=sized&&Object.freeze({...sized,elastic:true}),megabytes=request?.perJobMegabytes??policy.kinds.build.megabytes,session=currentAgentSessionV1({environment});"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "  withHostCapacityV1({...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment},async handle=>",
  "  withHostCapacityV1({observe:()=>Promise.resolve(0),...input,directory:input.directory??hostCapacityDirectoryV1(environment),policy,request,control,environment,session:{address:agentSessionAddressV1(session),weight:session.budget.weight}},async handle=>"),
]


def main():
    root, check = sys.argv[1], "--check" in sys.argv
    files = {}
    for rel, old, new in EDITS:
        path = os.path.join(root, rel)
        text = files.get(rel) or open(path, encoding="utf8").read()
        if new in text and text.count(new) == 1 and old not in text.replace(new, ""):
            files[rel] = text
            continue
        if text.count(old) != 1:
            print("ANCHOR", text.count(old), rel, old[:80].replace("\n", "\\n"))
            sys.exit(1)
        files[rel] = text.replace(old, new)
    if check:
        print("all %d anchors resolve" % len(EDITS))
        return
    for rel, text in files.items():
        path = os.path.join(root, rel)
        if open(path, encoding="utf8").read() != text:
            mode = os.stat(path).st_mode & 0o777
            tmp = path + ".agents16.tmp"
            open(tmp, "w", encoding="utf8").write(text)
            os.chmod(tmp, mode)
            os.replace(tmp, path)
            print("edited", rel)


if __name__ == "__main__":
    main()
