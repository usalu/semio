"""🕳️ Reach distribution and top hubs. Run with `cwd` = the generated folder."""
from sim import *
import subprocess,collections
from tree import ROOT
E=base
allc=list(C)
# reach for all crates via bitsets
r=revof(E)
idx={n:i for i,n in enumerate(allc)}
import sys
sys.setrecursionlimit(1000000)
memo={}
def f(x):
    if x in memo: return memo[x]
    memo[x]=0;b=0
    for y in r.get(x,()): b|=f(y)|(1<<idx[y])
    memo[x]=b;return b
reach={n:bin(f(n)).count('1') for n in allc}
vals=sorted(reach.values(),reverse=True)
for th in (2800,2500,2000,1500,1000,500,100):
    print('reach >=',th,sum(1 for v in vals if v>=th))
fm=set(fam)
top=sorted(allc,key=lambda n:-reach[n])
print('top 40 by reach:')
rows=[]
for n in top[:60]:
    c=C[n]
    cm=int(subprocess.run(['git','log','--format=%h','--',n],cwd=ROOT,capture_output=True,text=True).stdout.count('\n'))
    rows.append((reach[n],len(rev[n]),cm,'FAM' if n in fm else '',c['name'].replace('semio-framework-','')))
for x in rows: print(x)
json.dump({C[n]['name']:reach[n] for n in allc},open('reach.json','w'))
