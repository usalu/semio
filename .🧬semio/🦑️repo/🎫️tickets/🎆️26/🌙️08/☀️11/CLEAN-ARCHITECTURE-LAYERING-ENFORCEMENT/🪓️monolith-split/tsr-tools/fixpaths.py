import os,re,subprocess,sys
R="/Users/ueli/Documents/semio"
ROOT=R+"/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"
apply=len(sys.argv)>1 and sys.argv[1]=="apply"
files=subprocess.run(["git","-c","core.quotepath=false","ls-files","--",ROOT],cwd=R,capture_output=True,text=True).stdout.split("\n")
files=[f for f in files if f.endswith((".ts",".tsx")) and "/🧊️wgpu/" not in f]
pat=re.compile(r'((?:\.\./)+)(🧫️fixtures/[^"\'`\s]*)')
changed=0;unres=[]
for f in files:
    p=os.path.join(R,f)
    if not os.path.exists(p): continue
    t=open(p,encoding="utf8").read()
    d=os.path.dirname(p)
    def rep(m):
        ups,rest=m.group(1),m.group(2)
        tgt=os.path.normpath(os.path.join(d,ups+rest))
        if os.path.exists(tgt) and os.path.isfile(tgt): return m.group(0)
        if os.path.isdir(tgt) and not rest.endswith("/"): pass
        # search ancestors for <anc>/🧫️fixtures/<rest-after-prefix>
        sub=rest[len("🧫️fixtures/"):]
        anc=d
        while len(anc)>len(R):
            for cand in (anc+"/🧫️fixtures/"+sub, anc+"/🧪️tests/🧫️fixtures/"+sub):
                if os.path.exists(cand) and cand!=tgt:
                    rel=os.path.relpath(cand,d)
                    return rel
            anc=os.path.dirname(anc)
        unres.append((f,m.group(0)))
        return m.group(0)
    n=pat.sub(rep,t)
    if n!=t:
        changed+=1
        print("CHG",f.split("/renderer/")[-1][:120],sum(1 for a,b in zip(pat.findall(t),pat.findall(n)) if a!=b))
        if apply: open(p,"w",encoding="utf8").write(n)
print("files changed",changed)
for u in unres: print("UNRES",u[0].split("/renderer/")[-1][:100],u[1][:120])
