# 🔬 Static reference check: every quoted fixture-path literal in a subset's rs/ts files must resolve to an existing path relative to the file, its Cargo manifest dir or the fixtures root. usage: python3 ref-check.py <repoRoot> <subset> [out.json]
import os,re,sys,json,subprocess
repo,sub=sys.argv[1],sys.argv[2]
lit=re.compile(r'"([^"\n]*🧫️fixtures/🧬️mutations/[^"\n]*)"')
files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",sub],capture_output=True,text=True,cwd=repo).stdout.split("\n") if f.endswith((".rs",".ts",".py")) and "/target/" not in f]
def manifest_dir(d):
    while d and d!=repo:
        if os.path.isfile(os.path.join(d,"Cargo.toml")): return d
        d=os.path.dirname(d)
    return None
fx=os.path.join(repo,sub,"🧪️tests/🧫️fixtures/🧬️mutations")
total=0; miss=[]
for f in files:
    p=os.path.join(repo,f)
    try: t=open(p,encoding="utf8").read()
    except Exception: continue
    for m in lit.finditer(t):
        s=m.group(1); total+=1
        cands=[os.path.normpath(os.path.join(os.path.dirname(p),s))]
        md=manifest_dir(os.path.dirname(p))
        if md: cands.append(os.path.normpath(os.path.join(md,s.lstrip("/"))))
        i=s.find("🧫️fixtures/🧬️mutations/")
        cands.append(os.path.join(fx,s[i+len("🧫️fixtures/🧬️mutations/"):]))
        if not any(os.path.exists(c) for c in cands): miss.append((f[-70:],s[-90:]))
print(sub[-40:],"literals",total,"missing",len(miss))
if len(sys.argv)>3: json.dump(miss,open(sys.argv[3],"w"),ensure_ascii=False)
for x in miss[:4]: print("  ",x)
