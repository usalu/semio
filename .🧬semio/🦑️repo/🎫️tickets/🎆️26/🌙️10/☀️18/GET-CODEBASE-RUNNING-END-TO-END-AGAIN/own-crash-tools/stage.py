import hashlib,os,sys
ROOT="/Users/ueli/Documents/semio"
HERE=os.path.dirname(os.path.abspath(__file__))
S=os.path.join(HERE,"staged")
sys.path.insert(0, os.path.abspath(os.path.join(HERE, '..')))
from winpath import iter_logical, resolve
rows=[]
for rel in iter_logical(S):
    p=resolve(S, rel)
    live=os.path.join(ROOT,rel)
    h=lambda b:hashlib.sha256(b).hexdigest()
    base=h(open(live,"rb").read()) if os.path.exists(live) else "-"
    rows.append((rel,base,h(open(p,"rb").read())))
with open(os.path.join(HERE,"manifest.tsv"),"w",encoding="utf8") as o:
    for r in sorted(rows): o.write("\t".join(r)+"\n")
print(len(rows),"files anchored")
