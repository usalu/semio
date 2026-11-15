# -*- coding: utf-8 -*-
"""R26: snapshot the overlay files (new) and the live files they were derived from (base) into payload/. usage: build_payload.py <scratch>"""
import os,shutil,sys
R="/Users/Shared/semio";S=sys.argv[1];HERE=os.path.dirname(os.path.abspath(__file__));P=HERE+"/payload"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";F=L+"/🏗️build/🧷️format-stamps";W=L+"/🗂️workspaces/🟦️bun"
rows=[(W+"/🟨️.cjs","full"),(W+"/🧬️schema/🔣️.json","full"),(W+"/🧪️tests/⏩️frontier/🟦️.ts","full"),(W+"/🧪️tests/🌊️churn/🟦️.ts","full"),(F+"/🔄️reexec/🟨️.mjs","fs"),(F+"/🧬️schema/🔣️.json","fs"),(F+"/🧪️tests/🔁️implementation-edit/🟦️.ts","fs")]
for rel,src in rows:
    n=S+"/"+src+"/"+rel;os.makedirs(os.path.dirname(P+"/new/"+rel),exist_ok=True);shutil.copyfile(n,P+"/new/"+rel)
    if os.path.exists(R+"/"+rel):os.makedirs(os.path.dirname(P+"/base/"+rel),exist_ok=True);shutil.copyfile(R+"/"+rel,P+"/base/"+rel)
open(P+"/files.txt","w",encoding="utf8").write("\n".join(r for r,_ in rows)+"\n")
print("payload",len(rows))
