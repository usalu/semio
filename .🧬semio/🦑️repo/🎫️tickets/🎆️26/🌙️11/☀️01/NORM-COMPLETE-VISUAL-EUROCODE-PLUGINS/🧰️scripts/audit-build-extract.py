import re,sys,glob
ROOT="/home/deck/semio/"
files=["build-en1990/16.txt","build-en1991/17.txt","build-en1992/20.txt","build-en1993/13.txt","build-en1994/17.txt","build-en1995/30.txt"]
base="🗑️generated/"
seen={}
for f in files:
    lines=open(base+f,encoding="utf-8",errors="replace").read().split("\n")
    for i,l in enumerate(lines):
        m=re.match(r'^error(\[E\d+\])?: (.*)$',l)
        if not m or l.startswith("error: could not compile") or l.startswith("error: Owned"): continue
        loc=""
        for j in range(i+1,min(i+8,len(lines))):
            mm=re.search(r'-->\s+(\S+)',lines[j])
            if mm: loc=mm.group(1).replace(ROOT,""); break
        print(f"{f}\t{i+1}\t{m.group(1) or ''}\t{m.group(2)[:140]}\t{loc}")
