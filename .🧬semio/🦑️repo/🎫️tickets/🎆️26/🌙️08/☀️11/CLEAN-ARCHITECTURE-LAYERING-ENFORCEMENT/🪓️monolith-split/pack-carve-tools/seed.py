"""🌱 Seeded sensitivity check: breaks visibility in a scratch copy of the union sandbox and expects new errors."""
import os, re, shutil, sys, pickle
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import union, oracle as o
from lib import *
src = SP + "/S1"; dst = SP + "/S2"
shutil.rmtree(dst, ignore_errors=True); shutil.copytree(src, dst, symlinks=True)
def edit(rel, old, new, count=1):
    p = os.path.join(dst, rel); t = open(p, encoding="utf8").read()
    assert t.count(old) >= 1, (rel, old)
    open(p, "w", encoding="utf8").write(t.replace(old, new, count))
FMT = P + "📐️format/🦀️.rs"
edit(FMT, "pub const REQUIRED_KNOWN_MASK", "const REQUIRED_KNOWN_MASK")
edit(FMT, "pub struct RawManifest {\n    pub schema_symref", "pub struct RawManifest {\n    schema_symref")
edit(FMT, "pub async fn open_manifest<", "async fn open_manifest<") if "pub async fn open_manifest<" in open(os.path.join(dst, FMT), encoding="utf8").read() else None
SCH = P + "🌱️value/🧬️schema/🦀️.rs"
edit(SCH, "pub const TAG_ABSENT", "const TAG_ABSENT")
REC = P + "🌱️value/🦀️.rs"
lib = os.path.join(dst, P, "📦️packages/🦀️rust/🦀️.rs")
base = pickle.load(open(SP + "/base-pack.pkl", "rb"))
for tag in ("native", "test"):
    ex = list(o.CFGS[tag]) + ["--cfg", 'feature="deflate"']
    c, err = o.run(dst, lib, "pack", ex)
    b, bi = union.parse_sig(base[tag]); ce, ci = union.parse_sig(c)
    new = ci - bi
    print(tag, "new internal errors:", sum(new.values()))
    for (code, m), n in sorted(new.items())[:12]: print("  ", n, code, m[:150])
