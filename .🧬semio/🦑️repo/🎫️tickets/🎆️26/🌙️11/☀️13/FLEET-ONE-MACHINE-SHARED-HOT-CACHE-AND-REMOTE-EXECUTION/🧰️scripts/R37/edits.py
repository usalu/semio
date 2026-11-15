import json,os
HERE=os.path.dirname(os.path.abspath(__file__))
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
EDITS=[tuple(row) for row in json.load(open(HERE+"/edits.json",encoding="utf8"))]
NEWFILES=[]
for root,_,files in os.walk(HERE+"/new"):
    for f in sorted(files):
        src=os.path.relpath(os.path.join(root,f),HERE+"/new");NEWFILES.append((src,src))
NEWFILES.sort()
