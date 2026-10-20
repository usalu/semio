import hashlib,os
ROOT="/Users/ueli/Documents/semio"
HERE=os.path.dirname(os.path.abspath(__file__))
S=os.path.join(HERE,"staged")
rows=[]
for d,_,fs in os.walk(S):
    for f in fs:
        p=os.path.join(d,f); rel=os.path.relpath(p,S)
        live=os.path.join(ROOT,rel)
        h=lambda b:hashlib.sha256(b).hexdigest()
        base=h(open(live,"rb").read()) if os.path.exists(live) else "-"
        rows.append((rel,base,h(open(p,"rb").read())))
with open(os.path.join(HERE,"manifest.tsv"),"w",encoding="utf8") as o:
    for r in sorted(rows): o.write("\t".join(r)+"\n")
print(len(rows),"files anchored")
