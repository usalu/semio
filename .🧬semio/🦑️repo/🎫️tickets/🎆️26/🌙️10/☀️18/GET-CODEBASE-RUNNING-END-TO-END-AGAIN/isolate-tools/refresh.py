"""🔄 [isolate] after a tooling change: converts batch <b> from live into mirror is1, drops from the `new` ledger every file whose converted bytes changed, so the next sweep re-measures exactly those. usage: refresh.py <batch>"""
import hashlib, json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); M = "/private/tmp/m/is1"; b = sys.argv[1]
files = [l.strip() for l in open(os.path.join(HERE, "batches", b + ".txt"), encoding="utf-8") if l.strip()]
before = {f: hashlib.sha256(open(os.path.join(M, f), "rb").read()).hexdigest() for f in files if os.path.exists(os.path.join(M, f))}
subprocess.run([sys.executable, "-I", os.path.join(HERE, "convert.py"), b, "/Users/ueli/Documents/semio", M], check=True, stdout=subprocess.DEVNULL)
changed = {f for f in files if before.get(f) != hashlib.sha256(open(os.path.join(M, f), "rb").read()).hexdigest()}
ledger = os.path.join(C, "🗑️generated", "isolate", "sweep-vb-%s-new.jsonl" % b)
if os.path.exists(ledger):
    rows = [l for l in open(ledger, encoding="utf-8") if json.loads(l)["file"] not in changed]; open(ledger, "w", encoding="utf-8").writelines(rows)
print(b, "changed", len(changed)); [print("  ", f.split("🔨️modules/")[-1][-90:]) for f in sorted(changed)]
