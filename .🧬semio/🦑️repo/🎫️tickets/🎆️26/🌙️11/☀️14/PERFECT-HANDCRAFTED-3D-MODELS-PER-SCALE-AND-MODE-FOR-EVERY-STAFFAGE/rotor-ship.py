"""🧩 Splices only individually checked scales of the digital assets and their paper views."""
import json,sys
from runpy import run_path
from pathlib import Path
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'))
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/🔌️appliances'))
for slug in sys.argv[1:]:
    p=next((root/slug).iterdir());raw=p.read_text();text=raw;a=json.loads(raw)['asset']
    candidate=json.loads((t/'🗑️generated'/'rotor'/(slug+'.json')).read_bytes())['asset']
    for top in ['representations','volumes']:
        for scale in ['50','100','200','500']:
            if a[top][scale]==candidate[top][scale]:continue
            base=text.index('"'+top+'": {');start=text.index('{',text.index('"'+scale+'": {',base));end=sp['member_end'](text,start)
            text=text[:start]+sp['dump'](candidate[top][scale],3,'compact')+text[end:]
            a[top][scale]=candidate[top][scale]
    if slug=='circulation-pump' and a['materials']!=candidate['materials']:
        base=text.index('"materials": [');start=text.index('[',base);end=sp['member_end'](text,start)
        assert candidate['materials'][:len(a['materials'])]==a['materials']
        a['materials']=candidate['materials'];text=text[:start]+sp['dump'](a['materials'],2,'compact')+text[end:]
    if slug=='duct-fan' and a['clips']!=candidate['clips']:
        index=next(i for i,c in enumerate(a['clips']) if c['id']=='speed-steps')
        base=text.index('"id": "speed-steps"',text.index('"clips": ['))
        start=text.index('[',text.index('"keys": [',base));end=sp['member_end'](text,start)
        new=candidate['clips'][index]['keys'];text=text[:start]+sp['dump'](new,4,'compact')+text[end:];a['clips'][index]['keys']=new
        assert a['clips']==candidate['clips']
    assert json.loads(text)['asset']==a and p.read_text()==raw
    if text!=raw:p.write_text(text)

    source=t/'volumes'/'appliances'/(slug+'.json');raw=source.read_text();src=json.loads(raw)
    live=json.loads(p.read_bytes())['asset']['volumes']
    for scale in ['50','100','200','500']:
        assert live[scale]==candidate['volumes'][scale]
    if src['volumes']!=live:
        src['volumes']=live
        assert source.read_text()==raw
        source.write_text(json.dumps(src,ensure_ascii=False,indent=2)+'\n')
    assert json.loads(source.read_bytes())['volumes']==json.loads(p.read_bytes())['asset']['volumes']
