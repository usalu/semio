"""🧥 Verify each individual surface-area budget and install its measured garment surfaces."""
from pathlib import Path
from runpy import run_path
from hashlib import sha256
import json,os,sys,subprocess
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';bun=os.environ['BUN_EXE']
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
for n in json.loads((t/'digital-clothing-balance.json').read_bytes()):
 c=g/f'clothing-{n}.json';h=sha256(c.read_bytes()).hexdigest()
 for part in ('document','50','100','200','500','glb'):
  q=json.loads((g/f'clothing-{n}-{part}.json').read_bytes());assert q['hash']==h and q['signature']==sig and q['pass'] and q['complete'] and not q['findings']
 subprocess.run([sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/'digital-clothing-balance.ts'),str(c)],check=True)
 subprocess.run([sys.executable,str(t/'digital-rig-install.py'),n,bun,'digital-clothing-balance.ts',c.name],check=True)
