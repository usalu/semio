import json,glob,os,re,sys,time,datetime
S="/Users/Shared/semio-store/v1/origins/"
ansi=re.compile(r"\x1b\[[0-9;]*m")
for o in sys.argv[1:] or ["g1-m01","g1-m11","g1-m12"]:
    d=S+o+"/";row={"origin":o}
    try:
        r=json.load(open(d+"ready.json"));row.update(state=r.get("state"),stage=r.get("stage"),code=(r.get("refusal") or {}).get("code"),att=r.get("attempts"),upd=datetime.datetime.fromtimestamp(r["updatedMs"]/1000).strftime("%H:%M:%S"))
    except Exception as e: row["err"]=str(e)[:60]
    best=None;enospc=[]
    for f in glob.glob(d+"ready-log-*.txt"):
        t=ansi.sub("",open(f,errors="replace").read())
        m=re.findall(r"\.cache/b/r/m: (\d+) of (\d+)",t)
        if m and (best is None or os.path.getmtime(f)>best[0]): best=(os.path.getmtime(f),m[-1],os.path.basename(f))
        for line in t.splitlines():
            if "ENOSPC" in line and not re.match(r"^\s*\d+ \|",line): enospc.append((os.path.getmtime(f),os.path.basename(f),line[:110]))
    if best: row["r_free_of"]=best[1];row["at"]=datetime.datetime.fromtimestamp(best[0]).strftime("%H:%M:%S");row["log"]=best[2]
    row["enospc_logs"]=sorted(set((datetime.datetime.fromtimestamp(a).strftime("%H:%M"),b) for a,b,_ in enospc))[-4:]
    print(json.dumps(row,ensure_ascii=False))
