"""♻️ [isolate] re-measures the files of a staged batch whose live bytes drifted from the staged base: drops them from both ledgers and copies their live bytes into mirror is1 (so `phase.sh`-free sweeps measure base again, then `convert.py` + sweep measure new). usage: redo.py <batch> base|new"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); L = "/Users/ueli/Documents/semio"; M = "/private/tmp/m/is1"; b, step = sys.argv[1:3]
manifest = json.load(open(os.path.join(HERE, "staged", b, "manifest.json"), encoding="utf-8"))
drifted = [rel for rel, row in manifest.items() if hashlib.sha256(open(os.path.join(L, rel), "rb").read()).hexdigest() != row["base"]]
listing = os.path.join(C, "🗑️generated", "isolate", "redo-%s.txt" % b)
if step == "base":
    open(listing, "w", encoding="utf-8").write("\n".join(drifted) + "\n")
    for t in ("base", "new"):
        ledger = os.path.join(C, "🗑️generated", "isolate", "sweep-vb-%s-%s.jsonl" % (b, t))
        if os.path.exists(ledger): open(ledger + ".tmp", "w", encoding="utf-8").writelines(l for l in open(ledger, encoding="utf-8") if json.loads(l)["file"] not in drifted); os.replace(ledger + ".tmp", ledger)
    for rel in drifted: shutil.copyfile(os.path.join(L, rel), os.path.join(M, rel))
else: subprocess.run([sys.executable, "-I", os.path.join(HERE, "convert.py"), b, L, M], check=True, stdout=subprocess.DEVNULL)
print(b, step, len(drifted), "drifted"); [print("  ", r.split("🔨️modules/")[-1][-80:]) for r in drifted]
