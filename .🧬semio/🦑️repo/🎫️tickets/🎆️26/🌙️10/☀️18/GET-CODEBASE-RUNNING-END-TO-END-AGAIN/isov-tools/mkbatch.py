"""📦 [isov] shared batch writer: `build(batch, title, rows, expected)` writes `staged/<batch>/` (manifest.json {rel:{file,base,new}}, files/, base/, stage.py from the xplat5 template that keeps file modes, expected.json, verify.sh) from `rows = {rel: (base_bytes|None, new_bytes)}`; every edit script of `edits/` ends in it and reads its base from the CURRENT live files, so re-running it is the rebase."""
import hashlib, json, os, shutil
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE); LIVE = "/Users/ueli/Documents/semio"
sha = lambda data: hashlib.sha256(data).hexdigest()
def live_bytes(rel):
    with open(os.path.join(LIVE, rel), "rb") as f: return f.read()
def replace_once(text, old, new, label):
    if text.count(old) != 1: raise SystemExit("LOST ANCHOR (%d matches) %s" % (text.count(old), label))
    return text.replace(old, new)
def build(batch, title, rows, expected):
    out = os.path.join(HERE, "staged", batch)
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
    template = '"""' + title + "\n" + template[template.index("usage: python3 -I stage.py"):]
    open(os.path.join(out, "stage.py"), "w", encoding="utf-8").write(template)
    json.dump(expected, open(os.path.join(out, "expected.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    open(os.path.join(out, "verify.sh"), "w", encoding="utf-8").write('#!/bin/sh\n# ✅ laws of batch `%s` (lane isov): sh verify.sh <root>; stage check, parse scan, every staged test file runs (plain bun test, the official envelope when it asks for one) and fails only tests of its recorded set (expected.json), no live-store fence refusal, test-store-isolation law 0 findings. Expect `VERIFY OK %s`.\nexec python3 -I "$(cd "$(dirname "$0")/../.." && pwd)/verify_batch.py" "$(cd "$(dirname "$0")" && pwd)" "${1:?root}"\n' % (batch, batch))
    print(batch, len(manifest), "files staged ->", out)
