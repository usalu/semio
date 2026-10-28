#!/usr/bin/env python3
"""🧱 (Re)builds batch `excluded` from the CURRENT live files: script.ts through the anchored edit (`edit_excluded.py`, exit 1 = anchor lost), the law from `src/excluded`; stage.py comes from the mode-keeping template.
usage: python3 -I build_excluded.py [--root DIR]"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(HERE, *[".."] * 8))
assert os.path.isdir(os.path.join(root, "🧰️framework")), root
BAZEL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel"
LAW = BAZEL + "/♻️plan-reuse/🧪️tests/🕳️excluded-names"
out = os.path.join(HERE, "staged", "excluded")
for sub in ("files", "base"): shutil.rmtree(os.path.join(out, sub), ignore_errors=True); os.makedirs(os.path.join(out, sub))
sha = lambda path: hashlib.sha256(open(path, "rb").read()).hexdigest()
manifest = {}
script = os.path.join(root, BAZEL, "📜️script.ts")
shutil.copyfile(script, os.path.join(out, "base", "00.ts"))
if subprocess.run([sys.executable, "-I", os.path.join(HERE, "edit_excluded.py"), script, os.path.join(out, "files", "00.ts")]).returncode: sys.exit(1)
manifest[BAZEL + "/📜️script.ts"] = {"file": "00.ts", "base": sha(script), "new": sha(os.path.join(out, "files", "00.ts"))}
for index, (rel, name) in enumerate([("🟦️.ts", "01.ts"), ("🧫️fixtures/🟦️.ts", "02.ts"), ("🧫️fixtures/🔣️.json", "03.json")]):
    shutil.copyfile(os.path.join(HERE, "src", "excluded", rel), os.path.join(out, "files", name))
    live = os.path.join(root, LAW, rel)
    manifest[LAW + "/" + rel] = {"file": name, "base": sha(live) if os.path.exists(live) else None, "new": sha(os.path.join(out, "files", name))}
    if os.path.exists(live): shutil.copyfile(live, os.path.join(out, "base", name))
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read().replace('"""🌌 Staged batch `universe` (lane rustedit): print\'s Bazel plan becomes reusable although a dependency reads a root-anchored pattern.', '"""🕳 Staged batch `excluded` (lane audit3): the Bazel plan witness observes every directory git could list an untracked file in, also one named like a build output (nested `.🧬semio`, un-ignored fixture `target`).')
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template)
print("built", out, len(manifest), "files")
