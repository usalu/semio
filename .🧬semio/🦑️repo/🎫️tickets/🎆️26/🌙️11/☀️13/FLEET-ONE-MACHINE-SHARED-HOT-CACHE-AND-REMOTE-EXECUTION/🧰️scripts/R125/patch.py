import json,re,sys
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R125/"
def sub(path,a,b,count=1):
    t=open(path,encoding="utf8").read()
    if a not in t: sys.exit("anchor missing: "+a[:70])
    open(path,"w",encoding="utf8").write(t.replace(a,b,count))
d=S+"stage/decision.ts"
sub(d,'light=(request.operation==="graph"','ownChild=elastic&&request.holder!==undefined&&active.some(grant=>grant.id===request.holder),light=(request.operation==="graph"')
sub(d,'if(fits(candidate)&&(avoids||candidate*request.cpuPerJob<=freeLoad)){jobs=candidate;break;}','if(fits(candidate)&&(avoids||ownChild||candidate*request.cpuPerJob<=freeLoad)){jobs=candidate;break;}')
sub(d,'"blocker"|"blockerHost",values','"blocker"|"blockerHost"|"waitingShort",values')
g=S+"stage/gate.ts"
sub(g,'''/** ⏳️ The progress line of a waiting request, naming what it stands behind (`hostBlockersV1`). */
function waitingLine(''','''/** ⏳️ The progress line of a waiting request: the arithmetic of the room (available, floor, what is reserved but not yet resident, what was admitted ahead), how many megabytes it is short and who holds them (`hostBlockersV1`).
 * @see ../🧪️tests/🧾️blockers/🟦️.ts */
export function hostWaitingLineV1(''')
sub(g,'''const cap=Math.max(0,host.totalMegabytes-evaluation.floorMegabytes),position=evaluation.decisions.findIndex(row=>row.id===request.id),mine=evaluation.decisions[position]!,before=evaluation.decisions.slice(0,position),spare=evaluation.headroomMegabytes-before.reduce((sum,row)=>sum+row.reservedMegabytes,0);
 return formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:describeHostReasonV1(language,mine.reason as Exclude<HostReasonV1,"fits"|"reduced"|"override"|"shrunk"|"lent">),need:Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,cap),spare,floor:evaluation.floorMegabytes,load''','''const cap=Math.max(0,host.totalMegabytes-evaluation.floorMegabytes),position=evaluation.decisions.findIndex(row=>row.id===request.id),mine=evaluation.decisions[position]!,before=evaluation.decisions.slice(0,position),taken=before.reduce((sum,row)=>sum+row.reservedMegabytes,0),spare=evaluation.headroomMegabytes-taken,need=Math.min(request.fixedMegabytes+request.minimumJobs*request.perJobMegabytes,cap);
 return formatHostCapacityV1(language,"waiting",{seconds,operation:request.operation,reason:describeHostReasonV1(language,mine.reason as Exclude<HostReasonV1,"fits"|"reduced"|"override"|"shrunk"|"lent">),need,spare,available:host.availableMegabytes,held:evaluation.unrealizedMegabytes,taken,short:mine.reason==="memory"&&need>spare?formatHostCapacityV1(language,"waitingShort",{megabytes:need-spare}):"",floor:evaluation.floorMegabytes,load''')
sub(g,'write(waitingLine(language,','write(hostWaitingLineV1(language,')
p=S+"stage/schema.json"
j=json.load(open(p,encoding="utf8"))
m=j["definitions"]["Messages"]["const"]
m["en"]["waiting"]="[capacity] waiting {seconds}s for {operation}: {reason} (need {need} MB; {spare} MB spare = {available} MB available - {floor} MB floor - {held} MB reserved but not yet resident - {taken} MB admitted ahead{short}; load {load}; {running} running, {ahead} ahead); behind {behind}"
m["de"]["waiting"]="[capacity] wartet seit {seconds} s auf {operation}: {reason} (benötigt {need} MB; {spare} MB frei = {available} MB verfügbar - {floor} MB Untergrenze - {held} MB reserviert, aber noch nicht belegt - {taken} MB davor zugelassen{short}; Last {load}; {running} laufen, {ahead} davor); hinter {behind}"
m["en"]["waitingShort"]=", short by {megabytes} MB"
m["de"]["waitingShort"]=", es fehlen {megabytes} MB"
msg=j["definitions"]["Message"]
msg["required"].append("waitingShort");msg["properties"]["waitingShort"]={"type":"string"}
open(p,"w",encoding="utf8").write(json.dumps(j,ensure_ascii=False,indent=2)+"\n")
q=S+"stage/policy.json"
pj=json.load(open(q,encoding="utf8"))
hc=pj["hostCapacity"]
for op in hc["operations"]:
    if op["name"] in("show","graph"):op["fixedMegabytes"]=3072
hc["classCeilings"].append({"name":"read-parent","pattern":"^(?:show|graph)(?::|$)","megabytes":4096})
open(q,"w",encoding="utf8").write(json.dumps(pj,ensure_ascii=False,indent=2)+"\n")
print("ok")
