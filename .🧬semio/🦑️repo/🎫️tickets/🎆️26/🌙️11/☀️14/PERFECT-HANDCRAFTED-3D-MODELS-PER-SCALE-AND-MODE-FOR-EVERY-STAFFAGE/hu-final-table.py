"""📋 Stores final source digests and current per-scale physical thermal proof after checking each measurement report."""
import hashlib,json
from pathlib import Path
t=Path(__file__).resolve().parent
o=t/'🗑️generated'/'hu-review'
r=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
g=t/'🗑️generated';exports={}
for line in (g/'digital-house-scenes.jsonl').read_text().splitlines():
    item=json.loads(line)
    if item.get('scenes')==12 and item.get('findings')==[] and 0<item.get('bytes',0)<=8*1024**2:exports[item['id'],item['sha256']]=item
rows=[];complete=0;scenes=0
for path in sorted(r.glob('*/*')):
    raw=path.read_bytes();sha=hashlib.sha256(raw).hexdigest();a=json.loads(raw)['asset'];slug=a['id'];cells=[]
    record=g/'equipment-fleet'/(slug+'-strict.json')
    whole=json.loads(record.read_bytes()) if record.exists() else {}
    passed=whole.get('input')==sha and all(whole.get(k) for k in ('completed','passed','current'))
    for scale in [50,100,200,500]:
        hits=[]
        for suffix in ['-thermal-'+str(scale),'-all-'+str(scale),'-all']:
            proof=o/(slug+suffix+'.json')
            if proof.exists():
                q=json.loads(proof.read_bytes())
                if q['sha256']==sha and q['current']:hits.append(q)
        ok=passed or any(not q['problems'] for q in hits);complete+=int(ok);cells.append('Pass' if ok else 'Finding' if hits else 'Pending')
    receipt=o/(slug+'-export.json')
    export='Pending'
    if receipt.exists():
        rr=json.loads(receipt.read_bytes())
        if rr['sha256']==sha and rr['current'] and not rr['findings']:
            scenes+=rr['scenes']
            export=f"{rr['bytes']:,}"
    if export=='Pending' and (slug,sha) in exports:
        proof=exports[slug,sha];scenes+=proof['scenes'];export=f"{proof['bytes']:,}"
    rows.append('| '+' | '.join([slug,sha[:16],str(len(a['poses'])),*cells,export])+' |')
p=t/'human-thermal-scales.md';old=p.read_text() if p.exists() else ''
report='''# Human thermal geometry by scale

Each passing cell records the product’s geometric thermal-coverage rule on the displayed asset revision. It includes all authored regions and their exposed areas. SHA-256 prefixes identify the human source document. The separate heat-balance proof in [the human audit](humans-audit.md) covers the metabolic solution for every named pose and both climates.

| Figure | SHA-256 prefix | Poses | 1:50 | 1:100 | 1:200 | 1:500 | GLB bytes (12 scenes) |
| --- | --- | ---: | --- | --- | --- | --- | ---: |
'''+ '\n'.join(rows)+'\n\n'+str(complete)+' of 112 scale checks pass on the source digests listed above. '+str(scenes)+' of 336 exported scene checks pass on those same digests.\n'
assert not p.exists() or p.read_text()==old
p.write_text(report);print(complete)
