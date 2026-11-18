import hashlib, os, shutil, sys
R = "/Users/ueli/Documents/semio/"
B = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
LANE = B + "🪪️graph-state/📦️project-records/🚚️entry-lane/"
N = B + "🕸️graph/📥️inference/📦️project-nodes/"
FARM = N + "🏊️project-farm/"
SRC = os.path.dirname(os.path.abspath(__file__)) + "/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R97/backup/"
dry = "--dry" in sys.argv
def read(p): return open(p, encoding="utf8").read()
def once(s, old, new):
    assert s.count(old) == 1, ("anchor", s.count(old), old[:90])
    return s.replace(old, new)
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()[:12]
plan = {}
lane = read(LANE + "🟦️.ts")
staged = read(SRC + "lane.v4.ts")
i, j = lane.index(" const intake=async("), lane.index(" const forEach=async")
si, sj = staged.index(" const unpackOf="), staged.index(" const forEach=async")
lane = lane[:i] + staged[si:sj] + lane[j:]
plan[LANE + "🟦️.ts"] = lane
s = read(LANE + "🧬️schema/🔣️.json")
s = once(s, '"fetchConcurrency": 8', '"fetchConcurrency": 2')
plan[LANE + "🧬️schema/🔣️.json"] = once(s, '"offersInFlight": 4', '"offersInFlight": 1')
pool = read(FARM + "🏊️pool/🟨️.mjs")
pool = once(pool, 'aloneCapMegabytes = PLAN_STAGE_POLICY_V1.farmChildFootprintMegabytes["project-farm"].aloneCapMegabytes }) {', 'aloneCapMegabytes = PLAN_STAGE_POLICY_V1.farmChildFootprintMegabytes["project-farm"].aloneCapMegabytes, limitMegabytes = Infinity }) {')
pool = once(pool, "  const capOf = job => job.alone === true ? aloneCapMegabytes : capMegabytes;\n", "  const capOf = job => job.alone === true ? aloneCapMegabytes : capMegabytes;\n  const learned = new Map();\n  const knownOf = job => { const value = job?.payload?.knownMegabytes, declared = Number.isFinite(value) && value > 0 ? value : 0; return Math.max(declared, learned.get(job?.payload?.name) ?? 0); };\n  const runningKnown = () => slots.reduce((sum, slot) => sum + (slot.state === \"busy\" ? knownOf(slot.job) : 0), 0);\n  const crowded = busy => busy > 0 && queue.length > 0 && runningKnown() + knownOf(queue[0]) > limitMegabytes;\n")
pool = once(pool, "      if (held && busy > 0 || soloRunning || queue[0].alone === true && busy > 0) break;", "      if (held && busy > 0 || soloRunning || queue[0].alone === true && busy > 0 || crowded(busy)) break;")
pool = once(pool, "    const solo = soloRunning || queue[0]?.alone === true;", "    const solo = soloRunning || queue[0]?.alone === true || crowded(busy);")
pool = once(pool, "        job.footprint = Math.max(job.footprint ?? 0, megabytes);\n", "        job.footprint = Math.max(job.footprint ?? 0, megabytes);\n        if (job.payload?.name !== undefined) learned.set(job.payload.name, Math.max(learned.get(job.payload.name) ?? 0, megabytes));\n")
plan[FARM + "🏊️pool/🟨️.mjs"] = pool
dts = read(FARM + "🏊️pool/🟨️.d.mts")
plan[FARM + "🏊️pool/🟨️.d.mts"] = once(dts, "aloneCapMegabytes?:number}):GraphFarmV1;", "aloneCapMegabytes?:number;limitMegabytes?:number}):GraphFarmV1;")
farm = read(FARM + "🟨️.mjs")
plan[FARM + "🟨️.mjs"] = once(farm, "      entry: captured === undefined ? ENTRY : GRAPH_FARM_WORKER_ROLE_V1, service: SERVICE, workers, megabytes, policy,", "      entry: captured === undefined ? ENTRY : GRAPH_FARM_WORKER_ROLE_V1, service: SERVICE, workers, megabytes, policy, limitMegabytes: lean.limitMegabytes ?? Infinity,")
nodes = read(N + "🟨️.mjs")
plan[N + "🟨️.mjs"] = once(nodes, "job:{name,json:job.json,globals,warmKey:key,warm:true,directory,closureStore,", "job:{name,json:job.json,globals,warmKey:key,warm:true,knownMegabytes:Math.round((enforcement.known?.(name)??0)/1048576),directory,closureStore,")
tests = {LANE + "🧪️tests/🌊️streamed-intake/🟦️.ts": SRC + "intake.test.ts", FARM + "🧪️tests/🧮️admission/🟦️.ts": SRC + "admission.test.ts"}
if dry:
    for k in plan: print("ok", k[len(R):][-70:], len(plan[k]))
    sys.exit(0)
before = {k: sha(k) for k in plan}
for k in plan:
    os.makedirs(os.path.dirname(BK + k[len(R):]), exist_ok=True)
    shutil.copy(k, BK + k[len(R):])
for k, v in plan.items():
    tmp = k + ".r97tmp"
    open(tmp, "w", encoding="utf8").write(v)
    os.replace(tmp, k)
for dest, src in tests.items():
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copy(src, dest)
for k in plan: print(k[len(R):][-60:], before[k], "->", sha(k))
