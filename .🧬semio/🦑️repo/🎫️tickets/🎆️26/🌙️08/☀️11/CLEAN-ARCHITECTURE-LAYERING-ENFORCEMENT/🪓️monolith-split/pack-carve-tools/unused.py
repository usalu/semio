"""🧹 Reports unused imports (and other lint-level warnings) of the union sandbox with the crate-wide allow removed."""
import os, re, json, subprocess, sys, collections, shutil
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import union, oracle as o
from lib import *
src = SP + "/S1"; dst = SP + "/S4"
shutil.rmtree(dst, ignore_errors=True); shutil.copytree(src, dst, symlinks=True)
g = os.path.join(dst, P, "📦️packages/🦀️rust/🦀️.rs")
t = open(g, encoding="utf8").read()
assert "#![allow(ambiguous_glob_reexports, unused_imports)]" in t
open(g, "w", encoding="utf8").write(t.replace("#![allow(ambiguous_glob_reexports, unused_imports)]", ""))
def warnings(tag_extra, label):
    cmd = ["rustc", o.TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", "pack", "--emit=metadata", "-o", "/dev/null", "--error-format=json", "-W", "unused_imports", *tag_extra, "--cfg", 'feature="deflate"', g]
    p = subprocess.run(cmd, cwd=os.path.dirname(g), capture_output=True, text=True)
    out = collections.Counter(); detail = []
    for line in p.stderr.splitlines():
        try: d = json.loads(line)
        except Exception: continue
        if d.get("level") != "warning": continue
        code = (d.get("code") or {}).get("code")
        if code != "unused_imports": continue
        sp = next((s for s in d["spans"] if s["is_primary"]), None)
        if not sp: continue
        f = sp["file_name"].split("/")[-3] + "/" + sp["file_name"].split("/")[-2]
        out[f] += 1; detail.append((f, sp["line_start"], d["message"][:120], sp["text"][0]["text"].strip()[:120] if sp["text"] else ""))
    print(label, sum(out.values()))
    return detail
for label, extra in (("native", []), ("test", ["--test"])):
    det = warnings(extra, label)
    for x in det[:80]: print("  ", *x)
