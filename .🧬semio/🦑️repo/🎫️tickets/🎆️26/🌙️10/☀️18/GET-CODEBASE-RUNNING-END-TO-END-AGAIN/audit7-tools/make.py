"""🏗️ [audit7] Rebuilds the two flat staged batches from the LIVE bytes now: `contracts` (base = live) and `reclaim` (base = live + contracts; apply after it), by running the anchored edit scripts on a scratch copy. usage: python3 -I make.py [--compare <mirror-root>]   exit 1 = an anchor was lost (a peer edited the anchored text: re-anchor the edit script)."""
import hashlib, json, os, shutil, stat, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); LIVE = C
while not os.path.isdir(os.path.join(LIVE, "🧰️framework")): LIVE = os.path.dirname(LIVE)
N = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"; F = N + "🏊️project-farm/"
BATCHES = {
 "contracts": ("🧾️ Staged batch `contracts` (lane audit7): a graph farm worker infers with the generator contracts its parent keyed the job with (carried in the session), never with a later read of the taxonomy.",
  [N + "🟨️.mjs", F + "🟨️.mjs", F + "🟨️.d.mts", F + "🧪️tests/🟦️.ts", F + "🧪️tests/🔒️farm-close/🟦️.ts", F + "🧪️tests/🪆️captured/🟦️.ts"], ["contracts/edit_core.py", "contracts/edit_law.py"]),
 "reclaim": ("🧹️ Staged batch `reclaim` (lane audit7, apply AFTER `contracts`): farm temporary directories carry their process id and a farm removes those of processes that are gone.",
  [F + "🏊️pool/🟨️.mjs", F + "🏊️pool/🟨️.d.mts", F + "🏊️pool/🧬️schema/🔣️.json", F + "🟨️.mjs", F + "🧪️tests/🔒️farm-close/🟦️.ts"], ["reclaim/edit_core.py", "reclaim/edit_law.py"]),
}
sha = lambda b: hashlib.sha256(b).hexdigest()
scratch = os.path.join(C, "🗑️generated", "audit7", "build"); shutil.rmtree(scratch, ignore_errors=True)
for rel in sorted({rel for _p, rels, _e in BATCHES.values() for rel in rels}):
    os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True); shutil.copy2(os.path.join(LIVE, rel), os.path.join(scratch, rel))
template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read().split("\n")
for batch, (purpose, rels, edits) in BATCHES.items():
    base = {rel: open(os.path.join(scratch, rel), "rb").read() for rel in rels}
    for script in edits:
        if subprocess.run([sys.executable, "-I", os.path.join(HERE, script), scratch]).returncode: print("[audit7] anchor lost in", script); sys.exit(1)
    out = os.path.join(HERE, "staged", batch); verify = os.path.join(out, "verify.sh"); kept = open(verify, encoding="utf8").read() if os.path.exists(verify) else None
    shutil.rmtree(out, ignore_errors=True); os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    manifest = {}
    for index, rel in enumerate(rels):
        name = f"{index:02d}{os.path.splitext(rel)[1]}"; new = open(os.path.join(scratch, rel), "rb").read()
        open(os.path.join(out, "files", name), "wb").write(new); open(os.path.join(out, "base", name), "wb").write(base[rel])
        manifest[rel] = {"file": name, "base": sha(base[rel]), "new": sha(new)}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    lines = list(template); lines[1] = f'"""{purpose}'
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("\n".join(lines))
    if kept is not None: open(verify, "w", encoding="utf8").write(kept)
    print(out, len(rels), "files")
if "--compare" in sys.argv:
    mirror = sys.argv[sys.argv.index("--compare") + 1]; differ = [rel for rel in {r for _p, rs, _e in BATCHES.values() for r in rs} if open(os.path.join(mirror, rel), "rb").read() != open(os.path.join(scratch, rel), "rb").read()]
    print("mirror == rebuilt from live:", "yes" if not differ else "NO: " + ", ".join(r[-60:] for r in differ))
shutil.rmtree(scratch, ignore_errors=True)
