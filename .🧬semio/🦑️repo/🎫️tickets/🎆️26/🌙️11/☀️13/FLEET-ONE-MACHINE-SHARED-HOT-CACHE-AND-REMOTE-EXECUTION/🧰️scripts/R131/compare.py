import sys,re
def read(p):
    d={}
    for l in open(p):
        m=re.match(r"(\S.*?) (\d+)pass (\d+)fail a\d",l.strip())
        if m:d[m.group(1)]=(int(m.group(2)),int(m.group(3)))
    return d
b,a=read(sys.argv[1]),read(sys.argv[2]);bad=[]
for k in sorted(set(a)):
    pb,fb=b.get(k,(0,0));pa,fa=a.get(k,(0,0))
    flag="" if (fa<=fb and pa>=pb) else "WORSE"
    print(("%-90s"%k[-90:]),pb,fb,"->",pa,fa,flag)
    if flag:bad.append(k)
print("RED" if bad else "GREEN",len(bad));sys.exit(1 if bad else 0)
