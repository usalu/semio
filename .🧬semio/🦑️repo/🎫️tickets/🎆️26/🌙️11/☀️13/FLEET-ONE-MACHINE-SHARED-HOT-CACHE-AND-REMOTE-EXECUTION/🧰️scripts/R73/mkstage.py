import os,shutil
R="/Users/ueli/Documents/semio/";L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
F=L+"🏗️build/🪪️graph-state/📦️project-records/"
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R73/"
H=os.path.dirname(os.path.abspath(__file__))+"/stage/"
files=[(F+"🌱️fresh/"+x,SC+"ws/"+F+"🌱️fresh/"+x) for x in ["🟦️.ts","🧬️schema/🔣️.json","🧪️tests/🟦️.ts","🧪️tests/🐍️.py","🧪️tests/🔣️.json"]]+[(F+"🟦️.ts",SC+"base/pr.new.ts")]
rows=[]
for n,(rel,new) in enumerate(files):
    i="f%d"%n;shutil.copy(R+rel,H+i+".base") if rel!=F+"🟦️.ts" else shutil.copy(SC+"base/pr.ts",H+i+".base")
    shutil.copy(new,H+i+".new");rows.append("%s %s mod"%(i,rel))
open(H+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
