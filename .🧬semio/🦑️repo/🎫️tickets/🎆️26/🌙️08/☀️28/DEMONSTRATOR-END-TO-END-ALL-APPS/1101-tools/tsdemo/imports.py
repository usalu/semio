import json,os,re,subprocess,sys
root="/Users/ueli/Documents/semio"; os.chdir(root)
files=subprocess.run(["git","ls-files","-z","--","*package.json"],capture_output=True).stdout.decode().split("\0")
pk={}
for f in files:
    if not f or ".🧬semio" in f: continue
    try: d=json.load(open(f))
    except Exception: continue
    if d.get("name"): pk[d["name"]]=(f,d)
imp=re.compile(r'(?:from|import)\s*\(?\s*["\']([^"\']+)["\']')
def split(s):
    p=s.split("/"); n=p[0]+"/"+p[1] if s.startswith("@") else p[0]; sub="/".join(p[2:] if s.startswith("@") else p[1:]); return n,("./"+sub if sub else ".")
seen=set(); missing={}; bound={"@semio-tech/framework","@semio-tech/framework-replication","@semio-tech/schema-validator-ts"}
def visit(path,chain):
    if path in seen or not os.path.isfile(path): return
    seen.add(path)
    for s in imp.findall(open(path,encoding="utf8").read()):
        if s.startswith("node:") or s.startswith("bun:"): continue
        if s.startswith("."):
            t=os.path.normpath(os.path.join(os.path.dirname(path),s)); visit(t,chain+[path]); continue
        n,sub=split(s)
        if n not in pk: continue
        mf,d=pk[n]; ex=d.get("exports",{}).get(sub)
        if s not in bound: missing.setdefault(s,path)
        if isinstance(ex,str): visit(os.path.normpath(os.path.join(os.path.dirname(mf),ex)),chain+[path])
visit("✏️s/🧑‍💻dev/🧩️catalog/📣️publication/📜️script.ts",[])
for s,p in sorted(missing.items()): print(s,"<-",p)
print(len(seen),"files")
