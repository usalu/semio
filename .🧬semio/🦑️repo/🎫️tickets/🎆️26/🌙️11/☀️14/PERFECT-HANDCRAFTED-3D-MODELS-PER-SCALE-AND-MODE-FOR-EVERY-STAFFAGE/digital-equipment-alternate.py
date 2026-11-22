"""🔎 Render each obscured working face from the operating side or below the cabinet."""
import json
from pathlib import Path
import shutil, subprocess
T=Path(__file__).resolve().parent
for name,flags in [
    ('ceiling-diffuser',['--underside']),('kitchen-extractor-hood',['--underside','--opposite']),
    ('gas-meter',['--opposite']),('dining-table',['--underside']),('three-seat-sofa',['--opposite']),
    ('market-parasol',['--underside']),('patio-heater',[]),('operating-table',['--underside']),
    ('desktop-monitor',['--opposite']),('shower-head',['--underside']),('waste-bin',['--opposite'])
]:
    source=next(T.parents[6].glob('🎓*/*/*staffage/🗂*/*/'+name+'/🔣️.json'))
    pose=json.loads(source.read_bytes())['asset']['presentation']['defaultPose']
    subprocess.run([shutil.which('bun'),'--smol',T/'living-visual.ts',name,'all',pose,source,'--presentation','--shaded','--temperature',*flags],check=True)
