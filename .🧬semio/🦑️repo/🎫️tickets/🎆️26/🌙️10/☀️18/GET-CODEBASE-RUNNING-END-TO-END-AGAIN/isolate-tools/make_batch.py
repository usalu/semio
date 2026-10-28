"""📦 [isolate] stages one batch from the CURRENT live bytes: converts it (convert.py) into a scratch tree, writes `staged/<batch>/` with manifest.json {rel:{file,base,new}}, files/, base/, stage.py (xplat5 template, keeps file modes), verify.sh and expected.json (the failure set of each file in the batch's `new` ledger on mirror is1). Extra infrastructure files (helper/fence batches) come from `extra/<batch>.txt` as `rel` lines whose new bytes are taken from mirror is1 (new files have base null). usage: make_batch.py <batch>"""
import hashlib, json, os, shutil, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); LIVE = "/Users/ueli/Documents/semio"; MIRROR = "/private/tmp/m/is1"
batch = sys.argv[1]; out = os.path.join(HERE, "staged", batch); SHORT = {"builda", "buildb"}
sha = lambda data: hashlib.sha256(data).hexdigest()
rows = {}
listing = os.path.join(HERE, "batches", batch + ".txt")
if os.path.exists(listing):
    scratch = tempfile.mkdtemp(prefix="isolate-", dir=os.path.join(C, "🗑️generated", "isolate"))
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "convert.py"), batch, LIVE, scratch], check=True, stdout=subprocess.DEVNULL)
    for rel in [l.strip() for l in open(listing, encoding="utf-8") if l.strip()]:
        path = os.path.join(scratch, rel)
        if os.path.exists(path):
            rows[rel] = (open(os.path.join(LIVE, rel), "rb").read(), open(path, "rb").read())
            if not os.path.exists(os.path.join(MIRROR, rel)) or open(os.path.join(MIRROR, rel), "rb").read() != rows[rel][1]: print("UNVERIFIED (mirror is1 holds other bytes):", rel[-90:])
    shutil.rmtree(scratch)
extra = os.path.join(HERE, "extra", batch + ".txt")
if os.path.exists(extra):
    for rel in [l.strip() for l in open(extra, encoding="utf-8") if l.strip() and not l.startswith("#")]:
        live = os.path.join(LIVE, rel); rows[rel] = (open(live, "rb").read() if os.path.exists(live) else None, open(os.path.join(MIRROR, rel), "rb").read())
if os.path.exists(out): shutil.rmtree(out)
os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
manifest = {}
for i, (rel, (base, new)) in enumerate(sorted(rows.items())):
    if base == new: continue
    name = "f%03d" % i; open(os.path.join(out, "files", name), "wb").write(new)
    if base is not None: open(os.path.join(out, "base", name), "wb").write(base)
    manifest[rel] = {"file": name, "base": sha(base) if base is not None else None, "new": sha(new)}
json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf-8").read()
doc_end = template.index('usage: python3 -I stage.py')
template = '"""🧫 Staged batch `%s` (lane isolate): tests reach no live repository store (private repository per test process, live store fenced in test processes).\n' % batch + template[doc_end:]
open(os.path.join(out, "stage.py"), "w", encoding="utf-8").write(template)
ledger = os.path.join(C, "🗑️generated", "isolate", "sweep-vb-%s-new.jsonl" % batch); expected = {}
if os.path.exists(ledger):
    for line in open(ledger, encoding="utf-8"):
        row = json.loads(line)
        if row["file"] in manifest: expected[row["file"]] = {"mode": row.get("mode"), "pass": row.get("pass"), "fails": row.get("fails", []), **({"artifacts": "short"} if batch in SHORT else {})}
pinned = os.path.join(HERE, "expected", batch + ".json")
if os.path.exists(pinned): expected.update(json.load(open(pinned, encoding="utf-8")))
json.dump(expected, open(os.path.join(out, "expected.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
open(os.path.join(out, "verify.sh"), "w", encoding="utf-8").write('#!/bin/sh\n# ✅ laws of batch `%s` (lane isolate): sh verify.sh <root>; stage check, parse scan, then every staged test file runs (plain bun test, the official envelope when it asks for one) and may fail only tests of its recorded failure set (expected.json, measured on mirror is1, equal to the base set), must not hit the live-store fence, and (once the law module is present at <root>) carries no test-store-isolation finding. Expect `VERIFY OK %s`.\nexec python3 -I "$(cd "$(dirname "$0")/../.." && pwd)/verify_batch.py" "$(cd "$(dirname "$0")" && pwd)" "${1:?root}"\n' % (batch, batch))
print(batch, len(manifest), "files staged ->", out)
