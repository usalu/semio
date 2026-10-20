import os,sys,collections
root="/Users/ueli/Documents/semio/"
src,out=sys.argv[1:3]
files=set();dirs=set()
for l in open(src):
    op,p=l.rstrip("\n").split("\t",1)
    if not p.startswith(root): continue
    r=p[len(root):]
    if r.startswith((".cache/",".🧬semio/")) or "/node_modules/" in r or r.startswith("node_modules"): continue
    if os.path.isfile(p) and op in("readFile","open","copyFile","createReadStream"): files.add(r)
open(out,"w").write("\n".join(sorted(files))+"\n")
print(len(files))
