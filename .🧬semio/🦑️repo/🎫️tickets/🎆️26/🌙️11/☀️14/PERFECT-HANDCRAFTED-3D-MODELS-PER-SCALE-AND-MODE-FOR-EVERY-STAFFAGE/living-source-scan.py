"""🌱 Run source-level botanical and animal rules with a separate small-heap process per catalog record."""
import os
from pathlib import Path
import subprocess
import sys
ticket=Path(__file__).parent;root=next(Path.cwd().glob('*teaching/*architecture/*staffage/*catalog'));test=sys.argv[2] if len(sys.argv)>2 else 'living-source-rules.ts';log=ticket/'🗑️generated'/(test.removesuffix('.ts')+'-checked.log')
with log.open('w') as out:
    for p in sorted((*root.glob('*animals/*/*.json'),*root.glob('*vegetation/*/*.json')),key=lambda p:p.stat().st_size):
        name=p.parent.name
        rc=subprocess.run([sys.executable,ticket/'equipment-guard.py',sys.argv[1],'--smol',ticket/test,name],stdout=out,stderr=out,env={**os.environ,'EQUIPMENT_RSS_MIB':'280'}).returncode
        if rc:print('[DEBUG]',name,'deferred',rc,file=out,flush=True)

