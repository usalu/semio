"""🦿 Ends the athlete's skin at the shorts, socket liner and ankle sock, and opens the worker's hard hat at its suspension."""
from pathlib import Path
from runpy import run_path
import json
ticket=Path(__file__).resolve().parent
sp=run_path(str(ticket/'r3-kneeling-gardener-splice.py'))
for slug in ('worker-carrying-box','runner-with-blade-prosthesis'):
    p=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans/'+slug+'/*'))
    old=p.read_text();text=old;start=old.index('"volumes": ')
    for scale,v in json.loads(old)['asset']['volumes'].items():
        by={q['id']:q for q in v['body']}
        for q in v['thermal']:
            if slug=='worker-carrying-box':
                if q['id'] not in ('th-helmet','th-helmet-rim'):continue
                back=by.get(q.get('of',q['id'][3:]),q)
                if back['kind']!='loft':continue
                bottom=back['sections'][0]['at'][0]<back['sections'][-1]['at'][0]
                r={**back,**q,'sectionFaces':(['-u','+u'] if q['id']=='th-helmet-rim' else ['-u' if bottom else '+u'])}
                r.pop('of',None)
            else:
                if q['region']!='leg-skin' or q['id'] not in ('th-thigh-l','th-thigh-r','th-shank-l'):continue
                back=by.get(q.get('of',q['id'][3:]),q)
                if back.get('kind')!='loft':continue
                r={**back,**q};r.pop('of',None)
                hip=q['id']!='th-shank-l'
                lo=(by['shorts-leg-l']['sections'][-1]['at'][0] if hip and 'shorts-leg-l' in by else .125) if hip else 0
                hi=(.435 if q['id']=='th-thigh-l' else by['liner-thigh']['sections'][0]['at'][0] if 'liner-thigh' in by else .34) if hip else (by['sock-l']['sections'][0]['at'][0] if 'sock-l'in by else .38)
                sections=r['sections']
                def cut(x):
                    for a,b in zip(sections,sections[1:]):
                        if a['at'][0]<=x<=b['at'][0]:
                            u=(x-a['at'][0])/(b['at'][0]-a['at'][0])
                            def interp(v,w):return round(v+u*(w-v),6)
                            return {**a,'at':[interp(v,w) for v,w in zip(a['at'],b['at'])],'rx':interp(a['rx'],b['rx']),'ry':interp(a['ry'],b['ry'])}
                    return min(sections,key=lambda q:abs(q['at'][0]-x))
                r['sections']=[cut(lo),*[s for s in sections if lo+1e-6<s['at'][0]<hi-1e-6],cut(hi)]
                r['sectionFaces']=['-u','+u']
            i=text.index('{\n            "id": "'+q['id']+'"',start)
            j=sp['member_end'](text,i)
            term=sp['dump'](r,5,'compact');text=text[:i]+term+text[j:];start=i+len(term)
    assert json.loads(text) and p.read_text()==old
    p.write_text(text)
