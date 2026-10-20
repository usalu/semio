import hashlib,json,os,shutil,subprocess,sys
HERE=os.path.dirname(os.path.abspath(__file__))
def find_root():
    path=HERE
    while path!=os.path.dirname(path):
        if os.path.isfile(os.path.join(path,"bunfig.toml")) and os.path.isfile(os.path.join(path,"bun.lock")):return path
        path=os.path.dirname(path)
    raise SystemExit("repository root not found")
ROOT=find_root()
BACKUP=os.path.join(HERE,"backup")
def sha(path):return hashlib.sha256(open(path,"rb").read()).hexdigest() if os.path.isfile(path) else None
def walk(base):
    for current,_,names in os.walk(base):
        for name in names:yield os.path.relpath(os.path.join(current,name),base)
def group(staged,bases_file):
    base=os.path.join(HERE,staged)
    if not os.path.isdir(base):return {}
    bases=json.load(open(os.path.join(HERE,bases_file),encoding="utf8"))
    rows={}
    for rel in sorted(walk(base)):
        live=sha(os.path.join(ROOT,rel)); mine=sha(os.path.join(base,rel))
        if rel in bases:state="applied" if live==mine else "base" if live==bases[rel] else "drifted"
        else:state="applied" if live==mine else "new" if live is None else "drifted"
        rows[rel]=(state,base)
    return rows
def running():
    listing=subprocess.run(["ps","-ww","-eo","pid,command"],capture_output=True,text=True).stdout
    found=[]
    for line in listing.splitlines()[1:]:
        pid,_,command=line.strip().partition(" ")
        words=command.split()
        if not words or str(os.getpid())==pid:continue
        if os.path.basename(words[0])=="bun" and ("dependencies sync" in command or "dependencies lock" in command or "repo show" in command):found.append(line)
    return found
args=sys.argv[1:]
mode=args[0] if args else "check"
flags=set(args[1:])
unknown=flags-{"--write","--with-inheritance","--ignore-sync","--no-help"}
if unknown:raise SystemExit("refusing unknown argument(s) "+" ".join(sorted(unknown)))
rows=group("staged","bases.json")
if "--with-inheritance" in flags:rows.update(group("staged-optional","bases-optional.json"))
too_long=[rel for rel in rows if len(os.path.join(ROOT,rel))>256]
if too_long:raise SystemExit("refusing: path longer than 256 characters: "+too_long[0])
bad=[rel for rel,(state,_) in rows.items() if state=="drifted"]
for rel,(state,_) in rows.items():print(state.ljust(8),rel)
if mode=="check":
    if bad:raise SystemExit("REFUSED: %d target(s) changed since staging; re-run stage.py and re-verify"%len(bad))
    print("check ok:",sum(1 for s,_ in rows.values() if s=="base"),"patch,",sum(1 for s,_ in rows.values() if s=="new"),"new,",sum(1 for s,_ in rows.values() if s=="applied"),"already applied")
elif mode=="apply":
    if bad:raise SystemExit("REFUSED: %d target(s) changed since staging; re-run stage.py and re-verify"%len(bad))
    held=running()
    if held and "--ignore-sync" not in flags:
        print("refusing: a dependency sync or graph walk is running (it loads the lease module):")
        for line in held:print("  "+line[:160])
        raise SystemExit(2)
    todo=[rel for rel,(state,_) in rows.items() if state in("base","new")]
    if "--write" not in flags:
        print("dry run: would write",len(todo),"file(s); pass --write to apply");raise SystemExit(0)
    for rel in todo:
        target=os.path.join(ROOT,rel)
        if os.path.exists(target):
            back=os.path.join(BACKUP,rel);os.makedirs(os.path.dirname(back),exist_ok=True);shutil.copy2(target,back)
        os.makedirs(os.path.dirname(target),exist_ok=True)
        temporary=target+".conc-leases.tmp";shutil.copyfile(os.path.join(rows[rel][1],rel),temporary);os.replace(temporary,target)
    after=group("staged","bases.json")
    if any(state!="applied" for state,_ in after.values()):raise SystemExit("post-apply verification failed")
    print("applied",len(todo),"file(s)")
    if "--no-help" not in flags and any("catalog" in rel for rel in todo):
        subprocess.run(["bun",os.path.join(HERE,"regen-help.ts"),ROOT],check=True)
elif mode=="revert":
    bases=json.load(open(os.path.join(HERE,"bases.json"),encoding="utf8"))
    if os.path.isfile(os.path.join(HERE,"bases-optional.json")):bases.update(json.load(open(os.path.join(HERE,"bases-optional.json"),encoding="utf8")))
    for rel,(state,base) in rows.items():
        target=os.path.join(ROOT,rel)
        if state!="applied":print("skip (not staged content)",rel);continue
        if rel in bases:
            back=os.path.join(BACKUP,rel)
            if sha(back)!=bases[rel]:raise SystemExit("backup missing or altered for "+rel)
            print(("restore " if "--write" in flags else "would restore ")+rel)
            if "--write" in flags:shutil.copyfile(back,target)
        else:
            print(("remove " if "--write" in flags else "would remove ")+rel)
            if "--write" in flags:os.remove(target)
else:
    raise SystemExit("usage: apply.py check|apply|revert [--write] [--with-inheritance] [--ignore-sync] [--no-help]")
