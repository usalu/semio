import sys,re
# usage: tidy.py <file>...  : parent tidy after gsplit/hooks: no blank lines inside the Adapters region, one blank line after it, drop single-use `type <X>Core<N>` imports
for f in sys.argv[1:]:
    t=open(f,encoding="utf8").read()
    m=re.search(r'// #region 🔌️Adapters\n(.*?)// #endregion 🔌️Adapters\n',t,re.S)
    if m:
        body=m.group(1)
        body="\n".join(l for l in body.split("\n") if l.strip()!="")+"\n" if body.strip() else ""
        rest=t[m.end():]
        t=t[:m.start()]+"// #region 🔌️Adapters\n"+body+"// #endregion 🔌️Adapters\n"+("\n" if not rest.startswith("\n") else "")+rest
    def drop(mm):
        names=[x.strip() for x in mm.group(1).split(",")]
        keep=[]
        for n in names:
            bare=re.sub(r'^type\s+','',n)
            if re.fullmatch(r'\w+Core\d+',bare) and len(re.findall(r'\b'+bare+r'\b',t))==1: continue
            keep.append(n)
        if not keep: return ""
        return "import { "+", ".join(keep)+" } from "+mm.group(2)+";"
    t=re.sub(r'import \{ ([^}]*) \} from ("[^"]*");',drop,t)
    t=re.sub(r'\n\n\n+','\n\n',t)
    open(f,"w",encoding="utf8").write(t)
    print("tidied",f[-50:])
