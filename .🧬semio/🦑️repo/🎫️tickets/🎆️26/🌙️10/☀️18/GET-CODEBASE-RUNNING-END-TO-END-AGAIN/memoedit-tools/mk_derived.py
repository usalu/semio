"""🏗️ Builds the flat batch `staged/derived` (manifest.json {rel:{file,base,new}} + files/ + base/ + diffs/ + stage.py + verify.sh) from a mirror and the CURRENT live files.
usage: python3 -I mk_derived.py <mirror root>            build from the mirror's versions (base = live now)
       python3 -I mk_derived.py --rebase <mirror root>   re-derive every edited file on top of the then-live one first: memo core and command glue by their stored unified diffs (`patch`), the schema by edit_schema_derived.py, the law by edit_law_derived.py, the two host files by edit_hosts.py; the new derive module is copied as it is"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); LIVE = "/Users/ueli/Documents/semio"
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"; M = B + "/🧭️invocation/🕸️graph/🧠️memo"
FILES = [M + "/🟦️.ts", M + "/🚪️command/🟦️.ts", M + "/🧬️schema/🔣️.json", M + "/🧪️tests/✏️edit/🟦️.ts", M + "/🧮️derive/🟦️.ts", B + "/🧭️invocation/🕸️graph/🟦️.ts", B + "/⌨️command/⚙️runtime/🟦️.ts"]
OUT = os.path.join(HERE, "staged", "derived")
def sha(path): return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
args = sys.argv[1:]; rebase = args[:1] == ["--rebase"]; mirror = args[-1]
if rebase:
    keep = {FILES[4]: open(os.path.join(mirror, FILES[4]), "rb").read()}
    for index in (0, 1):
        shutil.copy2(os.path.join(LIVE, FILES[index]), os.path.join(mirror, FILES[index]))
        subprocess.run(["patch", "--forward", "-s", os.path.join(mirror, FILES[index]), os.path.join(OUT, "diffs", "%02d.diff" % index)], check=True)
    for index in (2, 3, 5, 6): shutil.copy2(os.path.join(LIVE, FILES[index]), os.path.join(mirror, FILES[index]))
    for script in ("edit_schema_derived.py", "edit_law_derived.py", "edit_hosts.py"): subprocess.run([sys.executable, "-I", os.path.join(HERE, script), mirror], check=True)
os.makedirs(OUT, exist_ok=True)
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
