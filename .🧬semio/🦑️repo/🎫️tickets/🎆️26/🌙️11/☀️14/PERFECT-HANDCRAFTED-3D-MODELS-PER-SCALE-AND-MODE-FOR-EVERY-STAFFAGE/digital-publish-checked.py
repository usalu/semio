"""📦 Publish the exact checked candidate after rebuilding it against the live source, then retain its receipt."""
from pathlib import Path
import hashlib,json,os,subprocess,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6]
study,script,bun,*names=sys.argv[1:]
for name in names:
    candidate=g/f'{study}-{name}.json'
    if not candidate.exists():continue
    raw=candidate.read_bytes();h=hashlib.sha256(raw).hexdigest();source=next(r.glob('🎓*/*/*staffage/*catalog/*/'+name+'/*.json'))
    receipt=g/f'published-{study}-{name}.json'
    if receipt.exists() and json.loads(receipt.read_bytes()).get('candidate')==h:continue
    proof=[]
    for part in ['50','100','200','500','document','glb']:
        path=g/f'{study}-{name}-{part}.json'
        if path.exists():
            row=json.loads(path.read_bytes())
            if row.get('hash')==h and row.get('complete') and row.get('pass'):proof.append(part)
    if len(proof)!=6:continue
    if json.loads(raw)['asset']!=json.loads(source.read_bytes())['asset']:
        subprocess.run([sys.executable,t/'digital-rig-install.py',name,bun,script,candidate.name],check=True)
    current=source.read_bytes()
    assert json.loads(current)['asset']==json.loads(raw)['asset']
    receipt.write_text(json.dumps({'candidate':h,'source':hashlib.sha256(current).hexdigest()}))
