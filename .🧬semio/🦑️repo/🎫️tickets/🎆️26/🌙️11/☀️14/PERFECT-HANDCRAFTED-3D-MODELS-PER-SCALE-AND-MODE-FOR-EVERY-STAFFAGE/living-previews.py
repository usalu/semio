"""🌿 Build each twelve-panel botanical or anatomical sheet with a separately guarded child."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

ticket=Path(__file__).parent
root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'))
out=ticket/'🗑️generated/living-current-views'
out.mkdir(exist_ok=True)
files=[f for g in ('*animals','*vegetation') for f in root.glob(g+'/*/*.json')]
names=sys.argv[2:]
for path in sorted(files,key=lambda p:p.stat().st_size):
    id=path.parent.name
    if names and id not in names:continue
    sha=hashlib.sha256(path.read_bytes()).hexdigest()
    if list(out.glob(id+'-*-all-'+sha[:10]+'.png')):continue
    cmd=[sys.executable,str(ticket/'equipment-guard.py'),sys.argv[1],'--smol',str(ticket/'living-visual.ts'),id,'all']
    run=subprocess.run(cmd,env={**os.environ,'EQUIPMENT_RSS_MIB':os.environ.get('EQUIPMENT_RSS_MIB','195'),'EQUIPMENT_HEADROOM_PERCENT':'35'})
    print('[DEBUG]',id,sha[:10],run.returncode,flush=True)

