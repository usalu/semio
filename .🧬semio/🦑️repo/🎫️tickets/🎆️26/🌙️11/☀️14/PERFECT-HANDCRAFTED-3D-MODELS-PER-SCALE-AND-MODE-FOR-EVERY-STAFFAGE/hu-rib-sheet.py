"""🩻 Draws the current costal centre lines in frontal and lateral views at the same physical scale."""
import json,re
from html import escape
from pathlib import Path
ticket=Path(__file__).resolve().parent
folder=next(ticket.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
out=ticket/'🗑️generated'/'hu-review'
for scale in ['50','500']:
    rows=[];W,H=272,157
    for i,p in enumerate(sorted(folder.glob('*/*'))):
        a=json.loads(p.read_text())['asset']
        torso=[q for q in a['volumes'][scale]['structure'] if q.get('bone')=='torso' and q.get('part')=='member']
        ribs=[q for q in torso if re.search(r'(?:costal-|rib-)',q['id']) and q.get('kind')=='loft']
        notes=[(q,'#253d47') for q in ribs]
        others=[q for q in torso if q['id'] in ['spine','sk-spine','thoracolumbar-spine','spine-bar','sternum','open-sternum','sk-sternum'] or q['id'].startswith('vertebra-t')]
        notes+= [(q,'#a04024') for q in others]
        dx=(i%4)*W;dy=(i//4)*H
        rows.append(f'<g transform="translate({dx} {dy})"><rect x="0" y="0" width="{W}" height="{H}" fill="white" stroke="#d8dddd"/><text x="7" y="17" font-family="Helvetica,sans-serif" font-size="12">{escape(p.parent.name)}</text>')
        points=lambda q: [r['at'] for r in q['sections']] if 'sections' in q else [q['p0'],q['p1']] if q['kind']=='capsule' else []
        top=max(x for q in ribs for x,y,z in points(q))
        for q,colour in notes:
            pts=points(q)
            if not pts:continue
            for center,axis in [(68,2),(192,1)]:
                field=' '.join(f'{center+P[axis]*365:.2f},{26+(top-P[0])*365:.2f}' for P in pts if P[0]>=top-.33)
                if not field:continue
                rows.append(f'<polyline points="{field}" stroke="{colour}" fill="none" stroke-width="1.15" stroke-linejoin="round"/>')
        rows.append('</g>')
    (out/('ribs-'+scale+'.svg')).write_text(f'<svg xmlns="http://www.w3.org/2000/svg" width="{W*4}" height="{H*7}">'+''.join(rows)+'</svg>')
