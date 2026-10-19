import subprocess,sys,os,re
ROOT="/Users/ueli/Documents/semio"
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad"
wire=subprocess.run(["bun",C+"/tst2-tools/envwire.ts"],cwd=ROOT,capture_output=True,text=True).stdout
env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=S+"/ta",NO_COLOR="1",SEMIO_REPO_ROOT=ROOT,SEMIO_SCRIPT_PROCESS_INVOCATION=wire,SEMIO_TICKET_DIR=C,SEMIO_BUILD_MODE="prototype")
for f in sys.argv[1:]:
    p=subprocess.run(["bun","test","--tsconfig-override",S+"/hoist/tsconfig.json","--timeout","120000","./"+f],cwd=ROOT,env=env,capture_output=True,timeout=600)
    t=(p.stdout+p.stderr).decode("utf-8","replace")
    res=[l.strip() for l in t.splitlines() if re.match(r"\s*\d+ (pass|fail)\b",l)]
    print(f.split("subsets/")[-1][:70],p.returncode,res)
    if p.returncode!=0:
        for l in t.splitlines():
            if l.startswith("(fail)"):print("   ",l[:200])
