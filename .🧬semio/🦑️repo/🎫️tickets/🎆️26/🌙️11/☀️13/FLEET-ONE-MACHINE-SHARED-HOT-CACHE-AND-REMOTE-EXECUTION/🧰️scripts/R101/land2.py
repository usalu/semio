import hashlib, os, sys, subprocess
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/".replace("2101","9f6d") if False else "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/"
HERE = os.path.dirname(os.path.abspath(__file__)) + "/"
BK = SC + "backup2/"
dry = "--dry" in sys.argv
def read(p): return open(p, encoding="utf8").read()
def once(s, old, new):
    assert s.count(old) == 1, ("anchor", s.count(old), old[:100])
    return s.replace(old, new)
PN = "🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/"
OB = "👁️observation/🟨️.mjs"; OD = "👁️observation/🟨️.d.mts"
plan = {}
o = read(B + OB)
i0 = o.index("export async function projectRowsHoldAtEndV1("); i1 = o.index("/** 🛎️ Brings the closure entries named")
o = o[:i0] + '''export const projectRowsHoldAtEndV1=(rows,value,nested,parent)=>projectRowsHoldV1(rows,value,rest=>nested?rest.every(row=>projectObservationHoldsV1(row,parent)):current(rest));
/** 🏁️ The same verdict for any probe of the rows that are not closure rows: `holdsRest` probes them, the closure rows are verified by the provider one closure at a time (see {@link projectRowsHoldAtEndV1}); a stored record found by a look is judged the same way, so a project that uses more closure entries than the cache holds is not refused for the entries it evicted itself. */
export async function projectRowsHoldV1(rows,value,holdsRest){
 const verify=closureProviderV1?.verify,closure=typeof verify==="function"?rows.filter(row=>row.kind==="closure"):[],others=closure.length===0?rows:rows.filter(row=>row.kind!=="closure");
 if(!holdsRest(others))return false;
 return closure.length===0||await verify.call(closureProviderV1,closure,value);
}
''' + o[i1:]
o = once(o, 'if(lookupLapV1("hold",()=>nestedProject?sharedObservationsHoldV1(header.observations,parent):current(header.observations)))candidate=', 'if(await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(header.observations,record.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest))))candidate=')
o = once(o, '&&lookupLapV1("hold",()=>nestedProject?sharedObservationsHoldV1(candidate.header.observations,parent):current(candidate.header.observations))){const{header,value,headerDigest}=candidate;', '&&await lookupLapAsyncV1("hold",()=>projectRowsHoldV1(candidate.header.observations,candidate.value,rest=>nestedProject?sharedObservationsHoldV1(rest,parent):current(rest)))){const{header,value,headerDigest}=candidate;')
plan[OB] = o
d = read(B + OD)
plan[OD] = once(d, "export function projectRowsHoldAtEndV1(", "export function projectRowsHoldV1(rows:readonly {kind:string;path:string;fact:unknown}[],value:unknown,holdsRest:(rest:readonly {kind:string;path:string;fact:unknown}[])=>boolean):Promise<boolean>;\nexport function projectRowsHoldAtEndV1(")
p = read(B + PN + "🟦️.ts"); p2 = read(SC + "pool2.ts"); p1 = read(SC + "pool.new.ts")
def seg(s, a, b): return s[s.index(a):s.index(b)]
a1 = 'const byCores=Math.max(0,host.cores-foreground.cpu-reserved)'
p = once(p, 'load:number|null;swapOutMegabytesPerSecond', 'load:number|null;idleCores?:number|null;swapOutMegabytesPerSecond')
old = seg(p, " const byCores=Math.max(0,host.cores-foreground.cpu-reserved)", " let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren);") + " let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren);"
new = seg(p2, " const minimum=Math.max(1,Math.ceil", " let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren);") + " let children=Math.min(byCores,byLoad,byMemory,policy.maximumChildren);"
p = once(p, old, new)
dold = seg(p, "otherwise every core and every free megabyte", "A pure function.\n * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */\nexport function ownerPoolTargetV1")
dnew = seg(p2, "otherwise every core and every free megabyte", "A pure function.\n * @see ./🧪️tests/🧫️fixtures/🐍️oracle.py */\nexport function ownerPoolTargetV1")
p = once(p, dold, dnew)
idle = seg(p2, "/** 🌡️ The idle cores between two samples", "/** 📊️ What one pull run did. */")
p = once(p, "/** 📊️ What one pull run did. */", idle + "/** 📊️ What one pull run did. */")
plan[PN + "🟦️.ts"] = p
plan[PN + "🧬️schema/🔣️.json"] = once(read(B + PN + "🧬️schema/🔣️.json"), '"noRecordRetries": 1', '"noRecordRetries": 1,\n        "minimumShareBasisPoints": 2500')
sp = "🌐️remote-execution/🛰️sidecar/🏊️pool/🟦️.ts"; s = read(B + sp); s2 = read(SC + "sidecar2.ts")
s = once(s, 'import{setPriority}from"node:os";', 'import{cpus,setPriority}from"node:os";')
s = once(s, 'import{OWNER_POOL_POLICY_V1 as P,ownerPoolTargetV1,', 'import{OWNER_POOL_POLICY_V1 as P,ownerPoolIdleCoresV1,ownerPoolTargetV1,')
c0 = " const capacity=async("; c1 = "\n try{\n  await(runOwnerPoolV1"
s = once(s, s[s.index(c0):s.index(c1)], s2[s2.index("let sampled=cpus();"):s2.index(c1)])
plan[sp] = s
orc = read(B + PN + "🧪️tests/🧫️fixtures/🐍️oracle.py")
orc = once(orc, '''    by_cores = max(0, h["cores"] - fg["cpu"] - reserved)
    external = 0 if h["load"] is None else max(0, h["load"] - case["children"])
    by_load = math.floor(h["cores"] * p["loadCeilingBasisPointsPerCore"] / 10000 - external)''', '''    minimum = max(1, math.ceil(h["cores"] * p["minimumShareBasisPoints"] / 10000))
    idle = h.get("idleCores")
    booked = max(0, h["cores"] - fg["cpu"] - reserved)
    by_cores = booked if fg["waiters"] > 0 else max(booked, minimum)
    external = 0 if h["load"] is None else max(0, h["load"] - case["children"])
    by_load = max(minimum, math.floor(h["cores"] * p["loadCeilingBasisPointsPerCore"] / 10000 - external) if idle is None else case["children"] + math.floor(idle))''')
