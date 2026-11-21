"""🌳 Relate the canopy's three fountain tiers and ragged hem to the same pendant branches at four scales."""
from pathlib import Path
from copy import deepcopy
import json
import re
import runpy
t=Path(__file__).parent
b=runpy.run_path(str(t/'living-botanical-shells.py'));p=next(b['root'].glob('*/weeping-willow/*.json'))
construction=json.loads((t/'🗑️generated/r5-weeping-willow/🔣️.json').read_text())['asset']
m=construction['volumes']['50']
for path in (p,t/'volumes/vegetation/weeping-willow.json'):
    raw=path.read_text();v=raw.index('{',raw.index('"volumes":'));d=json.JSONDecoder();edits=[]
    for k in ('100','200','500'):
        i=raw.index('{',raw.index('"'+k+'"',v));scale,end=d.raw_decode(raw,i)
        tresses=[]
        for part in m['body']:
            if not part['id'].startswith('tress-'):continue
            s=deepcopy(part);n=len(s['sections']);arc=s['id'].endswith('-arch');group=s['id'][6]
            if k=='100':
                keep=[j for j in range(n) if j%4!=2];sides=10 if arc else 8
            elif k=='200':
                keep=list(range(0,n,2));sides=8
            else:
                keep=list(range(0,n,3 if arc or group=='c' else 4));sides=8 if arc else 6
            keep=sorted({0,1,n-2,n-1,*keep})
            b['loft'](s,sides,keep);tresses.append(s)
        scale['body']=[s for s in scale['body'] if not s['id'].startswith('tress-')]+tresses
        scale['thermal']=[s for s in scale['thermal'] if not s.get('of','').startswith('tress-')]+[dict(id='th-'+s['id'],of=s['id'],region='leaf-curtain') for s in tresses]
        scale['decisions'][0]=dict(en='The three ranks of willow sprays follow their distinct arched shoulders and hanging shoots. The longest curtains retain their low ends and the shorter curtains open at the same recessed bays. The upper arches and hanging portions keep their own wind attachments. The intermediate cross sections reduce as the drawing scale decreases.',de='Die drei Reihen der Weidentriebe folgen ihren eigenen gewölbten Schultern und überhängenden Zweigen. Lange Vorhänge behalten ihre tiefen Enden. Kürzere Vorhänge öffnen dieselben zurückgesetzten Buchten. Bögen und hängende Abschnitte behalten ihre eigenen Windgelenke. Die Zahl der Zwischenquerschnitte nimmt mit dem Maßstab ab.')
        edits.append((i,end,json.dumps(scale,ensure_ascii=False,separators=(',',':'))))
    new=raw
    for i,j,s in reversed(edits):new=new[:i]+s+new[j:]
    if path==p:
        for key in re.finditer(r'"id"\s*:\s*"limb-bark"',new):
            i=new.rindex('{',0,key.start());r,end=d.raw_decode(new,i)
            if 'areaM2' not in r:continue
            r['areaM2']=29.35
            new=new[:i]+json.dumps(r,ensure_ascii=False,separators=(',',':'))+new[end:]
            break
        else:raise ValueError('Missing limb area')
    if raw!=path.read_text():raise RuntimeError(path)
    path.write_text(new)

