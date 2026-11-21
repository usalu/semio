"""📏 Verifies pan rim, fuel datum and grate rod diameters, with six separate open plumbing connections at each scale."""
import json,math,sys
from pathlib import Path
t=Path(__file__).resolve().parent;r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog'))
for slug in ['charcoal-grill','hot-water-cylinder']:
    path=(t/'🗑️generated'/'rotor'/(slug+'.json'))if '--candidate'in sys.argv else next(r.glob('*/'+slug+'/*'));a=json.loads(path.read_bytes())['asset']
    for d,v in a['volumes'].items():
        b={q['id']:q for q in v['body']};issues=[]
        def test(ok,key):
            if not ok:issues.append(key)
        def same(x,y):return math.isclose(x,y,abs_tol=1e-6)
        if slug=='charcoal-grill':
            pan=b['ash-pan']['profile'];bed=b['charcoal-bed']['profile']
            test(same(min(x[0]for x in pan),.385) and same(max(x[0]for x in pan),.418) and same(max(x[1]for x in pan),.156),'pan floor/rolled edge')
            test(same(min(x[0]for x in bed),-.002) and same(max(x[0]for x in bed),.038) and same(max(x[1]for x in bed),.195),'supported fuel bed')
            for name,n in [('pan-hanger-',3),('fuel-bar-',9),('grate-bar-',27),('briquette-',14)]:test(sum(q.startswith(name)for q in b)==n,name)
        else:
            for i in range(6):
                key='hot-out'if i==5 else 'stub-'+str(i);q=b.get(key,{})
                test(q.get('kind')=='revolve' and same(min([x[1]for x in q.get('profile',[])]or[0]),.011 if i==5 else .0085) and same(max([x[1]for x in q.get('profile',[])]or[0]),.015 if i==5 else .012),key+' diameter')
                test(q.get('material')=='enamelled-steel' and q.get('lateralScale',1)==1,key+' steel round pipe')
                if i<5:test(len(b.get('nut-'+str(i),{}).get('holes',[]))==1,key+' union bore')
        print(slug,d,'pass' if not issues else issues)
        if issues:sys.exitcode=1
if getattr(sys,'exitcode',0):sys.exit(1)
