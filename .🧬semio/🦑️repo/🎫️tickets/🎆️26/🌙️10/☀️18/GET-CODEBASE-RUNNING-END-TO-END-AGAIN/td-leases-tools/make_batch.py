"""📦 [td-leases] builds a flat staged batch from an edited mirror: base = the LIVE file now (None when absent), new = the mirror file; writes manifest.json + files/fNNN + base/fNNN, a stage.py from C/xplat5-tools/stage.template.py with the batch docstring and verify.sh from verify-<batch>.sh. usage: python3 -I make_batch.py <batch> <mirror-root> <docstring-file> <rel>..."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
batch, mirror, docfile, rels = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4:]
out = os.path.join(HERE, "staged", batch)
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
sha = lambda data: hashlib.sha256(data).hexdigest()
manifest = {}
for n, rel in enumerate(rels):
    name = f"f{n:03d}"; live = os.path.join(LIVE, rel); new = open(os.path.join(mirror, rel), "rb").read()
    base = open(live, "rb").read() if os.path.exists(live) else None
    if base == new: sys.exit("unchanged: " + rel)
    open(os.path.join(out, "files", name), "wb").write(new)
    if base is not None: open(os.path.join(out, "base", name), "wb").write(base)
    manifest[rel] = {"file": name, "base": None if base is None else sha(base), "new": sha(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read()
start = template.index('"""') + 3; end = template.index('\nusage:', start)
open(os.path.join(out, "stage.py"), "w", encoding="utf8").write(template[:start] + open(docfile, encoding="utf8").read().strip() + template[end:])
shutil.copyfile(os.path.join(HERE, f"verify-{batch}.sh"), os.path.join(out, "verify.sh"))
print(f"{batch}: {len(manifest)} files ({sum(1 for r in manifest.values() if r['base'] is None)} new) -> {out}")
