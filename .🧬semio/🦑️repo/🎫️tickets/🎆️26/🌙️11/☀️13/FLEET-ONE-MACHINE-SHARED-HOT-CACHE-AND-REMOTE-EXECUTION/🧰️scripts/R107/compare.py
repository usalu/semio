import re,sys
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R107/"
def load(n):
    d={}
    for line in open(SC+n+".txt",encoding="utf8"):
        m=re.match(r"(.*?) :\s*(?:(\d+) pass)?\s*(?:(\d+) fail)?",line)
        if line.startswith("MISSING"): d[line.strip().split("/")[-2]]=(0,0,True);continue
        if m: d[m.group(1)]=(int(m.group(2) or 0),int(m.group(3) or 0),False)
    return d
b,a=load(sys.argv[1]),load(sys.argv[2]);bad=[]
for d in (a,b):
    for k in [k for k in d if 'take-release' in k]:del d[k]
for k,(p,f,miss) in a.items():
    if k in b and not b[k][2] and (f>b[k][1] or p<b[k][0]):bad.append((k,b[k],(p,f)))
    if k not in b and not miss and f>0:bad.append((k,None,(p,f)))
for k,v in b.items():
    if not v[2] and (k not in a or a[k][2]):bad.append((k,v,"gone"))
print("RED" if bad else "GREEN");[print(x) for x in bad]
sys.exit(1 if bad else 0)
