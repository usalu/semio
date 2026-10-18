import re,sys,glob
out=[]
for p in sorted(glob.glob(sys.argv[1])):
    t=open(p,encoding="utf8",errors="replace").read()
    lines=t.split("\n")
    last_err=None; last_at=None
    for i,l in enumerate(lines):
        if l.startswith("error:") or l.startswith("error ") : last_err=l; last_at=None; k=i+1; blk=[]
        m=re.match(r"\s+at .*?\((/Users/ueli/Documents/semio/[^)]*)\)",l)
        if m and last_at is None and last_err is not None: last_at=m.group(1).replace("/Users/ueli/Documents/semio/","")
        f=re.match(r"\(fail\) (.*?)( \[[\d.]+m?s\])?$",l)
        if f:
            out.append((p.split("tsx-bun-")[1][:-4],f.group(1)[:90],(last_err or "")[:170],(last_at or "")[-120:]))
            last_err=None; last_at=None
for r in out: print(" | ".join(r))
print(len(out),"failures")
