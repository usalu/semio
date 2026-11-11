import json,sys,os,subprocess,re
G=sys.argv[1]
root=os.getcwd()
files=[f for f in subprocess.run(["grep","-rl","--include=*.rs","global_allocator","✏️s","🧰️framework","🌎️hub","🧪️tests","🧫️fixtures","--exclude-dir=target","--exclude-dir=.cache","--exclude-dir=node_modules"],capture_output=True,text=True).stdout.split("\n") if f]
claimed=set()
pat=re.compile(r'#\[path\s*=\s*"([^"]+)"\]')
for name in ["","-✏️s","-🌎️hub"]:
    m=json.load(open(G+"/alloc-metadata"+name+".json"))
    pk={p["id"]:p for p in m["packages"]}
    mem=set(m["workspace_members"])
    nodes={n["id"]:n for n in m["resolve"]["nodes"]}
    tg=[i for i,p in pk.items() if p["name"]=="semio-framework-value-testing-allocator"]
    if not tg: print("NOALLOC",name);continue
    target=tg[0]
    def deps(i,all_kinds):
        return [d["pkg"] for d in nodes[i]["deps"] if all_kinds or None in {k["kind"] for k in d["dep_kinds"]}]
    memo={}
    def reach(i):
        if i in memo: return memo[i]
        memo[i]=False
        r=i==target or any(reach(d) for d in deps(i,False))
        memo[i]=r;return r
    owner={}
    for i in mem:
        for t in pk[i]["targets"]:
            stack=[t["src_path"]];seen=set()
            while stack:
                f=stack.pop()
                if f in seen or not os.path.exists(f):continue
                seen.add(f);owner.setdefault(f,set()).add(i)
                try:s=open(f).read()
                except:continue
                for q in pat.findall(s):stack.append(os.path.normpath(os.path.join(os.path.dirname(f),q)))
    for f in files:
        a=os.path.join(root,f)
        for i in owner.get(a,()):
            claimed.add(f)
            c=i==target or any(reach(d) for d in nodes[i]["deps"] and deps(i,True))
            print("CONFLICT" if c else "ok",name or "root",pk[i]["name"],f)
for f in files:
    if f not in claimed:print("UNMAPPED",f)
