import re,sys
P="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/tealight/🔣️.json"
s=float(sys.argv[1])
lines=open(P,encoding="utf8").read().split("\n")
def num(x): return ("%.5f"%x).rstrip("0").rstrip(".")
def scale(d): return re.sub(r"(-?[0-9.]+) (-?[0-9.]+)",lambda m:num(float(m.group(1))*s)+" "+m.group(2),d)
e=next(k for k,l in enumerate(lines) if l.strip().startswith('"d": "M 0 0 C 0.0105 0.0025'))
f=next(k for k,l in enumerate(lines) if l.strip().startswith('"d": "M 0 0.0185 C 0.0105 0.02268'))
ed=re.match(r'(\s*"d": ")(.*)(",?)$',lines[e]); 
new_e=scale(ed.group(2))
lines[e]=ed.group(1)+new_e+ed.group(3)
fd=re.match(r'(\s*"d": ")(.*)(",?)$',lines[f])
shifted=re.sub(r"(-?[0-9.]+) (-?[0-9.]+)",lambda m:m.group(1)+" "+num(float(m.group(2))+0.0185),new_e)
lines[f]=fd.group(1)+shifted+fd.group(3)
start=next(k for k,l in enumerate(lines) if l.strip()=="[0.00163, 0.00439],")-2
assert lines[start].strip()=='"profile": ['
k=start+1
while lines[k].strip()!="],":
    m=re.match(r"(\s*\[)([^,]+), ([^\]]+)(\],?)$",lines[k])
    b=float(m.group(3))
    lines[k]=m.group(1)+m.group(2)+", "+("0.0" if b==0 else num(b*s))+m.group(4)
    k+=1
open(P,"w",encoding="utf8").write("\n".join(lines))
