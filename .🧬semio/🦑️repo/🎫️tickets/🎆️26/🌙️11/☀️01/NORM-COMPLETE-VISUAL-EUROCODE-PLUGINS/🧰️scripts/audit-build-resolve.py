import os,re,sys
ROOT="/home/deck/semio"
A=ROOT+"/✏️s/🔌️plugins/📕️norm/🗿️artifacts/"
rows=[l.rstrip("\n").split("\t") for l in open(sys.argv[1],encoding="utf-8")]
art={"en1990":"⚖️en1990","en1991":"🏋️en1991","en1992":"🏛️en1992","en1993":"🔩️en1993","en1994":"🧩️en1994","en1995":"🪵️en1995"}
for f,ln,code,msg,loc in rows:
    std=re.search(r'en\d{4}',f).group(0)
    m=re.match(r'(.*?):(\d+):(\d+)$',loc)
    if not m: print(f"{std}\t{code}\t?\t{msg}\t(no loc)"); continue
    rel,line,col=m.group(1),int(m.group(2)),int(m.group(3))
    cands=[os.path.normpath(os.path.join(A+art[std],rel)), os.path.normpath(os.path.join(ROOT,rel))]
    p=next((c for c in cands if os.path.isfile(c)),None)
    if not p: print(f"{std}\t{code}\tMISSING {rel}\t{msg}"); continue
    txt=open(p,encoding="utf-8",errors="replace").read().split("\n")
    src=txt[line-1]
    print(f"{std}\t{code}\t{os.path.relpath(p,ROOT)}:{line}:{col}\t{msg[:70]}\n\t  => {src.strip()[:220]}")
