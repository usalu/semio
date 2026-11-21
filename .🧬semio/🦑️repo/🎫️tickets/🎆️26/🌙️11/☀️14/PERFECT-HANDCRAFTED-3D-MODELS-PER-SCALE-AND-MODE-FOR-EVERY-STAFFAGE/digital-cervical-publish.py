"""🦴 Install a cervical study after its geometry, paper, export and kinematic checks all pass."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,subprocess,sys
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated'
signature=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
forms=json.loads((t/'digital-cervical-stations.json').read_bytes())
for n in sys.argv[1:] or forms:
    cat=next(r.glob('🎓*/*architecture/*staffage/*catalog/*/'+n+'/🔣️.json'))
    current=json.loads(cat.read_bytes())['asset']
    candidate=g/f'atlas-{n}.json';raw=candidate.read_bytes()
    if current==json.loads(raw)['asset']:continue
    if any(b['id']=='neck' for b in current['rig']['bones']):raise RuntimeError(n+' has a concurrent neck revision')
    checks=[g/f'neck-{n}-{part}.json' for part in ('document','50','100','200','500','glb')]
    if not all(x.exists() for x in checks):continue
    receipts=[json.loads(x.read_bytes()) for x in checks]
    if not all(q['hash']==sha256(raw).hexdigest() and q['signature']==signature and q['pass'] and q['complete'] and not q['findings'] for q in receipts):continue
    bun=os.environ['BUN_EXE']
    cmd=[sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/'digital-cervical-check.ts'),str(cat),str(candidate)]
    subprocess.run(cmd,check=True)
    subprocess.run([sys.executable,str(t/'digital-rig-install.py'),n,bun,'digital-cervical-forms.ts',candidate.name],check=True)
    assert json.loads(cat.read_bytes())['asset']==json.loads(raw)['asset']
