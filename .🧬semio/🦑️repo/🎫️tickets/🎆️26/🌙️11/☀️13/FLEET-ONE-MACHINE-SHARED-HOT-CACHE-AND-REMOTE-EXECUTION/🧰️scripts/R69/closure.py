import json,os,re,subprocess,sys
R="/Users/ueli/Documents/semio/"
POL=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🔌️provider-closures/🔣️.json"
pol=json.load(open(POL))
files=subprocess.run(["git","-c","core.quotepath=false","ls-files","🧰️framework"],cwd=R,capture_output=True,text=True).stdout.splitlines()
fs=set(files)
def resolve(entry):
    suf=entry
    while suf.startswith("../"):suf=suf[3:]
    c=[f for f in files if f.endswith("/"+suf)]
    return c
pat=re.compile(r'''(?:from|import|require)\s*\(?\s*["']([^"']+)["']''')
def deps(f):
    try:s=open(R+f,encoding="utf8").read()
    except:return []
    out=[]
    for m in pat.findall(s):
        if not m.startswith("."):continue
        p=os.path.normpath(os.path.join(os.path.dirname(f),m))
        for cand in (p,p+".ts",p+".mjs",p+"/🟦️.ts",p+".json"):
            if cand in fs:out.append(cand);break
    return out
targets=sys.argv[1:]
seen={};
for name,row in pol["tools"]["closures"].items():
    ents=[]
    for e in row["entries"]:
        r=resolve(e);ents+=r
    st=list(ents);vis=set(st)
    while st:
        f=st.pop()
        for d in deps(f):
            if d not in vis:vis.add(d);st.append(d)
    hit=[t for t in targets if t in vis]
    print(name,len(ents),len(vis),hit)
