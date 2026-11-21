"""🐈 Publish the complete feline study after checking its ocular dimensions and sitting trajectory."""
from pathlib import Path
from hashlib import sha256
from runpy import run_path
import json,os,sys,subprocess
t=Path(__file__).resolve().parent;g=t/'🗑️generated';name='domestic-cat';p=g/('cat-'+name+'.json');bun=os.environ['BUN_EXE']
h=sha256(p.read_bytes()).hexdigest();sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](t.parents[6],t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
for part in ('document','50','100','200','500','glb'):
 q=json.loads((g/f'cat-{name}-{part}.json').read_bytes());assert q['hash']==h and q['signature']==sig and q['pass'] and not q['findings']
subprocess.run([sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/'digital-cat-anatomy-test.ts'),str(p)],check=True)
subprocess.run([sys.executable,str(t/'digital-rig-install.py'),name,bun,'digital-cat-eye-tail.ts',p.name],check=True)
