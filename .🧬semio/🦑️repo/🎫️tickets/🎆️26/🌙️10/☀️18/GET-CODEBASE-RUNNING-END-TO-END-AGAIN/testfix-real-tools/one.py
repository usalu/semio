#!/usr/bin/env python3
"""Runs ONE explicit bun test file in the overlay (or LIVE=1 in the real checkout) with a minted process envelope. usage: one.py REL [--test-name-pattern RX]"""
import json, os, re, shutil, subprocess, sys, tempfile
LIVE = os.environ.get("LIVE") == "1"
ROOT = "/Users/ueli/Documents/semio" if LIVE else os.environ.get("TFR_ROOT", "/private/tmp/testfix-real/r")
T = os.path.dirname(os.path.abspath(__file__))
rel = sys.argv[1]; extra = sys.argv[2:]
art = "/private/tmp/testfix-real/art"; os.makedirs(art, exist_ok=True)
wire = ROOT + "/.tfr-envwire.ts"
shutil.copyfile(T + "/envwire.ts", wire)
w = subprocess.run(["bun", wire], cwd=ROOT, capture_output=True, text=True)
try: env_wire = json.loads(w.stdout)
except Exception: print("ENVWIRE FAILED", w.stderr[-800:]); env_wire = {}
d = tempfile.mkdtemp(dir=art)
env = dict(os.environ, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, REPO_ROOT=ROOT, SEMIO_BUILD_MODE="prototype", SEMIO_TEST_ARTIFACT_DIR=d, **env_wire)
p = subprocess.run(["perl", "-e", "alarm 280; exec @ARGV", "--", "bun", "test", "--timeout", "120000", "./" + rel] + extra, cwd=ROOT, env=env, capture_output=True)
txt = re.sub(r"\x1b\[[0-9;]*m", "", p.stdout.decode("utf-8", "replace") + p.stderr.decode("utf-8", "replace"))
shutil.rmtree(d, ignore_errors=True)
os.makedirs("/private/tmp/testfix-real/logs", exist_ok=True)
lp = "/private/tmp/testfix-real/logs/" + re.sub(r"[^A-Za-z0-9]+", "_", rel)[-100:] + ".log"
open(lp, "w").write(txt)
print("rc", p.returncode, "log", lp)
for l in txt.splitlines():
    if l.startswith("(fail)") or re.match(r"^\s*\d+ (pass|fail)\b", l): print(l[:240])
