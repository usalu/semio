import re,os,sys
root=os.getcwd()
files=[l.rstrip("\n") for l in open(sys.argv[1],encoding="utf-8") if l.strip()]
abs_rx=re.compile(r"(?:🧰️framework|✏️s|🌎️hub)[^\"'`\s,)\]}]*?🎁️browser-package[^\"'`\s,)\]}]*")
rel_rx=re.compile(r"(?:\.\./)+🎁️browser-package[^\"'`\s,)\]}]*")
bad=[];n=0
for f in files:
    if not os.path.exists(f): continue
    s=open(f,encoding="utf-8").read()
    for m in abs_rx.finditer(s):
        p=m.group(0); n+=1
        if "*" in p: p=p.split("*")[0].rstrip("/")
        if not os.path.exists(os.path.join(root,p)): bad.append((f,p))
    for m in rel_rx.finditer(s):
        p=m.group(0); n+=1
        full=os.path.normpath(os.path.join(os.path.dirname(f),p))
        if not os.path.exists(full):
            bad.append((f,p+" (from file dir)"))
print("checked",n,"bad",len(bad))
for f,p in bad: print(f[-70:],"|",p)
