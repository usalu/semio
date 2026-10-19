#!/usr/bin/env python3
import os, re, subprocess, sys
ROOT = "/Users/ueli/Documents/semio"
C = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
ART = SP + "/ta"; os.makedirs(ART, exist_ok=True)
wire = subprocess.run(["bun", C + "/law1-tools/envwire.ts"], cwd=ROOT, capture_output=True, text=True).stdout
env = dict(os.environ, SEMIO_TEST_ARTIFACT_DIR=ART, NO_COLOR="1", SEMIO_REPO_ROOT=ROOT, SEMIO_SCRIPT_PROCESS_INVOCATION=wire, SEMIO_TICKET_DIR=C, SEMIO_BUILD_MODE="prototype")
f = sys.argv[1]; pat = sys.argv[2]
hoist = SP + "/" + os.environ.get("LAW1_HOIST", "hoist") + "/tsconfig.json"
extra = ["--tsconfig-override", hoist] if os.path.exists(hoist) else []
p = subprocess.run(["bun", "test"] + extra + ["--timeout", "120000", "./" + f, "-t", pat], cwd=ROOT, env=env, capture_output=True, timeout=600)
out = p.stdout.decode("utf-8", "replace") + p.stderr.decode("utf-8", "replace")
pas = re.findall(r"^\s*(\d+) pass", out, re.M); fai = re.findall(r"^\s*(\d+) fail", out, re.M)
print("pass=%s fail=%s" % (pas[-1] if pas else "?", fai[-1] if fai else "?"))
if len(sys.argv) > 3:
    import urllib.parse
    for i, l in enumerate(out.split("\n")):
        if l.startswith("error:") or re.match(r"^[A-Za-z]*Error", l): print(urllib.parse.unquote(l)[:int(sys.argv[3])])
