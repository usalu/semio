"""🧾️ agents16 batch `behind` (stacked on `allrun`): every waiting and growing progress line names what the waiter stands behind, in English and German: the runs of ours that hold the resource its reason names (memory, cores, ramp slots, quiet slots) or the requests served before it (queue order), by the label of their request (the target of `repo run`), state, jobs and megabytes; `N running` no longer counts the waiter's own quiet grant. Anchored and idempotent; every anchor exactly once.
Usage: python3 -I edit_behind.py <root> [--check]
"""
import os, sys

HC = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
EN_WAIT = '"waiting": "[capacity] waiting {seconds}s for {operation}: {reason} (need {need} MB; {spare} MB spare above the {floor} MB floor, load {load}; {running} running, {ahead} ahead)",'
DE_WAIT = '"waiting": "[capacity] wartet seit {seconds} s auf {operation}: {reason} (benötigt {need} MB; {spare} MB frei oberhalb der Untergrenze von {floor} MB, Last {load}; {running} laufen, {ahead} davor)",'
EN_UP = '"upgrading": "[capacity] {operation}: work has to run, waiting {seconds}s to grow the reservation from {from} MB to {to} MB: {reason} ({spare} MB spare above the {floor} MB floor; {running} running, {ahead} ahead)",'
DE_UP = '"upgrading": "[capacity] {operation}: es muss Arbeit laufen, wartet seit {seconds} s darauf, die Reservierung von {from} MB auf {to} MB zu vergroessern: {reason} ({spare} MB frei oberhalb der Untergrenze von {floor} MB; {running} laufen, {ahead} davor)",'
LABEL = '        "label": {\n          "type": "string",\n          "pattern": "^[^\\\\p{Cc}]{1,120}$"\n        },\n'
EDITS = [
 (HC + "🧬️schema/🔣️.json", EN_WAIT, EN_WAIT.replace('{ahead} ahead)",', '{ahead} ahead); behind {behind}",') + '\n          "blocker": "{label} ({state}, {jobs} job(s), {megabytes} MB)",\n          "blockerHost": "the host itself (nothing of ours)",\n          "blockerStates": {\n            "running": "running",\n            "growing": "waiting to grow",\n            "quiet": "quiet",\n            "waiting": "waiting ahead"\n          },'),
 (HC + "🧬️schema/🔣️.json", DE_WAIT, DE_WAIT.replace('{ahead} davor)",', '{ahead} davor); hinter {behind}",') + '\n          "blocker": "{label} ({state}, {jobs} Job(s), {megabytes} MB)",\n          "blockerHost": "dem Rechner selbst (nichts von uns)",\n          "blockerStates": {\n            "running": "läuft",\n            "growing": "wartet aufs Wachsen",\n            "quiet": "ruhig",\n            "waiting": "wartet davor"\n          },'),
 (HC + "🧬️schema/🔣️.json", EN_UP, EN_UP.replace('{ahead} ahead)",', '{ahead} ahead); behind {behind}",')),
 (HC + "🧬️schema/🔣️.json", DE_UP, DE_UP.replace('{ahead} davor)",', '{ahead} davor); hinter {behind}",')),
 (HC + "🧬️schema/🔣️.json",
  '        "grown"\n      ],\n      "properties": {\n        "waiting": {\n          "type": "string"\n        },\n',
  '        "grown",\n        "blocker",\n        "blockerHost",\n        "blockerStates"\n      ],\n      "properties": {\n        "waiting": {\n          "type": "string"\n        },\n        "blocker": {\n          "type": "string"\n        },\n        "blockerHost": {\n          "type": "string"\n        },\n        "blockerStates": {\n          "type": "object",\n          "additionalProperties": false,\n          "required": [\n            "running",\n            "growing",\n            "quiet",\n            "waiting"\n          ],\n          "properties": {\n            "running": {\n              "type": "string"\n            },\n            "growing": {\n              "type": "string"\n            },\n            "quiet": {\n              "type": "string"\n            },\n            "waiting": {\n              "type": "string"\n            }\n          }\n        },\n'),
 (HC + "🧬️schema/🔣️.json",
  '        "elastic": {\n          "type": "boolean"\n        }\n      }\n    },\n    "Waiter": {',
  '        "elastic": {\n          "type": "boolean"\n        },\n' + LABEL.rstrip(",\n") + '\n      }\n    },\n    "Waiter": {'),
 (HC + "🧬️schema/🔣️.json",
  '        "session": {\n          "type": "string",\n          "pattern": "^[0-9a-f]{12}\\\\.[a-z][a-z0-9]{0,11}$"\n        },\n        "lentSinceMs": {',
  LABEL + '        "session": {\n          "type": "string",\n          "pattern": "^[0-9a-f]{12}\\\\.[a-z][a-z0-9]{0,11}$"\n        },\n        "lentSinceMs": {'),
 (HC + "📏️decision/🟦️.ts",
  "rampSlot?:boolean;quiet?:boolean;elastic?:boolean}>;\n/** ⏳️",
  "rampSlot?:boolean;quiet?:boolean;elastic?:boolean;label?:string}>;\n/** ⏳️"),
 (HC + "📏️decision/🟦️.ts",
  "/** 🎫️ What one operation asks for (`Request` of the schema); ",
  "/** 🎫️ What one operation asks for (`Request` of the schema); `label` names what it builds (the target of `repo run`), so the progress lines of others can say whom they stand behind; "),
 (HC + "📏️decision/🟦️.ts",
  "grownAtMs?:number;phase?:\"assembly\"|\"plan\"|\"rebuild\"|\"execution\"}>;",
  "grownAtMs?:number;label?:string;phase?:\"assembly\"|\"plan\"|\"rebuild\"|\"execution\"}>;"),
 (HC + "📏️decision/🟦️.ts",
  "|\"countersPartial\",values:",
  "|\"countersPartial\"|\"blocker\"|\"blockerHost\",values:"),
 (HC + "📏️decision/🟦️.ts",
  "/** ⌛️ Names a wait or override reason in the chosen language. */",
  "/** 🧾️ One thing of ours a waiting request stands behind: the label of its request, whether it runs, is quiet, waits to grow or waits ahead in the queue, its jobs and the megabytes it holds (committed) or asks for. */\nexport type HostBlockerV1=Readonly<{label:string;state:\"running\"|\"growing\"|\"quiet\"|\"waiting\";jobs:number;megabytes:number}>;\n\n/** 🧭️ Names what a request that waits in `evaluation` stands behind, decided by its reason: for `memory` the live grants of ours by the megabytes they count (committed: the reservation while it ramps, then what the tree holds), for `load` the grants by the cores they book, for `ramp` the grants that ramp in a slot (each with the requests admitted before it in the same evaluation, which run from now on), for `quiet-limit` the quiet grants, for `queue-order` the requests served before it that wait; a lent grant and the request's own grant never count; at most `limit`, largest first, then by label; empty for a reason that is the host's own (pressure, swap, files, images). A pure function.\n * @see ../🧪️tests/🧾️blockers/🐍️oracle/🐍️.py */\nexport function hostBlockersV1(policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],evaluation:HostEvaluationV1,id:string,nowMs:number,limit:number=3):readonly HostBlockerV1[]{\n const position=evaluation.decisions.findIndex(row=>row.id===id),mine=evaluation.decisions[position];\n if(mine===undefined||mine.verdict!==\"wait\")return Object.freeze([]);\n const ramping=(grant:HostGrantV1)=>nowMs-(grant.resumedAtMs??grant.admittedAtMs)<(grant.rampMilliseconds??policy.memory.rampMilliseconds),committed=(grant:HostGrantV1)=>ramping(grant)?Math.max(grant.reservedMegabytes,grant.observedMegabytes??0):grant.observedMegabytes??grant.reservedMegabytes;\n const growing=new Set(waiters.filter(waiter=>waiter.upgrade===true).map(waiter=>waiter.request.id)),grants=ledger.filter(grant=>grant.lentSinceMs===undefined&&grant.id!==id);\n const ofGrant=(grant:HostGrantV1,megabytes:number):HostBlockerV1=>Object.freeze({label:grant.label??`${grant.operation} ${grant.id.slice(0,6)}`,state:grant.quiet===true?growing.has(grant.id)?\"growing\" as const:\"quiet\" as const:\"running\" as const,jobs:grant.jobs,megabytes});\n const ahead=evaluation.decisions.slice(0,position).filter(row=>row.verdict===\"wait\").flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1)).map(waiter=>Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:\"waiting\" as const,jobs:waiter.request.minimumJobs,megabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes}));\n const admitted=evaluation.decisions.slice(0,position).filter(row=>row.verdict===\"admit\"&&!ledger.some(grant=>grant.id===row.id)).flatMap(row=>waiters.filter(waiter=>waiter.request.id===row.id).slice(0,1).map(waiter=>Object.freeze({blocker:Object.freeze({label:waiter.request.label??`${waiter.request.operation} ${waiter.request.id.slice(0,6)}`,state:\"running\" as const,jobs:row.jobs,megabytes:row.reservedMegabytes}),slot:waiter.request.rampSlot===true})));\n const rows=mine.reason===\"memory\"?[...grants.map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason===\"load\"?[...grants.filter(grant=>grant.cpu>0).map(grant=>ofGrant(grant,committed(grant))),...admitted.map(row=>row.blocker)]:mine.reason===\"ramp\"?[...grants.filter(grant=>grant.rampSlot===true&&ramping(grant)).map(grant=>ofGrant(grant,committed(grant))),...admitted.filter(row=>row.slot).map(row=>row.blocker)]:mine.reason===\"quiet-limit\"?grants.filter(grant=>grant.quiet===true).map(grant=>ofGrant(grant,committed(grant))):mine.reason===\"queue-order\"?ahead:[];\n const key=(row:HostBlockerV1)=>mine.reason===\"load\"?row.jobs:row.megabytes;\n return Object.freeze([...rows].sort((left,right)=>key(right)-key(left)||(left.label<right.label?-1:left.label>right.label?1:0)).slice(0,limit));\n}\n\n/** 🗂️ Renders what a waiter stands behind in the chosen language: each blocker as `label (state, jobs, megabytes)`, or the host itself when nothing of ours holds it back. */\nexport function describeHostBlockersV1(language:\"en\"|\"de\",blockers:readonly HostBlockerV1[]):string{\n const messages=schema.definitions.Messages.const[language];\n return blockers.length===0?messages.blockerHost:blockers.map(row=>fillHostCapacityV1(messages.blocker,{label:row.label,state:messages.blockerStates[row.state],jobs:row.jobs,megabytes:row.megabytes})).join(\", \");\n}\n\n/** ⌛️ Names a wait or override reason in the chosen language. */"),
 (HC + "🎫️gate/🟦️.ts",
  "import {decideHostHandoverV1,hostBootAnchorV1,type HostBootAnchorV1,describeHostHandoverRefusalV1,describeHostReasonV1,",
  "import {decideHostHandoverV1,describeHostBlockersV1,hostBlockersV1,hostBootAnchorV1,type HostBootAnchorV1,describeHostHandoverRefusalV1,describeHostReasonV1,"),
 (HC + "🎫️gate/🟦️.ts",
  "function waitingLine(language:\"en\"|\"de\",request:HostRequestV1,host:HostSampleV1,evaluation:HostEvaluationV1,running:number,seconds:number):string{",
  "/** ⏳️ The progress line of a waiting request, naming what it stands behind (`hostBlockersV1`). */\nfunction waitingLine(language:\"en\"|\"de\",request:HostRequestV1,host:HostSampleV1,evaluation:HostEvaluationV1,running:number,seconds:number,policy:HostCapacityPolicyV1,ledger:readonly HostGrantV1[],waiters:readonly HostWaiterV1[],nowMs:number):string{"),
 (HC + "🎫️gate/🟦️.ts",
  "floor:evaluation.floorMegabytes,load:host.load===null?\"n/a\":host.load,running,ahead:position});",
  "floor:evaluation.floorMegabytes,load:host.load===null?\"n/a\":host.load,running,ahead:position,behind:describeHostBlockersV1(language,hostBlockersV1(policy,ledger,waiters,evaluation,request.id,nowMs))});"),
 (HC + "🎫️gate/🟦️.ts",
  "write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000)));",
  "write(waitingLine(language,request,host,evaluation,ledger.length,Math.round((at-arrivedAtMs)/1000),policy,ledger,waiters,at));"),
 (HC + "🎫️gate/🟦️.ts",
  "...options.session===undefined?{}:{session:options.session.address},...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},",
  "...options.session===undefined?{}:{session:options.session.address},...request.label===undefined?{}:{label:request.label},...request.rampMilliseconds===undefined?{}:{rampMilliseconds:request.rampMilliseconds},"),
 (HC + "🔼️upgrade/🟦️.ts",
  "import {describeHostReasonV1,evaluateHostCapacityV1,",
  "import {describeHostBlockersV1,describeHostReasonV1,evaluateHostCapacityV1,hostBlockersV1,"),
 (HC + "🔼️upgrade/🟦️.ts",
  "spare,floor:evaluation.floorMegabytes,running:grants.length,ahead:position}));",
  "spare,floor:evaluation.floorMegabytes,running:grants.filter(row=>row.id!==request.id).length,ahead:position,behind:describeHostBlockersV1(language,hostBlockersV1(policy,grants,waiters,evaluation,request.id,at))}));"),
 (HC + "🔼️upgrade/🟦️.ts",
  "Progress lines come in the language of the run;",
  "Progress lines come in the language of the run and name what the growth stands behind (the runs of ours that hold the memory or cores it needs, or the requests served first);"),
 (HC + "🌦️forecast/🟦️.ts",
  "return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:full.id,operation:full.operation,priority:full.priority,",
  "return requireHostCapacityV1<HostRequestV1>(\"Request\",Object.freeze({id:full.id,operation:full.operation,...full.label===undefined?{}:{label:full.label},priority:full.priority,"),
 (HC + "🌦️forecast/🟦️.ts",
  "/** 🪶️ The small request of a forecast-quiet run: the learned quiet size as one fixed block of one job under the id of the full request,",
  "/** 🪶️ The small request of a forecast-quiet run: the learned quiet size as one fixed block of one job under the id (and label) of the full request,"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  " const base:HostRequestV1|undefined=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets});",
  " const sized=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:ask.operation,preferredJobs:ask.preferredJobs,kinds:ask.kinds,targets:ask.targets}),base:HostRequestV1|undefined=sized&&Object.freeze({...sized,label:(ask.lineage.length<2?\"\":ask.lineage[ask.lineage.length-1]!.slice(ask.lineage[ask.lineage.length-2]!.length+1)).replace(/\\p{Cc}/gu,\"\").slice(0,120)||ask.operation});"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "request=sized&&Object.freeze({...sized,elastic:true}),",
  "request=sized&&Object.freeze({...sized,elastic:true,label:\"graph-farm\"}),"),
 (HC + "🧪️tests/🟦️.ts",
  "ahead:1,jobs:3,preferred:8,reserved:7000,available:9000,switch:\"SEMIO_HOST_CAPACITY\"};",
  "ahead:1,behind:\"x\",jobs:3,preferred:8,reserved:7000,available:9000,switch:\"SEMIO_HOST_CAPACITY\"};"),
 (LIB + "🌡️host-capacity/🟦️.ts",
  "/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy: waits fairly with progress lines,",
  "/** 🚪️ Runs an invocation under the machine-wide capacity grant when its command is heavy: waits fairly with progress lines that name what it stands behind (its request is labelled with what it builds, e.g. the target of `repo run`, without control characters),"),
]


def main():
    root, check = sys.argv[1], "--check" in sys.argv
    files = {}
    for rel, old, new in EDITS:
        text = files.get(rel) or open(os.path.join(root, rel), encoding="utf8").read()
        if text.count(new) == 1 and text.count(old) <= (1 if old in new else 0):
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
