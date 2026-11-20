import subprocess,time,os,glob,json,sys,re
S="/Users/ueli/Documents/semio/.cache/b/g/m/generations/s"
GS="/Users/ueli/Documents/semio/.cache/f/graph-server"
end=time.time()+float(sys.argv[1])
def ps():
    out=subprocess.run(["ps","-axo","pid=,ppid=,etime=,%cpu=,command="],capture_output=True,text=True).stdout
    r=[]
    for l in out.splitlines():
        if "ready-tools" in l: continue
        if re.search(r"show project|repo run|repo graph|read-record|repo service|graph-server",l) and "ps.py" not in l and "watch.py" not in l and "Claude" not in l and "zsh -c" not in l:
            l=re.sub(r"/Users/ueli/Documents/semio/\.cache/b/t/m/bun/[0-9a-f]+/bun ","bun ",l)
            r.append(l[:230])
    return r
last=None
while time.time()<end:
    cl=[]
    for f in sorted(glob.glob(S+"/*.json")):
        try:
            d=json.load(open(f))
            if "holder" in d:
                pid=d["holder"].get("pid");alive=False
                try: os.kill(pid,0);alive=True
                except Exception: pass
                cl.append((os.path.basename(f)[:6],d["state"],pid,alive,d["holder"]["command"][-40:]))
        except Exception as e: pass
    cur=(tuple(cl),tuple(ps()))
    if cur!=last:
        print(time.strftime("%H:%M:%SZ",time.gmtime()),"claims",cl)
        for p in cur[1]: print("   ",p)
        last=cur;sys.stdout.flush()
    time.sleep(10)
