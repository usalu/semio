import re,sys
def table(p):
    r={}
    for l in open(p,encoding="utf8"):
        m=re.match(r"(\S+)\s+pass=(-?\d+) fail=(-?\d+)",l)
        if m:r[m.group(1)]=(int(m.group(2)),int(m.group(3)))
    return r
b=table(sys.argv[1]);a=table(sys.argv[2]);bad=0
for k,(p,f) in a.items():
    if k in b:
        bp,bf=b[k]
        if f>bf or p<bp:print("WORSE",k,b[k],"->",a[k]);bad=1
    elif f!=0 or p<1:print("RED-NEW",k,a[k]);bad=1
sys.exit(bad)
