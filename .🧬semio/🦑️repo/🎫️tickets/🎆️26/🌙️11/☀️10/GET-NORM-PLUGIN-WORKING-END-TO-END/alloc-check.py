import tomllib,subprocess,os,sys
root=os.getcwd();G=sys.argv[1];names=sys.argv[2:]
tomls=subprocess.run(["find","✏️s","🧰️framework","🌎️hub","-name","Cargo.toml","-not","-path","*/target/*","-not","-path","*/node_modules/*"],capture_output=True,text=True).stdout.split("\n")
idx={}
for t in tomls:
    if not t:continue
    try:d=tomllib.load(open(t,"rb"))
    except Exception:continue
    if "package" in d:idx[d["package"]["name"]]=(t,d)
for n in names:
    mode="test --no-run" if n.startswith("+") else "check --tests"
    n=n.lstrip("+")
    t,d=idx[n]
    w=d["package"].get("workspace")
    wd=os.path.normpath(os.path.join(os.path.dirname(t),w)) if isinstance(w,str) else root
    log=f"{G}/alloc-check-{n}.log"
    cmd=["repo","cargo","--store=shared","--configuration=prototype","--parallelism=4","--"]+mode.split()[:1]+["-p",n]+(["--tests"] if mode.startswith("check") else ["--no-run"])
    r=subprocess.run(cmd,cwd=wd,capture_output=True,text=True)
    open(log,"w").write(r.stdout+r.stderr)
    errs=[l for l in (r.stdout+r.stderr).split("\n") if l.startswith("error")]
    print(n,"exit",r.returncode,errs[:3],flush=True)
