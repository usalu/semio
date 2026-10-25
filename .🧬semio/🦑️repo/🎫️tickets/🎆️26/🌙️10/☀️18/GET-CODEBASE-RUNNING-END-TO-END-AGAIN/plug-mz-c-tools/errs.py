import re,sys,collections
log=open(sys.argv[1],errors="replace").read().splitlines()
for l in log:
    if l.startswith("error: could not compile"): print(l[:200])
agg=collections.OrderedDict()
for l in log:
    m=re.match(r"(.*?):(\d+):(\d+): error(\[E\d+\])?: (.*)",l)
    if m:
        msg=re.sub(r"`[^`]*`","`_`",m.group(5))[:110] if len(sys.argv)>2 else m.group(5)[:200]
        k=(m.group(4) or "",msg)
        e=agg.setdefault(k,[0,m.group(1)[-90:]+":"+m.group(2)])
        e[0]+=1
for (c,msg),(n,loc) in agg.items(): print(n,c,msg,"|",loc)
