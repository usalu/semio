import sys
OLD="🌊️flow/🫀️core"; NEW="🌊️flow/🎁️browser-package"
root=sys.argv[1] if len(sys.argv)>1 else "."
files=["package.json","bun.lock","♻️mit-bestand/package.json","♻️mit-bestand/bun.lock","✏️s/package.json","✏️s/bun.lock","🌎️hub/package.json","🌎️hub/bun.lock","🎓️teaching/package.json","🎓️teaching/bun.lock","🏢️semio-tech/package.json","🏢️semio-tech/bun.lock"]
for f in files:
    p=root+"/"+f
    s=open(p,encoding="utf-8").read()
    n=s.count(OLD)
    if n: open(p,"wb").write(s.replace(OLD,NEW).encode("utf-8"))
    print(f,n)
