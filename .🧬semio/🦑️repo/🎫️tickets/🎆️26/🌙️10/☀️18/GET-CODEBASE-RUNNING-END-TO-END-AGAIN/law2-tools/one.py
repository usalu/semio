#!/usr/bin/env python3
import os,subprocess,sys,re
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
ART=S+"/ta"; os.makedirs(ART,exist_ok=True)
wire=subprocess.run(["bun",C+"/tst2-tools/envwire.ts"],cwd=ROOT,capture_output=True,text=True).stdout
env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=ART,NO_COLOR="1",SEMIO_REPO_ROOT=ROOT,SEMIO_SCRIPT_PROCESS_INVOCATION=wire,SEMIO_TICKET_DIR=C,SEMIO_BUILD_MODE="prototype")
f=sys.argv[1]
extra=sys.argv[2:]
p=subprocess.run(["bun","test","--tsconfig-override",S+"/hoist/tsconfig.json","--timeout","120000"]+extra+["./"+f],cwd=ROOT,env=env,capture_output=True,timeout=600)
t=(p.stdout+p.stderr).decode("utf-8","replace")
open(C+"/🗑️generated/law2-out/last.log","w",encoding="utf-8").write(t)
lines=t.splitlines()
print("\n".join(lines[-int(os.environ.get("TAIL","60")):]) if os.environ.get("RAW") else "")
print("rc",p.returncode,[l for l in lines if re.match(r"\s*\d+ (pass|fail)\b",l)])
