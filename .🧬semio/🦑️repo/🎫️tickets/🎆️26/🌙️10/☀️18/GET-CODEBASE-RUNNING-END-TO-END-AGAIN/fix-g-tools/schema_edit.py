"""🧬️ One-shot, idempotent edit of the host capacity schema for fix-g item 1: per-class time to peak (ramp), the ramp slots and the outcome status."""
import json
P="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"
d=json.loads(open(P,encoding="utf8").read());D=d["definitions"]
integer=lambda lo,hi=None:{"type":"integer","minimum":lo,**({"maximum":hi} if hi is not None else {})}
def add(definition,name,spec,required=False):
    D[definition]["properties"][name]=spec
    if required and name not in D[definition]["required"]:D[definition]["required"].append(name)
add("Policy","maximumRamping",integer(1,64),False)
D["Policy"]  # memory lives one level down
mem=D["Policy"]["properties"]["memory"]
mem["properties"]["maximumRamping"]=integer(1,64)
if "maximumRamping" not in mem["required"]:mem["required"].append("maximumRamping")
del D["Policy"]["properties"]["maximumRamping"]
est=D["Estimates"]
for name in ("defaultRampMilliseconds","maximumRampMilliseconds"):
    est["properties"][name]=integer(1000)
    if name not in est["required"]:est["required"].append(name)
for definition in ("Request","Grant"):
    add(definition,"rampMilliseconds",integer(0))
    add(definition,"rampSlot",{"type":"boolean"})
if "ramp" not in D["Reason"]["enum"]:D["Reason"]["enum"].append("ramp")
add("EstimateSample","peakAtMilliseconds",integer(0))
add("Outcome","peakAtMilliseconds",integer(0),True)
add("Estimate","rampMilliseconds",integer(1000),True)
msg=D["Messages"]["const"]
text={"en":"too many learned reservations are still growing to their peak","de":"zu viele gelernte Reservierungen wachsen noch auf ihren Spitzenwert an"}
for language in ("en","de"):msg[language]["reasons"]["ramp"]=text[language]
reasons=D["Message"]["properties"]["reasons"]
reasons["properties"]["ramp"]={"type":"string"}
if "ramp" not in reasons["required"]:reasons["required"].append("ramp")
open(P,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
print("ok")
