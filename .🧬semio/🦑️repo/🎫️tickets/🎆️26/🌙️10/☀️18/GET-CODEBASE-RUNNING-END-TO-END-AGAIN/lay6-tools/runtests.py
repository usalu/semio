#!/usr/bin/env python3
import json, os, re, subprocess, sys, time
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
lst, out = sys.argv[1], sys.argv[2]
files = [l.strip() for l in open(lst, encoding="utf-8") if l.strip()]
ART = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta"
os.makedirs(ART, exist_ok=True)
wire = subprocess.run(["bun", C + "/tst2-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
res = {}
for f in files:
    s = time.time()
    try:
        p = subprocess.run(["bun", "test", "--timeout", "120000", "./" + f], cwd=ROOT, env=env, capture_output=True, timeout=900)
        txt = (p.stdout or b"").decode("utf-8", "replace") + (p.stderr or b"").decode("utf-8", "replace"); rc = p.returncode
    except subprocess.TimeoutExpired:
        txt = "[runner] HARD CAP"; rc = -999
    def num(k):
        m = re.findall(r"^\s*(\d+) " + k + r"\b", txt, re.M); return int(m[-1]) if m else 0
    res[f] = dict(rc=rc, pass_=num("pass"), fail=num("fail"), err=num("error"), secs=round(time.time()-s,1), tail=txt[-600:] if rc else "")
    print(f.split("💻️os/")[-1][-90:], res[f]["rc"], res[f]["pass_"], res[f]["fail"], res[f]["secs"], flush=True)
json.dump(res, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
