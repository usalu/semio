import re,sys,collections
t=open(sys.argv[1],encoding="utf8").read()
rows=[]
for l in t.split("\n"):
    m=re.match(r"^(\S.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",l)
    if m: rows.append((m.group(1),m.group(4),m.group(5)[:160]))
open(sys.argv[2],"w",encoding="utf8").write("\n".join("\t".join(r) for r in sorted(rows))+"\n")
print(len(rows))
