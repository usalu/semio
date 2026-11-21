"""🌡️ Terminates skin at the wardrobe, hairline and anatomical joint interfaces of the individual figures."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
def bounded(sections,lo,hi):
    def interpolate(x):
        for a,b in zip(sections,sections[1:]):
            if a['at'][0]<=x<=b['at'][0]:
                u=(x-a['at'][0])/(b['at'][0]-a['at'][0])
                def mix(v,w):return round(v+u*(w-v),6)
                return {**a,'at':[mix(v,w) for v,w in zip(a['at'],b['at'])],'rx':mix(a['rx'],b['rx']),'ry':mix(a['ry'],b['ry'])}
        return min(sections,key=lambda a:abs(a['at'][0]-x))
    return [interpolate(lo),*[dict(q) for q in sections if lo+1e-6<q['at'][0]<hi-1e-6],interpolate(hi)]
masks={
 'tour-guide':{'th-head':(.051,.197),'th-neck':(.0506,.104),'th-wrist-r':(.201,.231)},
 'person-with-umbrella':{'th-head':(.022,.202),'th-neck':(.0341,.073)},
 'parent-carrying-toddler':{'th-toddler-head':(.020,.131)},
 'runner-with-blade-prosthesis':{**{'th-arm-'+s:(.018,.33) for s in ('l','r')},**{'th-forearm-'+s:(0,.26) for s in ('l','r')}}
}
for slug,limits in masks.items():
    path=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=path.read_text();text=old
    a=json.loads(old)['asset']
    start=text.index('"volumes": ')
    for key,v in a['volumes'].items():
        for p in v['thermal']:
            if p['id'] not in limits:continue
            ref=p.get('of',p['id'][3:])
            body=next(q for q in v['body'] if q['id']==ref)
            low,high=limits[p['id']]
            if p['id']=='th-toddler-head':
                hat=next(q for q in v['body'] if q['id']=='toddler-hat')
                high=hat['sections'][0]['at'][0]
            profile=bounded(body['sections'],low,high)
            if p['id']=='th-head':
                chin,temple=(.08,.185) if slug=='tour-guide' else (.055,.176)
                for q in profile:
                    x,y,z=q['at']
                    u=min(1,max(0,(x-chin)/(temple-chin)))
                    front=y-q['ry']
                    back=y+q['ry']*(1-u*(.99 if slug=='tour-guide' else .78))
                    q['at']=[x,round((front+back)/2,6),z]
                    q['ry']=round((back-front)/2,6)
            q={**body,**p,'sections':profile,'sectionFaces':['-u','+u']}
            q.pop('of',None)
            i=text.index('{\n            "id": "'+p['id']+'"',start)
            j=sp['member_end'](text,i)
            term=sp['dump'](q,5,'compact')
            text=text[:i]+term+text[j:]
            start=i+len(term)
    assert json.loads(text)
    assert path.read_text()==old
    path.write_text(text)
