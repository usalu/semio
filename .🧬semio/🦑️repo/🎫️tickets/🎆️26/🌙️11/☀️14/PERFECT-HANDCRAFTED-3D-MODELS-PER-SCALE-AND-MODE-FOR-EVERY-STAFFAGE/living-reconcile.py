"""🌿 Reconcile the authored source and product scale records against the original round-five tree baselines."""
import copy
import json
import math
from pathlib import Path

ticket=Path(__file__).parent
root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
decoder=json.JSONDecoder()
def replace(raw,key,value,start_at=0):
    k=raw.index('"'+key+'":',start_at)
    start=k+len(key)+3
    while raw[start].isspace():start+=1
    old,end=decoder.raw_decode(raw,start)
    if old==value:return raw
    indent=' '*max(0,k-raw.rfind('\n',0,k)-1) if raw[raw.rfind('\n',0,k)+1:k].isspace() else ''
    data=json.dumps(value,separators=(',',':'),ensure_ascii=False) if not indent else json.dumps(value,ensure_ascii=False,indent=2).replace('\n','\n'+indent)
    return raw[:start]+data+raw[end:]

def write(path,raw,value,extra):
    new=raw
    for k,v in value.items():new=replace(new,k,v,new.index('"volumes":'))
    for k,v in extra.items():new=replace(new,k,v)
    if new==raw:return
    if path.read_text()!=raw:raise RuntimeError(path)
    path.write_text(new)

def at(points,t):
    n=len(points)-1
    i=min(n-1,int(t*n));f=t*n-i
    return [a+(b-a)*f for a,b in zip(points[i],points[i+1])]

def position_delta(rows,live,base):
    for i,section in enumerate(rows):
        t=i/(len(rows)-1)
        l=at(live,t);o=at(base,t)
        section['at']=[round(x+y-z,7) for x,y,z in zip(section['at'],l,o)]

def combine(name,live,authored,base):
    if live==authored or authored==base:return live
    if live==base:return authored
    if live is None or authored is None:return live
    if name=='japanese-maple':return authored
    out=copy.deepcopy(authored)
    if live.get('kind')=='loft' and base.get('kind')=='capsule' and authored.get('kind')=='capsule':
        out=copy.deepcopy(live)
        position_delta(out['sections'],[authored['p0'],authored['p1']],[base['p0'],base['p1']])
    elif all(v.get('kind')=='loft' for v in (live,base,authored)):
        position_delta(out['sections'],[q['at'] for q in live['sections']],[q['at'] for q in base['sections']])
    for field in live:
        if field!='sections' and live.get(field)!=base.get(field) and authored.get(field)==base.get(field):out[field]=live[field]
    return out

def merge(name,l,s,o):
    result=copy.deepcopy(l)
    for k in l:
        for field in ('body','thermal','structure'):
            av,bv,ov=({x['id']:x for x in v[k][field]} for v in (l,s,o))
            values=[combine(name,av.get(i),bv.get(i),ov.get(i)) for i in dict.fromkeys([*av,*bv])]
            result[k][field]=[x for x in values if x is not None]
        for item in s[k]['decisions']:
            if item not in o[k]['decisions'] and item not in result[k]['decisions']:result[k]['decisions'].append(item)
    return result

for name in ('goat','pig','sheep','date-palm','english-oak','hornbeam-hedge','scots-pine','multi-stem-birch','japanese-maple','london-plane-pollarded'):
    p=next(root.glob('*/'+name+'/*.json'));q=next((ticket/'volumes').glob('*/'+name+'.json'))
    raw,authorraw=p.read_text(),q.read_text()
    a=json.loads(raw)['asset'];l=a['volumes'];s=json.loads(authorraw)['volumes']
    if l==s:continue
    chosen=l;extra={}
    if name in ('japanese-maple','multi-stem-birch','london-plane-pollarded'):
        folder=ticket/'🗑️generated'/('r5-'+name)
        baseline=next(folder.glob('before/source.json'),folder/'source-before-r5.json')
        chosen=merge(name,l,s,json.loads(baseline.read_text())['volumes'])
        if name=='london-plane-pollarded':
            anchors=json.loads((folder/'anchors.json').read_text())
            for item in a['anchors']:
                if item['id'] in anchors:item.update(anchors[item['id']])
            extra['anchors']=a['anchors']
    if name=='pig':
        chosen['500']['decisions'][0]={'en':'Fourteen body solids retain the two barrel sections, ham, short neck, head, separate ear flaps and an upper and lower mass for each leg. The lower legs reach the ground.','de':'Vierzehn Körper erhalten die beiden Rumpfabschnitte, Schinken, kurzen Hals, Kopf, getrennten Ohrlappen sowie je zwei Abschnitte für jedes Bein. Die unteren Beine reichen bis zum Boden.'}
        chosen['500']['decisions'][1]={'en':'The thermal model follows each ear flap. Distinct snout, hoof and tail surfaces carry their own regions. Breath leaves through the nasal openings.','de':'Das thermische Modell folgt beiden Ohrlappen. Rüssel, Klauen und Schwanz besitzen eigene Wärmeflächen. Die Atemluft tritt aus den Nasenöffnungen aus.'}
    for k in chosen:
        print('[DEBUG]',name,k,len(chosen[k]['body']),len(chosen[k]['thermal']),len(chosen[k]['structure'])) if chosen[k]!=l[k] or chosen[k]!=s[k] else None
    write(p,raw,chosen,extra);write(q,authorraw,chosen,{})

