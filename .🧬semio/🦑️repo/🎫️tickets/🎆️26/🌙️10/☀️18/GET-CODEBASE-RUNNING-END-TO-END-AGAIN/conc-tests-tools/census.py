#!/usr/bin/env python3
"""Census of concurrency-unsafe tests (reference implementation of the sandbox law scanner). usage: census.py PROTOCOL OUT.json [--root DIR]. Scope: test files owned by a project manifest (project.json, Cargo.toml, go.mod, package.json), index listing only, no repository-wide content search."""
import json,os,re,subprocess,sys,collections
args=sys.argv[1:]
protocol=json.load(open(args[0],encoding="utf8"))
out=args[1]
root=args[args.index("--root")+1] if "--root" in args else "/Users/ueli/Documents/semio"
TICKETS=".🧬semio/🦑️repo/🎫️tickets/"
sc=protocol["scan"]
def ls(*spec):
    o=subprocess.run(["git","ls-files","-z","--",*spec],cwd=root,capture_output=True).stdout.decode("utf8","replace").split("\0")
    return [f for f in o if f and not f.startswith(TICKETS)]
manifest_names=("📋️project.json","Cargo.toml","go.mod","package.json")
owners=set()
for n in manifest_names:
    for f in ls("*"+n):
        if os.path.basename(f)==n: owners.add(os.path.dirname(f))
owners=sorted(owners)
owner_set=set(owners)
def owner_of(path):
    d=os.path.dirname(path)
    while True:
        if d in owner_set and d: return d
        if not d: return None
        d=os.path.dirname(d)
cargo_tests=set()
for f in ls("*Cargo.toml"):
    if os.path.basename(f)!="Cargo.toml": continue
    try: text=open(root+"/"+f,encoding="utf8").read()
    except OSError: continue
    for m in re.finditer(r'\[\[test\]\][^\[]*?path\s*=\s*"([^"]+)"',text,re.S):
        cargo_tests.add(os.path.normpath(os.path.join(os.path.dirname(f),m.group(1))))
ext_lang={e:l for l,es in sc["extensions"].items() for e in es}
seg=sc["testSegment"]; skip=set(sc["skipSegments"])
def lang_of(path):
    e=path.rsplit(".",1)[-1] if "." in os.path.basename(path) else ""
    return ext_lang.get(e)
def is_test(path):
    parts=path.split("/")
    if any(p in skip for p in parts): return False
    base=parts[-1]
    return seg in parts[:-1] or path in cargo_tests or base.endswith("_test.go") or re.search(r"\.(test|spec)\.[a-z]+$",base) is not None
cand=[f for f in ls("*."+"*") if False]
files=set()
for e in ext_lang:
    for f in ls("*."+e):
        if is_test(f) and lang_of(f): files.add(f)
files=sorted(files)
rules=[]
for r in protocol["rules"]:
    c=dict(r); fl=re.I if r.get("flags")=="i" else 0
    if c["scope"]=="line":
        c["_p"]=re.compile(c["pattern"],fl); c["_also"]=[re.compile(x,fl) for x in c.get("also",[])]; c["_ex"]=[re.compile(x,fl) for x in c.get("exempt",[])]
    else:
        c["_all"]=[re.compile(x,fl) for x in c["all"]]; c["_none"]=[re.compile(x,fl) for x in c["none"]]
    rules.append(c)
comment={k:re.compile(v) for k,v in sc["commentLine"].items()}
res=[]; unowned=[]; helpers=[]; scanned=0; skipped=0
for f in files:
    own=owner_of(f)
    if own is None: unowned.append(f)
    p=root+"/"+f
    try:
        if os.path.getsize(p)>sc["maximumBytes"]: skipped+=1; continue
        text=open(p,encoding="utf8",errors="replace").read()
    except OSError: skipped+=1; continue
    lang=lang_of(f)
    if not re.search(sc["suite"][lang],text): helpers.append(f); continue
    scanned+=1
    lines=text.split("\n"); cm=comment[lang]; code="\n".join("" if cm.match(x) else x for x in lines)
    for r in rules:
        if lang not in r["languages"]: continue
        if r["scope"]=="line":
            for i,line in enumerate(lines):
                if cm.match(line): continue
                if r["_p"].search(line) and all(a.search(line) for a in r["_also"]) and not any(e.search(line) for e in r["_ex"]): res.append({"file":f,"rule":r["id"],"severity":r["severity"],"line":i+1})
        elif r["scope"]=="file":
            if all(a.search(code) for a in r["_all"]) and not any(n.search(code) for n in r["_none"]):
                first=next((i+1 for i,line in enumerate(lines) if r["_all"][0].search(line)),1)
                res.append({"file":f,"rule":r["id"],"severity":r["severity"],"line":first})
texts_by_owner=collections.defaultdict(list)
for f in files:
    parts=f.split("/")
    if seg in parts[:-1]:
        i=parts.index(seg); 
        if lang_of(f)=="go":
            try: texts_by_owner["/".join(parts[:i])].append((f,open(root+"/"+f,encoding="utf8",errors="replace").read()))
            except OSError: pass
for r in rules:
    if r["scope"]!="owner": continue
    for ow,items in sorted(texts_by_owner.items()):
        alltext=chr(10).join(t for _,t in items)
        if all(a.search(alltext) for a in r["_all"]) and not any(n.search(alltext) for n in r["_none"]):
            res.append({"file":ow+"/"+seg,"rule":r["id"],"severity":r["severity"],"line":1})
json.dump({"helpers":len(helpers),"scanned":scanned,"skipped":skipped,"owners":len(owners),"files":len(files),"unowned":unowned,"findings":res},open(out,"w",encoding="utf8"),ensure_ascii=False)
byrule=collections.Counter((x["rule"],x["severity"]) for x in res)
print("owners",len(owners),"test files",len(files),"scanned",scanned,"unowned",len(unowned),"findings",len(res))
for k,v in sorted(byrule.items(),key=lambda kv:-kv[1]): print(v,k)
