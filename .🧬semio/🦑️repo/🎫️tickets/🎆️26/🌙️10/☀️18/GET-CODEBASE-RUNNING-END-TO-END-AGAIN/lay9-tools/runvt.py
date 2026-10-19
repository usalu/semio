#!/usr/bin/env python3
import json, os, re, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/lay9-run"
ART = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta"
cfg, out = sys.argv[1], sys.argv[2]
extra = sys.argv[3:]
os.makedirs(ART, exist_ok=True)
wire = subprocess.run(["bun", C + "/tst2-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
abscfg = os.path.join(ROOT, cfg)
cwd = os.path.normpath(os.path.join(os.path.dirname(abscfg), "..", ".."))
pkg = os.path.join(cwd, "📦️packages", "🟦️typescript")
if os.path.isdir(pkg): cwd = pkg
bun = subprocess.run(["which","bun"],capture_output=True,text=True).stdout.strip()
policy = dict(version=1, cwd=cwd, toolPath=ROOT + "/node_modules/vitest/vitest.mjs", runtime=bun, coverageRuntime=bun, cacheRoot=G + "/vtcache", coverageDirectory=G + "/vtcov", budgetMs=120000)
env = dict(os.environ, SEMIO_VITEST_POLICY=json.dumps(policy), SEMIO_TEST_ARTIFACT_DIR=ART, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TEST_LEVEL="long", NO_COLOR="1", CI="1")
s = time.time()
p = subprocess.run(["node", ROOT + "/node_modules/vitest/vitest.mjs", "run", "--config", abscfg, "--testTimeout", "120000", "--hookTimeout", "120000", "--reporter=default"] + extra, cwd=cwd, env=env, capture_output=True, text=True, timeout=1700)
txt = (p.stdout or "") + (p.stderr or "")
open(out, "w", encoding="utf-8").write(txt)
print("rc", p.returncode, "secs", round(time.time()-s))
print("\n".join(re.findall(r"(?:Test Files|Tests)\s+.*", txt)))
print("\n".join([l for l in txt.splitlines() if "FAIL" in l or "Error" in l][:20]))
