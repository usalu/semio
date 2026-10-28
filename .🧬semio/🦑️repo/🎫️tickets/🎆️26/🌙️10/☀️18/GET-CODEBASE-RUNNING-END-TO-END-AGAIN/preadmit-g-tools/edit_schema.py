"""🧬️ One-shot, idempotent schema-first edit of the host capacity contract for preadmit-g, applied to a root (the overlay while staged): quiet forecast requests, grant upgrades, the policy knobs of the forecast and its EN/DE texts, plus the same policy fields in every complete policy (fixtures, build policy). Usage: python3 edit_schema.py <root>"""
import json,re,sys
root=sys.argv[1]
F="🧰️framework"
HC=f"{root}/{F}/🔨️modules/🏃️process/🏗️build/🌡️host-capacity"
LIB=f"{root}/{F}/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
P=f"{HC}/🧬️schema/🔣️.json"
d=json.loads(open(P,encoding="utf8").read());D=d["definitions"]
integer=lambda lo,hi=None:{"type":"integer","minimum":lo,**({"maximum":hi} if hi is not None else {})}
def put(definition,name,spec,required=False):
    D[definition]["properties"][name]=spec
    if required and name not in D[definition]["required"]:D[definition]["required"].append(name)
c=D["Protocol"]["const"]
c["upgradeSuffix"]=".upgrade";c["upgradePollMilliseconds"]=100;c["maximumUpgradeBytes"]=1024
put("Request","quiet",{"type":"boolean"})
put("Waiter","upgrade",{"type":"boolean"})
put("Grant","quiet",{"type":"boolean"});put("Grant","arrivedAtMs",integer(0));put("Grant","upgradedAtMs",integer(0))
put("Operation","forecast",{"type":"boolean"})
memory=D["Policy"]["properties"]["memory"];memory["properties"]["maximumQuiet"]=integer(1,64)
if "maximumQuiet" not in memory["required"]:memory["required"].append("maximumQuiet")
for name,spec in {"quietReferenceMegabytes":integer(1,1048576),"quietShareBasisPoints":integer(1,10000),"quietMinimumSamples":integer(1,64),"quietMaximumMegabytes":integer(1,1048576)}.items():
    put("Estimates",name,spec,True)
text={
 "quiet":{"en":"[capacity] {operation}: forecast quiet from {samples} earlier run(s) of its class ({percent}% of them executed nothing): {reserved} MB reserved now, the full {full} MB only if work has to run","de":"[capacity] {operation}: ruhig prognostiziert aus {samples} frueheren Laeufen seiner Klasse ({percent}% fuehrten nichts aus): jetzt {reserved} MB reserviert, die vollen {full} MB nur, wenn Arbeit laufen muss"},
 "upgrading":{"en":"[capacity] {operation}: work has to run, waiting {seconds}s to grow the reservation from {from} MB to {to} MB: {reason} ({spare} MB spare above the {floor} MB floor; {running} running, {ahead} ahead)","de":"[capacity] {operation}: es muss Arbeit laufen, wartet seit {seconds} s darauf, die Reservierung von {from} MB auf {to} MB zu vergroessern: {reason} ({spare} MB frei oberhalb der Untergrenze von {floor} MB; {running} laufen, {ahead} davor)"},
 "upgraded":{"en":"[capacity] {operation}: reservation grown from {from} MB to {to} MB after {seconds}s ({jobs} job(s), {available} MB available, {floor} MB floor)","de":"[capacity] {operation}: Reservierung nach {seconds} s von {from} MB auf {to} MB vergroessert ({jobs} Job(s), {available} MB verfuegbar, Untergrenze {floor} MB)"},
 "grantQuiet":{"en":"[capacity]   quiet {id} {operation}: forecast to execute nothing, {reserved} MB reserved instead of its full request, {observed} MB observed, pid {pid}, since {seconds}s","de":"[capacity]   ruhig {id} {operation}: soll nichts ausfuehren, {reserved} MB statt der vollen Anfrage reserviert, {observed} MB beobachtet, pid {pid}, seit {seconds} s"},
 "waiterUpgrade":{"en":"[capacity]   growing {id} {operation}: wants {jobs} job(s), {need} MB, since {seconds}s, pid {pid}","de":"[capacity]   waechst {id} {operation}: moechte {jobs} Job(s), {need} MB, seit {seconds} s, pid {pid}"},
 "upgradeWithdrawn":{"en":"[capacity] {operation}: the request to grow the reservation was withdrawn after {seconds}s; the run keeps its {reserved} MB","de":"[capacity] {operation}: die Anfrage, die Reservierung zu vergroessern, wurde nach {seconds} s zurueckgezogen; der Lauf behaelt seine {reserved} MB"},
}
message=D["Message"]
for name,rows in text.items():
    for language in ("en","de"):D["Messages"]["const"][language][name]=rows[language]
    message["properties"][name]={"type":"string"}
    if name not in message["required"]:message["required"].append(name)
