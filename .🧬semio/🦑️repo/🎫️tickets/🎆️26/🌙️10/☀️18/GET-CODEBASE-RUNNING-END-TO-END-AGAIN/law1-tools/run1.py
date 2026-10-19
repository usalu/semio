#!/usr/bin/env python3
import os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G = C + "/🗑️generated/law1-out"
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
os.makedirs(G, exist_ok=True)
ART = SP + "/ta"
os.makedirs(ART, exist_ok=True)
wire = subprocess.run(["bun", C + "/law1-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
for f in sys.argv[1:]:
    if f.startswith("M/"): f = "🧰️framework/🔨️modules/" + f[2:]
    extra = ["--tsconfig-override", SP + "/" + os.environ.get("LAW1_HOIST", "hoist") + "/tsconfig.json"] if os.path.exists(SP + "/" + os.environ.get("LAW1_HOIST", "hoist") + "/tsconfig.json") else []
    try:
        p = subprocess.run(["bun", "test"] + extra + ["--timeout", "120000", "./" + f], cwd=ROOT, env=env, capture_output=True, timeout=900)
        txt = p.stdout.decode("utf-8", "replace") + p.stderr.decode("utf-8", "replace")
    except subprocess.TimeoutExpired:
        txt = "[runner] HARD CAP"
    import hashlib
    lp = G + "/" + hashlib.md5(f.encode()).hexdigest()[:8] + "-" + re.sub(r"[^A-Za-z0-9]+", "_", f)[-60:] + ".log"
    open(lp, "w", encoding="utf-8").write(txt)
    pas = re.findall(r"^\s*(\d+) pass", txt, re.M); fa = re.findall(r"^\s*(\d+) fail", txt, re.M)
    print(f"=== {f[-100:]}  pass={pas[-1] if pas else '?'} fail={fa[-1] if fa else '?'}  log={lp}")
    for m in re.finditer(r"^\(fail\) (.*)$", txt, re.M): print("  FAIL", m.group(1)[:200])
