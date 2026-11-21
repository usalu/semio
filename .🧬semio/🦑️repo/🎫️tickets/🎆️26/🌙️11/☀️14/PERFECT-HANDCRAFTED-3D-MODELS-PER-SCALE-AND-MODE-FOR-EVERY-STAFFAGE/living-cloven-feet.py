"""🐾 Keep two separate weight-bearing claws, the coronary band and native distal articulation for each ungulate."""
from copy import deepcopy
import json
import sys
from pathlib import Path
t=Path(__file__).parent
def reduced(part,stations):
    q=deepcopy(part)
    if q['kind']=='loft':
        sections=q['sections'];q['sections']=[sections[i] for i in sorted(set(round((len(sections)-1)*x) for x in stations))]
    return q
layouts={
    'dairy-cow':{'100':((0,.14,.31,.48,.69,.85,1),(0,.18,.45,.74,1)),'200':((0,.18,.39,.72,1),(0,.28,.72,1)),'500':((0,.28,.64,1),(0,.56,1))},
    'goat':{'100':((0,.35,.67,1),(0,.2,.55,.8,1)),'200':((0,.66,1),(0,.28,.75,1)),'500':((0,.34,1),(0,.49,1))},
    'sheep':{'100':((0,.2,.58,.8,1),(0,.5,1)),'200':((0,.39,.82,1),(0,.5,1)),'500':((0,.59,1),(0,.5,1))},
    'pig':{'100':((0,.18,.38,.78,1),(0,.5,1)),'200':((0,.2,.61,1),(0,.5,1)),'500':((0,.39,1),(0,.5,1))},
    'roe-deer':{'100':((0,.21,.61,.79,1),(0,.17,.56,.84,1)),'200':((0,.42,.78,1),(0,.28,.73,1)),'500':((0,.61,1),(0,.43,1))}
}
for slug in sys.argv[1:] or layouts:
    cat=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog/*animals/'+slug+'/*.json'))
    for p in [cat,t/'volumes/animals'/f'{slug}.json']:
        raw=p.read_text();v=raw.index('{',raw.index('"volumes":'));d=json.JSONDecoder()
        records,_=d.raw_decode(raw,v);fine=records['50']
        parts={s['id']:s for s in fine['body']};heat={s['of']:s['region'] for s in fine['thermal'] if 'of'in s}
        edits=[]
        for k in ('100','200','500'):
            pos=raw.index('{',raw.index('"'+k+'"',v));scale,end=d.raw_decode(raw,pos)
            claws,pasterns=layouts[slug][k]
            old=set();new={};outlets=set()
            for side,letter in [('near','n'),('far','f')]:
                for limb,initial in [('hind','h'),('fore','f')]:
                    prefix=side+'-'+limb;short=initial+letter
                    united='hoof-'+short if slug=='goat' else prefix+'-hoof'
                    old.add(united);outlets.add('th-'+united)
                    if slug=='pig':outlets.add('th-'+prefix+'-claws')
                    leg='pastern-'+short if slug=='goat' else prefix+'-pastern'
                    toes=['claw-'+short+'-o','claw-'+short+'-i'] if slug=='goat' else [prefix+'-claw-1',prefix+'-claw-2']
                    if slug=='dairy-cow':toes=[prefix+'-claw-inner',prefix+'-claw-outer']
                    new[leg]=reduced(parts[leg],pasterns)
                    for toe in toes:new[toe]=reduced(parts[toe],claws)
                    if slug=='dairy-cow' and k=='500':
                        cannon=reduced(parts[prefix+'-cannon'],(0,.21,.68,1));cannon['id']=prefix+'-lower'
                        new[cannon['id']]=cannon;heat[cannon['id']]=heat[prefix+'-cannon']
            scale['body']=[s for s in scale['body'] if s['id'] not in old|new.keys()]+list(new.values())
            scale['thermal']=[s for s in scale['thermal'] if s.get('of') not in old|new.keys() and s['id'] not in outlets|{'th-'+id for id in new}]
            scale['thermal'] += [dict(id='th-'+id,of=id,region=heat[id]) for id in new if id in heat]
            text=dict(en='The four sloping pasterns carry eight distinct weight-bearing claws. The medial and lateral keratin walls retain their own widths and sole planes. Each pair moves about its own fetlock.',de='Die vier schrägen Fesseln tragen acht getrennte tragende Klauen. Innere und äußere Hornwände behalten ihre eigenen Breiten und Sohlenebenen. Jedes Klauenpaar bewegt sich um sein eigenes Fesselgelenk.')
            if text not in scale['decisions']:scale['decisions'].append(text)
            edits.append((pos,end,json.dumps(scale,ensure_ascii=False,separators=(',',':'))))
        revised=raw
        for i,j,s in reversed(edits):revised=revised[:i]+s+revised[j:]
        if raw!=p.read_text():raise RuntimeError(p)
        p.write_text(revised)

