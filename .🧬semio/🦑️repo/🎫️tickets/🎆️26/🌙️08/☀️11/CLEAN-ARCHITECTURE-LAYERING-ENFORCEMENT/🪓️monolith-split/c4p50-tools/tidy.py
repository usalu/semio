import sys,re
# usage: tidy.py <file>...: one blank line after every `//#endregion` line that is followed by code or a `//#region`, and before every `//#region` that follows code; no 3+ blank runs
for f in sys.argv[1:]:
    t=open(f,encoding="utf8").read()
    lines=t.split("\n")
    out=[]
    for i,l in enumerate(lines):
        s=l.strip()
        if s.startswith("//#region") and out and out[-1].strip()!="" and not out[-1].strip().startswith("//#region"):
            out.append("")
        out.append(l)
        if s.startswith("//#endregion") and i+1<len(lines):
            n=lines[i+1].strip()
            if n!="" and not n.startswith("//#endregion"):
                out.append("")
    if len(out)>1 and out[0].startswith("/**") and out[0].rstrip().endswith("*/") and out[1].strip()!="":
        out.insert(1,"")
    t="\n".join(out)
    t=re.sub(r'\n\n\n+','\n\n',t)
    open(f,"w",encoding="utf8").write(t)
    print("tidied",f.split("/")[-2:])
