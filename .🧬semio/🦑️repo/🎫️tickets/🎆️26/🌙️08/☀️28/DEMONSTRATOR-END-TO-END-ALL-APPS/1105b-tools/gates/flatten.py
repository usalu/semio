import re,sys,os,collections
s=open(sys.argv[1],errors="replace").read().split("\n")
edits=collections.defaultdict(set)
i=0
while i<len(s):
    m=re.match(r"error\[(E0599|E0277)\]: (.*)",s[i])
    if m and ("not an iterator" in m.group(2) or "can only be applied to values that implement `Try`" in m.group(2)):
        loc=re.match(r"\s*--> (.*?):(\d+):(\d+)$",s[i+1])
        if loc and sys.argv[2] in loc.group(1):
            j=i+2
            while j<len(s) and not re.match(r"\s*\|\s+\^+",s[j]): j+=1
            carets=re.match(r"\s*\|\s+(\^+)",s[j]).group(1)
            edits[(os.path.normpath(loc.group(1)),int(loc.group(2)))].add((m.group(1),int(loc.group(3)),len(carets)))
    i+=1
tot=0
for (f,ln),lst in edits.items():
    lines=open(f,encoding="utf8").read().split("\n");t=lines[ln-1]
    for kind,col,n in sorted(lst,key=lambda x:-x[1]):
        if kind=="E0599":
            if t[col-1:col+2]!="map":print("SKIP599",f[-30:],ln,col,repr(t[col-1:col+6]));continue
            t=t[:col-1]+"result."+t[col-1:]
        else:
            k=col-1;depth=0;seen=False;end=None
            while k<len(t):
                ch=t[k]
                if ch in "([{":depth+=1;seen=True
                elif ch in ")]}":
                    depth-=1
                    if seen and depth==0 and ch==")" and t[k+1:k+2]=="?":end=k+1;break
                k+=1
            if end is None:print("SKIP277",f[-30:],ln,col);continue
            t=t[:end]+".result"+t[end:]
        tot+=1
    lines[ln-1]=t;open(f,"w",encoding="utf8").write("\n".join(lines))
print("edits",tot)
