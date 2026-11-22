"""🗂️ Show each candidate and published component hash and last verification finding."""
import hashlib,json
from pathlib import Path
from runpy import run_path
T=Path(__file__).resolve().parent;G=T/'🗑️generated';catalog=run_path(str(T/'digital-building-equipment.py'))['catalog']
for n in ('solar-thermal-collector','glass-balustrade','sliding-door','solid-brick-wall-1960','tied-canopy','sectional-garage-door','masonry-wall-with-lintel','curtain-wall-bay','tilt-window'):
    source=catalog(n);candidate=G/('building-'+n+'.json')
    same=json.loads(source.read_bytes())==json.loads(candidate.read_bytes())
    rows=[]
    for kind in ('before-publish','staged-glb','canonical'):
        f=G/(n+'-art-'+kind+'.log')
        if f.exists():
            lines=[l for l in f.read_text().splitlines() if l.startswith(('VOLUMES','ASSET','INVALID','VALID','{'))]
            if lines:rows.append((kind,lines[-1][:95]))
    print(n,hashlib.sha256(source.read_bytes()).hexdigest()[:10],same,rows)
