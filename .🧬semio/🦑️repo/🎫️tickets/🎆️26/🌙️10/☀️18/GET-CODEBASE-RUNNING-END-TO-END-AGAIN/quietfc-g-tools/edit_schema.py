"""🧬️ quietfc-g: one-shot, idempotent schema-first edit of the host capacity contract, applied to a root (the overlay while staged): the follow-up window of the quiet forecast, quiet grants that ignore the soft stops, declared envelopes for multi-worker targets, their EN/DE texts, plus the same policy fields in every complete policy (fixtures with the new behaviour OFF so the stored vectors stay valid, the build policy with it ON). Usage: python3 edit_schema.py <root>"""
import json,re,sys
root=sys.argv[1]
F="🧰️framework"
HC=f"{root}/{F}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB=f"{root}/{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
P=f"{HC}/🧬️schema/🔣️.json"
d=json.loads(open(P,encoding="utf8").read());D=d["definitions"]
integer=lambda lo,hi=None:{"type":"integer","minimum":lo,**({"maximum":hi} if hi is not None else {})}
def put(owner,name,spec):
    owner["properties"][name]=spec
    if name not in owner["required"]:owner["required"].append(name)
put(D["Policy"]["properties"]["memory"],"quietSoftStopGrants",integer(0,64))
put(D["Estimates"],"quietFollowUpMilliseconds",integer(0))
D["Envelope"]={"type":"object","additionalProperties":False,"required":["name","pattern","cpu","megabytes"],"properties":{"name":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"pattern":{"type":"string","minLength":1,"maxLength":200},"cpu":integer(1,1024),"megabytes":integer(1,1048576)},"description":"🧰️ The resources a target holds at its peak when it runs several workers of its own (end-to-end suites): any target whose name matches `pattern` is sized at least at `cpu` cores and `megabytes` per job, whatever kind the worker pool classifies it as."}
put(D["Policy"],"envelopes",{"type":"array","maxItems":64,"items":{"$ref":"#/definitions/Envelope"}})
text={"quietFollowUp":{"en":"[capacity] {operation}: forecast quiet because the earlier run of its class finished {seconds}s ago and left its results cached: {reserved} MB reserved now, the full {full} MB only if work has to run","de":"[capacity] {operation}: ruhig prognostiziert, weil der frühere Lauf seiner Klasse vor {seconds} s endete und seine Ergebnisse zwischengespeichert hinterließ: jetzt {reserved} MB reserviert, die vollen {full} MB nur, wenn Arbeit laufen muss"}}
message=D["Message"]
for name,rows in text.items():
    for language in ("en","de"):D["Messages"]["const"][language][name]=rows[language]
    message["properties"][name]={"type":"string"}
    if name not in message["required"]:message["required"].append(name)
D["Ceiling"]={"type":"object","additionalProperties":False,"required":["name","pattern","megabytes"],"properties":{"name":{"type":"string","pattern":"^[a-z][a-z0-9-]*$"},"pattern":{"type":"string","minLength":1,"maxLength":200},"megabytes":integer(1,1048576)},"description":"The most a class of runs may ever ask for, whatever its learned ratio says: a declared bound on the class key (`operation:kind:project:target`) that holds until the class has measured enough runs to be capped by its own peaks."}
put(D["Policy"],"classCeilings",{"type":"array","maxItems":64,"items":{"$ref":"#/definitions/Ceiling"}})
if "quiet-limit" not in D["Reason"]["enum"]:D["Reason"]["enum"].append("quiet-limit")
reasons=D["Message"]["properties"]["reasons"]
reasons["properties"]["quiet-limit"]={"type":"string"}
if "quiet-limit" not in reasons["required"]:reasons["required"].append("quiet-limit")
D["Messages"]["const"]["en"]["reasons"]["quiet-limit"]="enough quiet reservations are already held"
D["Messages"]["const"]["de"]["reasons"]["quiet-limit"]="es werden schon genug ruhige Reservierungen gehalten"
D["EstimateSample"]["properties"]["peakMegabytes"]=integer(1,1048576)
c=D["Protocol"]["const"]
c["coresEnvironment"]="SEMIO_HOST_CAPACITY_CORES"
open(P,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
def regex(path,pattern,template,marker,count=1):
    t=open(path,encoding="utf8").read()
    if marker in t:return "kept"
    fresh,n=re.subn(pattern,template,t)
    if n!=count:raise SystemExit(f"{path}: {n} matches of {pattern}, wanted {count}")
    open(path,"w",encoding="utf8").write(fresh);return "patched"
OFF_ESTIMATE=',"quietFollowUpMilliseconds":0'
decision=f"{HC}/🧪️tests/🧫️fixtures/🔣️.json"
print(regex(decision,r'("maximumQuiet":\d+)',r'\1,"quietSoftStopGrants":0','"quietSoftStopGrants"'))
print(regex(decision,r'("quietMaximumMegabytes":\d+)',r'\1'+OFF_ESTIMATE,'"quietFollowUpMilliseconds"'))
print(regex(decision,r'(,"estimates":\{[^{}]*\})\}',r'\1,"envelopes":[],"classCeilings":[]}','"envelopes"'))
for rel in (f"{HC}/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json",f"{HC}/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json"):print(regex(rel,r'("quietMaximumMegabytes":\d+)',r'\1'+OFF_ESTIMATE,'"quietFollowUpMilliseconds"'))
oracle=f"{HC}/🧪️tests/📐️estimates/🐍️oracle/🐍️.py"
print(regex(oracle,r'("quietMaximumMegabytes": \d+)',r'\1, "quietFollowUpMilliseconds": 0','"quietFollowUpMilliseconds"'))
policy=f"{LIB}/🔣️policy.json"
print(regex(policy,r'(\n(\s+)"maximumQuiet": \d+)(\n)',lambda m:m.group(1)+f',\n{m.group(2)}"quietSoftStopGrants": 1'+m.group(3),'"quietSoftStopGrants"'))
print(regex(policy,r'(\n(\s+)"quietMaximumMegabytes": \d+)(\n)',lambda m:m.group(1)+f',\n{m.group(2)}"quietFollowUpMilliseconds": 1800000'+m.group(3),'"quietFollowUpMilliseconds"'))
raw=open(policy,encoding="utf8").read()
if '"envelopes"' not in raw:
    m=re.search(r'(\n(\s+)"estimates": \{)',raw)
    raw=raw[:m.start()]+f'\n{m.group(2)}"envelopes": [\n{m.group(2)}  {{\n{m.group(2)}    "name": "end-to-end",\n{m.group(2)}    "pattern": "^test-e2e(?:-|$)",\n{m.group(2)}    "cpu": 4,\n{m.group(2)}    "megabytes": 6144\n{m.group(2)}  }}\n{m.group(2)}],'+raw[m.start():]
    open(policy,"w",encoding="utf8").write(raw)
raw=open(policy,encoding="utf8").read()
if '"classCeilings"' not in raw:
    m=re.search(r'(\n(\s+)"estimates": \{)',raw)
    raw=raw[:m.start()]+f'\n{m.group(2)}"classCeilings": [\n{m.group(2)}  {{\n{m.group(2)}    "name": "course-build",\n{m.group(2)}    "pattern": "^run:build:@teaching/architecture-course:build$",\n{m.group(2)}    "megabytes": 12288\n{m.group(2)}  }}\n{m.group(2)}],'+raw[m.start():]
    open(policy,"w",encoding="utf8").write(raw)
chk=json.loads(open(policy,encoding="utf8").read())["hostCapacity"];print(chk["memory"],chk["envelopes"],chk["estimates"]["quietFollowUpMilliseconds"])
print("ok")
