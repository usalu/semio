"""🦴 Assigns the human skeleton's anatomical material and the two carried-load mass markers in each source volume."""
import json
import re
from pathlib import Path
from runpy import run_path
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
humans=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
bone=next(m for m in json.loads(next((humans/'tour-guide').glob('*')).read_text())['asset']['materials'] if m['id']=='bone')
for path in sorted(humans.glob('*/*')):
    slug=path.parent.name
    old=path.read_text();text=old;asset=json.loads(old)['asset']
    changed=0
    for k,v in asset['volumes'].items():
        r=sp['member_range'](text[text.index('"'+k+'": {',text.index('"volumes": {')):],'        ','structure')
        offset=text.index('"'+k+'": {',text.index('"volumes": {'))
        i,j=offset+r[0],offset+r[1]
        source=text[i:j]
        decisions={}
        for q in v['structure']:
            if q.get('material') not in ('skin','skin-child'):continue
            if q['id']=='mass-pack':material='nylon-pack'
            elif q['id']=='mass-box':material='cardboard'
            elif q['part'] in ('member','joint') and not q['id'].startswith('joint-grip-'):material='bone'
            else:continue
            decisions[q['id']]=material;q['material']=material
        if decisions:
            changed+=len(decisions)
            groups=re.split(r'(?=^          \{)',source,flags=re.M)
            for index,group in enumerate(groups):
                key=re.search(r'"id"\s*:\s*"([^"]+)"',group)
                if key and key[1] in decisions:
                    groups[index],n=re.subn(r'("material"\s*:\s*")skin(-child)?(")',lambda m:m[1]+decisions[key[1]]+m[3],group,count=1)
                    assert n==1
            text=text[:i]+''.join(groups)+text[j:]
    if not changed:continue
    i,j=sp['member_range'](text,'    ','materials')
    previous=text[i:j]
    if not any(m['id']=='bone' for m in asset['materials']):
        asset['materials'].append(bone)
        position=previous.rindex('}')
        fresh=previous[:position+1]+',\n      '+sp['dump'](bone,3,'compact')+previous[position+1:]
        text=text[:i]+fresh+text[j:]
    parsed=json.loads(text)['asset']
    for k,v in asset['volumes'].items():
        actual=parsed['volumes'][k]['structure']
        diffs=[(q['id'],q.get('material'),r.get('material')) for q,r in zip(v['structure'],actual) if q!=r]
        assert not diffs,(slug,k,diffs[:4])
    assert parsed==asset and path.read_text()==old
    path.write_text(text)
    snap=ticket/'volumes'/'humans'/(slug+'.json')
    source=snap.read_text();draft=json.loads(source)
    for k,v in asset['volumes'].items():
        byid={q['id']:q for q in v['structure']}
        for q in draft['volumes'][k]['structure']:
            current=byid.get(q['id'],{})
            if q.get('material') in ('skin','skin-child') and current.get('material') in ('bone','cardboard','nylon-pack'):q['material']=current['material']
    assert snap.read_text()==source
    snap.write_text(json.dumps(draft,ensure_ascii=False,separators=(',',':'))+'\n')
    print(slug,changed)
