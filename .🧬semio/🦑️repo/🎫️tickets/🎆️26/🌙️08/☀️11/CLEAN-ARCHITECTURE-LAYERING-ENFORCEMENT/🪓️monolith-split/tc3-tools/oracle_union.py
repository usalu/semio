"""Scoped resolution differential for a crate + its new companion: baseline = git HEAD of <dir>, current = live tree with the companion mounted into the lib root (extern crate self as <crate>). --test. usage: oracle_union.py <tag> <dir> <lib root rel to dir> <crate> <companion root rel to dir>"""
import os, re, shutil, subprocess, sys, tarfile, io, collections
ROOT = "/Users/ueli/Documents/semio"; SB = "/Users/ueli/.cache/tc3"
G = f"{ROOT}/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
tag, rel, lib, crate, comp = sys.argv[1:6]
sb = f"{SB}/{tag}"
if os.path.exists(sb): shutil.rmtree(sb)
os.makedirs(sb + "/base"); os.makedirs(sb + "/cur")
arc = subprocess.run(["git", "-c", "core.quotepath=false", "archive", "HEAD", rel], cwd=ROOT, capture_output=True).stdout
tarfile.open(fileobj=io.BytesIO(arc)).extractall(sb + "/base")
for dp, dn, fn in os.walk(f"{ROOT}/{rel}"):
    dn[:] = [d for d in dn if d not in ("target", "node_modules")]
    for f in fn:
        if f.endswith((".rs", ".json", ".toml", ".sql", ".feature")):
            s = os.path.join(dp, f); d = f"{sb}/cur/{rel}" + s[len(f"{ROOT}/{rel}"):]
            os.makedirs(os.path.dirname(d), exist_ok=True); shutil.copyfile(s, d)
cur_root = f"{sb}/cur/{rel}/{lib}"
t = open(cur_root, encoding="utf-8").read()
t += f'\nextern crate self as {crate};\n#[cfg(test)]\n#[path = "{comp}"]\nmod companion_union;\n'
open(cur_root, "w", encoding="utf-8").write(t)
def run(root, manifest):
    cmd = ["nice", "-n", "10", "rustc", "+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--test", root]
    return subprocess.run(cmd, capture_output=True, text=True, env=dict(os.environ, CARGO_MANIFEST_DIR=manifest)).stderr
mf = f"{ROOT}/{rel}/" + os.environ.get("MANIFEST_REL", "📦️packages/🦀️rust")
eb = run(f"{sb}/base/{rel}/{lib}", mf); ec = run(cur_root, mf)
open(f"{G}/tc3-oracle-{tag}-base.err", "w").write(eb); open(f"{G}/tc3-oracle-{tag}-cur.err", "w").write(ec)
print("wrote", tag)
