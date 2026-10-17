"""Runs the test-closure census over the top crate, every bin, companion and fixture crate and every crate the slice edited. usage: census_run.py <tag> [extra suffix ...]"""
import subprocess, sys
from lib import *
tag = sys.argv[1]
crates = crate_table()
suffixes = ["fixtures", "frame-tests-1", "test-support", "engine-attach-tests", "scene-ink-tests", "ui-engine-tests", "runtime-mailbox-tests", "host-winit", "frame-job", "frame-actions", "engine-registry", "engine-attach", "scene-state", "engine-graph", "kernel-protocol", "kernel-runtime", "kernel-pool", "program-bridge", "scene-ink", "scene-tables", "dock", "native-probe"] + [s for s in crates if s.startswith("shell-tests-")] + sys.argv[2:]
manifests = [rel(T + "/📦️packages/🦀️rust/Cargo.toml")]
for s in dict.fromkeys(suffixes):
    if s in crates:
        manifests.append(rel(T + "/" + crates[s] + "/📦️packages/🦀️rust/Cargo.toml"))
env = dict(os.environ, MANIFESTS=";".join(manifests), DUMP_INPUTS="1")
log = open(GEN + f"/rtb-census-{tag}.log", "w")
p = subprocess.run(["bun", "rtb-census.ts"], cwd=HERE, env=env, stdout=log, stderr=subprocess.STDOUT)
print("exit", p.returncode, "manifests", len(manifests))
