import os,re,subprocess,sys,urllib.parse
R="/Users/ueli/Documents/semio"
T=os.path.dirname(os.path.abspath(__file__))
entry=sys.argv[1]; fwd=sys.argv[2:] 
FROZEN=("💾️storage","🔏️revision")
def probe():
    p=subprocess.run(["node","-e",'require("%s")'%entry],cwd=R,capture_output=True,text=True)
    return p.returncode,p.stderr
def importers(j):
    base=os.path.basename(j); d=os.path.dirname(j)
    for _ in range(7):
        out=subprocess.run(["git","grep","-l","-F",base,"--",d],cwd=R,capture_output=True,text=True).stdout.split("\n")
        out=[f for f in out if f and f.endswith((".ts",".mts",".tsx"))]
        if out: return out
        d=os.path.dirname(d)
    return []
for i in range(120):
    rc,t=probe()
    if rc==0: print("OK"); break
    m=re.search(r'Module "file://([^"]+)" needs an import attribute',t)
    if m:
        j=urllib.parse.unquote(m.group(1)); fixed=False
        for f in importers(j):
            f=os.path.join(R,f)
            rel=os.path.relpath(j,os.path.dirname(f))
            if not rel.startswith("."): rel="./"+rel
            s=open(f,encoding="utf-8").read()
            pat=re.compile(r'((?:import|export)\b[^;\n]*?\bfrom\s*"'+re.escape(rel)+r'")(\s*;)',re.M)
            if pat.search(s):
                if any(x in f for x in FROZEN): print("FROZEN",f); continue
                open(f,"w",encoding="utf-8").write(pat.sub(r'\1 with { type: "json" }\2',s)); print("fixed json",f); fixed=True
        if not fixed: print("STOP json",j); break
        continue
    m=re.search(r'file://([^\n:]+\.m?ts):(\d+)',t)
    if "parameter property" in t and m:
        f=urllib.parse.unquote(m.group(1))
        r=subprocess.run(["python3",T+"/fixpp2.py",f],capture_output=True,text=True).stdout.strip()
        print("pp",r[-120:])
        if r.endswith(" 0") or "MANUAL" in r: print("STOP pp",f); break
        continue
    print("STOP other"); print(t[:1200]); break
