import re,sys
root="/Users/ueli/Documents/semio/"
bad=0;total=0
for line in open(sys.argv[1],encoding="utf8"):
    m=re.match(r"^(.*?)\((\d+),(\d+)\): error (TS\d+): (.*)$",line.rstrip("\n"))
    if not m: continue
    total+=1
    path,ln=m.group(1),int(m.group(2))
    try: src=open(root+path,encoding="utf8").read().split("\n")[ln-1]
    except Exception: continue
    if "Closed(" in src:
        bad+=1; print("CLOSED-LINE", path[-70:], ln, m.group(4), m.group(5)[:120])
print("diagnostics",total,"on lines calling Closed:",bad)
