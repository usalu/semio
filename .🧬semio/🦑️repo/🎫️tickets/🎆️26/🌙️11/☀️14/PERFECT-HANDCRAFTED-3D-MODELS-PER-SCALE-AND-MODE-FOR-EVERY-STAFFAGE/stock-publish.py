"""🪡 Publishes the checked grill and cylinder roots and their four matching authoring scales."""
from pathlib import Path
from runpy import run_path
import json,re,sys
t=Path(__file__).resolve().parent;sp=run_path(str(t/'r3-kneeling-gardener-splice.py'));r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog'))
for id in sys.argv[1:]:
    src=next(r.glob('*/'+id+'/*'));raw=src.read_text();text=raw;old=json.loads(raw);asset=old['asset'];data=json.loads((t/'🗑️generated'/'rotor'/(id+'.json')).read_bytes())['asset'];category='objects'if id=='charcoal-grill'else'appliances'
    for k in ['poses','thermal','representations','volumes']:
        if asset[k]==data[k]:continue
        if k in ('poses','thermal'):
            m=re.search(r'^    "'+k+r'":\s*([\[{])',text,re.M);start=m.end()-1;end=sp['member_end'](text,start)
            text=text[:start]+sp['dump'](data[k],2,'compact')+text[end:]
        else:
            for d in ['50','100','200','500']:
                if asset[k][d]==data[k][d]:continue
                base=text.index('"'+k+'": {');start=text.index('{',text.index('"'+d+'": {',base));end=sp['member_end'](text,start)
                text=text[:start]+sp['dump'](data[k][d],3,'compact')+text[end:]
        asset[k]=data[k]
    assert json.loads(text)==old and src.read_text()==raw
    if raw!=text:src.write_text(text)
    author=t/'volumes'/category/(id+'.json');authorRaw=author.read_text();doc=json.loads(authorRaw);doc['volumes']=asset['volumes']
    if json.loads(authorRaw)!=doc:
        assert author.read_text()==authorRaw;author.write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n')
    assert json.loads(src.read_bytes())['asset']['volumes']==json.loads(author.read_bytes())['volumes']
