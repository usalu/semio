#!/usr/bin/env python3
"""📦 Cuts a staged batch from a verified mirror: usage `python3 -I make-batch.py <batch> <mirror-root> <live-relative-path>...`
For every path: base = the CURRENT live bytes (absent for a new file), new = the mirror bytes. Writes `staged/<batch>/{manifest.json,files/,base/,stage.py,verify.sh}`; stage.py is the mode-keeping template of xplat5 (law: python3 -I C/xplat5-tools/tests/test_stage_mode.py <stage.py>); verify.sh comes from `<batch>.verify.sh` beside this script."""
import hashlib, json, os, shutil, sys
here = os.path.dirname(os.path.abspath(__file__)); live = os.path.abspath(os.path.join(here, *([".."] * 8)))
batch, mirror, rels = sys.argv[1], os.path.abspath(sys.argv[2]), sys.argv[3:]
out = os.path.join(here, "staged", batch)
shutil.rmtree(out, ignore_errors=True)
for sub in ("files", "base"): os.makedirs(os.path.join(out, sub))
def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None
manifest = {}
for index, rel in enumerate(rels):
    name = f"f{index:02d}" + os.path.splitext(rel)[1]
    new = os.path.join(mirror, rel); base = os.path.join(live, rel)
    shutil.copyfile(new, os.path.join(out, "files", name))
    if os.path.exists(base): shutil.copyfile(base, os.path.join(out, "base", name))
    manifest[rel] = {"file": name, "base": digest(base), "new": digest(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
emoji = {"dashjourneys": "🧭", "fixable": "🩹", "coursevt": "🎓"}.get(batch, "📦")
template = open(os.path.join(here, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read().replace("🌌 Staged batch `universe` (lane rustedit): print's Bazel plan becomes reusable although a dependency reads a root-anchored pattern.", f"{emoji} Staged batch `{batch}` (lane td-products).")
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template); os.chmod(os.path.join(out, "stage.py"), 0o755)
verify = os.path.join(here, f"{batch}.verify.sh")
if os.path.exists(verify): shutil.copyfile(verify, os.path.join(out, "verify.sh")); os.chmod(os.path.join(out, "verify.sh"), 0o755)
print(batch, len(rels), "files ->", out)
