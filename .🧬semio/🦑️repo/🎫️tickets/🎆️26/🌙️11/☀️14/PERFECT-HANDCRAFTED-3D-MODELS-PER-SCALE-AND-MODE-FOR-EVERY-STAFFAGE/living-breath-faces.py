"""🫁 Locate each species' nasal and oral apertures on its own tissue envelope at true size."""
from pathlib import Path
from copy import deepcopy
import json
import math
import runpy
import sys

ticket=Path(__file__).parent
catalog=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals'))
dimensions={
    'red-fox':{'nose':(.00115,.0020,.003),'mouth':(.00135,.007,.0024)},
    'labrador-dog':{'nose':(.0018,.0045,.006),'mouth':(.0025,.004,.015)},
    'dairy-cow':{'nose':(.0038,.008,.0125)},
    'horse':{'nose':(.020,.0105,.004)},
    'roe-deer':{'nose':(.0021,.0045,.007)},
    'herring-gull':{'nose':(.003,.0014,.0006)},
    'mallard-duck':{'nose':(.003,.0015,.00065)},
    'hedgehog':{'nose':(.00055,.00115,.002)},
    'red-squirrel':{'nose':(.00042,.00085,.0015)},
    'domestic-cat':{'nose':(.0009,.00225,.0035),'mouth':(.0013,.010,.0021)}
}
def shape(source,scale,radii):
    s=deepcopy(source);s['role']='detail'
    if 'texture' in s:s.pop('texture')
    if scale=='50':
        s['radii']=list(radii)
        return s
    normal=min(range(3),key=lambda i:radii[i]);axis=[(1,2),(2,0),(0,1)][normal];plane=['yz','zx','xy'][normal]
    rx,ry=[radii[i] for i in axis];n={'100':12,'200':8,'500':6}[scale]
    if n==6:angles=(0,60,90,180,240,270)
    else:angles=tuple(j*360/n for j in range(n))
    contour=[]
    for deg in angles:
        h=math.radians(deg);u,v=math.cos(h),math.sin(h);skew=1-.11*abs(u*v)
        contour.append([round(rx*u*skew,8),round(ry*v*skew,8)])
    s.pop('radii',None);s.update(kind='extrude',plane=plane,profile=contour)
    s['from']=-radii[normal];s['to']=radii[normal]
    return s
for slug in sys.argv[1:] or dimensions:
    canonical=next((catalog/slug).glob('*.json'));a=json.loads(canonical.read_text())['asset']
    sources=[s for s in a['volumes']['50']['thermal'] if s['id'].startswith('th-breath-')]
    for file in [canonical,ticket/'volumes/animals'/f'{slug}.json']:
        raw=file.read_text();decoder=json.JSONDecoder();p=raw.index('{',raw.index('"volumes":'));updates=[]
        for scale in ('50','100','200','500'):
            p=raw.index('{',raw.index('"'+scale+'"',p))
            i=raw.index('[',raw.index('"thermal":',p));items,end=decoder.raw_decode(raw,i)
            replacements=[shape(s,scale,dimensions[slug]['mouth' if 'mouth' in s['id'] else 'nose']) for s in sources]
            pos=next(n for n,s in enumerate(items) if s['id'].startswith('th-breath-'))
            keep=[s for s in items if not s['id'].startswith('th-breath-')]
            if slug=='red-fox' and scale!='50':
                keep=[s for s in keep if s['id']!='th-rhinarium']
            if slug in ('labrador-dog','roe-deer') and scale!='50':
                fine={s['id']:s for s in a['volumes']['50']['body']}
                for n,s in enumerate(keep):
                    name=s['id']
                    if 'of' in s:continue
                    source_id={'th-nose-leather':'nose-leather','th-nose':'nose-leather','th-tongue':'tongue'}.get(name)
                    if slug=='labrador-dog' and name.startswith('th-pad-'):source_id=name[3:]
                    if source_id in fine:
                        tissue=deepcopy(fine[source_id]);tissue.update(id=name,region=s['region'],role='detail')
                        if tissue['kind']=='loft':
                            sec=tissue['sections'];count=min(len(sec),{'100':7,'200':5,'500':4}[scale])
                            tissue['sections']=[sec[round(j*(len(sec)-1)/(count-1))] for j in range(count)]
                        keep[n]=tissue
            revised=keep[:pos]+replacements+keep[pos:]
            if revised!=items:updates.append((i,end,json.dumps(revised,ensure_ascii=False,separators=(',',':'))))
            p=end
        revised=raw
        for i,end,s in reversed(updates):revised=revised[:i]+s+revised[end:]
        if revised!=raw:
            if file.read_text()!=raw:raise RuntimeError(file)
            file.write_text(revised)
        if slug=='red-fox':
            def muzzle(k,m,s):
                if k!='200' or m!='body' or s['id']!='head':return False
                at=s['sections'][-1];base=[q for q in s['sections'] if q['at'][0]<.135]
                for x,y,w,h in ((.14,-.001,.014,.0125),(.152,-.0018,.0091,.0086),(.1595,-.0024,.0068,.0062)):
                    base.append({**at,'at':[x,y,0],'rx':w,'ry':h})
                s['sections']=base;return True
            runpy.run_path(str(ticket/'living-shape-rows.py'))['edit'](file,muzzle)
    if slug=='labrador-dog':
        raw=canonical.read_text();i=raw.index('[',raw.index('"anchors":'));anchors,n=json.JSONDecoder().raw_decode(raw,i)
        lip=next(s for s in sources if 'mouth' in s['id'])
        at=lip['transform']['translation'];mouth=next(s for s in anchors if s['id']=='mouth')
        mouth.update(bone=lip['bone'],x=at[0],y=at[1])
        new=raw[:i]+json.dumps(anchors,ensure_ascii=False,separators=(',',':'))+raw[n:]
        if raw!=canonical.read_text():raise RuntimeError(canonical)
        canonical.write_text(new)

