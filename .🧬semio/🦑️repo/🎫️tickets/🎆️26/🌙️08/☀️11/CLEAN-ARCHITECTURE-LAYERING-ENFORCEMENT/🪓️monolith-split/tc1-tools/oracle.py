"""Scoped full-resolution differential (RESOLUTION-ORACLE.md) of the touched oracle crates: baseline sandbox (originals restored from orig/) vs current tree, --test cfg. Sandboxes live under /Users/ueli/.cache/tc1 only. usage: oracle.py <artifact-dir-name> <lib root rel to artifact> <crate-name> [more rustc args]"""
import os, re, shutil, subprocess, sys, collections
ROOT = "/Users/ueli/Documents/semio"
SB = "/Users/ueli/.cache/tc1"
HERE = os.path.dirname(os.path.abspath(__file__))
art, root_rel, crate, manifest_rel = sys.argv[1:5]
extra = sys.argv[5:]
ART = f"{ROOT}/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{art}"
idx = {}
for line in open(f"{HERE}/orig/index.tsv"):
    key, path = line.rstrip("\n").split("\t")
    idx[path] = f"{HERE}/orig/{key}.rs"
def tree(name, restore):
    dst = f"{SB}/{crate}/{name}"
    if os.path.exists(dst): shutil.rmtree(dst)
    os.makedirs(dst)
    for dp, dn, fn in os.walk(ART):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        for f in fn:
            if not (f.endswith((".rs", ".json", ".feature", ".semio", ".sql", ".graphql", ".proto", ".ts", ".tiff", ".jpg", ".pdf", ".ifc", ".mp4", ".pptx", ".docx", ".toml"))): continue
            s = os.path.join(dp, f); d = dst + s[len(ART):]
            os.makedirs(os.path.dirname(d), exist_ok=True)
            shutil.copyfile(s, d)
    if restore:
        for path, orig in idx.items():
            if path.startswith(ART + "/") and path.endswith(".rs"):
                d = dst + path[len(ART):]
                assert os.path.realpath(d).startswith(os.path.realpath(dst))
                shutil.copyfile(orig, d)
    return dst
def patch(dst):
    f = dst + "/" + root_rel
    t = open(f, encoding="utf-8").read()
    t = re.sub(r"^extern crate [^;]*;\n", "", t, flags=re.M)
    open(f, "w", encoding="utf-8").write(t)
def run(dst):
    if os.environ.get("PATCH"): patch(dst)
    cmd = ["nice", "-n", "10", "rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate.replace("-", "_"), "--emit=metadata", "-o", "/dev/null", "--test", *extra, dst + "/" + root_rel]
    p = subprocess.run(cmd, capture_output=True, text=True, env=dict(os.environ, CARGO_MANIFEST_DIR=dst + "/" + manifest_rel))
    return p.stderr
def norm(err, dst):
    rows = collections.Counter()
    lines = err.splitlines()
    for i, l in enumerate(lines):
        if l.startswith("error") and "aborting due to" not in l and "warning" not in l:
            span = ""
            for j in range(i + 1, min(i + 4, len(lines))):
                m = re.match(r"\s*--> (.*?):\d+:\d+", lines[j])
                if m: span = m.group(1).replace(dst + "/", ""); break
            rows[(re.sub(r"`[^`]*`", "`_`", l), span)] += 1
    return rows
base = tree("base", True); cur = tree("cur", False)
eb, ec = run(base), run(cur)
open(f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tc1-oracle-{crate}-base.err", "w").write(eb)
open(f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/tc1-oracle-{crate}-cur.err", "w").write(ec)
nb, nc = norm(eb, base), norm(ec, cur)
new = nc - nb; gone = nb - nc
print(crate, "base errors", sum(nb.values()), "cur errors", sum(nc.values()), "new", sum(new.values()), "gone", sum(gone.values()))
for (k, s), n in sorted(new.items()): print("  NEW", n, k[:140], "|", s[-90:])
for (k, s), n in sorted(gone.items()): print("  GONE", n, k[:140], "|", s[-90:])
