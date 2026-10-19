#!/usr/bin/env python3
# usage: r.py <path-substring> [maxlines]  runs one candidate suite, prints failing tests + errors
import os,subprocess,sys,re
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
ART=ROOT+"/.cache/repot2/ta"; os.makedirs(ART,exist_ok=True)
wire=subprocess.run(["bun",C+"/repot2-tools/envwire.ts"],cwd=ROOT,capture_output=True,text=True,env=dict(os.environ,SEMIO_REPO_ROOT=ROOT)).stdout
import json as _j
W=_j.loads(wire)
env=dict(os.environ,NODE_PATH="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/hoist/node_modules",SEMIO_TEST_ARTIFACT_DIR=ART,NO_COLOR="1",SEMIO_REPO_ROOT=ROOT,SEMIO_SCRIPT_PROCESS_INVOCATION=W["invocation"],SEMIO_SCRIPT_PROVIDERS=W["providers"],SEMIO_TICKET_DIR=C,SEMIO_BUILD_MODE="prototype",SEMIO_WORKSPACE_ROOT=ROOT,SEMIO_BUILD_TOOL="nx")
L=[l.strip() for l in open(C+"/repot2-tools/list.txt",encoding="utf-8") if l.strip()]
m=[f for f in L if sys.argv[1] in f]
if not m and os.path.exists(sys.argv[1]): m=[sys.argv[1]]
if len(m)!=1:
    print("matches",len(m)); [print(x) for x in m[:10]]; sys.exit(1)
f=m[0]
extra=sys.argv[3:]
p=subprocess.run(["bun","test","--tsconfig-override",S+"/hoist/tsconfig.json","--timeout","120000"]+extra+["./"+f],cwd=ROOT,env=env,capture_output=True,timeout=900)
t=(p.stdout+p.stderr).decode("utf-8","replace")
os.makedirs(C+"/🗑️generated/repot2-out",exist_ok=True)
open(C+"/🗑️generated/repot2-out/last.log","w",encoding="utf-8").write(t)
lines=t.splitlines()
n=int(sys.argv[2]) if len(sys.argv)>2 else 40
fails=[l for l in lines if l.startswith("(fail)")]
print("\n".join(l[:200] for l in fails[:30]))
errs=[]
for i,l in enumerate(lines):
    if re.match(r"\s*(error|TypeError|ReferenceError|SyntaxError|Error)\b",l): errs.append(i)
shown=0
for i in errs:
    print("\n".join(x[:260] for x in lines[max(0,i-2):i+6])); print("--"); shown+=1
    if shown*8>n: break
seen=set()
for l in lines:
    m=re.search(r"at <anonymous> \((/[^)]+\.tsx?):(\d+):(\d+)\)",l)
    if m and m.groups() not in seen and len(seen)<int(os.environ.get("LOCS","4")):
        seen.add(m.groups())
        try:
            L=open(m.group(1),encoding="utf-8").read().split("\n")[int(m.group(2))-1]; c=int(m.group(3))
            print("@L%s:%s %s"%(m.group(2),m.group(3),L[max(0,c-int(os.environ.get("W","260"))):c+120]))
        except Exception as e: print("loc?",e)
print("rc",p.returncode,[l for l in lines if re.match(r"\s*\d+ (pass|fail)\b",l)])
