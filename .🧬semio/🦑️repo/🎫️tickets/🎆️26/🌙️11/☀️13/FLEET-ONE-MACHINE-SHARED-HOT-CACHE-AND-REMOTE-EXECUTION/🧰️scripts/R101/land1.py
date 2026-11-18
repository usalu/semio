import hashlib, os, sys
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R101/"
BK = SC + "backup1/"
dry = "--dry" in sys.argv
def read(p): return open(p, encoding="utf8").read()
def once(s, old, new):
    assert s.count(old) == 1, ("anchor", s.count(old), old[:90])
    return s.replace(old, new)
OB = "👁️observation/🟨️.mjs"; SI = "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"; OD = "👁️observation/🟨️.d.mts"
PN = "🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/"
plan = {}
o = read(SC + "observation.new.mjs"); live = read(B + OB)
a = "(nestedProject?rows.every(row=>projectObservationHoldsV1(row,parent)):current(rows))) {"
live = once(live, a, "await projectRowsHoldAtEndV1(rows,result,nestedProject,parent)) {")
fn = o[o.index("/** 🏁️ Whether the rows one inference observed"):o.index("/** 🛎️ Brings the closure entries named")]
live = once(live, "/** 🛎️ Brings the closure entries named", fn + "/** 🛎️ Brings the closure entries named")
plan[OB] = live
s = read(SC + "si.new.mjs"); t = read(B + SI)
v = s[s.index("  async verify(rows, value) {"):s.index("  async settle(paths, value) {")]
plan[SI] = once(t, "  async settle(paths, value) {\n    prepareSourceAnalysesV1();", v + "  async settle(paths, value) {\n    prepareSourceAnalysesV1();")
d = read(B + OD)
d = once(d, "export function registerClosureProviderV1(provider:{answer(path:string):string;settle(paths:readonly string[],value?:unknown):Promise<void>}):void;", "export function registerClosureProviderV1(provider:{answer(path:string):string;settle(paths:readonly string[],value?:unknown):Promise<void>;verify?(rows:readonly {kind:string;path:string;fact:unknown}[],value?:unknown):Promise<boolean>}):void;\n/** 🏁️ Whether the rows one inference observed still hold when it ends (closure rows are verified by the provider). */\nexport function projectRowsHoldAtEndV1(rows:readonly {kind:string;path:string;fact:unknown}[],value:unknown,nested:boolean,parent:unknown):Promise<boolean>;")
plan[OD] = d
p = read(B + PN + "🟦️.ts"); po = read(SC + "pool.new.ts"); pa = read(SC + "pool.orig.ts")
x0 = pa.index("  const began=now();let ok=false;"); x1 = pa.index("  finally{running.delete(unit.name)")
y0 = po.index("  const began=now();let ok=false,held=claim;"); y1 = po.index("  finally{running.delete(unit.name)")
p = once(p, pa[x0:x1], po[y0:y1])
plan[PN + "🟦️.ts"] = p
sch = read(B + PN + "🧬️schema/🔣️.json")
plan[PN + "🧬️schema/🔣️.json"] = once(sch, '"heavyMegabytes": 2048', '"heavyMegabytes": 2048,\n        "noRecordRetries": 1')
if dry:
    for k in plan: print("ok", k, len(plan[k]))
    sys.exit(0)
os.makedirs(BK, exist_ok=True)
for k, v in plan.items():
    bp = BK + hashlib.md5(k.encode()).hexdigest()
    open(bp, "w", encoding="utf8").write(read(B + k)); open(bp + ".name", "w", encoding="utf8").write(k)
for k, v in plan.items(): open(B + k, "w", encoding="utf8").write(v)
print("landed", len(plan))
