"""📐 Draws the five tappings at their installed heights, including the two end-on views and crown opening."""
from pathlib import Path
import json,copy
t=Path(__file__).resolve().parent;p=t/'🗑️generated'/'rotor'/'hot-water-cylinder.json';doc=json.loads(p.read_bytes());a=doc['asset'];fine=copy.deepcopy(a['representations']['50']['geometry'])
for scale,rep in a['representations'].items():
    for view in ['elevation','front','plan']:
        new=[]
        for src in fine[view]:
            if src.get('bone')!='connections':continue
            q=copy.deepcopy(src);nut=q['id'].startswith('nut')
            q['material']='brass'if nut else'enamelled-steel';q['region']='unions' if nut else 'connections'
            if scale!='50':q['penMm']=.2
            new.append(q)
            if (scale in ['50','100'])and ((view=='front'and q['id'] in ['stub-3','stub-4'])or(view=='plan'and q['id']=='hot-out')):
                cx,cy,rad=(.2039,0,.011)if view=='plan'else(0,.9227 if q['id']=='stub-3'else .5102,.0085)
                new.append({'id':'bore-'+q['id'],'bone':'connections','role':'detail','z':3.4,'kind':'ellipse','cx':cx,'cy':cy,'rx':rad,'ry':rad,'filled':True,'tone':.15,'penMm':.2})
        rep['geometry'][view]=[q for q in rep['geometry'][view]if q['bone']!='connections']+new
p.write_text(json.dumps(doc,ensure_ascii=False))
