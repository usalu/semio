import json,re,sys
m=json.load(open(sys.argv[1],encoding="utf-8"))
n=0
for f,pairs in m.items():
    s=open(f,encoding="utf-8").read();o=s
    for old,new in pairs.items():
        s=re.sub(r"(?<![A-Za-z0-9_])"+re.escape(old)+r"(?![A-Za-z0-9_])",new,s)
    if s!=o:
        open(f,"w",encoding="utf-8").write(s);n+=1
print("files",n)
