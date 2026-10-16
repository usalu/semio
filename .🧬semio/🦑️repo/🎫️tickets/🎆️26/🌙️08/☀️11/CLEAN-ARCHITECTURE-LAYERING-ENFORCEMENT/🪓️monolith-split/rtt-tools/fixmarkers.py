import glob,re,sys
R="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
import json
names=json.load(open(sys.argv[1]))
dirs=[f"{R}/🧪️tests/{n}" for n in names["pr_ec"]]+[f"{R}/🧱️elements/📃️UiDocumentStore/🧪️tests/{n}" for n in names["tw"]]
changed=0
for d in dirs:
    for f in glob.glob(d+"/🟦️.ts*"):
        lines=open(f,encoding="utf8").read().split("\n")
        out=list(lines)
        for i,l in enumerate(lines):
            m=re.match(r"^(\s*)//#(end)?region\b",l)
            if not m: continue
            cur=len(m.group(1))
            if m.group(2):
                j=i-1
                while j>=0 and lines[j].strip()=="" : j-=1
                if j<0: continue
                prev=lines[j]
                want=len(prev)-len(prev.lstrip())
                if prev.strip().startswith("//#"): want=len(prev)-len(prev.lstrip())
            else:
                j=i+1
                while j<len(lines) and lines[j].strip()=="" : j+=1
                if j>=len(lines): continue
                nxt=lines[j]
                want=len(nxt)-len(nxt.lstrip())
                if nxt.strip().startswith("//#"): want=len(nxt)-len(nxt.lstrip())
            if want!=cur and cur==0:
                out[i]=" "*want+l.lstrip()
        if out!=lines:
            open(f,"w",encoding="utf8").write("\n".join(out)); changed+=1; print("fixed",f[-70:])
print("changed",changed)
