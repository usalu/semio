"""📦 [rusttests] builds the flat staged batch `envelope` from S0 (base bytes: live plus the unlanded stack) and S1 (S0 plus edits): manifest.json, files/fNNN, base/fNNN, stage.py from C/xplat5-tools/stage.template.py with the docstring, verify.sh. usage: make_batch.py <batch> <S0> <S1> <docstring-file>"""
import os as _o, sys as _s; _s.path.insert(0, _o.path.dirname(_o.path.abspath(__file__)))
import json, shutil, sys
from lib import *
batch, S0, S1, docfile = sys.argv[1:5]
out = os.path.join(HERE, "staged", batch)
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
rels = []
for d, _, fs in os.walk(S1):
    for f in fs:
        rel = os.path.relpath(os.path.join(d, f), S1)
        if rel == ".stack": continue
        old = read(os.path.join(S0, rel)); new = read(os.path.join(S1, rel))
        if old != new: rels.append(rel)
rels.sort(); manifest = {}
for n, rel in enumerate(rels):
    name = f"f{n:03d}"; new = read(os.path.join(S1, rel)); base = read(os.path.join(S0, rel))
    open(os.path.join(out, "files", name), "wb").write(new)
    if base is not None: open(os.path.join(out, "base", name), "wb").write(base)
    manifest[rel] = {"file": name, "base": None if base is None else sha(base), "new": sha(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
start = template.index('"""') + 3; end = template.index('\nusage:', start)
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template[:start] + open(docfile, encoding="utf8").read().strip() + template[end:])
shutil.copyfile(os.path.join(HERE, f"verify-{batch}.sh"), os.path.join(out, "verify.sh"))
shutil.copyfile(os.path.join(HERE, "expect.tsv"), os.path.join(out, "expect.tsv"))
print(f"{batch}: {len(manifest)} files ({sum(1 for r in manifest.values() if r['base'] is None)} new) -> {out}")
