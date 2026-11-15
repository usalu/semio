import re,sys,collections
t=open(sys.argv[1],errors="ignore").read()
cur=None
rows=[]
for l in t.splitlines():
    m=re.match(r"\s+Running (\S+(?: \S+)?) \(.*?/build/([^/]+)/",l)
    if m: cur=(m.group(2).replace("semio-s-plugin-norm-","").replace("semio-s-artifact-norm-","A:"),m.group(1))
    m=re.match(r"test result: (\w+)\. (\d+) passed; (\d+) failed",l)
    if m and cur: rows.append((cur[0],cur[1].replace("unittests ","lib ").replace("../../",""),m.group(1),int(m.group(2)),int(m.group(3))))
    if m: cur=None
for r in rows:
    if r[3] or r[4]: print(*r)
print("TOTAL pass",sum(r[3] for r in rows),"fail",sum(r[4] for r in rows))
