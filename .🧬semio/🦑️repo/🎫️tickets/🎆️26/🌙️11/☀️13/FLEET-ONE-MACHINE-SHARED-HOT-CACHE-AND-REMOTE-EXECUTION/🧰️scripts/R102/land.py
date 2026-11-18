import os, sys, shutil
B = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
SC = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R102/"
PN = "🕸️graph/📥️inference/📦️project-nodes/"
OP = PN + "🌐️owner-pool/"
apply = "--apply" in sys.argv
def read(p): return open(p, encoding="utf8").read()
def once(s, old, new):
    assert s.count(old) == 1, ("anchor", s.count(old), old[:90])
    return s.replace(old, new)
plan = {}
p = read(B + OP + "🟦️.ts")
p = once(p, "/** 🎲️ The next number in [0, 1)", "/** 🧭️ The rendezvous score of an owner for one puller, in [0, 1): the lower it is the earlier the puller takes the owner, so every puller has an order of its own over the same owners and N pullers start on N different ones; a pure function of both names. */\nexport const ownerPoolRendezvousV1=(puller:string,name:string)=>parseInt(sha(puller+\"\\u0000\"+name).slice(0,8),16)/4294967296;\n\n/** 🎲️ The next number in [0, 1)")
p = once(p, "let random=ownerPoolSeedV1(input.puller),cursor=-1,streak=0,placed=false,jumps=0;", "let random=ownerPoolSeedV1(input.puller),cursor=-1,streak=0,storedRun=0,placed=false;")
p = once(p, "  for(const unit of list){if(picked.length>=count)break;if(ownerPoolHeavyV1(unit,policy)&&!finished.has(unit.name)&&!running.has(unit.name)&&!queuedHeavy.has(unit.name)&&(retryAt.get(unit.name)??0)<=now())picked.push(unit);}\n",
 "  const heavy:{unit:U;score:number}[]=[];\n  for(const unit of list)if(ownerPoolHeavyV1(unit,policy)&&!finished.has(unit.name)&&!running.has(unit.name)&&!queuedHeavy.has(unit.name)&&(retryAt.get(unit.name)??0)<=now())heavy.push({unit,score:ownerPoolRendezvousV1(input.puller,unit.name)});\n  heavy.sort((a,b)=>a.score-b.score);\n  for(const row of heavy){if(picked.length>=count)break;picked.push(row.unit);}\n")
