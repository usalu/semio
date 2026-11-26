"""🔬️ Measures the pinned details of lane CL-H in catalog files or trial copies: probe.py [folder with <id>.json] (default: the catalog)."""
import json,math,sys,os
C='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/'
def load(a):
    p=os.path.join(sys.argv[1],a+'.json') if len(sys.argv)>1 else C+a+'/🔣️.json'
    return json.load(open(p))['asset']
def sec(s):
    if s is None: return None
    if s['kind']=='capsule': return (s['r0'],s['r1'])
    if s['kind']=='loft':
        n=min(s['sections'],key=lambda b:b['rx']*b['ry']); return (n['rx'],n['ry'])
    return s['kind']
for a in ['person-on-crutches','tour-guide']:
    d=load(a); print(a)
    for sh in ['femur','tibia','humerus','radius','ulna']:
        for side in 'lr':
            i=sh+'-'+side
            hi=sec(next((x for x in d['volumes']['50']['structure'] if x['id']==i),None))
            row=[]
            for sc,v in d['volumes'].items():
                s=next((x for x in v['structure'] if x['id']==i),None); c=sec(s)
                row.append((sc,s and s['kind'],c, round(math.dist(c,hi)*1000,3) if c and hi and not isinstance(c,str) else None))
            print(' ',i,row)
for a,outs in [('parent-carrying-toddler',[('toddler-nostrils','th-toddler-nose-breath'),('toddler-mouth','th-toddler-mouth-breath')]),('parent-with-stroller',[('child-nose','th-child-nose-breath'),('child-mouth','th-child-mouth-breath')])]:
    d=load(a); print(a)
    for lm,i in outs:
        an=next(q for q in d['anchors'] if q['id']==lm); print(' ',an)
        for sc,v in d['volumes'].items():
            o=next((q for q in v['thermal'] if q['id']==i),None)
            t=o['transform']['translation']; print('    ',sc,o.get('bone'),t, round(1000*math.dist(t,(an['x'],an['y'],an.get('z',0))),3),'mm', {k:o[k] for k in o if k not in('transform',)})
d=load('parent-with-stroller')
for sc,v in d['volumes'].items():
    for i in ['child-foot-l','child-foot-r','child-shin-l','child-hand-l']:
        print(' ',sc,i,'body',[ (s['id'],s['kind']) for s in v['body'] if s.get('bone')==i],'struct',[(s['id'],s['kind'],s.get('part')) for s in v['structure'] if s.get('member')==i])
