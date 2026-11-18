#!/usr/bin/env python3
"""R92 stage builder: base = live file now, new = base with anchored replacements; new files from the scratch stage. usage: mk.py STAGE"""
import os,sys,shutil
R="/Users/ueli/Documents/semio"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";B=L+"🏗️build/";SI=B+"🕸️graph/📥️inference/🧭️script-inputs/";GS=B+"🪪️graph-state/";PR=GS+"📦️project-records/";OBS=B+"👁️observation/"
S=sys.argv[1];NEW=S+"/new"
os.makedirs(S,exist_ok=True)
rows=[]
def rd(p):return open(R+"/"+p,encoding="utf8").read()
def edit(rel,pairs):
    base=rd(rel);new=base
    for old,rep in pairs:
        assert new.count(old)==1,(rel,old[:80],new.count(old))
        new=new.replace(old,rep)
    i=str(len(rows));open(S+"/"+i+".base","w",encoding="utf8").write(base);open(S+"/"+i+".new","w",encoding="utf8").write(new);rows.append((i,rel,"edit"))
def add(rel,srcname):
    i=str(len(rows));open(S+"/"+i+".new","w",encoding="utf8").write(open(NEW+"/"+srcname,encoding="utf8").read());rows.append((i,rel,"new"))

# ---- script-inputs: replace stale on import, fleet seam, generation teach, prefetch
FLEET_IMPORT='"../../../🪪️graph-state/📦️project-records/🚚️entry-lane/🟦️.ts"'
edit(SI+"🟨️.mjs",[
('import { createHash } from "node:crypto";\n',
 'import { createHash } from "node:crypto";\n'),
("const closureCountsV1 = { derived: 0, loaded: 0, stale: 0, checked: 0 };",
 "const closureCountsV1 = { derived: 0, loaded: 0, stale: 0, checked: 0, fetched: 0 };"),
("closureCountsV1.derived = closureCountsV1.loaded = closureCountsV1.stale = closureCountsV1.checked = 0; }",
 "closureCountsV1.derived = closureCountsV1.loaded = closureCountsV1.stale = closureCountsV1.checked = closureCountsV1.fetched = 0; }"),
# import replaces a stale stored entry whose replacement holds
("        const id = workspaceRoot + \"\\0\" + recipeKey;\n        if (await store.load(id) === undefined) { await store.adopt(id, bundle); taken++; }",
 "        const id = workspaceRoot + \"\\0\" + recipeKey, stored = await store.load(id);\n        if (stored === undefined || await bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle)) { await store.adopt(id, bundle); taken++; }"),
("export const closureTravelV1 = Object.freeze({",
 """/** 🔁️ Whether a bundle taken from the fleet replaces the entry this store already holds under its recipe: it does when the stored entry no longer holds in this tree and the bundle does, so a stale entry (derived before a change, or taken with a row that could never hold) never blocks an entry another machine derived for the same inputs; a bundle that does not hold here leaves the stored entry alone. */
async function bundleSupersedesStoredV1(workspaceRoot, store, stored, bundle) {
  const scripts = closureScriptsV1.get(workspaceRoot), step = cooperativeStepV1(scripts, "build:closure-entry-rows");
  if (!await observationRowsHoldAsyncV1(bundle.rows, step)) return false;
  for (const layer of bundle.layers) if (!await observationLayerHoldsAsyncV1(layer, step)) return false;
  return !await closureEntryHoldsAsyncV1({ ...stored, layers: undefined, store, answers: new Map(), checked: null }, scripts);
}
export const closureTravelV1 = Object.freeze({"""),
# generation + fleet seam
("async function produceClosureEntryV1(workspaceRoot, recipe) {\n  const id = workspaceRoot + \"\\0\" + closureRecipeKeyV1(recipe), known = closureEntriesV1.get(id);",
 """const closureFleetLanesV1 = new Map();
/** 🧶️ The fleet lane of closure entries of one workspace (see `🪪️graph-state/📦️project-records/🚚️entry-lane`), or undefined when no fleet cache is configured here or `SEMIO_CLOSURE_FLEET=off`; opened once. */
export function closureFleetLaneV1(workspaceRoot) {
  if (process.env.SEMIO_CLOSURE_FLEET === "off") return Promise.resolve(undefined);
  let lane = closureFleetLanesV1.get(workspaceRoot);
  if (lane === undefined) { lane = import(__FLEET_IMPORT__).then(module => module.createClosureEntryLaneV1({ root: workspaceRoot, environment: process.env }), () => undefined).catch(() => undefined); closureFleetLanesV1.set(workspaceRoot, lane); }
  return lane;
}
/** 🧶️ Takes the entries of `recipes` that this workspace's store lacks from the fleet in one batch, before any of them is derived; answers how many were taken. */
export async function prefetchClosureEntriesV1(workspaceRoot, recipes) {
  const store = closureStoreOfV1(workspaceRoot), lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
  if (!lane) return 0;
  const missing = [];
  for (const recipe of recipes) { const id = workspaceRoot + "\\0" + closureRecipeKeyV1(recipe); if (closureEntriesV1.peek(id) === undefined && await store.stamp(id) === undefined) missing.push(recipe); }
  if (missing.length === 0) return 0;
  const taken = await lane.prefetch(missing, closureTravelV1).catch(() => 0);
  closureCountsV1.fetched += taken;
  return taken;
}
async function produceClosureEntryV1(workspaceRoot, recipe) {
  const id = workspaceRoot + "\\0" + closureRecipeKeyV1(recipe), known = closureEntriesV1.get(id);""".replace("__FLEET_IMPORT__",FLEET_IMPORT)),
("if (entry) closureCountsV1.stale++; return derive(); })",
 """if (entry) closureCountsV1.stale++;
      const lane = store ? await closureFleetLaneV1(workspaceRoot) : undefined;
      if (!lane) return derive();
      const taken = await lane.take(recipe, closureTravelV1).catch(() => undefined);
      if (taken?.outcome === "taken") { const again = await store.load(id), next = again && { ...again, layers: undefined, store, answers: new Map(), checked: null }; if (next && await closureEntryHoldsAsyncV1(next, scripts)) { closureCountsV1.fetched++; closureEntriesV1.set(id, next); return next; } }
      let derived; try { derived = await derive(); } catch (error) { await taken?.hold?.release().catch(() => undefined); throw error; }
      if (derived.digests) await Promise.race([lane.offer(recipe, closureTravelV1, taken?.hold).catch(() => undefined), new Promise(done => { setTimeout(done, 15000).unref?.(); })]); else await taken?.hold?.release().catch(() => undefined);
      return derived; })"""),
# prefetch before the settle loop
("    for (const [workspaceRoot, recipe, key] of wanted.values()) { try { await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe)); } catch { closureEntriesV1.delete(workspaceRoot + \"\\0\" + key); } }\n  },",
 """    const byRoot = new Map();
    for (const [workspaceRoot, recipe] of wanted.values()) { byRoot.set(workspaceRoot, [...byRoot.get(workspaceRoot) ?? [], recipe]); }
    for (const [workspaceRoot, recipes] of byRoot) { try { await prefetchClosureEntriesV1(workspaceRoot, recipes); } catch {} }
    for (const [workspaceRoot, recipe, key] of wanted.values()) { try { await ensureProviderEntriesV1(workspaceRoot, await ensureClosureEntryV1(workspaceRoot, recipe)); } catch { closureEntriesV1.delete(workspaceRoot + "\\0" + key); } }
  },"""),
])

# ---- the lane, its schema and its tests
add(PR+"🚚️entry-lane/🟦️.ts","entries.ts")
add(PR+"🚚️entry-lane/🧬️schema/🔣️.json","entries-schema.json")
add(PR+"🚚️entry-lane/🧪️tests/🟦️.ts","lane-test.ts")

open(S+"/index.txt","w",encoding="utf8").write("\n".join(" ".join(r) for r in rows)+"\n")
print("staged",len(rows),"files")
