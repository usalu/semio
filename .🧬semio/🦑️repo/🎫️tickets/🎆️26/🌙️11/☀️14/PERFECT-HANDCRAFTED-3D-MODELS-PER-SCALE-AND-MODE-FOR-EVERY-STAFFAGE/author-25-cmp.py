import glob
import json,sys
base='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/'
slug=sys.argv[1]; a,b=sys.argv[2],sys.argv[3]
d=json.load(open(glob.glob('/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*/'+slug+'/🔣️.json')[0]))['asset']['representations']
def bb(s):
    pts=s.get('points')
    if not pts: return None
    xs=[p[0] for p in pts]; ys=[p[1] for p in pts]
    return (min(xs),max(xs),min(ys),max(ys))
for view in ['elevation','front','plan']:
    A={}
    for s in d[a]['geometry'][view]:
        if s['role']=='body' and bb(s): A.setdefault(s['bone'],[]).append((s['id'],bb(s)))
    for s in d[b]['geometry'][view]:
        if s['role']!='body': continue
        q=bb(s)
        if not q: print(view,s['id'],s['bone'],'nopoints',s.get('kind')); continue
        r=A.get(s['bone'])
        if not r: print(view,s['id'],s['bone'],'no twin at',a); continue
        best=min(max(abs(q[i]-t[1][i]) for i in range(4)) for t in r)
        print(view,s['id'],s['bone'],'local bbox diff vs',a,round(best,3), 'OK' if best<0.04 else 'BAD')
