import os,shutil,sys,hashlib
import patch
LIVE="/Users/ueli/Documents/semio"; S=sys.argv[1]
rows=patch.TESTS+patch.LIVE
files=[]
for f,_,_ in rows:
    if f not in files: files.append(f)
shutil.rmtree(S+"/stage",ignore_errors=True)
for sub in ("base","new"):
    for f in files:
        d=os.path.join(S,"stage",sub,f); os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copy2(os.path.join(LIVE,f),d)
patch.apply(S+"/stage/new",rows)
with open(os.path.join(S,"stage","land.tsv"),"w",encoding="utf8") as o:
    for f in files: o.write(f+"\n")
print(len(files),"files")
