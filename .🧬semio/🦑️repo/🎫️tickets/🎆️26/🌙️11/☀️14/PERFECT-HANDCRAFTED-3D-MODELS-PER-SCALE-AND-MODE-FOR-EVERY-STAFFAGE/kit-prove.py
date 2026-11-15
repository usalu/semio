"""🧪️ End-to-end run of the kit on one asset into a scratch source: fit 1:50, coarsen to 100/200/500, thermal, structure, decisions, check. Usage: python3 kit-prove.py <category>/<slug> [--seed-from-source]"""
import json, os, subprocess, sys, glob
T = os.path.dirname(os.path.abspath(__file__))
BUN = sorted(glob.glob(os.path.expanduser("~/Library/Caches/semio/runtimes/bun/*/bun")))[0]
spec = sys.argv[1]
slug = spec.split("/")[1]
src = f"{T}/🗑️generated/kit/src/{slug}.json"
def run(*a):
    p = subprocess.run([BUN, *a], cwd=T, capture_output=True, text=True)
    out = p.stdout + p.stderr
    print(f"$ {' '.join(a[:1])} {' '.join(a[1:])}\n" + "\n".join(l for l in out.splitlines() if not l.startswith("absorbed"))[-1800:])
    return out
orig = json.load(open(f"{T}/volumes/{spec}.json"))
vols = orig["volumes"]
for k in vols:
    v = vols[k]
    if v.get("status") == "authored":
        v["body"], v["thermal"], v["structure"], v["decisions"] = ([] if k != "50" else v["body"]), [], [], []
json.dump({"volumes": vols}, open(src, "w"))
S = ["--source", src]
run("kit-fit.ts", spec, "--scale", "50", "--rounds", "3", "--shift", "--write", *S)
avail = [int(k) for k, v in vols.items() if v.get("status") == "authored"]
for n in avail:
    if n != 50:
        run("kit-coarsen.ts", spec, "--from", "50", "--to", str(n), "--apply", "--force", *S)
for n in avail:
    run("kit-thermal.ts", spec, "--scale", str(n), "--write", *S)
    run("kit-structure.ts", spec, "--scale", str(n), "--write", *S)
d = json.load(open(src))
for n in avail:
    v = d["volumes"][str(n)]
    v["decisions"] = [{"en": f"1:{n}: generated first pass by the shared kit from this scale's own drawings (fit, coarsen, thermal, structure); to be reviewed against the render sheet.", "de": f"1:{n}: Erstentwurf des gemeinsamen Kits aus den eigenen Zeichnungen dieses Maßstabs (Anpassung, Vergröberung, Thermik, Tragwerk); anhand des Renderblatts zu prüfen."}]
json.dump(d, open(src, "w"))
for n in avail:
    run("volumes-check.ts", spec, "--scale", str(n), "--quiet", *S)
