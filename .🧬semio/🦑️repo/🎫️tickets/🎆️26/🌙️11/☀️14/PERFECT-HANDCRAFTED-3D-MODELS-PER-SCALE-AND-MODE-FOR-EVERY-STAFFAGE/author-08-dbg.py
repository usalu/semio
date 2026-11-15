import sys, json
from author_08_lib import *
slug=sys.argv[1]; sc=sys.argv[2]
m=measure('humans',slug)[sc]
bb=m['bbox']; ft=FEATURE[sc]
print('bbox',[round(x,3) for x in bb['min']],[round(x,3) for x in bb['max']])
for r in m['thermal']:
    if r['min'] is None: continue
    out=[k for k in range(3) if r['min'][k]<bb['min'][k]-ft or r['max'][k]>bb['max'][k]+ft]
    if out: print('thermal outside',r['id'],out,[round(x,3) for x in r['min']],[round(x,3) for x in r['max']])
for r in m['structure']:
    if r['id'].startswith('support') or r['id'].startswith('mass'): print(r['id'],[round(x,3) for x in r['min']],[round(x,3) for x in r['max']])
