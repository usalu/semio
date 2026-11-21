"""🩻 Retain vertebral taper, paired rib stations, pelvic wings and distal native phalanges on each animal skeleton."""
from copy import deepcopy
import json
from pathlib import Path
t=Path(__file__).parent
species={'goat':(), 'sheep':(2,7,11), 'pig':(3,7,13), 'roe-deer':(2,6,12), 'labrador-dog':(3,8),'horse':(),'dairy-cow':()}
for slug,ribs in species.items():
    cat=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/'+slug+'/*.json'))
    for p in (cat,t/'volumes/animals'/f'{slug}.json'):
        raw=p.read_text();d=json.JSONDecoder();v=raw.index('{',raw.index('"volumes":'));rec,_=d.raw_decode(raw,v)
        i=raw.index('{',raw.index('"500"',v));c,end=d.raw_decode(raw,i);f={s['id']:s for s in rec['50']['structure']};shells={s['id']:s for s in c['structure']}
        def insert(name,source,stations):
            if source not in f:return
            q=deepcopy(f[source]);q['id']=name
            if q.get('kind')=='loft':
                sec=q['sections'];q['sections']=[sec[i] for i in sorted(set(round((len(sec)-1)*u) for u in stations))]
            shells[name]=q
        for name,s in list(shells.items()):
            b=s.get('bone','')
            if s.get('part')=='member' and b in ('neck','neck-1','neck-2','neck-3','head','jaw','thorax','lumbar','tail-1','tail-2','tail-3','tail-4') and not name.startswith(('rib','near-rib','far-rib')):
                insert(name,name,(0,.23,.54,.8,1) if b in ('head','jaw') else (0,.35,.68,1))
        if slug=='goat':
            shells.pop('pelvis-girdle',None)
            for source in ('sacrum','hip-bone-n','hip-bone-f'):insert(source,source,(0,.32,.67,1))
            for short in ('hn','hf','fn','ff'):
                shells.pop('phalanges-'+short,None)
                for s in ('o','i'):
                    for name in ('phalanx','coffin'):
                        source=name+'-'+short+'-'+s;insert(source,source,(0,.5,1))
        if slug in ('sheep','pig','roe-deer'):
            shells.pop('pelvis-bone',None);shells.pop('pelvic-girdle',None)
            for source in ('sacrum','near-ilium','far-ilium'):insert(source,source,(0,.3,.65,1))
            for side in ('near','far'):
                for index in ribs:
                    source=side+'-rib-'+str(index);insert(source,source,(0,.25,.55,.77,1))
                for endName in ('hind','fore'):
                    for group in ('digit','digits'):shells.pop(side+'-'+endName+'-'+group,None)
                    for toe in (1,2):
                        source=side+'-'+endName+'-digit-'+str(toe);insert(source,source,(0,.5,1))
        if slug=='labrador-dog':
            shells.pop('rib-girth',None)
            for index in ribs:insert('rib-'+str(index),'rib-'+str(index),(0,.5,1))
        c['structure']=list(shells.values())
        text=dict(en='The native skull and jaw, spinal shafts, selected rib stations and distal limb bones retain their attachment frames and real widths. Finer models resolve additional shafts, joints and surfaces.',de='Schädel, Kiefer, Wirbelsäulenabschnitte, ausgewählte Rippen und distale Gliedmaßenknochen behalten ihre Ansatzrahmen und tatsächlichen Breiten. Die feineren Modelle unterscheiden weitere Knochen, Gelenke und Oberflächen.')
        old=next((n for n,s in enumerate(c['decisions']) if s['en'].lower().startswith('structure')),None)
        if old is not None:c['decisions'][old]=text
        elif text not in c['decisions']:c['decisions'].append(text)
        new=raw[:i]+json.dumps(c,ensure_ascii=False,separators=(',',':'))+raw[end:]
        if p.read_text()!=raw:raise RuntimeError(p)
        p.write_text(new)

