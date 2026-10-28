#!/usr/bin/env python3
"""🪣 [farmid] Builds the staged batch `declined` from the CURRENT files of a root (default: live): once the graph cache declines a worker's leaves the farm sends no further project to a worker and closes its pools (gate g46: 142 finished projects declined and inferred again by the parent). Anchored edits (each anchor exactly once) plus new/; writes files/, base/, manifest.json. Rebase = run it again.
usage: python3 -I make.py [--root DIR]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
F = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
NEW = {F + "🧪️tests/🪣️declined/🟦️.ts": "test.ts"}
EDITS = {
    F + "🟨️.mjs": [
        ("request = requestBuildFarmCapacityV1, cores", "request = requestBuildFarmCapacityV1, adopt = adoptStagedGraphLeavesV1, cores"),
        ("adoption = adoptStagedGraphLeavesV1(workspaceRoot, join(pool.base, \"w\" + outcome.slot + \"-g\" + outcome.generation), outcome.reply.names);",
         "adoption = adopt(workspaceRoot, join(pool.base, \"w\" + outcome.slot + \"-g\" + outcome.generation), outcome.reply.names);"),
        ('      if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); return false; }',
         '      if (adoption.declined) { counts.declined++; say("declined", { project: job.name }); if (state !== "off") { stop("degraded", { reason: "the graph cache declined the leaves of " + job.name }); for (const each of pools) { each.ready = false; void each.farm.close().finally(() => each.release()); } } return false; }'),
        (" * A pool that is disabled (its workers refused for a differing identity, or degraded) gives its host grant back at once,",
         " * Once the graph cache declines the leaves a worker produced (its optional quota is full), no further project goes to a worker and every pool is closed at once: a declined project is inferred again by this process, so going on would infer each one twice and keep the workers' memory and grants.\n * A pool that is disabled (its workers refused for a differing identity, or degraded) gives its host grant back at once,"),
    ],
    F + "🟨️.d.mts": [
        ("=>ProjectFarmCapacityV1;cores?:number;", "=>ProjectFarmCapacityV1;adopt?:(workspaceRoot:string,stagingRoot:string,names:readonly string[])=>{readonly published:number;readonly present:number;readonly declined:boolean};cores?:number;"),
    ],
}
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    for name in ("files", "base"): shutil.rmtree(os.path.join(HERE, name), ignore_errors=True); os.makedirs(os.path.join(HERE, name))
    manifest, failed = {}, False
    def put(rel, base, new):
        flat = "%02d" % len(manifest) + "-" + os.path.basename(os.path.dirname(rel)) + "-" + os.path.basename(rel)
        open(os.path.join(HERE, "files", flat), "wb").write(new)
        if base is not None: open(os.path.join(HERE, "base", flat), "wb").write(base)
        manifest[rel] = {"file": flat, "base": None if base is None else sha(base), "new": sha(new)}
    for rel, edits in EDITS.items():
        base = open(os.path.join(root, rel), "rb").read(); text = base.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1: print("ANCHOR", text.count(old), rel[-60:], repr(old[:80])); failed = True; continue
            text = text.replace(old, new)
        put(rel, base, text.encode("utf8"))
    for rel, source in NEW.items():
        if os.path.exists(os.path.join(root, rel)): print("EXISTS", rel); failed = True; continue
        put(rel, None, open(os.path.join(HERE, "new", source), "rb").read())
    if failed: sys.exit(2)
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("staged", len(manifest), "files from", root)
main()
