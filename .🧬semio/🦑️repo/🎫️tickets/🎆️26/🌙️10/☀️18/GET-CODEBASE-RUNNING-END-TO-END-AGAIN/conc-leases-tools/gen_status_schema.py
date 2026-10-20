import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
OUT=os.path.join(HERE,"staged","🧰️framework","🔨️modules","🏃️process","🔒️leases","📋️status","🧬️schema")
os.makedirs(OUT,exist_ok=True)
mode={"enum":["shared","exclusive"]}
def obj(required,props):return {"type":"object","additionalProperties":False,"required":required,"properties":props}
i=lambda lo=0:{"type":"integer","minimum":lo}
s={"type":"string"}
msg=lambda en,de:{"en":en,"de":de}
messages={
 "header":msg("Leases in {directory}: {holders} held, {waiters} waiting, {cycles} deadlock cycles, {violations} recent order violations.","Leases in {directory}: {holders} gehalten, {waiters} wartend, {cycles} Verklemmungszyklen, {violations} jüngste Reihenfolgeverletzungen."),
 "holder":msg("HELD    {mode} {resource}  process {pid}  {seconds} s  [{class}]{command}","GEHALTEN {mode} {resource}  Prozess {pid}  {seconds} s  [{class}]{command}"),
 "waiter":msg("WAITING {mode} {resource}  process {pid}  {seconds} s{position}{behind}{command}","WARTET   {mode} {resource}  Prozess {pid}  {seconds} s{position}{behind}{command}"),
 "position":msg("  position {position} of {queued}","  Position {position} von {queued}"),
 "behind":msg("  waiting for {blockers}","  wartet auf {blockers}"),
 "cycle":msg("DEADLOCK {members}","VERKLEMMUNG {members}"),
 "violation":msg("ORDER   {message}","REIHENFOLGE {message}"),
 "admissionHeader":msg("Machine admission: {spare} MB spare above the {floor} MB floor, {grants} granted, {waiters} waiting.","Rechner-Zulassung: {spare} MB frei oberhalb der Untergrenze von {floor} MB, {grants} zugesagt, {waiters} wartend."),
 "admissionGrant":msg("GRANTED {operation}  process {pid}  {jobs} jobs  {reserved} MB reserved, {observed} MB used  {seconds} s","ZUGESAGT {operation}  Prozess {pid}  {jobs} Jobs  {reserved} MB reserviert, {observed} MB belegt  {seconds} s"),
 "admissionWaiter":msg("QUEUED  {operation}  process {pid}  needs {need} MB  {seconds} s  {reason}","WARTET   {operation}  Prozess {pid}  braucht {need} MB  {seconds} s  {reason}"),
 "empty":msg("No process holds or awaits a lease in {directory}.","Kein Prozess hält oder erwartet ein Lease in {directory}."),
 "unnamed":msg("(unnamed {hash})","(unbenannt {hash})"),
 "directories":msg("Lease directories with registered processes:","Lease-Verzeichnisse mit registrierten Prozessen:"),
}
defs={
 "Mode":mode,
 "Holder":obj(["resource","hash","mode","pid","task","sinceMs","heldSeconds","class","rank","source"],{"resource":s,"hash":{"type":"string","pattern":"^[a-f0-9]{64}$"},"mode":mode,"pid":i(1),"task":i(),"sinceMs":i(),"heldSeconds":i(),"class":s,"rank":i(),"source":{"enum":["registry","queue"]},"command":s}),
 "Waiter":obj(["resource","hash","mode","pid","task","sinceMs","waitedSeconds","class","rank","source","blockers"],{"resource":s,"hash":{"type":"string","pattern":"^[a-f0-9]{64}$"},"mode":mode,"pid":i(1),"task":i(),"sinceMs":i(),"waitedSeconds":i(),"class":s,"rank":i(),"source":{"enum":["registry","queue"]},"position":i(1),"queued":i(1),"blockers":{"type":"array","items":s},"command":s}),
 "Edge":obj(["from","to","resource"],{"from":s,"to":s,"resource":s}),
 "Cycle":obj(["members","victim"],{"members":{"type":"array","minItems":1,"items":s},"victim":s}),
 "AdmissionGrant":obj(["id","operation","pid","jobs","reservedMegabytes","heldSeconds"],{"id":s,"operation":s,"pid":i(1),"jobs":i(),"reservedMegabytes":i(),"observedMegabytes":{"type":["integer","null"]},"heldSeconds":i()}),
 "AdmissionWaiter":obj(["id","operation","pid","needMegabytes","waitedSeconds","reason","reasonText"],{"id":s,"operation":s,"pid":i(1),"needMegabytes":i(),"waitedSeconds":i(),"reason":s,"reasonText":s}),
 "Admission":obj(["floorMegabytes","headroomMegabytes","grants","waiters"],{"floorMegabytes":i(),"headroomMegabytes":{"type":"integer"},"grants":{"type":"array","items":{"$ref":"#/definitions/AdmissionGrant"}},"waiters":{"type":"array","items":{"$ref":"#/definitions/AdmissionWaiter"}}}),
 "Report":obj(["version","directory","atMs","holders","waiters","waitFor","cycles","violations"],{"admission":{"$ref":"#/definitions/Admission"},"version":{"const":1},"directory":s,"atMs":i(),"holders":{"type":"array","items":{"$ref":"#/definitions/Holder"}},"waiters":{"type":"array","items":{"$ref":"#/definitions/Waiter"}},"waitFor":{"type":"array","items":{"$ref":"#/definitions/Edge"}},"cycles":{"type":"array","items":{"$ref":"#/definitions/Cycle"}},"violations":{"type":"array","items":{"type":"object"}}}),
 "Messages":{"const":messages},
 "Protocol":{"const":{"defaultViolations":10,"maximumQueueEntries":2048}},
}
schema={"$schema":"http://json-schema.org/draft-07/schema#","$id":"https://semio.tech/framework/process/lease-status/v1","title":"Lease status: holders, waiters, wait-for graph and deadlock cycles","definitions":defs}
open(os.path.join(OUT,"🔣️.json"),"w",encoding="utf8").write(json.dumps(schema,ensure_ascii=False,indent=2)+"\n")
print("ok")
