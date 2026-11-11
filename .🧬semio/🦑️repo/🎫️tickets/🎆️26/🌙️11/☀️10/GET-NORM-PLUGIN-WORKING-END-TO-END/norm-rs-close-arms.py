import re,sys
hdr=re.compile(r'^(\s*)("[^"]+" => \()$')
for p in sys.argv[1:]:
    L=open(p).read().split("\n"); out=[]; n=0
    for i,l in enumerate(L):
        m=hdr.match(l)
        if m and out and "include_str!" in out[-1]:
            out.append(m.group(1)+"),"); n+=1
        out.append(l)
    open(p,"w").write("\n".join(out)); print(n)
