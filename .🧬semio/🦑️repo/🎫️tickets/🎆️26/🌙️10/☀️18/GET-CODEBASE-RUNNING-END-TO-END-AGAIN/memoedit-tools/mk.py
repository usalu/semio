"""🏗️ Builds the flat batch `staged/memoedit` (house style: manifest.json {rel:{file,base,new}} + files/ + base/ + stage.py + verify.sh) from the edited files of a mirror and the CURRENT live files.
usage: python3 -I mk.py <mirror root>            build from the mirror's versions (base = live now; refused when live drifted from the recorded diffs' base)
       python3 -I mk.py --rebase <mirror root>   re-derive the edited files on top of the current live ones: the unified diffs in staged/memoedit/diffs/ are applied with `patch` (fuzz allowed) into the mirror first and the schema is edited structurally again (edit_schema.py), then the batch is built"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); LIVE = "/Users/ueli/Documents/semio"
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo"
FILES = [M + "/🟦️.ts", M + "/📤️relevance/🟦️.ts", M + "/🚪️command/🟦️.ts", M + "/🧬️schema/🔣️.json", M + "/🧪️tests/✏️edit/🟦️.ts"]
OUT = os.path.join(HERE, "staged", "memoedit")
def sha(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
args = sys.argv[1:]; rebase = args[:1] == ["--rebase"]; mirror = args[-1]
if rebase:
    for index, rel in enumerate(FILES):
        diff = os.path.join(OUT, "diffs", "%02d.diff" % index)
        if not os.path.exists(diff): continue
        shutil.copy2(os.path.join(LIVE, rel), os.path.join(mirror, rel))
        if rel.endswith(".json"): continue
        subprocess.run(["patch", "--forward", "-s", os.path.join(mirror, rel), diff], check=True)
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit_schema.py"), mirror], check=True)
for sub in ("files", "base", "diffs"): shutil.rmtree(os.path.join(OUT, sub), ignore_errors=True); os.makedirs(os.path.join(OUT, sub))
manifest = {}
for index, rel in enumerate(FILES):
    name = "%02d%s" % (index, os.path.splitext(rel)[1]); live, new = os.path.join(LIVE, rel), os.path.join(mirror, rel)
    shutil.copyfile(new, os.path.join(OUT, "files", name))
    if os.path.exists(live):
        shutil.copyfile(live, os.path.join(OUT, "base", name))
        with open(os.path.join(OUT, "diffs", "%02d.diff" % index), "w") as f: subprocess.run(["diff", "-u", live, new], stdout=f)
    manifest[rel] = {"file": name, "base": sha(live), "new": sha(new)}
json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w"), indent=1, ensure_ascii=False)
print("[DEBUG] batch built", len(manifest), "files")
