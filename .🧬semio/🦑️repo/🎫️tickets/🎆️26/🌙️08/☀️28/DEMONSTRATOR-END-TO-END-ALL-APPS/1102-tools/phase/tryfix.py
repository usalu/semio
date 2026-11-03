import re,sys,collections
txt=sys.argv[1];base=sys.argv[2]
L=open(txt,encoding="utf8").read().split("\n")
edits=collections.defaultdict(list)
for i,l in enumerate(L):
    if l.startswith("error[E0277]: the `?` operator can only be applied"):
        m=re.search(r'--> (.*?)📦️packages/🦀️rust/\.\./\.\./(.*?):(\d+):(\d+)',L[i+1])
        if not m:continue
        path,line,col=m.group(2),int(m.group(3)),int(m.group(4))
        cj=next((x for x in L[i+2:i+8] if re.match(r'\s*\|\s*\^+',x)),None)
        # underline may be on the next source line row
        under=None
        for x in L[i+2:i+10]:
            mm=re.match(r'\s*\|\s*( *)(\^+)',x)
            if mm:under=len(mm.group(2));break
        if under:edits[path].append((line,col,under))
for path,es in edits.items():
    f=base+"/"+path;S=open(f,encoding="utf8").read().split("\n")
    for line,col,u in sorted(es,key=lambda e:(-e[0],-e[1])):
        s=S[line-1];chars=list(s);end=col-1+u-1
        assert chars[end]=="?",(path,line,col,s[col-1:col+u])
        S[line-1]=s[:end]+".result"+s[end:]
    open(f,"w",encoding="utf8").write("\n".join(S));print("fixed",path,len(es))
