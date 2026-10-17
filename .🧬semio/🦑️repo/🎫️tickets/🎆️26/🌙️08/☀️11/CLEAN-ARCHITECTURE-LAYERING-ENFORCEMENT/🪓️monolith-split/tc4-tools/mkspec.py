"""📐️ Builds measure.ts specs: one variant per block of the generation3d-app-laws root. Read-only."""
import json,re,os,sys
G="/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d"
root=open(G+"/🦀️.rs",encoding="utf8").read()
i=root.index('#[path = "🧪️tests/🔒️serial-lock-discipline/🦀️.rs"]')
pre=root[:i]
variants=[{"name":"preamble","root":pre,"files":{}}]
for m in re.finditer(r'#\[path = "(🧪️tests/[^"]+)"\]\n(?:pub\(crate\) )?mod (\w+);',root):
    variants.append({"name":"root:"+m.group(2),"root":pre+m.group(0)+"\n","files":{}})
blocks={}
for d in ["🔭️viewer-laws","🪟️editor-mode-laws","🧭️editor-structure-laws","🎮️editor-command-laws"]:
    t=open(G+"/"+d+"/🦀️.rs",encoding="utf8").read()
    parts=re.split(r'(?m)^(?=#\[path = "\.\."\]\nmod )',t)
    for p in parts:
        mm=re.match(r'#\[path = "\.\."\]\n(?:pub\(crate\) )?mod (\w+)',p)
        if not mm: continue
        name=mm.group(1)
        blocks[name]=(d,p)
        variants.append({"name":d+":"+name,"root":pre+'include!("%s/🦀️.rs");\n'%d,"files":{d+"/🦀️.rs":p}})
json.dump({"crateDir":"/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust","gen3dDir":G,"variants":variants},open(sys.argv[1],"w"),ensure_ascii=False)
json.dump({k:v for k,v in blocks.items()},open(sys.argv[1]+".blocks","w"),ensure_ascii=False)
print(len(variants))
