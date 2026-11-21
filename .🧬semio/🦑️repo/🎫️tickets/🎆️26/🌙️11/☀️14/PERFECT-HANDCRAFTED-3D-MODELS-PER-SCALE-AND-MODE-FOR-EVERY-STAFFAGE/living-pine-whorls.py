"""🌲 Define the needle pads of the Scots pine on their genuine open whorls and lateral crown carriers."""
from copy import deepcopy
from pathlib import Path
import runpy
import json
t=Path(__file__).parent;b=runpy.run_path(str(t/'living-botanical-shells.py'));p=next(b['root'].glob('*/scots-pine/*.json'))
for path in (p,t/'volumes/vegetation/scots-pine.json'):
    raw=path.read_text();d=json.JSONDecoder();v=raw.index('{',raw.index('"volumes":'));records,_=d.raw_decode(raw,v)
    fine=records['50'];needles=[s for s in fine['body'] if 'pine-needles' in s['material']]
    regions={s['of']:s['region'] for s in fine['thermal'] if 'of' in s}
    edits=[]
    for k in ('100','200','500'):
        i=raw.index('{',raw.index('"'+k+'"',v));scale,end=d.raw_decode(raw,i)
        old={s['id'] for s in scale['body'] if 'pine-needles' in s['material']}
        clumps=[]
        for s in needles:
            rad=[max(abs(pt[k]) for pt in s['vertices']) for k in range(3)]
            tip='-t' in s['id'][3:];centre='-c' in s['id'];core=s['bone']=='pl-top'
            if k=='100':width=16 if centre else 12;latitude=(-65,-40,-17,0,21,44,68)
            if k=='200':width=12 if not tip else 10;latitude=(-57,-27,0,29,59)
            if k=='500':width=10 if core or centre else 8;latitude=(-50,-20,0,23,52) if core or centre else (-49,0,49)
            q=deepcopy(s);b['oval'](q,rad,width,latitude);clumps.append(q)
        scale['body']=[s for s in scale['body'] if s['id'] not in old]+clumps
        scale['thermal']=[s for s in scale['thermal'] if s.get('of') not in old]+[dict(id='th-'+s['id'],of=s['id'],region=regions[s['id']]) for s in clumps]
        scale['decisions'][0]=dict(en='Six lateral needle platforms, the leader and the two depth-facing clusters retain their own asymmetric radial, central and terminal pads. Lower platforms are shaded and their neighbouring branches remain separated by open air. The coarser surfaces use fewer radial faces and vertical sections, especially at the shoot tips.',de='Sechs seitliche Nadelplattformen, der Leittrieb und die beiden in die Tiefe gerichteten Gruppen behalten ihre eigenen asymmetrischen Rand-, Mittel- und Endpolster. Die unteren Plattformen liegen im Schatten. Zwischen den benachbarten Ästen bleiben offene Lücken. Zu den groben Maßstäben nehmen die Umfangsflächen und Höhenquerschnitte besonders an den Triebspitzen ab.')
        edits.append((i,end,json.dumps(scale,ensure_ascii=False,separators=(',',':'))))
    new=raw
    for i,j,s in reversed(edits):new=new[:i]+s+new[j:]
    if raw!=path.read_text():raise RuntimeError(path)
    path.write_text(new)

