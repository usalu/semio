"""🧒 Publish a tested child rig after validating the posed respiratory anchors in the final candidate."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,subprocess,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6]
n,author=sys.argv[1:];candidate=g/('passenger-'+n+'.json');sha=sha256(candidate.read_bytes()).hexdigest()
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
for part in ('document','50','100','200','500','glb'):
 q=json.loads((g/f'passenger-{n}-{part}.json').read_bytes());assert q['hash']==sha and q['signature']==sig and q['pass'] and q['complete'] and not q['findings']
bun=os.environ['BUN_EXE']
subprocess.run([sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/'digital-passenger-joints.ts'),str(candidate)],check=True)
subprocess.run([sys.executable,str(t/'digital-rig-install.py'),n,bun,author,candidate.name],check=True)
