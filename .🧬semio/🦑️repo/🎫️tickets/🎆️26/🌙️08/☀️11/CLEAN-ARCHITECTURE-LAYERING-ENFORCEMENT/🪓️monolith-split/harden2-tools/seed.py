import sys,re,shutil,json,collections
import ho
# usage: seed.py target cfg relpath-in-nsbx regex-substitution-mode
t,cfg,rel=sys.argv[1:4]; mode=sys.argv[4]
H=ho.WORK
f=H+"/nsbx-"+t+"-split/"+rel
bak=f+".seedbak"
shutil.copy(f,bak)
try:
    s=open(f,encoding="utf-8").read()
    if mode=="items": s2,n=re.subn(r"(?m)^pub\(super\) (fn|struct|enum|const|static|type|trait) ",r"\1 ",s)
    elif mode=="items-crate": s2,n=re.subn(r"(?m)^pub\(crate\) (fn|struct|enum|const|static|type|trait) ",r"\1 ",s)
    elif mode=="first-item":
        m=re.search(r"(?m)^pub\((super|crate)\) (fn|struct|enum|const|static|type|trait) ",s); s2=s[:m.start()]+s[m.start()+len(m.group(0))-len(m.group(2))-1:]; n=1
    open(f,"w",encoding="utf-8").write(s2)
    print("seeded",n,"substitutions in",rel[-60:])
    sp=ho.SPEC[t]
    A=ho.rustc(H+"/nsbx-"+t+"-base/"+sp["root"],sp["crate"],sp["ct"],ho.CFG[cfg],H+"/seed-b","" if False else sp["fam"])[0]
    B=ho.rustc(H+"/nsbx-"+t+"-split/"+sp["root"],sp["crate"],sp["ct"],ho.CFG[cfg],H+"/seed-s",sp["fam"])[0]
    more=B-A
    print("baseline",sum(A.values()),"seeded split",sum(B.values()),"split-only",sum(more.values()))
    c=collections.Counter(k[0] for k in more.elements()); print(dict(c))
    for k,v in list(more.items())[:5]: print("  ",v,tuple(str(x)[:100] for x in k))
finally:
    shutil.move(bak,f)
