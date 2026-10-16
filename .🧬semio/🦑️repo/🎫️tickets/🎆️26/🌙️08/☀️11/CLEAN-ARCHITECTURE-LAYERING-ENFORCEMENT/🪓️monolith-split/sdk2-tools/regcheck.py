# -*- coding: utf-8 -*-
import json,os,unicodedata,sys
R="/Users/ueli/Documents/semio/"
tax=json.load(open(R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"))
fold=lambda s: unicodedata.normalize("NFC",s).replace("️","").replace("︎","")
names={}
for k,v in tax["semanticDirectoryMemberKinds"].items():
    for n in v["memberNames"]: names.setdefault(fold(n),[]).append(k)
cand=sys.argv[1:]
for n in cand:
    print(("REG " if fold(n) in names else "    ")+n, names.get(fold(n),[])[:2])
