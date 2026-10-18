import re,sys,glob
for p in sorted(glob.glob(sys.argv[1])):
    t=open(p,encoding="utf8",errors="replace").read()
    blocks=re.split(r"\n(?=\((?:pass|fail|skip)\) )",t)
    for b in blocks:
        m=re.match(r"\(fail\) (.*)",b)
        if not m: continue
    # blocks are split BEFORE the pass/fail lines; error text precedes its (fail) line, so it sits at the tail of the previous block
    prev=None
    for i,b in enumerate(blocks):
        m=re.match(r"\(fail\) (.*)",b)
        if m and i>0:
            body=blocks[i-1]
            # drop the '(pass)/(fail) name' first line of prev block
            body="\n".join(body.split("\n")[1:])
            err=re.search(r"\n?(error:.*?)(?:\n\s+at |\Z)",body,re.S)
            at=re.search(r"\n\s+at (?:.*?\()?(/Users/ueli/Documents/semio/[^):]*(?:\:\d+:\d+)?)",body)
            print("##",p.split("tsx-bun-")[1][:-4],"|",m.group(1)[:110])
            print("   AT:",(at.group(1).replace("/Users/ueli/Documents/semio/","") if at else "?"))
            print("   ",(err.group(1)[:420] if err else "?").replace("\n","\n    "))
