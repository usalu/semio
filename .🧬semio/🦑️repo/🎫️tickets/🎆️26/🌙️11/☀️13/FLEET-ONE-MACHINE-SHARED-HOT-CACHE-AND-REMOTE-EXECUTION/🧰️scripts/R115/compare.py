import re,sys
def rows(path):
    out={}
    for line in open(path,encoding="utf8"):
        m=re.search(r"(\d+) fail.*<- (.+)$",line.strip())
        k=re.search(r"<- (.+)$",line.strip())
        if m:out[m.group(2).strip()]=int(m.group(1))
        if k and ("genuine-test failed" in line or not m):out[k.group(1).strip()]=max(out.get(k.group(1).strip(),0),99)
    return out
b,a=rows(sys.argv[1]),rows(sys.argv[2])
bad=[k for k,v in a.items() if v>b.get(k,0)]
print("worse:",bad) if bad else print("none worse")
sys.exit(1 if bad else 0)
