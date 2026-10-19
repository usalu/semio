#!/usr/bin/env python3
# usage: triage.py <candidates.txt> <from> <to>  -> runs each suite, prints pass/fail, first error and failing expression location
import json,sys,subprocess,re,os
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
R="/Users/ueli/Documents/semio/"
rows=[json.loads(l) for l in open(sys.argv[1],encoding="utf-8")]
a,b=int(sys.argv[2]),int(sys.argv[3])
for i,(f,p,fl,k) in enumerate(rows[a:b],start=a):
    full="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"+f if not f.startswith("🧰️framework") else f
    out=subprocess.run([sys.executable,C+"/law2-tools/one.py",full],capture_output=True,text=True).stdout.strip().split("\n")[-1]
    fl_=subprocess.run([sys.executable,C+"/law2-tools/fl.py","170"],capture_output=True,text=True).stdout.strip().split("\n")[:3]
    loc=subprocess.run([sys.executable,C+"/law2-tools/loc.py"],capture_output=True,text=True).stdout.strip().split("\n")[:2]
    print("##",i,full[-90:]); print("  ",out)
    for x in fl_: print("   F:",x[:250])
    for x in loc: print("   L:",x[:260])
