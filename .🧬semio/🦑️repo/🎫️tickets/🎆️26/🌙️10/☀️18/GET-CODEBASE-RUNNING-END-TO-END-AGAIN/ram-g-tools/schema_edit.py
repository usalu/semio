"""🧬️ One-shot, idempotent edit of the host capacity schema: the measured-demand `Estimates` policy section, the estimate store/outcome definitions, the Protocol file names and the EN/DE `estimated` line."""
import json,sys
P="🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"
d=json.loads(open(P,encoding="utf8").read());D=d["definitions"]
integer=lambda lo,hi=None:{"type":"integer","minimum":lo,**({"maximum":hi} if hi is not None else {})}
D["Estimates"]={"type":"object","additionalProperties":False,"required":["halfLifeMilliseconds","quantileBasisPoints","marginBasisPoints","minimumSamples","maximumSamples","maximumClasses","minimumRatioBasisPoints","maximumRatioBasisPoints","minimumMegabytes","quickMilliseconds"],"properties":{"halfLifeMilliseconds":integer(1000),"quantileBasisPoints":integer(1,10000),"marginBasisPoints":integer(0,100000),"minimumSamples":integer(1,64),"maximumSamples":integer(1,256),"maximumClasses":integer(1,4096),"minimumRatioBasisPoints":integer(1,100000),"maximumRatioBasisPoints":integer(1,1000000),"minimumMegabytes":integer(1,1048576),"quickMilliseconds":integer(0)}}
D["Policy"]["properties"]["estimates"]={"$ref":"#/definitions/Estimates"}
if "estimates" not in D["Policy"]["required"]:D["Policy"]["required"].append("estimates")
D["EstimateSample"]={"type":"object","additionalProperties":False,"required":["atMs","ratioBasisPoints","activeMilliseconds"],"properties":{"atMs":integer(0),"ratioBasisPoints":integer(1,100000000),"activeMilliseconds":integer(0)}}
D["EstimateClass"]={"type":"object","additionalProperties":False,"required":["samples"],"properties":{"samples":{"type":"array","maxItems":256,"items":{"$ref":"#/definitions/EstimateSample"}}}}
D["EstimateStore"]={"type":"object","additionalProperties":False,"required":["version","classes"],"properties":{"version":{"const":1},"classes":{"type":"object","maxProperties":4096,"propertyNames":{"pattern":"^[^\\s]{1,200}$"},"additionalProperties":{"$ref":"#/definitions/EstimateClass"}}}}
D["Outcome"]={"type":"object","additionalProperties":False,"required":["operation","jobs","peakMegabytes","defaultMegabytes","activeMilliseconds","endedAtMs"],"properties":{"operation":D["Operation"]["properties"]["name"],"jobs":integer(1,1024),"peakMegabytes":integer(0,1048576),"defaultMegabytes":integer(1,1048576),"activeMilliseconds":integer(0),"endedAtMs":integer(0)}}
D["Estimate"]={"type":"object","additionalProperties":False,"required":["ratioBasisPoints","samples","quick"],"properties":{"ratioBasisPoints":integer(1,1000000),"samples":integer(0),"quick":{"type":"boolean"}}}
P0=D["Protocol"]["const"];P0["estimatesFile"]="estimates.json";P0["maximumEstimateBytes"]=1048576
M=D["Messages"]["const"]
M["en"]["estimated"]="[capacity] {operation}: sized at {percent}% of the default from {samples} earlier run(s) of its class"
M["de"]["estimated"]="[capacity] {operation}: auf {percent}% des Standardwerts bemessen, aus {samples} frueheren Laeufen seiner Klasse"
open(P,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
print("ok",list(D["Estimates"]["properties"]))
d=json.loads(open(P,encoding="utf8").read());Msg=d["definitions"]["Message"]
if "estimated" not in Msg["required"]:Msg["required"].append("estimated")
Msg["properties"]["estimated"]={"type":"string"}
open(P,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
d=json.loads(open(P,encoding="utf8").read())
d["definitions"]["Request"]["properties"]["envelopePerJobMegabytes"]={"type":"integer","minimum":0,"maximum":1048576}
open(P,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
d=json.loads(open(P,encoding="utf8").read())
pr=d["definitions"]["Policy"]["properties"]["pressure"]
for name in ("compressedWarnBasisPoints","compressedCriticalBasisPoints"):
    pr["properties"][name]={"type":"integer","minimum":0,"maximum":10000}
    if name not in pr["required"]:pr["required"].append(name)
open(P,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
d=json.loads(open(P,encoding="utf8").read())
es=d["definitions"]["Estimates"]
es["properties"]["maximumShareBasisPoints"]=integer(1,10000)
if "maximumShareBasisPoints" not in es["required"]:es["required"].append("maximumShareBasisPoints")
open(P,"w",encoding="utf8").write(json.dumps(d,indent=2,ensure_ascii=False)+"\n")
