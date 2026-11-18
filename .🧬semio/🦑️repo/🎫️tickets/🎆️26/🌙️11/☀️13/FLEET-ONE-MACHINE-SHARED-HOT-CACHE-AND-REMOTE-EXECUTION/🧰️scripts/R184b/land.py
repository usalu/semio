import os,sys,shutil,subprocess,time,datetime,hashlib
sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import patches as P
L=os.environ.get("R184_ROOT","/Users/ueli/Documents/semio")
TEST=os.environ.get("R184_ROOT") is not None
T=L+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R184"
BUN="/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
LOCK=T+"/landing.lock"
now=lambda:datetime.datetime.now(datetime.timezone.utc)
def window():
    m=now().minute
    return 0<=m<5 or 30<=m<35
dry=len(sys.argv)>1 and sys.argv[1]=="--dry"
patched={rel:P.patched(L,rel) for rel in P.P}
for rel in P.NEW: assert not os.path.exists(f"{L}/{rel}"),("exists",rel)
if dry:
    for rel,t in patched.items(): print("ok",len(t),rel[-60:])
    sys.exit(0)
while not TEST and not window():
    if not (28<=now().minute<30 or 58<=now().minute): print("not near a window",now()); sys.exit(1)
    time.sleep(1)
waited=0
while True:
    try: os.mkdir(LOCK); break
    except FileExistsError:
        time.sleep(2);waited+=2
        if waited>280 or (not TEST and not window()): print("no lock:",open(LOCK+"/owner").read() if os.path.exists(LOCK+"/owner") else "?"); sys.exit(1)
start=now();open(LOCK+"/owner","w").write("claude-R184 "+start.strftime("%Y-%m-%dT%H:%M:%SZ")+"\n")
base={}
def restore():
    for rel,t in base.items(): open(f"{L}/{rel}","w",encoding="utf8").write(t)
    for rel in P.NEW:
        if os.path.exists(f"{L}/{rel}"): os.remove(f"{L}/{rel}")
    print("restored",now())
def release():
    try: os.remove(LOCK+"/owner")
    except FileNotFoundError: pass
    try: os.rmdir(LOCK)
    except OSError: pass
try:
    for rel in P.P: base[rel]=open(f"{L}/{rel}",encoding="utf8").read()
    os.makedirs(S+"/livebase_b",exist_ok=True)
    for i,(rel,t) in enumerate(base.items()): open(f"{S}/livebase_b/{i}.orig","w",encoding="utf8").write(t)
    for rel in P.P: patched[rel]=P.patched(L,rel)
    for rel,t in patched.items(): open(f"{L}/{rel}","w",encoding="utf8").write(t)
    for rel in P.NEW:
        os.makedirs(os.path.dirname(f"{L}/{rel}"),exist_ok=True); shutil.copyfile(f"{S}/mirror/{rel}",f"{L}/{rel}")
    env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))};env["BUN_RUNTIME_TRANSPILER_CACHE_PATH"]="0"
    B=P.B;OB=P.OB
    SN=P.SN
    tests=[(f"{P.N}/🧪️tests/🫥️self-progress/🟦️.ts",0),(f"{P.N}/🧪️tests/🔁️retry-alone/🟦️.ts",0),(f"{P.N}/🧪️tests/🪂️set-aside/🟦️.ts",0),(f"{P.N}/🧪️tests/🌍️bigger-host/🟦️.ts",0),(f"{SN}/🧪️tests/🤖️machine-local-dirs/🟦️.ts",0),(f"{SN}/🧪️tests/🕳️hollow/🟦️.ts",0),(f"{SN}/🧪️tests/🟦️.ts",2),(f"{OB}/🧪️tests/🪜️row-sequence/🟦️.ts",0),(f"{B}/🕸️graph/🕰️generations/🧪️tests/🧱️stable-directories/🟦️.ts",0),(f"{B}/🕸️graph/🧪️tests/🪶️hold-order/🟦️.ts",0),(f"{B}/🪪️graph-state/📦️project-records/🧪️tests/🤝️pool-reader/🟦️.ts",0),(f"{B}/🪪️graph-state/📦️project-records/🌱️fresh/🧪️tests/🟦️.ts",0)]
    bad="forced" if os.environ.get("R184_FORCE_RED") else None
    for rel,allowed in ([] if TEST else tests):
        d=os.path.dirname(f"{L}/{rel}");f=os.path.basename(rel)
        res=subprocess.run(["nice","-n","10",BUN,"test","./"+f,"--timeout","120000"],cwd=d,env=env,capture_output=True,text=True)
        text=res.stdout+res.stderr
        fails=[int(l.split()[0]) for l in text.splitlines() if l.strip().endswith(" fail") and l.split()[0].isdigit()]
        passes=[int(l.split()[0]) for l in text.splitlines() if l.strip().endswith(" pass") and l.split()[0].isdigit()]
        print(rel[-70:],passes,fails,flush=True)
        if not fails or fails[0]>allowed or not passes: bad=rel;break
    if bad is None and not TEST:
        smoke=subprocess.run([BUN,"-e",f'await import("{L}/{OB}/🟨️.mjs");await import("{L}/{SN}/🟨️.mjs");await import("{L}/{P.N}/🟨️.mjs");console.log("smoke ok")'],env=env,capture_output=True,text=True);print(smoke.stdout.strip(),smoke.stderr.strip()[:300])
        if "smoke ok" not in smoke.stdout: bad="smoke"
    if bad is not None:
        print("RED",bad);restore()
    else: print("GREEN",start.strftime("%H:%M:%S"),now().strftime("%H:%M:%S"))
except BaseException as e:
    print("ERROR",repr(e));restore()
finally:
    release()
