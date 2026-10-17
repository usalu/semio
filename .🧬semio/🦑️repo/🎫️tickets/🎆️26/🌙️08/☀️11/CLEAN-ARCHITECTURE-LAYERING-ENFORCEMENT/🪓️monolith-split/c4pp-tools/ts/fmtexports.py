import sys,re
# usage: fmtexports.py <file> <needle-substring-of-from-spec>... : reflow single-line export lists (> 120 chars) that import from a spec containing a needle into multi-line form
f=sys.argv[1]; needles=sys.argv[2:]
t=open(f,encoding="utf8").read()
out=[];n=0
for line in t.split("\n"):
    m=re.match(r'^export (type )?\{ (.*) \} from ("[^"]*");$',line)
    if m and len(line)>120 and any(x in m.group(3) for x in needles):
        names=[x.strip() for x in m.group(2).split(", ")]
        out.append("export "+(m.group(1) or "")+"{\n"+",\n".join("  "+x for x in names)+"\n} from "+m.group(3)+";"); n+=1
    else: out.append(line)
open(f,"w",encoding="utf8").write("\n".join(out))
print("reflowed",n)
