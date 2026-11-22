import json,os,subprocess,sys,time
HERE=os.path.dirname(os.path.abspath(__file__))
G=HERE+"/../../wave-1122/🗑️generated/E19/"
ROOT="/Users/ueli/Documents/semio/"
budget=float(sys.argv[1]); out=sys.argv[2]; lst=sys.argv[3]
files=[l.strip() for l in open(G+lst) if l.strip()]
done=set(json.loads(l)["file"] for l in open(G+out)) if os.path.exists(G+out) else set()
t0=time.time()
for f in files:
    if f in done: continue
    if time.time()-t0>budget: break
    while os.getloadavg()[0]>40 and time.time()-t0<budget: time.sleep(20)
    s=time.time()
    try:
        p=subprocess.run([HERE+"/sf.sh",ROOT+f],capture_output=True,text=True,errors="replace",timeout=900); rc=p.returncode; txt=p.stdout+p.stderr
    except subprocess.TimeoutExpired:
        rc,txt=-9,"WALL-TIMEOUT"
    import re
    dig=[l[:230] for l in txt.split("\n") if re.match(r"^\s*(error|Expected|Received|\(fail\)|\[E19\])|^\s*\d+ (pass|fail)$|^\s+at .*(🧪️tests)",l)]
    open(G+"sfdigest.txt","a").write("=== %s rc=%s\n%s\n"%(f,rc,"\n".join(dig[:40])))
    open(G+"sflogs.txt","a").write("=== %s rc=%s %.1fs\n%s\n"%(f,rc,time.time()-s,txt[-3500:] if rc else txt[-300:]))
    open(G+out,"a").write(json.dumps({"file":f,"rc":rc,"sec":round(time.time()-s,1)},ensure_ascii=False)+"\n")
