#!/usr/bin/env python3
"""show-target.py <projectname:target>...: prints the authored manifest target (compact) from the census TSV."""
import json,sys,os
C=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
W="/Users/ueli/Documents/semio/"
rows={l.split("\t")[0]:l.rstrip("\n").split("\t") for l in open(C+"/🗑️generated/bzl-nocache/authored-uncached.tsv")}
for a in sys.argv[1:]:
    r=rows.get(a)
    if not r: print("??",a); continue
    mf=json.load(open(W+r[3])); t=mf["targets"][a.split(":",1)[1]]
    t={k:v for k,v in t.items() if k!="metadata" or True}
    print("##",a,"|",r[3].rsplit("/",1)[0][-90:]); print(json.dumps(t,ensure_ascii=False)[:900])
