"""📐 Independently reopen the reviewed GLB scenes and volume authoring, then publish the measured models."""
import hashlib
import json
from pathlib import Path
from runpy import run_path
import shutil, subprocess, sys
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
source=run_path(str(T/'digital-heater-bench.py'))
publish=run_path(str(T/'digital-publish-asset.py'))['publish']

def check(label,cmd):
    with (G/(label+'.log')).open('w') as out:
        completed=subprocess.run([shutil.which('bun'),'--smol',*cmd],stdout=out,stderr=out)
    result=(G/(label+'.log')).read_text()
    print('[DEBUG]',label,completed.returncode,result[-1800:],flush=True)
    assert completed.returncode==0
    return result

for name,author in [('electric-fan-heater',source['heater']),('park-bench',source['bench'])]:
    cat=source['catalog'](name)
    candidate=G/('digital-art-'+name+'.json')
    assert 'schema 0, validateAsset 0, checkVolumes 0' in check(name+'-art-before-publish',[T/'r3-validate.ts','--file',candidate,'--volumes'])
    publish(author,cat,candidate)
    raw=cat.read_bytes()
    doc=json.loads(raw)
    assert doc==json.loads(candidate.read_bytes())
    assert doc['asset']['volumes']==json.loads((T/'volumes'/doc['asset']['category']/(name+'.json')).read_bytes())['volumes']
    scenes=check(name+'-art-glb',[T/'digital-scenes.ts','--file',cat])
    var=json.loads(next(s for s in scenes.splitlines() if s.startswith('{')))
    assert var['scenes']==12 and var['findings']==[]
    for pos in [[],['--opposite','--presentation','--shaded']]:
        check(name+'-art-sheet'+('-reverse' if pos else ''),[T/'living-visual.ts',name,'all',doc['asset']['presentation']['defaultPose'],cat,'--temperature',*pos])
    assert 'schema 0, validateAsset 0, checkVolumes 0' in check(name+'-art-canonical',[T/'r3-validate.ts','--file',cat,'--volumes'])
    assert cat.read_bytes()==raw
    print('[DEBUG]',name,hashlib.sha256(raw).hexdigest(),flush=True)
