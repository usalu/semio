"""📐 Independently reopen the reviewed GLB scenes and volume authoring, then publish the measured models."""
import hashlib
import json
from pathlib import Path
from runpy import run_path
import shutil, subprocess, sys
T=Path(__file__).resolve().parent
G=T/'🗑️generated'
source=run_path(str(T/'digital-building-equipment.py'))
publish=run_path(str(T/'digital-publish-asset.py'))['publish']

def check(label,cmd):
    with (G/(label+'.log')).open('w') as out:
        completed=subprocess.run([shutil.which('bun'),'--smol',*cmd],stdout=out,stderr=out)
    result=(G/(label+'.log')).read_text()
    print('[DEBUG]',label,completed.returncode,result[-1800:],flush=True)
    assert completed.returncode==0
    return result

def strict(name,label,path):
    if name not in ('diesel-generator','tied-canopy','masonry-wall-with-lintel','sectional-garage-door','curtain-wall-bay'):
        return check(label,[T/'r3-validate.ts','--file',path,'--compact','--volumes'])
    with (G/(label+'.log')).open('w') as out:
        status=subprocess.run([sys.executable,T/'digital-building-strict.py',path],stdout=out,stderr=out).returncode
    result=(G/(label+'.log')).read_text()
    print('[DEBUG]',label,status,result[-1500:],flush=True)
    assert status==0
    return result

for name in sys.argv[1:]:
    author=source['AUTHORS'][name]
    cat=source['catalog'](name)
    candidate=G/('building-'+name+'.json')
    assert 'schema 0, validateAsset 0, checkVolumes 0' in strict(name,name+'-art-before-publish',candidate)
    if name in ('gas-boiler','expansion-valve','finned-heat-exchanger','heat-pump-outdoor-unit','solar-thermal-collector','masonry-wall-with-lintel','glass-balustrade','sliding-door','solid-brick-wall-1960','sectional-garage-door','tied-canopy','curtain-wall-bay','tilt-window'):
        staged=check(name+'-art-staged-glb',[T/'digital-scenes.ts','--file',candidate]);result=json.loads(next(line for line in staged.splitlines() if line.startswith('{')))
        assert result.get('scenes')==12 and result.get('findings')==[],result
    publish(author,cat,candidate)
    raw=cat.read_bytes()
    doc=json.loads(raw)
    assert doc==json.loads(candidate.read_bytes())
    assert doc['asset']['volumes']==json.loads((T/'volumes'/doc['asset']['category']/(name+'.json')).read_bytes())['volumes']
    scenes=check(name+'-art-glb',[T/'digital-scenes.ts','--file',cat])
    var=json.loads(next(s for s in scenes.splitlines() if s.startswith('{')))
    assert var['scenes']==12 and var['findings']==[]
    for pos in [[],['--opposite','--presentation','--shaded','--short-name']]:
        check(name+'-art-sheet'+('-reverse' if pos else ''),[T/'living-visual.ts',name,'all',doc['asset']['presentation']['defaultPose'],cat,'--temperature',*pos])
    if name=='diesel-generator':
        check(name+'-art-running',[T/'living-visual.ts',name,'all','rated-load',cat,'--temperature','--opposite','--presentation','--shaded','--short-name'])
    moving={'sliding-door':'half-open','sectional-garage-door':'half-open','tilt-window':'fully-tilted'}
    if name in moving:
        check(name+'-art-moving',[T/'living-visual.ts',name,'all',moving[name],cat,'--temperature','--opposite','--presentation','--shaded','--short-name'])
    if name=='scroll-compressor':
        check(name+'-art-underside',[T/'living-visual.ts',name,'all',doc['asset']['presentation']['defaultPose'],cat,'--temperature','--underside','--presentation','--shaded','--short-name'])
    assert 'schema 0, validateAsset 0, checkVolumes 0' in strict(name,name+'-art-canonical',cat)
    assert cat.read_bytes()==raw
    print('[DEBUG]',name,hashlib.sha256(raw).hexdigest(),flush=True)
