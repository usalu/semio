import os,sys,shutil,subprocess,json,re
sys.path.insert(0,os.path.dirname(__file__))
from famify import famify
from ed import ROOT
C=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
ART="/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/ta"
wire=subprocess.run(["bun",C+"/tst2-tools/envwire.ts"],cwd=ROOT,capture_output=True,text=True).stdout
env=dict(os.environ,SEMIO_TEST_ARTIFACT_DIR=ART,SEMIO_SCRIPT_PROCESS_INVOCATION=wire,SEMIO_TICKET_DIR=C,SEMIO_BUILD_MODE="prototype",NO_COLOR="1",SEMIO_REPO_ROOT=ROOT)
def run(f):
    p=subprocess.run(["bun","test","--tsconfig-override","/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad/hoist/tsconfig.json","--timeout","120000","./"+f],cwd=ROOT,env=env,capture_output=True,timeout=900)
    return p.returncode,(p.stdout+p.stderr).decode("utf-8","replace")
res=[]
for f in sys.argv[1:]:
    p=os.path.join(ROOT,f); bak=p+".tst2bak"
    shutil.copy(p,bak)
    n=famify(f)
    if n==0:
        os.remove(bak); print("NOCHANGE",f[-70:]); continue
    rc,out=run(f)
    if rc==0:
        os.remove(bak); print("FIXED  ",n,f[-70:])
    else:
        shutil.move(bak,p); m=re.findall(r"^\s*(\d+) fail",out,re.M); print("REVERT ",n,m[-1:] ,f[-70:])
