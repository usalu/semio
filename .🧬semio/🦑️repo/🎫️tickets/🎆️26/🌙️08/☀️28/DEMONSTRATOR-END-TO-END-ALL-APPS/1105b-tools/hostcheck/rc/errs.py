import re,sys
t=open(sys.argv[1],encoding="utf-8").read()
print("errors:",len(re.findall(r"^error\[",t,re.M)))
seen=set()
for m in re.finditer(r"^error(\[E\d+\])?: ([^\n]{0,150})\n\s*--> ([^\n]+)",t,re.M):
    loc=m.group(3);loc=loc.split("/../")[-1] if "/../" in loc else loc
    k=(m.group(2)[:70],loc)
    if k in seen: continue
    seen.add(k);print(m.group(1),m.group(2)[:100],"|",loc[-70:])
