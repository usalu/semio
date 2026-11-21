"""🛢️ Seat the packing and gland follower around the polished rod and close the fluid boundary at grade."""
from pathlib import Path
from copy import deepcopy
import json
T=Path(__file__).resolve().parent
CAT=next(Path.cwd().glob('🎓*/*architecture/*staffage/*catalog/*/oil-pumpjack/🔣️.json'))
def author(a):
    a['materials'].append(dict(id='rod-packing',title={'en':'Reinforced elastomer rod packing','de':'Gewebeverstärkte Elastomer-Stangenpackung'},emissivity=.93,solarAbsorptivity=.85,densityKgM3=1250,conductivityWmK=.3,specificHeatJKgK=1350,source='authored-estimate'))
    next(r for r in a['thermal']['regions']if r['id']=='wellhead')['areaM2']=1.49
    for d,v in a['volumes'].items():
        body={q['id']:q for q in v['body']};head=next(body[i]for i in ['stuffing-box','wellhead-top','wellhead']if i in body)
        profile=head['profile'];top=profile[-2][0];assert profile[-2][1]==.0285
        profile[-2:-1]=[[top,.039],[round(top-.058,5),.039],[round(top-.058,5),.0285]]
        for name,z,h,inner,outer,mat in [('rod-packing',1.222,.04,.025,.039,'rod-packing'),('packing-follower',1.262,.018,.0255,.0388,'painted-steel')]:
            s=dict(id=name,kind='revolve',bone='base',role='detail',material=mat,tone=.25 if name=='rod-packing'else .52,axis=[0,1,0],origin=[-3,z,0],profile=[[0,inner],[0,outer],[h,outer],[h,inner],[0,inner]])
            v['body'].append(s);v['thermal'].append(dict(id='th-'+name,of=name,region='wellhead'))
            if name=='packing-follower':v['structure'].append(dict(id='member-'+name,of=name,member='base',part='member'))
        c=next(q for q in v['thermal'] if q['id']=='th-well-casing');src=deepcopy(body['well-casing'])
        src.update(id=c['id'],region='wellhead',sectionFaces=c['sectionFaces'],profile=[[0,0],[0,.085],[1.3,.085],[1.3,0]])
        c.clear();c.update(src)
        ground=next(q for q in v['thermal']if q.get('of')in ('wellhead-base','wellhead'));ground['sectionFaces']=['-v']
    a['thermal']['assumptions'].append({'en':'The buried, fluid-filled well casing has no atmospheric exchange. The reinforced packing seals the 50 mm polished rod inside the stepped 78 mm stuffing-box bore. Only the exposed head, valve, gland and flowline transfer heat to the air.','de':'Die erdverlegte, fluidgefüllte Verrohrung tauscht keine Wärme mit der Außenluft aus. Die verstärkte Packung dichtet die 50-mm-Polierstange in der auf 78 mm erweiterten Stopfbuchsbohrung ab. Nur die frei liegenden Flächen von Kopf, Ventil, Stopfbuchse und Förderleitung geben Wärme an die Luft ab.'})
if __name__=='__main__':
    doc=json.loads(CAT.read_bytes());author(doc['asset'])
    (T/'🗑️generated/digital-pump-gland.json').write_text(json.dumps(doc,ensure_ascii=False,separators=(',',':')))
