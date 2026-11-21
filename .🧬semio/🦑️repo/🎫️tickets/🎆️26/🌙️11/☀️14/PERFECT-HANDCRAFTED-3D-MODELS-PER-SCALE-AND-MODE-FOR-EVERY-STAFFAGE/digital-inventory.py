"""🧾 Compare all independently authored scale volumes with the documents actually loaded by the product."""
import json
from pathlib import Path

T=Path(__file__).resolve().parent
catalog=next(T.parents[6].glob('🎓*/*/*staffage/🗂*catalog'))
count=0;scales=0;changes=[];empty=[];orphans=[];status={}
for p in sorted(catalog.glob('*/*/🔣️.json')):
    a=json.loads(p.read_bytes())['asset'];source=T/'volumes'/a['category']/(a['id']+'.json');count+=1
    authored=json.loads(source.read_bytes())['volumes'] if source.exists() else {}
    for k in ['50','100','200','500']:
        v=a['volumes'][k];scales+=v['status']=='authored';pkey=a['category']+'/'+a['id']+'@'+k
        if authored.get(k)!=v:changes.append(pkey)
        for g in ['body','thermal','structure']:
            if not v.get(g):empty.append(pkey+'/'+g)
        ids={s['id'] for s in v.get('body',[])}
        for g in ['thermal','structure']:
            for s in v.get(g,[]):
                if 'of' in s and s['of'] not in ids:orphans.append(pkey+'/'+s['id'])
        print_status=a['representations'][k]['status'];status[print_status]=status.get(print_status,0)+1
print(json.dumps(dict(assets=count,authoredScales=scales,print=status,empty=empty,orphans=orphans,differentAuthorSources=changes),ensure_ascii=False,indent=2))
