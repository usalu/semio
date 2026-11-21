"""🐐 Restore the caprine cannon, coronet, horns and load-bearing shafts on the actual kinematic bones."""
import json
from copy import deepcopy
from pathlib import Path
t=Path(__file__).parent;slug='goat';source=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/goat/*.json'))
def sample(s,stations,id):
    q=deepcopy(s);q['id']=id;sects=q.get('sections')
    if sects:
        indices=sorted(set(round((len(sects)-1)*x) for x in stations))
        q['sections']=[sects[i] for i in indices]
    return q
for p in [source,t/'volumes/animals/goat.json']:
    raw=p.read_text();d=json.JSONDecoder();v=raw.index('{',raw.index('"volumes":'))
    volumes,_=d.raw_decode(raw,v);fine=volumes['50'];medium=volumes['200']
    parts={s['id']:s for s in fine['body']};mid={s['id']:s for s in medium['body']};bones={s['id']:s for s in fine['structure']}
    region={s['of']:s['region'] for s in medium['thermal'] if 'of' in s}
    k=raw.index('{',raw.index('"500"',v));coarse,end=d.raw_decode(raw,k)
    changes={};extras={}
    horns=(0,.17,.43,.72,1);ears=(0,.3,.6,1);gaskin=(0,.28,.61,1);forearm=(0,.26,.7,1);cannon=(0,.17,.78,1)
    for side in ('n','f'):
        for name,stations in [('horn-'+side,horns),('ear-'+side,ears)]:
            changes[name]=sample(parts[name],stations,name)
        for endBone,old,new,stations in [('h','hind-upper','gaskin',gaskin),('f','fore-upper','forearm',forearm)]:
            ident=old+'-'+side;changes[ident]=sample(parts[new+'-'+side],stations,ident)
            name=('hind-lower' if endBone=='h' else 'fore-lower')+'-'+side
            changes[name]=sample(parts['cannon-'+endBone+side],cannon,name)
        for name in ('thigh-'+side,'shoulder-'+side):
            extras[name]=sample(parts[name],(0,.27,.66,1),name)
    coarse['body']=[changes.get(s['id'],s) for s in coarse['body'] if s['id'] not in extras]+list(extras.values())
    coarse['thermal']=[s for s in coarse['thermal'] if s['id'] not in {'th-'+id for id in extras}]
    coarse['thermal'] += [dict(id='th-'+id,of=id,region='hooves' if id.startswith('hoof-') else region[id]) for id in extras]
    for n,s in enumerate(coarse['structure']):
        if s.get('part')=='member' and s.get('id') in bones and (s.get('bone','').endswith(('hn','hf','fn','ff')) or s.get('bone')=='jaw'):
            finebone=bones[s['id']]
            stations=(0,.22,.71,1) if 'tibia' in s['id'] else (0,.43,1)
            coarse['structure'][n]=sample(finebone,stations,s['id'])
    coarse['decisions'][0]=dict(en='The folded ear and recurved horn retain their measured thickness and taper. Both pairs of cannon bones stop at the fetlocks. Each sloped pastern and hoof turns with its own distal joint. The thigh and shoulder bulges are carried on their respective bones.',de='Das gefaltete Ohr und das zurückgebogene Horn behalten ihre Dicke und Verjüngung. Alle vier Röhrenbeine enden an den Fesselgelenken. Fessel und Klaue drehen sich mit ihrem eigenen Endgelenk. Oberschenkel und Schulter folgen ihren jeweiligen Knochen.')
    coarse['decisions'][1]=dict(en='Keratin at the four feet follows the articulated coronet and sloped hoof. The two nasal apertures and the lip retain their measured widths.',de='Das Horn an allen vier Füßen folgt der beweglichen Krone und der schrägen Klaue. Die beiden Nasenöffnungen und die Lippe behalten ihre gemessenen Breiten.')
    coarse['decisions'][2]=dict(en='The radius, tibia, metapodials and phalanges retain their anatomical shaft widths and ends at the true kinematic joint centres. Paired rib arches support the chest.',de='Radius, Tibia, Röhrenbeine und Zehenglieder behalten ihre anatomischen Schaftbreiten. Ihre Enden liegen an den tatsächlichen Gelenkzentren. Paarige Rippenbögen tragen den Brustkorb.')
    text=raw[:k]+json.dumps(coarse,ensure_ascii=False,separators=(',',':'))+raw[end:]
    if raw!=p.read_text():raise RuntimeError(p)
    p.write_text(text)

