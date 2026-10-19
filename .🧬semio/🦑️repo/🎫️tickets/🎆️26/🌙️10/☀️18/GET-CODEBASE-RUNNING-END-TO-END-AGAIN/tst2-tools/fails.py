import re,subprocess,sys,os
T=sys.argv[1]
if not T.startswith("/"): T="/Users/ueli/Documents/semio/"+T
env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta",NO_COLOR="1")
w=subprocess.run(["bun","/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/tst2-tools/envwire.ts"],capture_output=True,text=True).stdout
env["SEMIO_SCRIPT_PROCESS_INVOCATION"]=w;env["SEMIO_TICKET_DIR"]="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out=subprocess.run(["bun","test","--timeout","120000",T],capture_output=True,text=True,cwd="/Users/ueli/Documents/semio",env=env).stderr
print(" | ".join(re.findall(r"^\s*(\d+ (?:pass|fail|skip))",out,re.M)))
width=int(sys.argv[2]) if len(sys.argv)>2 else 260
src=open(T,encoding="utf-8").read().split("\n")
seen=set()
for m in re.finditer(r"at (?:async )?<anonymous> \(([^)]*?):(\d+):(\d+)\)",out):
    f,ln,col=m.group(1),int(m.group(2)),int(m.group(3))
    if f!=T: continue
    if (ln,col) in seen: continue
    seen.add((ln,col))
    line=src[ln-1]
    print(f"L{ln}:{col}", line[max(0,col-width):col+50].replace("\n"," "))
for m in re.finditer(r"^error: (.*)$",out,re.M): print("  ERR",m.group(1)[:200])