errors=D["Message"]["properties"]["errors"]
errors["properties"]["quietUpgradeRequired"]={"type":"string"}
if "quietUpgradeRequired" not in errors["required"]:errors["required"].append("quietUpgradeRequired")
D["Messages"]["const"]["en"]["errors"]["quietUpgradeRequired"]="a quiet request needs the full request it may grow into, under the same id"
D["Messages"]["const"]["de"]["errors"]["quietUpgradeRequired"]="eine ruhige Anfrage braucht die volle Anfrage, in die sie wachsen darf, unter derselben Kennung"
open(P,"w",encoding="utf8").write(json.dumps(d,ensure_ascii=False,indent=2)+"\n")
def regex(path,pattern,template,marker,count=1):
    t=open(path,encoding="utf8").read()
    if marker in t:return "kept"
    fresh,n=re.subn(pattern,template,t)
    if n!=count:raise SystemExit(f"{path}: {n} matches of {pattern}, wanted {count}")
    open(path,"w",encoding="utf8").write(fresh);return "patched"
ESTIMATE=',"quietReferenceMegabytes":2048,"quietShareBasisPoints":8000,"quietMinimumSamples":2,"quietMaximumMegabytes":4096'
decision=f"{HC}/🧪️tests/🧫️fixtures/🔣️.json"
print(regex(decision,r'("maximumRamping":\d+)',r'\1,"maximumQuiet":4','"maximumQuiet"'))
for rel in (decision,f"{HC}/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json",f"{HC}/🧪️tests/🪜️phases/🧫️fixtures/🔣️.json"):print(regex(rel,r'("maximumRebuildSlots":\d+)',r'\1'+ESTIMATE,'"quietReferenceMegabytes"'))
oracle=f"{HC}/🧪️tests/📐️estimates/🐍️oracle/🐍️.py"
print(regex(oracle,r'("maximumRebuildSlots": \d+)',r'\1, "quietReferenceMegabytes": 2048, "quietShareBasisPoints": 8000, "quietMinimumSamples": 2, "quietMaximumMegabytes": 4096','"quietReferenceMegabytes"'))
policy=f"{LIB}/🔣️policy.json"
print(regex(policy,r'(\n(\s+)"maximumRamping": \d+)(\n)',lambda m:m.group(1)+f',\n{m.group(2)}"maximumQuiet": 4'+m.group(3),'"maximumQuiet"'))
print(regex(policy,r'(\n(\s+)"maximumRebuildSlots": \d+)(\n)',lambda m:m.group(1)+"".join(f',\n{m.group(2)}"{k}": {v}' for k,v in {"quietReferenceMegabytes":2048,"quietShareBasisPoints":8000,"quietMinimumSamples":2,"quietMaximumMegabytes":4096}.items())+m.group(3),'"quietReferenceMegabytes"'))
t=json.loads(open(policy,encoding="utf8").read())
raw=open(policy,encoding="utf8").read()
for name in ("run","run-many","affected"):
    pattern=re.compile(r'(\{\s*"name": "'+re.escape(name)+r'",[^{}]*?"priority": "(?:batch|interactive)")(\s*\})')
    if re.search(pattern,raw) and '"forecast"' not in re.search(pattern,raw).group(0):raw=pattern.sub(lambda m:m.group(1)+',\n        "forecast": true'+m.group(2),raw,count=1)
open(policy,"w",encoding="utf8").write(raw)
chk=json.loads(raw);print([ (r["name"],r.get("forecast")) for r in chk["hostCapacity"]["operations"]])
print("ok")
