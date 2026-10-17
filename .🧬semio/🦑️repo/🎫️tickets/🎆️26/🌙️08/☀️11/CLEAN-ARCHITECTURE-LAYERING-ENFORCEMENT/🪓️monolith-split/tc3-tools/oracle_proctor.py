"""Scoped full-resolution differential of the proctor carve (--test): baseline = git HEAD of 🎓️teaching/🛂️proctor (clean at start), current = live tree with the three crates unioned into one crate (extern crate self as ...). Sandboxes under /Users/ueli/.cache/tc3/proctor only. usage: oracle_proctor.py [lib|test]"""
import os, re, shutil, subprocess, sys, collections, tarfile, io
ROOT = "/Users/ueli/Documents/semio"
SB = "/Users/ueli/.cache/tc3/proctor"
G = f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
mode = sys.argv[1] if len(sys.argv) > 1 else "test"
REL = "🎓️teaching/🛂️proctor"
if os.path.exists(SB): shutil.rmtree(SB)
os.makedirs(SB + "/base"); os.makedirs(SB + "/cur")
arc = subprocess.run(["git", "-c", "core.quotepath=false", "archive", "HEAD", REL], cwd=ROOT, capture_output=True).stdout
tarfile.open(fileobj=io.BytesIO(arc)).extractall(SB + "/base")
def copy(src, dst):
    for dp, dn, fn in os.walk(src):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        for f in fn:
            if f.endswith((".rs", ".json", ".toml", ".sql")):
                s = os.path.join(dp, f); d = dst + s[len(src):]
                os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copyfile(s, d)
copy(f"{ROOT}/{REL}", f"{SB}/cur/{REL}")
mods = ["🗄️storage", "📚️catalog", "🎚️config", "👥️presence", "🎭️actors", "👪️crowd", "🔭️projections", "❓️queries", "🧩️instance", "⌨️cli"]
names = {"🗄️storage": "storage", "📚️catalog": "catalog", "🎚️config": "config", "👪️crowd": "crowd", "👥️presence": "presence", "🎭️actors": "actors", "🔭️projections": "projections", "❓️queries": "queries", "🧩️instance": "instance", "⌨️cli": "cli"}
union = '#![allow(async_fn_in_trait)]\nextern crate self as teaching_proctor_base;\nextern crate self as teaching_proctor_domain;\n' + "".join(f'#[path = "../../🔨️modules/{m}/🦀️.rs"]\npub mod {names[m]};\n' for m in mods)
cur_root = f"{SB}/cur/{REL}/📦️packages/🦀️rust/union.rs"
open(cur_root, "w", encoding="utf-8").write(union)
base_root = f"{SB}/base/{REL}/📦️packages/🦀️rust/🦀️.rs"
def run(root, dst):
    cmd = ["nice", "-n", "10", "rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", "proctor", "--emit=metadata", "-o", "/dev/null"] + (["--test"] if mode == "test" else []) + ["--cfg", 'feature="testkit"'] + [root]
    p = subprocess.run(cmd, capture_output=True, text=True, env=dict(os.environ, CARGO_MANIFEST_DIR=os.path.dirname(root)))
    return p.stderr
def norm(err, dst):
    rows = collections.Counter(); lines = err.splitlines()
    for i, l in enumerate(lines):
        if l.startswith("error") and "aborting due to" not in l and "temp dir" not in l:
            span = ""
            for j in range(i + 1, min(i + 4, len(lines))):
                m = re.match(r"\s*--> (.*?):\d+:\d+", lines[j])
                if m: span = m.group(1).replace(dst + "/", ""); break
            rows[(re.sub(r"`[^`]*`", "`_`", l), span)] += 1
    return rows
eb, ec = run(base_root, f"{SB}/base"), run(cur_root, f"{SB}/cur")
open(f"{G}/tc3-oracle-proctor-{mode}-base.err", "w").write(eb); open(f"{G}/tc3-oracle-proctor-{mode}-cur.err", "w").write(ec)
nb, nc = norm(eb, f"{SB}/base"), norm(ec, f"{SB}/cur")
print(mode, "base errors", sum(nb.values()), "cur errors", sum(nc.values()))
bk = collections.Counter(); ck = collections.Counter()
for (k, s), n in nb.items(): bk[k] += n
for (k, s), n in nc.items(): ck[k] += n
for k in sorted(set(bk) | set(ck)):
    if bk[k] != ck[k]: print("  DIFF base", bk[k], "cur", ck[k], k[:150])
