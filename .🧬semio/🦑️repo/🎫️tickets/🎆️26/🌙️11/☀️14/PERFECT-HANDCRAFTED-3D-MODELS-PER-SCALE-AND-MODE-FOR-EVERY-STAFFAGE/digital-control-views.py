"""🧍 Render the tested canonical anatomy and running gear in the requested pose and three presentation modes."""
from pathlib import Path
import sys,subprocess,os,json,hashlib
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6];family=sys.argv[1]
groups={'people':{'elderly-with-cane':'cane-planted','person-on-crutches':'body-swing','person-with-rollator':'walking-midstance','person-with-umbrella':'umbrella-furled','person-with-white-cane':'walking-midstance','standing-phone-user':'phone-call','tour-guide':'standing-pointing','worker-carrying-box':'lift-mid'}}
cases=groups.get(family,dict(x.split('=',1) for x in sys.argv[2:]))
for name,pose in cases.items():
    p=next(r.glob('🎓*/*/*staffage/*catalog/*/'+name+'/*.json'));raw=p.read_bytes();h=hashlib.sha256(raw).hexdigest()
    if pose=='default':pose=json.loads(raw)['asset']['presentation']['defaultPose']
    dest=g/'living-current-views'/f'{name}-{pose}-all-presentation-shaded-temperature-{h[:10]}.png'
    if dest.is_file():continue
    with (g/f'control-view-{name}.log').open('w') as log:
        ret=subprocess.run([sys.executable,t/'equipment-guard.py',os.environ['BUN_EXE'],'--smol',t/'living-visual.ts',name,'all',pose,p,'--presentation','--shaded','--temperature'],env={**os.environ,'EQUIPMENT_RSS_MIB':'580','EQUIPMENT_HEADROOM_PERCENT':'48'},stdout=log,stderr=log).returncode
    print(name,ret,dest.is_file(),flush=True)
