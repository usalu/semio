import hashlib,json,os,shutil,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__))
def find_root():
    path=HERE
    while path!=os.path.dirname(path):
        if os.path.isfile(os.path.join(path,"bunfig.toml")) and os.path.isfile(os.path.join(path,"bun.lock")):return path
        path=os.path.dirname(path)
    raise SystemExit("repository root not found")
ROOT=find_root()
STAGED=os.path.join(HERE,"staged")
BACKUP=os.path.join(HERE,"backup")
bases=json.load(open(os.path.join(HERE,"bases.json"),encoding="utf8"))
def sha(path):
    return hashlib.sha256(open(path,"rb").read()).hexdigest() if os.path.isfile(path) else None
def staged(rel):
    return sha(os.path.join(STAGED,rel))
def drift():
    rows=[]
    for rel,base in bases.items():
        now=sha(os.path.join(ROOT,rel))
        if now!=base and now!=staged(rel):rows.append((rel,base,now))
    return rows
def state():
    out={}
    for rel in bases:
        now=sha(os.path.join(ROOT,rel))
        out[rel]="applied" if now==staged(rel) else "base" if now==bases[rel] else "drifted"
    return out
def sync_running():
    listing=subprocess.run(["ps","-ww","-eo","pid,command"],capture_output=True,text=True).stdout
    found=[]
    for line in listing.splitlines()[1:]:
        pid,_,command=line.strip().partition(" ")
        words=command.split()
        if not words or str(os.getpid())==pid:continue
        executable=os.path.basename(words[0])
        if executable=="bun" and ("dependencies sync" in command or "dependencies lock" in command):found.append(line)
        elif executable in ("sh","bash","zsh") and any(name in command for name in ("deps4-run.sh","deps4-loop.sh","deps4-loop2.sh")) and "-c" not in words[1:2]:found.append(line)
    return found
def guard():
    running=sync_running()
    if running and "--ignore-sync" not in sys.argv:
        print("refusing: a dependency sync or lock is running (the lease module is loaded by its storage path):")
        for line in running:print("  "+line[:160])
        raise SystemExit(2)
FLAGS={"--dry-run","--ignore-sync"}
unknown=[word for word in sys.argv[2:] if word not in FLAGS]
if unknown:raise SystemExit("refusing unknown argument(s) "+" ".join(unknown)+" (this set has no rebase mode)")
mode=sys.argv[1] if len(sys.argv)>1 else "check"
dry="--dry-run" in sys.argv
if mode=="check":
    status=state();bad=[r for r,v in status.items() if v=="drifted"]
    for rel,v in status.items():print(v.ljust(8),rel)
    if bad:raise SystemExit("REFUSED: "+str(len(bad))+" target(s) changed since staging; re-run stage.sh and re-verify")
    print("check ok")
elif mode=="apply":
    guard()
    status=state();bad=[r for r,v in status.items() if v=="drifted"]
    if bad:
        for r in bad:print("drifted",r)
        raise SystemExit("REFUSED: targets changed since staging; re-run stage.sh and re-verify")
    todo=[r for r,v in status.items() if v=="base"]
    for rel in todo:print(("would apply " if dry else "apply ")+rel)
    if dry:raise SystemExit(0)
    for rel in todo:
        target=os.path.join(ROOT,rel)
        if os.path.exists(target):
            back=os.path.join(BACKUP,rel);os.makedirs(os.path.dirname(back),exist_ok=True);shutil.copy2(target,back)
        os.makedirs(os.path.dirname(target),exist_ok=True)
        tmp=target+".lease-fair.tmp";shutil.copyfile(os.path.join(STAGED,rel),tmp);os.replace(tmp,target)
    final=state()
    if any(v!="applied" for v in final.values()):raise SystemExit("post-apply verification failed: "+json.dumps(final,ensure_ascii=False))
    print("applied",len(todo),"file(s); all targets match staged hashes")
elif mode=="revert":
    guard()
    for rel,base in bases.items():
        target=os.path.join(ROOT,rel)
        if sha(target)!=staged(rel):
            print("skip (not staged content)",rel);continue
        if base is None:
            print(("would remove " if dry else "remove ")+rel)
            if not dry:os.remove(target)
        else:
            back=os.path.join(BACKUP,rel)
            if sha(back)!=base:raise SystemExit("backup missing or altered for "+rel)
            print(("would restore " if dry else "restore ")+rel)
            if not dry:shutil.copyfile(back,target)
else:
    raise SystemExit("usage: apply.sh check|apply|revert [--dry-run]")
