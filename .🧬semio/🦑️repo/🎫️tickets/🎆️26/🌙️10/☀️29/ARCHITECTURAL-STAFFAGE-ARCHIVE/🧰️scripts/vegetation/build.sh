#!/bin/bash
# usage: build.sh <id> ; substitutes @SWAY@/@GUST@/@GUSTTRACKS@/@DIURNAL@ in design/<id>-c.json and runs w2l into assets/<id>.json
D=$(dirname "$0"); cd "$D"; ID=$1
[ -f design/$ID-bones.txt ] && python3 mkrig.py $ID
[ -f design/$ID-hand.json ] && python3 mkgeo.py $ID
[ -f design/$ID-cs.json ] && python3 mkc.py $ID
python3 - "$ID" <<'P'
import sys,subprocess
i=sys.argv[1]; s=open(f"design/{i}-c.json").read()
def gen(mode): return subprocess.check_output(["python3","sway.py",f"design/{i}-spec.json",mode],text=True).strip().replace("}\n{","},\n{")
rep={"@SWAY@":gen("sway").replace("}}\n{","}},\n{"),"@GUST@":gen("gust").replace("}}\n{","}},\n{")}
for k in ("GUSTTRACKS","DIURNAL"):
    try: rep[f"@{k}@"]=open(f"design/{i}-{k.lower()}.txt").read().strip()
    except FileNotFoundError: rep[f"@{k}@"]=""
import os
if os.path.exists(f"design/{i}-loads.txt"):
    for line in open(f"design/{i}-loads.txt"):
        if not line.strip(): continue
        n,k,r,it,*st=line.rstrip("\n").split("|")
        out=subprocess.check_output(["python3","mkloads.py",k,r,f"design/{i}-a.json",it]+([st[0]] if st and st[0] else []),text=True).strip()
        rep[f"@LOADS_{n}@"]=out
for k,v in rep.items(): s=s.replace(k,v)
open(f"design/{i}-c.built.json","w").write(s)
P
FILES="design/$ID-a.json design/$ID-geo.json design/$ID-c.built.json"
repo tools exec bun -- w2l.ts assets/$ID.json $FILES
