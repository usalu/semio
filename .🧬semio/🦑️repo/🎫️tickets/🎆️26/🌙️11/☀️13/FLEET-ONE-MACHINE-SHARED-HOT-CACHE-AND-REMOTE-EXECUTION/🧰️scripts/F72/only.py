import sys,os,re;sys.path.insert(0,os.path.dirname(__file__))
from f72lib import *
import sys
pats=sys.argv[1:]
s=open(TEST,encoding="utf8").read()
out=[]
for line in s.split("\n"):
    m=re.match(r'^test\("([^"]*)"',line)
    if m and not any(p in m.group(1) for p in pats):line=line.replace('test("','test.skip("',1)
    out.append(line)
d=os.path.dirname(TEST)
write(d+"/🟦️f72.ts","\n".join(out))
print("written",d+"/🟦️f72.ts")
