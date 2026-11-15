import json,subprocess,sys,glob,os
S='/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog'
def load(slug):
    p=glob.glob(f'{S}/*/{slug}/🔣️.json')[0]
    return json.load(open(p))['asset'],p
def head(p):
    rel=p.replace('/Users/ueli/Documents/semio/','')
    try:
        return json.loads(subprocess.check_output(['git','-C','/Users/ueli/Documents/semio','show','HEAD:'+rel]))['asset']
    except Exception: return None
def r2(x): return round(x,4)
def sol(s):
    k=s.get('kind');o=s['id']
    if k=='box': g=s['size']
    elif k=='capsule' or k=='cylinder': g=[s.get('r0',s.get('radius')),s.get('r1'),s.get('p0'),s.get('p1'),s.get('height')]
    elif k=='ellipsoid': g=s['radii']
    elif k=='loft': g=[len(s['sections']),'x'+str(s.get('repeat',{}).get('count',1))]
    else: g=[v for kk,v in s.items() if kk in('radius','height','size','radii')]
    return f"{o}:{k}:{json.dumps(g)}"
def main(slug,solids=True):
    a,p=load(slug);h=head(p)
    print('=====',slug,json.dumps(a['dimensions']),a['summary']['en'][:200])
    old={r['id']:r for r in (h['thermal']['regions'] if h else [])}
    thsol={}
    v=a.get('volumes',{}).get('50',{})
    bodyby={s['id']:s for s in v.get('body',[])}
    for t in v.get('thermal',[]):
        if 'region' in t: thsol.setdefault(t['region'],[]).append(t.get('of',t['id']))
    for r in a['thermal']['regions']:
        o=old.get(r['id'],{})
        lay=[(l.get('thicknessM'),l.get('material')) for l in r['layers']]
        extra=''
        if r.get('operating'): extra+=f" heatW={r['operating']['heatW']}(old {o.get('operating',{}).get('heatW')})"
        if r.get('areaBasis'): extra+=' basis='+r['areaBasis']
        if r.get('body'): extra+=' body='+r['body']['segment']
        if r.get('hidden'): extra+=' hidden'
        ch=' CHANGED' if o and o.get('areaM2')!=r['areaM2'] else ''
        print(f" R {r['id']} [{r['kind']}/{r['model']}/{r['material']}] old={o.get('areaM2')} cur={r['areaM2']}{ch} L={r['characteristicLengthM']} {r['geometry']}{extra} layers={lay}")
        if solids and ch:
            print('    solids:',' | '.join(sol(bodyby[i]) for i in thsol.get(r['id'],[]) if i in bodyby)[:900])
if __name__=='__main__':
    for s in sys.argv[1:]: main(s)
