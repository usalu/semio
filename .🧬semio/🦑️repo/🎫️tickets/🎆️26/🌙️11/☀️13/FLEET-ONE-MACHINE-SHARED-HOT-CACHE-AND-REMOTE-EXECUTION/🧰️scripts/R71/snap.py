import json,glob,os,time,re,datetime
S="/Users/Shared/semio-store/v1/origins/"
now=time.time()
print("== snap",datetime.datetime.utcnow().strftime("%H:%M:%SZ"))
for d in sorted(glob.glob(S+"g1-m*")):
    o=os.path.basename(d)
    try: r=json.load(open(d+"/ready.json"))
    except Exception: continue
    log=d+"/ready-log-workspace.txt"; hdr=tail=""
    if os.path.exists(log):
        L=open(log,errors="replace").read().splitlines()
        hdr=L[0][:24] if L else ""
        nb=[l for l in L if not re.match(r"^\[repo\] still",l)]
        tail=(nb[-1] if nb else "")[:170]
    fl=d+"/ready-log-workspace-failed.txt"; fh=""
    if os.path.exists(fl):
        F=open(fl,errors="replace").read().splitlines(); fh=F[0][:24]+" | "+(F[-1][:230] if F else "")
    print(o,r.get("state"),"READY" if r.get("ready") else "-",r.get("stage"),"try",r.get("attempts"),"exit",r.get("exit"),"peak",r.get("peakMegabytes"),"|log",hdr,"|",tail)
    if fh and r.get("stage")=="workspace": print("    failed:",fh)
