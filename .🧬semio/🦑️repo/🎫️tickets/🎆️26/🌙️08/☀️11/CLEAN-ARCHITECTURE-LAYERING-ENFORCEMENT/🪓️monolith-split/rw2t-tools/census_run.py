"""📊 rw2t: test-closure census of the WP2 crates and the bins that host their laws. usage: census_run.py <tag> [extra suffix ...]"""
import subprocess, sys, os
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rtb-tools")
from lib import *
HERE2 = os.path.dirname(os.path.abspath(__file__))
tag = sys.argv[1]
crates = crate_table()
WP2 = ["engine-registry", "engine-text", "engine-board", "engine-graph", "engine-attach", "scene-state", "canvas2d-host", "scene-canvas2d", "scene-ink", "scene-feeds", "scene-tables", "scene-text-vfs", "icon-host", "scene-render"]
BINS = ["engine-attach-tests", "scene-ink-tests", "ui-engine-tests"]
suffixes = WP2 + BINS + sys.argv[2:]
manifests = []
for s in dict.fromkeys(suffixes):
    if s in crates:
        manifests.append(rel(T + "/" + crates[s] + "/📦️packages/🦀️rust/Cargo.toml"))
env = dict(os.environ, MANIFESTS=";".join(manifests), DUMP_INPUTS="1")
log = open(GEN + f"/rw2t-census-{tag}.log", "w")
p = subprocess.run(["bun", HERE2 + "/census.ts"], cwd=HERE2, env=env, stdout=log, stderr=subprocess.STDOUT)
print("exit", p.returncode, "manifests", len(manifests))
