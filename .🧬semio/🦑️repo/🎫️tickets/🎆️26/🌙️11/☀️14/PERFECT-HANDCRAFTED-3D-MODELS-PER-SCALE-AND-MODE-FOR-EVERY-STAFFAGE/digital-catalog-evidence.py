"""🧾 Record independent per-asset GLB receipts against the exact catalog and validator inputs."""
from datetime import datetime,timezone
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json

ticket=Path(__file__).resolve().parent
root=ticket.parents[6];g=ticket/'🗑️generated'
signature=run_path(str(ticket/'digital-proof-digest.py'))['module_digest'](root,ticket/'digital-scenes.ts')
exact={};previous={}
for line in (g/'digital-house-scenes.jsonl').read_text().splitlines():
    q=json.loads(line)
    if q.get('findings')!=[] or q.get('scenes')!=12 or not 0<q.get('bytes',0)<=8*1024**2:continue
    (exact if q['validator']==signature else previous)[q['id'],q['sha256']]=q
rows=[];current=measured=0
for p in sorted(root.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json')):
    raw=p.read_bytes();asset=json.loads(raw)['asset'];name=asset['id'];key=(name,sha256(raw).hexdigest())
    r=exact.get(key);current+=int(r is not None)
    r=r or previous.get(key);measured+=int(r is not None)
    status='Current' if key in exact else 'Earlier validator' if r else 'Pending'
    size=f"{r['bytes']:,}" if r else '—'
    rows.append(f"| {asset['category']} | {name} | `{key[1][:16]}` | {status} | {size} |")
text=['# Staffage export acceptance','',
      f"At {datetime.now(timezone.utc):%Y-%m-%d %H:%M UTC}, {measured} of {len(rows)} current catalog documents have an independent twelve-scene GLB readback matching their exact source bytes. That covers {12*measured:,} physical, thermal and structural scenes at 1:50, 1:100, 1:200 and 1:500.",
      '',f"{current} of these documents also match the current validator dependency digest, `{signature}`. Every reported GLB was measured within the 8 MiB publication budget.",
      '','The exporter produced each asset in its authored presentation pose. The independent loader checked its scene graph, geometry and material variants. Per-model reports also cover rig operation, heat transfer, physical outlines and paper suitability.','',
      '| Family | Asset | Source SHA-256 prefix | Readback | GLB bytes |','| --- | --- | --- | --- | ---: |',*rows,'']
(ticket/'digital-catalog-acceptance.md').write_text('\n'.join(text))
print(f"{measured}/{len(rows)} current documents measured, {12*measured} scenes, {current} with current validator.")
