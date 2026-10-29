import sys,re
for p in sys.argv[1:]:
    out=[];seen={};changed=False;sec=None
    for l in open(p,encoding="utf-8").read().split("\n"):
        m=re.match(r"\s*\[(.*)\]\s*$",l)
        if m: sec=m.group(1);seen={}
        k=re.match(r"([A-Za-z0-9_.\"-]+)\s*=",l)
        if k and sec is not None:
            key=k.group(1)
            if key in seen:
                if seen[key]==l: changed=True;print("dup",p,key);continue
                print("CONFLICT",p,key)
            seen[key]=l
        out.append(l)
    if changed: open(p,"w",encoding="utf-8").write("\n".join(out))
