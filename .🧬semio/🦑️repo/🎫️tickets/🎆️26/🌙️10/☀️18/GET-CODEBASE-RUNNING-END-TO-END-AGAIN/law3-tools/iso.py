#!/usr/bin/env python3
import os, re, subprocess, sys, hashlib
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/law3-out"
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
ART = (C + "/🗑️generated/ta") if os.environ.get("LAW3_ART") == "ticket" else SP + "/ta"
os.makedirs(G, exist_ok=True); os.makedirs(ART, exist_ok=True)
def env_for():
    wire = subprocess.run(["bun", C + "/law3-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
    return dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
hoist = SP + "/" + os.environ.get("LAW3_HOIST", "hoist") + "/tsconfig.json"
extra = ["--tsconfig-override", hoist] if os.path.exists(hoist) else []
f = sys.argv[1]
if f.startswith("M/"): f = "🧰️framework/🔨️modules/" + f[2:]
only = sys.argv[2:] 
def run(args, timeout=400):
    try:
        p = subprocess.run(["bun", "test"] + extra + ["--timeout", "120000"] + args, cwd=ROOT, env=env_for(), capture_output=True, timeout=timeout)
        return p.stdout.decode("utf-8", "replace") + p.stderr.decode("utf-8", "replace")
    except subprocess.TimeoutExpired:
        return "[runner] HARD CAP"
full = run(["./" + f])
names = re.findall(r"^\(fail\) (.*?)(?: \[[\d.]+ms\])?$", full, re.M)
names = [n for n in dict.fromkeys(names) if n != "(unnamed)"]
pf = re.findall(r"^\s*(\d+) pass", full, re.M); ff = re.findall(r"^\s*(\d+) fail", full, re.M)
print(f"=== {f[-100:]} full: pass={pf[-1] if pf else '?'} fail={ff[-1] if ff else '?'}")
for n in names:
    if only and not any(o in n for o in only): continue
    out = run(["./" + f, "-t", "^" + re.escape(n) + "$"])
    pas = re.findall(r"^\s*(\d+) pass", out, re.M); fai = re.findall(r"^\s*(\d+) fail", out, re.M)
    ok = bool(pas) and int(pas[-1]) >= 1 and bool(fai) and int(fai[-1]) == 0
    lp = G + "/iso-" + hashlib.md5((f + n).encode()).hexdigest()[:8] + ".log"
    open(lp, "w", encoding="utf-8").write(out)
    print(("PASS " if ok else "FAIL ") + n[:150] + ("" if ok else "  log=" + os.path.basename(lp)))
