import glob,subprocess,os,sys
root=os.getcwd();G=sys.argv[1]
for i,f in enumerate(sorted(glob.glob(G+"/🗑️generated/alloc4-ws-*.txt"))):
    lines=open(f,encoding="utf-8").read().split("\n");wd=lines[0];names=[n for n in lines[1:] if n]
    cmd=["repo","cargo","--store=shared","--configuration=prototype","--parallelism=4","--","check","--tests"]
    for n in names:cmd+=["-p",n]
    r=subprocess.run(cmd,cwd=os.path.join(root,wd),capture_output=True,text=True)
    open(f"{G}/🗑️generated/alloc4-check-{i}.log","w",encoding="utf-8").write(wd+"\n"+r.stdout+r.stderr)
    errs=[l for l in (r.stdout+r.stderr).split("\n") if l.startswith("error")]
    print(i,wd,"exit",r.returncode,len(errs),flush=True)
