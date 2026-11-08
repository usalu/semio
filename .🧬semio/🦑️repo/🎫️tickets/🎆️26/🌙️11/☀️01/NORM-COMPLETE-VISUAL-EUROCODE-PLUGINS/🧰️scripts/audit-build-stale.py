import os,re
ROOT="/home/deck/semio"
G=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS/🗑️generated/"
A=ROOT+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
logs={"build-en1990/16.txt":"⚖️en1990","build-en1991/17.txt":None,"build-en1992/20.txt":"🏛️en1992","build-en1993/13.txt":"🔩️en1993","build-en1994/17.txt":"🧩️en1994","build-en1995/30.txt":None}
for lf,art in logs.items():
    L=open(G+lf,encoding="utf-8",errors="replace").read().split("\n")
    for i,l in enumerate(L):
        m=re.match(r'^error(\[E\d+\])?: (.*)$',l)
        if not m or l.startswith("error: could not compile") or l.startswith("error: Owned"): continue
        loc=None
        for j in range(i+1,min(i+8,len(L))):
            mm=re.search(r'-->\s+(\S+):(\d+):(\d+)',L[j])
            if mm: loc=mm.groups(); break
        if not loc: print(lf,"NOLOC",m.group(2)[:60]); continue
        rel,ln=loc[0],int(loc[1])
        sn=""
        for j in range(i+1,min(i+12,len(L))):
            ms=re.match(r'^\s*%d \|\s?(.*)$'%ln,L[j])
            if ms: sn=ms.group(1).strip(); break
        if rel.startswith(ROOT+"/"): tgt=os.path.normpath(rel)
        else: tgt=os.path.normpath(os.path.join(A+art,rel))
        if not os.path.isfile(tgt): print(lf,"MISSING",rel[-80:]); continue
        body=open(tgt,encoding="utf-8",errors="replace").read()
        present = (sn in body) if sn else "n/a"
        print(f"{lf}\t{m.group(1) or ''}\t{os.path.relpath(tgt,ROOT)}:{ln}\tpresent={present}\t{m.group(2)[:60]}\n\t LOG: {sn[:180]}")
