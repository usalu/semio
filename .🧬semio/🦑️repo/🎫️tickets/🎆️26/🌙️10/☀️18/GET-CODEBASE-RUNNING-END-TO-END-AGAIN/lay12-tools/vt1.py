#!/usr/bin/env python3
# usage: vt1.py <config relative to repo> <test file relative to repo> [more files]
import json, os, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/lay12-vt"
os.makedirs(G, exist_ok=True)
cfg = os.path.join(ROOT, sys.argv[1]); files = sys.argv[2:]
wire = subprocess.run(["bun", C + "/tst2-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
cwd = os.path.normpath(os.path.join(os.path.dirname(cfg), "..", ".."))
pkg = os.path.join(cwd, "📦️packages", "🟦️typescript")
if os.path.isdir(pkg): cwd = pkg
bun = subprocess.run(["which", "bun"], capture_output=True, text=True).stdout.strip()
policy = dict(version=1, cwd=cwd, toolPath=ROOT + "/node_modules/vitest/vitest.mjs", runtime=bun, coverageRuntime=bun, cacheRoot=G + "/vtcache", coverageDirectory=G + "/vtcov", budgetMs=600000)
env = dict(os.environ, SEMIO_VITEST_POLICY=json.dumps(policy), SEMIO_TEST_ARTIFACT_DIR=G + "/art", SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TEST_LEVEL=os.environ.get("VT_LEVEL","long"), NO_COLOR="1", CI="1")
os.makedirs(G + "/art", exist_ok=True)
p = subprocess.run(["node", ROOT + "/node_modules/vitest/vitest.mjs", "run", "--config", cfg, "--testTimeout", "300000", "--hookTimeout", "300000", "--reporter=default", *files], cwd=cwd, env=env, capture_output=True, text=True)
out = (p.stdout or "") + (p.stderr or "")
open(G + "/last.log", "w", encoding="utf-8").write(out); print(out[-2500:])
print("rc", p.returncode)
