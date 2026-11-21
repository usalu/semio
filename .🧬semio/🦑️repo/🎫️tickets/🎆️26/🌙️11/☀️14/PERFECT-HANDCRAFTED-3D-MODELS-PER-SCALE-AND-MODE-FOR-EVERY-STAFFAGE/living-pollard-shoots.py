"""🌳 Rebuild the upright and splayed London plane shoots from their own leaf groups on the five pollard heads."""
from pathlib import Path
from copy import deepcopy
import json
import runpy
t=Path(__file__).parent;b=runpy.run_path(str(t/'living-botanical-shells.py'));catalog=next(b['root'].glob('*/london-plane-pollarded/*.json'))
parts=json.loads((t/'🗑️generated/living-plane-controls.json').read_text())
for path in (catalog,t/'volumes/vegetation/london-plane-pollarded.json'):
    raw=path.read_text();v=raw.index('{',raw.index('"volumes":'));decoder=json.JSONDecoder()
    records,_=decoder.raw_decode(raw,v);fine=records['50'];materials={s['id']:s['material'] for s in fine['body']}
    regions={s['of']:s['region'] for s in fine['thermal'] if 'of' in s}
    edits=[]
    for k in ('100','200','500'):
        start=raw.index('{',raw.index('"'+k+'"',v));scale,end=decoder.raw_decode(raw,start)
        old={s['id'] for s in scale['body'] if s['material'].startswith('plane-leaf')};new=[]
        for s in parts:
            if s['material']!=materials[s['id']]:raise ValueError(s['id'])
            q=deepcopy(s);sections=q['sections'];n=len(sections);tip=q['id'].endswith('-tip');core=q['bone']=='fol-core'
            if k=='100':idx=list(range(n)) if tip else [0,1,2,3,n-3,n-2,n-1];sides=12
            if k=='200':idx=[0,1,round((n-1)*.42),round((n-1)*.68),n-2,n-1];sides=10 if not tip else 8
            if k=='500':idx=[0,round((n-1)*.26),round((n-1)*.57),n-2,n-1];sides=8 if core or not tip else 6
            b['loft'](q,sides,sorted(set(idx)));new.append(q)
        scale['body']=[s for s in scale['body'] if s['id'] not in old]+new
        scale['thermal']=[s for s in scale['thermal'] if s.get('of') not in old]+[dict(id='th-'+s['id'],of=s['id'],region=regions[s['id']]) for s in new]
        scale['decisions'][0]=dict(en='The upright leaders and splayed lateral shoots remain attached to the five pollard heads. Each shoot carries its basal, middle, upper and terminal foliage. Seven recessed groups occupy the shaded centre. Terminal groups become flatter polygons and the basal swellings keep fewer vertical sections at coarser scales.',de='Die aufrechten Leittriebe und die gespreizten Seitentriebe bleiben an den fünf Kopfästen angesetzt. Jeder Trieb trägt basale, mittlere, obere und endständige Laubpartien. Sieben zurückgesetzte Gruppen füllen die schattige Mitte. Endständige Gruppen erhalten zu den gröberen Maßstäben weniger Seitenflächen und die basalen Verdickungen weniger Höhenquerschnitte.')
        edits.append((start,end,json.dumps(scale,ensure_ascii=False,separators=(',',':'))))
    new=raw
    for i,j,s in reversed(edits):new=new[:i]+s+new[j:]
    if raw!=path.read_text():raise RuntimeError(path)
    path.write_text(new)

