"""🧑 Publish a verified subject from its current authored inputs after independently rebuilding its candidate."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,sys,subprocess
t=Path(__file__).resolve().parent
slug,bun,script,candidate=sys.argv[1:];g=t/'🗑️generated'
cat=next(t.parents[6].glob('🎓*/*architecture/*staffage/*catalog/*/'+slug+'/*.json'))
previous=cat.read_bytes();draft=g/('validated-'+slug+'.json');checked=json.loads((g/candidate).read_bytes())['asset']
small={'digital-cycle-rims.ts','digital-industrial-wheels.ts','digital-vehicle-joints.ts','digital-transit-windows.ts','digital-chalkboard-ruler.ts','digital-occupied-wheels.ts','digital-stroller-wheels.ts','digital-guitar-grip.ts','digital-schoolchild-bones.ts','digital-limb-gauge.ts'}
env={**os.environ,'EQUIPMENT_HEADROOM_PERCENT':'48','EQUIPMENT_RSS_MIB':'320' if script in small else '820'}
subprocess.run([sys.executable,str(t/'equipment-guard.py'),bun,'--smol',str(t/script),str(cat),str(draft)],check=True,env=env)
assert json.loads(draft.read_bytes())['asset']==checked and cat.read_bytes()==previous
def author(asset):
    assert json.loads(previous)['asset']==asset
    for k in ('rig','anchors','poses','clips','volumes','representations','structure','thermal','materials','dimensions','symbols'):asset[k]=checked[k]
    assert asset==checked
run_path(str(t/'digital-publish-asset.py'))['publish'](author,cat,draft)
print(sha256(cat.read_bytes()).hexdigest())
