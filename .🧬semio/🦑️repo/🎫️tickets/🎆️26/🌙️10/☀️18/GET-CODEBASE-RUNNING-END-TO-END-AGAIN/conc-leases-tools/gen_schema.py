import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.join(HERE,"staged","🧰️framework","🔨️modules","🏃️process","🔒️leases","🧭️order","🧬️schema")
os.makedirs(OUT,exist_ok=True)
L={"type":"object","additionalProperties":False,"required":["en","de"],"properties":{"en":{"type":"string","minLength":1},"de":{"type":"string","minLength":1}}}
ms=lambda lo=0,hi=86400000:{"type":"integer","minimum":lo,"maximum":hi}
rule={"enum":["enforce","observe","off"]}
mode={"enum":["shared","exclusive"]}
def obj(required,props):return {"type":"object","additionalProperties":False,"required":required,"properties":props}
msg=lambda en,de:{"en":en,"de":de}
messages={
 "outOfOrder":msg("Lock order violated: {requested} ({requestedClass}, rank {requestedRank}) is requested while {held} ({heldClass}, rank {heldRank}) is held {heldMode}. Acquire locks in ascending rank or release {held} first.","Sperrreihenfolge verletzt: {requested} ({requestedClass}, Rang {requestedRank}) wird angefordert, während {held} ({heldClass}, Rang {heldRank}) {heldMode} gehalten wird. Sperren in aufsteigendem Rang nehmen oder {held} zuerst freigeben."),
 "holdAndWait":msg("Refusing to wait: {requested} ({requestedMode}) has been unavailable for {seconds} s{blocker} while this process holds {held} exclusively, which blocks everyone who needs {held}. Release {held} before waiting, or acquire {requested} first.","Warten abgelehnt: {requested} ({requestedMode}) ist seit {seconds} s nicht verfügbar{blocker}, während dieser Prozess {held} exklusiv haelt und damit alle blockiert, die {held} brauchen. {held} vor dem Warten freigeben oder {requested} zuerst nehmen."),
 "selfDeadlock":msg("Self-deadlock: {requested} is requested {requestedMode} while this process already holds it {heldMode}. Take the strongest mode up front or use the queued upgrade.","Selbst-Verklemmung: {requested} wird {requestedMode} angefordert, obwohl dieser Prozess es bereits {heldMode} haelt. Den stärksten Modus gleich nehmen oder das Upgrade der Warteschlange benutzen."),
 "cycle":msg("Deadlock detected: {cycle}. This request was chosen to fail so the others can proceed.","Verklemmung erkannt: {cycle}. Diese Anfrage wurde zum Scheitern gewaehlt, damit die anderen fortfahren können."),
 "blocker":msg(", held by process {pid} for {heldSeconds} s",", gehalten von Prozess {pid} seit {heldSeconds} s"),
 "observed":msg("[lease-order] {message}","[lease-order] {message}"),
}
defs={
 "Rule":rule,"Mode":mode,
 "Match":obj(["kind","value"],{"kind":{"enum":["prefix","exact"]},"value":{"type":"string","minLength":1,"maxLength":512}}),
 "Class":obj(["id","rank","scope","owner","title","match"],{
  "id":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},
  "rank":{"type":"integer","minimum":0,"maximum":1000},
  "scope":{"enum":["machine","checkout","project","crate","generation","path","session"]},
  "owner":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},
  "title":L,"match":{"type":"array","minItems":1,"items":{"$ref":"#/definitions/Match"}}}),
 "Allowed":obj(["from","to","reason"],{"from":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"to":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"reason":L}),
 "Policy":obj(["version","rules","holdAndWait","cycle","registry","fairness","gate","classes","allowed"],{
  "version":{"const":1},
  "rules":obj(["outOfOrder","holdAndWait","selfDeadlock","cycle"],{"outOfOrder":rule,"holdAndWait":rule,"selfDeadlock":rule,"cycle":rule}),
  "holdAndWait":obj(["graceMilliseconds","pollMilliseconds"],{"graceMilliseconds":ms(0),"pollMilliseconds":ms(10,60000)}),
  "cycle":obj(["intervalMilliseconds"],{"intervalMilliseconds":ms(10,60000)}),
  "registry":obj(["flushMilliseconds","maximumEntryBytes","maximumViolations"],{"flushMilliseconds":ms(0,60000),"maximumEntryBytes":{"type":"integer","minimum":1024,"maximum":1048576},"maximumViolations":{"type":"integer","minimum":1,"maximum":10000}}),
  "gate":obj(["releaseWhileWaiting","pollInitialMilliseconds","pollMaximumMilliseconds"],{"releaseWhileWaiting":{"type":"boolean"},"pollInitialMilliseconds":ms(1,60000),"pollMaximumMilliseconds":ms(1,60000)}),
  "fairness":obj(["queuedWithResourceLeases"],{"queuedWithResourceLeases":{"type":"boolean"}}),
  "classes":{"type":"array","minItems":1,"items":{"$ref":"#/definitions/Class"}},
  "allowed":{"type":"array","items":{"$ref":"#/definitions/Allowed"}}}),
 "Held":obj(["resource","mode","atMs"],{"resource":{"type":"string","minLength":1,"maxLength":4096},"mode":mode,"atMs":{"type":"integer","minimum":0}}),
 "Violation":obj(["version","kind","held","heldMode","heldRank","heldClass","requested","requestedMode","requestedRank","requestedClass","atMs","pid"],{
  "version":{"const":1},"kind":{"enum":["out-of-order","hold-and-wait","self-deadlock","cycle"]},
  "held":{"type":"string","maxLength":4096},"heldMode":mode,"heldRank":{"type":"integer","minimum":0},"heldClass":{"type":"string"},
  "requested":{"type":"string","maxLength":4096},"requestedMode":mode,"requestedRank":{"type":"integer","minimum":0},"requestedClass":{"type":"string"},
  "waitedMilliseconds":{"type":"integer","minimum":0},"blockerPid":{"type":"integer","minimum":1},"blockerHeldSeconds":{"type":"integer","minimum":0},
  "cycle":{"type":"array","items":{"type":"string"}},"atMs":{"type":"integer","minimum":0},"pid":{"type":"integer","minimum":1},"enforced":{"type":"boolean"}}),
 "Messages":{"const":messages},
 "Protocol":{"const":{"registryDirectoryEnvironment":"SEMIO_LEASE_REGISTRY_DIRECTORY","policyEnvironment":"SEMIO_LEASE_ORDER_POLICY","modeEnvironment":"SEMIO_LEASE_ORDER","inheritedEnvironment":"SEMIO_LEASE_INHERITED","maximumReportedResourceCharacters":120,"maximumInherited":64}},
}
schema={"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://semio.tech/framework/process/lease-order/v1","title":"Lease lock order, hold-and-wait and deadlock policy","definitions":defs}
open(os.path.join(OUT,"🔣️.json"),"w",encoding="utf8").write(json.dumps(schema,ensure_ascii=False,indent=2)+"\n")
print("schema written")
