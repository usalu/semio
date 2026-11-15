import json,sys,re
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/%s/🔣️.json"
def bbox(s):
    k=s.get('kind')
    if k=='rect': return (s['x'],s['y'],s['x']+s['w'],s['y']+s['h'])
    if k=='circle' or k=='ellipse':
        cx,cy=s.get('cx',s.get('x')),s.get('cy',s.get('y')); 
        rx=s.get('r',s.get('rx')); ry=s.get('r',s.get('ry'))
        return (cx-rx,cy-ry,cx+rx,cy+ry)
    pts=None
    if k=='path':
        d=s['d']; n=[float(x) for x in re.findall(r'-?\d+\.?\d*(?:e-?\d+)?',d)]
        if 'A' in d or 'a' in d: return ('arc',d[:80])
        pts=list(zip(n[0::2],n[1::2]))
    elif 'points' in s: pts=s['points']
    elif k=='line' or k=='polyline': pts=s.get('points')
    if pts:
        xs=[p[0] for p in pts];ys=[p[1] for p in pts]
        return (min(xs),min(ys),max(xs),max(ys))
    return None
def main(slug,scales,detail=True):
    a=json.load(open(S%slug))['asset']
    print(slug,json.dumps(a['dimensions']),a['presentation'])
    print('materials',[ (m['id']) for m in a['materials']])
    print('BONES')
    for b in a['rig']['bones']:
        print(' ',b['id'],'<-',b['parent'],'x',b.get('x'),'y',b.get('y'),'z',b.get('z'),'ang',b.get('angleDeg'),'len',b.get('lengthM'),'sec',b.get('section'),'j',b.get('joint',{}).get('type'),'m',b.get('mass',{}).get('kg'),b.get('mass',{}).get('comX'),b.get('mass',{}).get('comY'))
    print('ANCHORS',json.dumps(a.get('anchors'))[:1500])
    t=a['thermal']
    print('REGIONS')
    for r in t['regions']:
        print(' ',r['id'],r.get('kind'),r.get('material'),'area',r.get('areaM2'),'hidden',r.get('hidden'),[(l['id'],l.get('thicknessM'),l.get('material')) for l in r.get('layers',[])])
    print('EMITTERS',json.dumps(t.get('emitters'))[:1800])
    print('SUPPORTS',json.dumps(a['structure'])[:800])
    print('POSES',[(p['id'],p.get('bones')) for p in a['poses']])
    for sc in scales:
        r=a['representations'][sc]
        print('=== scale',sc,r.get('status'))
        if r.get('status')!='authored' and 'geometry' not in r: continue
        for v,l in r['geometry'].items():
            print(' --',v)
            for s in l:
                print('   ',s.get('id'),s.get('bone'),s.get('kind'),s.get('role'),s.get('filled'),s.get('region'),bbox(s) and tuple(round(x,4) if isinstance(x,float) else x for x in bbox(s)))
main(sys.argv[1],sys.argv[2].split(',') if len(sys.argv)>2 else ['50','100','200'])
