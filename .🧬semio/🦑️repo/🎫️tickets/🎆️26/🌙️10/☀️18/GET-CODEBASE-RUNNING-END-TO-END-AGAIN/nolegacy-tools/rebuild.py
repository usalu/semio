#!/usr/bin/env python3
"""🔄️ Rebases every nl batch on the CURRENT live files (the gate lands every ~30 min): reverts the nl batches in the nl-main mirror, copies each batch file from live into the mirror (compliance's new bytes when live still holds compliance's base), rebuilds the batches in order from their specs, re-applies them in the mirror and runs the whole-repo language law there. python3 -I rebuild.py   (a moved anchor stops the build instead of guessing)."""
import glob, hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = "/Users/ueli/Documents/semio/"
MIRROR = os.environ.get("NL_ROOT", "/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/nl-main") + "/"
ORDER = ("lease", "reapi", "nobaseline", "lang")
def digest(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
def stage(batch, verb): subprocess.run([sys.executable, "-I", os.path.join(HERE, "staged", batch, "stage.py"), verb, "--root", MIRROR], check=True, stdout=subprocess.DEVNULL)
compliance = {}
for manifest in glob.glob(os.path.join(C, "compliance-tools", "staged", "*", "manifest.json")):
    folder = os.path.dirname(manifest)
    for rel, row in json.load(open(manifest, encoding="utf8")).items(): compliance[rel] = (folder, row)
for batch in reversed(ORDER):
    if os.path.exists(os.path.join(HERE, "staged", batch, "stage.py")): subprocess.run([sys.executable, "-I", os.path.join(HERE, "staged", batch, "stage.py"), "revert", "--root", MIRROR], stdout=subprocess.DEVNULL)
for batch in ORDER:
    if not os.path.exists(os.path.join(HERE, "staged", batch, "manifest.json")): continue
    for rel, row in json.load(open(os.path.join(HERE, "staged", batch, "manifest.json"), encoding="utf8")).items():
        live = digest(LIVE + rel)
        source = LIVE + rel
        if rel in compliance and live == compliance[rel][1]["base"] and compliance[rel][1]["new"] is not None: source = os.path.join(compliance[rel][0], "files", compliance[rel][1]["file"])
        if live is None or row["new"] is None and not os.path.exists(LIVE + rel): continue
        os.makedirs(os.path.dirname(MIRROR + rel), exist_ok=True)
        if digest(MIRROR + rel) != digest(source): shutil.copyfile(source, MIRROR + rel); print("[DEBUG] rebased", rel[-80:])
for batch in ORDER:
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "make.py"), batch], check=True)
    stage(batch, "apply")
out = os.path.join(C, "🗑️generated", "nl", "law-rebuild.json")
subprocess.run(["bun", os.path.join(C, "compliance-tools", "cp-lawfull.ts"), out], check=True, env=dict(os.environ, CP_ROOT=MIRROR.rstrip("/")), cwd=C)
findings = json.load(open(out, encoding="utf8"))["findings"]
for row in findings: print("LAW", row["rule"], row["detail"], row["path"][-90:])
print("rebuild: language law findings in the mirror:", len(findings))
sys.exit(1 if findings else 0)