plan[PN + "🧪️tests/🧫️fixtures/🐍️oracle.py"] = orc
import json
fx = json.load(open(B + PN + "🧪️tests/🧫️fixtures/🔣️.json", encoding="utf8")); nf = json.load(open(HERE + "fixture2.json", encoding="utf8"))
assert fx["target"] == nf["target"][:len(fx["target"])] or True
base = [t for t in fx["target"]]
assert len(base) == 22 or len(base) == len(nf["target"]) - 3, len(base)
plan[PN + "🧪️tests/🧫️fixtures/🔣️.json"] = read(HERE + "fixture2.json")
t = read(B + PN + "🧪️tests/🟦️.ts")
t = once(t, "for(let booked=0;booked<=10;booked++){", "for(let booked=0;booked<=10;booked++){")
t = once(t, "expect(target).toBe(Math.max(1,10-booked));", "expect(target).toBe(Math.max(Math.ceil(10*P.minimumShareBasisPoints/10000),10-booked));")
t = once(t, 'expect(loud.children).toBe(1);expect(loud.reason).toBe("yielding");', 'expect(loud.children).toBe(Math.ceil(10*P.minimumShareBasisPoints/10000));expect(loud.reason).toBe("yielding");')
plan[PN + "🧪️tests/🟦️.ts"] = t
newfiles = {PN + "🧪️tests/🎚️lanes/🟦️.ts": read(HERE + "lanes.test.ts"), "👁️observation/🧪️tests/🔎️lookup-holds/🟦️.ts": read(HERE + "lookup.test.ts")}
if dry:
    for k in plan: print("ok", k, len(plan[k]))
    sys.exit(0)
os.makedirs(BK, exist_ok=True)
for k in plan:
    bp = BK + hashlib.md5(k.encode()).hexdigest()
    open(bp, "w", encoding="utf8").write(read(B + k)); open(bp + ".name", "w", encoding="utf8").write(k)
for k, v in plan.items(): open(B + k, "w", encoding="utf8").write(v)
for k, v in newfiles.items():
    os.makedirs(os.path.dirname(B + k), exist_ok=True); open(B + k, "w", encoding="utf8").write(v)
print("landed", len(plan))
