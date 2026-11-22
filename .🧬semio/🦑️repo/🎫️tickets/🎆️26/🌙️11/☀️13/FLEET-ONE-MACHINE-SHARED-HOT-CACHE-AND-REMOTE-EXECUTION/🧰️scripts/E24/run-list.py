import subprocess,sys,os,hashlib,re,concurrent.futures as cf
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
OUT=T+"/wave-1122/🗑️generated/E24"
lst,par,tmo=sys.argv[1],int(sys.argv[2]),int(sys.argv[3])
files=[l.split(" || ")[0].strip() if " || " in l else l.strip() for l in open(lst) if l.strip()]
def run(f):
    key=hashlib.sha1(f.encode()).hexdigest()[:10]
    log=f"{OUT}/{key}.log"
    env=dict(os.environ,SEMIO_TEST_PYTHON_BINARY=subprocess.check_output(["/bin/sh","-c","command -v python3"]).decode().strip())
    try:
        p=subprocess.run([T+"/🧰️scripts/E19/sf.sh","⚡️caching/"+f],capture_output=True,text=True,timeout=tmo,env=env)
        out=p.stdout+p.stderr;rc=p.returncode
    except subprocess.TimeoutExpired as e:
        out=(e.stdout or b"").decode(errors="replace") if isinstance(e.stdout,bytes) else (e.stdout or "");rc="TIMEOUT"
    open(log,"w").write(out)
    m=re.search(r"^\s*(\d+) pass",out,re.M);k=re.search(r"^\s*(\d+) fail",out,re.M)
    err=re.findall(r"^error: (.*)$",out,re.M)
    return f"{key} rc={rc} pass={m.group(1) if m else '?'} fail={k.group(1) if k else '?'} {f} :: {(err[0] if err else '')[:160]}"
with cf.ThreadPoolExecutor(par) as ex:
    for r in ex.map(run,files):
        print(r,flush=True)
