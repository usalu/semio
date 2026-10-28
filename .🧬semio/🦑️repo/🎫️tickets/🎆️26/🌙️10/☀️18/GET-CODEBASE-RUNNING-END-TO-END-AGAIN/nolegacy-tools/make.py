#!/usr/bin/env python3
"""🏭️ Builds one staged batch of lane nolegacy from a root that already holds compliance's batches (the nl-main mirror): python3 -I make.py <batch> [--root DIR].
A spec is specs/<batch>.py defining TITLE, EDITS {rel: [(old, new, count) | callable(text)->text]}, optional NEW {rel: text}, DELETE [rel], TESTS [bun test paths starting with ./].
Every anchor must occur exactly `count` times (count "any" = at least once); a callable edit returns the new text. Output: staged/<batch>/{manifest.json, files/, base/, stage.py (compliance's delete-capable, mode-keeping template), verify.sh}."""
import hashlib, importlib.util, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
MIRROR = os.environ.get("NL_ROOT", "/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/nl-main")
def sha(data): return hashlib.sha256(data).hexdigest()
batch = sys.argv[1]
root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else MIRROR
spec_path = os.path.join(HERE, "specs", batch + ".py")
loader = importlib.util.spec_from_file_location("spec_" + batch, spec_path)
module = importlib.util.module_from_spec(loader)
module.ROOT = root
loader.loader.exec_module(module)
out = os.path.join(HERE, "staged", batch)
shutil.rmtree(out, ignore_errors=True)
for d in ("files", "base"): os.makedirs(os.path.join(out, d))
manifest, index = {}, 0
def name_of(rel):
    global index
    ext = rel.rsplit(".", 1)[-1] if "." in os.path.basename(rel) else "txt"
    index += 1
    return "%03d.%s" % (index, ext)
for rel, edits in module.EDITS.items():
    data = open(os.path.join(root, rel), "rb").read()
    text = data.decode("utf8")
    for edit in edits:
        if callable(edit):
            text = edit(text); continue
        old, new, count = edit
        found = text.count(old)
        if (count == "any" and found < 1) or (count != "any" and found != count): sys.exit("anchor count %d != %s in %s: %r" % (found, count, rel[-70:], old[:90]))
        text = text.replace(old, new)
    new_bytes = text.encode("utf8")
    if new_bytes == data: sys.exit("no change in " + rel)
    name = name_of(rel)
    open(os.path.join(out, "base", name), "wb").write(data)
    open(os.path.join(out, "files", name), "wb").write(new_bytes)
    manifest[rel] = {"file": name, "base": sha(data), "new": sha(new_bytes)}
for rel, text in getattr(module, "NEW", {}).items():
    if os.path.exists(os.path.join(root, rel)): sys.exit("new file exists: " + rel)
    name = name_of(rel)
    open(os.path.join(out, "files", name), "wb").write(text.encode("utf8"))
    manifest[rel] = {"file": name, "base": None, "new": sha(text.encode("utf8"))}
for rel in getattr(module, "DELETE", []):
    data = open(os.path.join(root, rel), "rb").read()
    name = name_of(rel)
    open(os.path.join(out, "base", name), "wb").write(data)
    manifest[rel] = {"file": name, "base": sha(data), "new": None, "mode": os.stat(os.path.join(root, rel)).st_mode & 0o7777}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(C, "compliance-tools", "stage.template.py"), encoding="utf8").read()
open_at = template.index('"""') + 3
close_at = template.index('"""', open_at)
first = template[open_at:close_at].split("\n", 1)[1]
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template[:open_at] + module.TITLE + "\n" + first + template[close_at:])
os.chmod(os.path.join(out, "stage.py"), 0o755)
tests = " ".join("'%s'" % t for t in getattr(module, "TESTS", []))
verify = f'''#!/bin/sh
# {module.TITLE}
# usage: VERIFY_ROOT=<repository or mirror root> sh verify.sh   (stage state, Bun parse-check via gate-g-tools/scan.ts, docstring/emoji/[DEBUG] censuses and the language law on the batch files via compliance-tools/verify_batch.py, then the batch's laws)
set -e
here=$(cd "$(dirname "$0")" && pwd)
c=$(cd "$here/../../.." && pwd)
root=${{VERIFY_ROOT:?VERIFY_ROOT required}}
python3 -I "$here/stage.py" check --root "$root"
python3 -I "$c/compliance-tools/verify_batch.py" "$here" "$root"
cd "$root"
'''
if tests: verify += f'bun test {tests}\n'
open(os.path.join(out, "verify.sh"), "w", encoding="utf8").write(verify)
os.chmod(os.path.join(out, "verify.sh"), 0o755)
print(batch, "rows", len(manifest), "->", out)
