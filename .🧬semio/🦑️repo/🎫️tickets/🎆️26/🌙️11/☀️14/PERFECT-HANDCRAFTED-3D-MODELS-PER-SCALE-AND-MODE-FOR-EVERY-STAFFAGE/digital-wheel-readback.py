"""🛞 Read, measure and reopen the published equipment geometry one process at a time."""
import hashlib
import json
from pathlib import Path
from runpy import run_path
import os
import shutil
import subprocess
import sys

t=Path(__file__).resolve().parent
g=t/'🗑️generated'
assets=run_path(str(t/'digital-rolling-contacts.py'))
for name in sys.argv[1:]:
    catalog=assets['catalog'](name)
    raw=catalog.read_bytes()
    doc=json.loads(raw)
    study = 'digital-access-' if name=='street-lamp' else 'digital-roll-'
    assert doc==json.loads((g/(study+name+'.json')).read_bytes())
    assert doc['asset']['volumes']==json.loads((t/'volumes'/doc['asset']['category']/(name+'.json')).read_bytes())['volumes']
    for kind,cmd in [
        ('scenes',[t/'digital-scenes.ts','--file',catalog]),
        ('render',[t/'living-visual.ts',name,'all',doc['asset']['presentation']['defaultPose'],catalog,'--temperature']),
        ('strict',[t/'r3-validate.ts','--file',catalog,'--volumes'])]:
        with (g/f'{name}-rolling-canonical-{kind}.log').open('w') as log:
            result=subprocess.run([sys.executable,t/'equipment-guard.py',shutil.which('bun'),'--smol',*cmd],
                env={**os.environ,'EQUIPMENT_RSS_MIB':'1140' if kind=='strict' and name=='street-lamp' else ('1000' if kind=='strict' else '680'),'EQUIPMENT_HEADROOM_PERCENT':'35'},stdout=log,stderr=log)
        output=(g/f'{name}-rolling-canonical-{kind}.log').read_text()
        print('[DEBUG]',name,kind,output[-900:],flush=True)
        assert result.returncode==0 and catalog.read_bytes()==raw
        if kind=='scenes':
            row=json.loads(next(l for l in output.splitlines() if l.startswith('{')))
            assert row['scenes']==12 and row['findings']==[]
        if kind=='strict':
            assert 'schema 0, validateAsset 0, checkVolumes 0' in output
    if name=='street-lamp':
        for target,aim,span,turn,tilt in [('catch','.89,5.85,0','.15','12','30'),('lock','0,1.11,-.081','.12','0','0')]:
            cmd=[shutil.which('bun'),'--smol',t/'r3-street-lamp-zoom.ts','--scale','50','--list','body','--width','640','--at',aim,'--span',span,'--turn',turn,'--tilt',tilt,'--name','access-'+target]
            subprocess.run([sys.executable,t/'equipment-guard.py',*cmd],env={**os.environ,'EQUIPMENT_RSS_MIB':'650','EQUIPMENT_HEADROOM_PERCENT':'35'},check=True)
    print('[DEBUG]',name,hashlib.sha256(raw).hexdigest(),flush=True)