p = once(p, "jumps<policy.doneJumps){", "storedRun<policy.doneStoredMinimum){")
p = once(p, "  const want=Math.min(policy.claimWindow,slots+policy.prefetchClaims-running.size-ready.length);\n", "  const searching=storedRun>=2*policy.claimWindow&&ready.length===0,want=searching?policy.claimWindowMaximum:Math.min(policy.claimWindow,slots+policy.prefetchClaims-running.size-ready.length);\n")
p = once(p, 'finished.add(unit.name);input.stored?.(unit);stats.stored++;streak++;}', 'finished.add(unit.name);input.stored?.(unit);stats.stored++;streak++;storedRun++;}')
p = once(p, 'finished.add(unit.name);stats.refused++;streak++;}', 'finished.add(unit.name);stats.refused++;streak++;storedRun++;}')
p = once(p, "stats.held++;streak++;jumps=0;}", "stats.held++;streak++;storedRun=0;}")
p = once(p, "stats.faulted++;streak++;jumps=0;}", "stats.faulted++;streak++;storedRun=0;}")
p = once(p, "stats.claimed++;streak=0;jumps=0;progressed=true;}", "stats.claimed++;streak=0;storedRun=0;progressed=true;}")
p = once(p, "streak=0;jumps++;}", "streak=0;}")
p = once(p, "    else{ready.push({unit,claim});stats.claimed++;", "    else if(searching&&ready.length+running.size>=slots+policy.prefetchClaims){await claim.abandon().catch(()=>undefined);}\n    else{ready.push({unit,claim});stats.claimed++;")
p = once(p, "and `doneJumps` jumps in a row that find nothing but stored owners end the pull (the wave is done as far as this puller can tell; the supervisor looks again later), so no puller has to look at every owner of a wave.", "and `doneStoredMinimum` stored owners in a row end the pull (the wave is done as far as this puller can tell; the supervisor looks again later: with 3 percent of a wave open the old twelve were enough to end 7 of 10 pulls while 1100 owners were open), so no puller has to look at every owner of a wave, and while the run of stored owners is long (`2 x claimWindow`) and nothing is waiting, one round claims `claimWindowMaximum` owners instead of the window. Heavy owners that fit this puller are taken first, in the order of its rendezvous scores ({@link ownerPoolRendezvousV1}), so no two pullers meet on the same heavy owner first.")
plan[OP + "🟦️.ts"] = p
s = read(B + OP + "🧬️schema/🔣️.json")
s = once(s, '"doneJumps": 3,', '"claimWindowMaximum": 16,\n        "doneStoredMinimum": 192,')
plan[OP + "🧬️schema/🔣️.json"] = s
m = read(B + PN + "🟨️.mjs")
m = once(m, "/** 🪨️ Reports and cancels inference settlement rounds", "/** 🛣️ The projects inferred at once by one read: `explicit` when the caller names it, else the cores (at most eight), the memory the parent may grow by divided by what one owner is measured to hold (`parentMegabytes` / `ownerMegabytes`), and never fewer than the cut of the load allows (`cut`, Infinity when uncut): the cut ladder halves the lanes after a plan overrun, but the governor admits every owner by its measured footprint, so a cut load keeps the lanes its memory fits (the worker that read with 1 lane held 1.3 to 3.4 GB of a 6.5 GB plan).\n * @see ./🧪️tests/🛣️lanes/🟦️.ts */\nexport function inferenceLanesV1({ cores, cut, parentMegabytes, ownerMegabytes, explicit }) {\n  if (explicit !== undefined) return Math.max(1, Math.min(explicit, cut));\n  const byCores = Math.max(1, Math.min(8, cores)), byMemory = Math.max(1, Math.floor(parentMegabytes / Math.max(1, ownerMegabytes || 800)));\n  return Math.max(1, Math.min(byCores, Math.max(Number.isFinite(cut) ? cut : 8, byMemory)));\n}\n\n/** 🪨️ Reports and cancels inference settlement rounds")
m = once(m, "const limit=Math.min(projectConcurrencyV1(options),jobs.length),governor=", "const ownerSizes=jobs.map(job=>enforcement.known?.(job.name)).filter(value=>Number.isFinite(value)).sort((a,b)=>a-b),limit=Math.min(inferenceLanesV1({cores:availableParallelism(),cut:graphCutConcurrencyV1(process.env),parentMegabytes:parentBytes/1048576,ownerMegabytes:ownerSizes.length===0?undefined:ownerSizes[Math.floor(ownerSizes.length*0.75)],explicit:options?.projectConcurrency}),jobs.length),governor=")
plan[PN + "🟨️.mjs"] = m
tf = read(B + OP + "🧪️tests/🟦️.ts")
plan[OP + "🧪️tests/🟦️.ts"] = once(tf, "const fast={...P,sweepPollMilliseconds:5,", "const fast={...P,claimWindowMaximum:P.claimWindow,doneStoredMinimum:12,sweepPollMilliseconds:5,")
dt = B + PN + "🟨️.d.mts"
d = read(dt) if os.path.exists(dt) else ""
if "projectConcurrencyV1" in d:
    i = d.index("export function projectConcurrencyV1")
    j = d.index("\n", i) + 1
    plan[PN + "🟨️.d.mts"] = d[:j] + "export function inferenceLanesV1(input:{cores:number;cut:number;parentMegabytes:number;ownerMegabytes?:number;explicit?:number}):number;\n" + d[j:]
os.makedirs(SC + "stage", exist_ok=True)
for rel, text in plan.items():
    out = SC + "stage/" + rel.replace("/", "__")
    open(out, "w", encoding="utf8").write(text)
    print("staged", rel.split("/")[-1], len(text))
if apply:
    os.makedirs(SC + "backup", exist_ok=True)
    for rel in plan:
        shutil.copy(B + rel, SC + "backup/" + rel.replace("/", "__"))
        open(B + rel, "w", encoding="utf8").write(plan[rel])
    print("applied")
