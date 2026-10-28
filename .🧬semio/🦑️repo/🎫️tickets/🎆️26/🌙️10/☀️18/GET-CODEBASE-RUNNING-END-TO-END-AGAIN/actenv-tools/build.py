"""🏗️ Builds the staged batch `actenv` from the LIVE checkout: base = live 📜️script.ts now, new = base + edit_script.py's edits; the five new files come from staged/files (01..05). Re-run it right before landing; it prints the base hash and refuses (exit 2) when an anchor of the edit moved. usage: build.py [ROOT]"""
import hashlib, importlib.util, json, os, sys
HERE = os.path.dirname(os.path.abspath(__file__)); STAGED = os.path.join(HERE, "staged")
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and here != "/": here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else root_of(HERE)
R = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
A = R + "/🌿️environment/🏃️action"
spec = importlib.util.spec_from_file_location("edit_script", os.path.join(HERE, "edit_script.py")); edit = importlib.util.module_from_spec(spec); spec.loader.exec_module(edit)
sha = lambda data: hashlib.sha256(data).hexdigest()
live = open(os.path.join(ROOT, R + "/📜️script.ts"), "rb").read()
new = edit.transform(live.decode("utf8"), "apply").encode("utf8")
open(os.path.join(STAGED, "base", "00.ts"), "wb").write(live); open(os.path.join(STAGED, "files", "00.ts"), "wb").write(new)
rows = {R + "/📜️script.ts": ("00.ts", sha(live), sha(new))}
for name, rel in [("01.ts", A + "/🟦️.ts"), ("02.json", A + "/🧬️schema/🔣️.json"), ("03.ts", A + "/🧪️tests/🟦️.ts"), ("04.json", A + "/🧪️tests/🧫️fixtures/🔣️.json"), ("05.py", A + "/🧪️tests/🧮️resources/🐍️oracle/🐍️.py")]:
    rows[rel] = (name, None, sha(open(os.path.join(STAGED, "files", name), "rb").read()))
json.dump({rel: {"file": f, "base": b, "new": n} for rel, (f, b, n) in rows.items()}, open(os.path.join(STAGED, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
print("base script.ts", sha(live)[:12], "new", sha(new)[:12])
