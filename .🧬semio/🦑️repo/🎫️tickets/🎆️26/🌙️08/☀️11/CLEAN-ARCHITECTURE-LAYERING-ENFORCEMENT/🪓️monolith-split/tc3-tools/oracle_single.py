"""Scoped resolution differential of one crate target: baseline = live tree with my pre-edit snapshots (tc3-tools/orig) restored, current = live tree. --test. usage: oracle_single.py <tag> <dir> <root rel to dir> <crate> [manifest dir rel to dir]"""
import os, shutil, subprocess, sys, tarfile, io
ROOT = "/Users/ueli/Documents/semio"; SB = "/Users/ueli/.cache/tc3"
G = f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
tag, rels, root, crate = sys.argv[1:5]
rel = rels.split(",")[0]
manifest = sys.argv[5] if len(sys.argv) > 5 else "📦️packages/🦀️rust"
sb = f"{SB}/{tag}"
if os.path.exists(sb): shutil.rmtree(sb)
os.makedirs(sb + "/base"); os.makedirs(sb + "/cur")
HERE = os.path.dirname(os.path.abspath(__file__))
for r in rels.split(","):
    for dp, dn, fn in os.walk(f"{ROOT}/{r}"):
        dn[:] = [d for d in dn if d not in ("target", "node_modules")]
        for f in fn:
            if f.endswith((".rs", ".json", ".toml", ".sql", ".feature", ".ts")):
                s = os.path.join(dp, f)
                for side in ("cur", "base"):
                    d = f"{sb}/{side}/{r}" + s[len(f"{ROOT}/{r}"):]
                    os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copyfile(s, d)
    for line in open(f"{HERE}/orig/index.tsv").read().splitlines():
        key, path = line.split("\t")[:2]
        if path.startswith(f"{ROOT}/{r}/"):
            d = f"{sb}/base/{r}" + path[len(f"{ROOT}/{r}"):]
            assert os.path.realpath(d).startswith(os.path.realpath(sb))
            shutil.copyfile(f"{HERE}/orig/{key}.orig", d)
def run(side):
    cmd = ["nice", "-n", "10", "rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--test", f"{sb}/{side}/{rel}/{root}"]
    return subprocess.run(cmd, capture_output=True, text=True, env=dict(os.environ, CARGO_MANIFEST_DIR=f"{sb}/{side}/{rel}/{manifest}")).stderr
open(f"{G}/tc3-oracle-{tag}-base.err", "w").write(run("base")); open(f"{G}/tc3-oracle-{tag}-cur.err", "w").write(run("cur"))
print("wrote", tag)
