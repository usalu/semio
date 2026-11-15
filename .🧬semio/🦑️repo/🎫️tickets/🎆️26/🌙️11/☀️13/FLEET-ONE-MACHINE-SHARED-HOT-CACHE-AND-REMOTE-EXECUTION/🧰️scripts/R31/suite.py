"""R31: runs the test files of land.py on the live tree without landing. usage: suite.py"""
import sys,os,subprocess
sys.argv=[sys.argv[0],"--none"]
src=open(os.path.join(os.path.dirname(os.path.abspath(__file__)),"land.py"),encoding="utf8").read().replace("\nmain()\n","\n")
ns={"__file__":os.path.abspath(__file__)};exec(compile(src,"land.py","exec"),ns)
R,BUN,TESTS,edits=ns["R"],ns["BUN"],ns["TESTS"],ns["edits"]
bad=0
for rel in TESTS:
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    r=subprocess.run([BUN,"test","./🟦️.ts","--timeout","180000"],cwd=R+"/"+rel,env=env,capture_output=True,text=True)
    text=r.stdout+r.stderr;failed=[l for l in text.splitlines() if l.startswith("(fail)")];summary=[l.strip() for l in text.splitlines() if l.strip().endswith(("pass","fail"))]
    bad+=r.returncode!=0;print("ok  " if r.returncode==0 else "RED ",rel[len(edits.L):],summary,*failed,flush=True)
print("red files:",bad)
