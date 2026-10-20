"""📂 Per-hub module report: every mounted file with lines and commit count, prod vs test. Run `python3 modules.py <hub>`."""
from hubenv import *
h=sys.argv[1];d=hubdir(h);fs=tree(h);ch=churn_files(fs)
base=os.path.join(ROOT,d)
allc=set()
for f in fs: allc|=ch.get(f,set())
prodc=set()
for f in fs:
    if isprod(f): prodc|=ch.get(f,set())
print(h,'files',len(fs),'prod lines',sum(nlines(f) for f in fs if isprod(f)),'commits(all)',len(allc),'commits(prod)',len(prodc))
for f in sorted(fs):
    print('%5d %3d %s %s'%(nlines(f),len(ch.get(f,())),'P' if isprod(f) else 't',os.path.relpath(f,os.path.join(ROOT,d))))
