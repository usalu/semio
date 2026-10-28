#!/usr/bin/env python3
"""🧱 Builds a flat staged batch from a verified mirror: python3 -I make-batch.py <batch> <title> <mirror-root> <live-root> <rel>...
Each rel is taken from the mirror (new) against the live file now (base); the stage tool is the mode-keeping template of xplat5."""
import hashlib, json, os, shutil, sys
here = os.path.dirname(os.path.abspath(__file__))
batch, title, mirror, live, *rels = sys.argv[1:]
out = os.path.join(here, "staged", batch)
for sub in ("files", "base"):
    shutil.rmtree(os.path.join(out, sub), ignore_errors=True)
    os.makedirs(os.path.join(out, sub))
def digest(path):
    if not os.path.exists(path):
        return None
    return hashlib.sha256(open(path, "rb").read()).hexdigest()
manifest = {}
for index, rel in enumerate(rels):
    new, base = os.path.join(mirror, rel), os.path.join(live, rel)
    name = f"{index:02d}-{os.path.basename(rel)}"
    shutil.copyfile(new, os.path.join(out, "files", name))
    if os.path.exists(base):
        shutil.copyfile(base, os.path.join(out, "base", name))
    manifest[rel] = {"file": name, "base": digest(base), "new": digest(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(os.path.dirname(here), "xplat5-tools", "stage.template.py"), encoding="utf8").read()
lines = template.split("\n")
lines[1] = f'"""{title}'
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("\n".join(lines))
os.chmod(os.path.join(out, "stage.py"), 0o755)
print(f"{batch}: {len(rels)} files -> {out}")
