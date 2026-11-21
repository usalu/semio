"""🩻 Confirms passenger and reduced-rib attachments, bilateral courses and strut aspect in their actual local frames."""
from pathlib import Path
from math import dist
from runpy import run_path
from contextlib import redirect_stdout
import io,json
t=Path(__file__).resolve().parent
with redirect_stdout(io.StringIO()):f=run_path(str(t/'hu-spine-check.py'))
root=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
count=0
for path in root.glob('*/*'):
    asset=json.loads(path.read_text())['asset']
    for k,v in asset['volumes'].items():
        st={q['id']:q for q in v['structure']}
        for prefix in ['open-','child-','toddler-']:
            ribs=[q for q in st.values() if q['id'].startswith(prefix+('rib-' if prefix=='open-' else 'costal-'))]
            if not ribs:continue
            spine=next(q for q in st.values() if q['id'] in (['spine','spine-bar','sk-spine'] if prefix=='open-' else [prefix+'spine']))
            middle=spine.get('sections',[{'at':spine.get('p0')}])[0]['at'][2]
            sternum=st[prefix+'sternum'];maximum=-1;aspect=100;floatClear=[]
            for q in ribs:
                pts=[s['at'] for s in q['sections']];r=q['sections'][0]['rx'];n=int(q['id'].split('-')[-2])
                assert len({tuple(m) for m in pts})==len(pts),(path.parent.name,k,q['id'])
                assert all((p[2]>middle)==q['id'].endswith('-l') for p in pts),(path.parent.name,k,q['id'])
                gap=f['clearance'](pts[0],spine)-r
                if n<=7:gap=max(gap,f['clearance'](pts[-1],sternum)-r)
                if n in [8,9,10]:
                    rib=st[q['id'].replace('-'+str(n)+'-','-7-')]
                    gap=max(gap,f['clearance'](pts[-1],rib)-r)
                if n>=11:floatClear.append(abs(middle-pts[-1][2]))
                if q.get('slender'):aspect=min(aspect,sum(dist(x,y) for x,y in zip(pts,pts[1:]))/(2*r))
                maximum=max(maximum,gap)
            print(path.parent.name,k,prefix,'maximum gap mm',round(maximum*1000,2),'minimum slenderness',round(aspect,2))
            assert maximum<.0004 and aspect>=20 and (not floatClear or min(floatClear)>.022)
            count+=1
print(count,'scale/subject checks')
