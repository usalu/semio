"""🧊️ Verify current shipping WASI IFC bytes against independently measured native outputs."""
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
import subprocess,sys,hashlib
ticket=Path(__file__).resolve().parent
g=ticket/'🗑️generated';out=g/'ifc-shipping-wasi';out.mkdir(exist_ok=True)
r=max((g/'compiler-retry').glob('w-ifc-*.success.json'),key=lambda f:f.stat().st_mtime_ns)
src=out/'export.rs';src.write_text('\n'.join(l for l in (ticket/'exchange-fixtures.rs').read_text().splitlines() if 'bim_svg' not in l and '"svg" =>' not in l))
wasm=out/'export.wasm'
if not wasm.exists() or r.stat().st_mtime_ns>wasm.stat().st_mtime_ns:subprocess.run([sys.executable,ticket/'link-captured-wasi.py',r,src,wasm,'bim_ifc=self','bim_snapshot=semio_s_artifact_bim_model_snapshot'],check=True)
b=Path('✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any').resolve()
f=b/'🧪️tests/🧫️fixtures'
cases=[]
for version,group in [('2x3','🏗️ifc'),('4','🏢️ifc4')]:
    cases.append((version,f/'💡️inferences/🛝️ramp-runs/🏞️ramps/📸️snapshot/🔣️.json',f/group/'🛝️ramps/🛝️ramps.ifc'))
    cases.append((version,f/'🏗️ifc/🔥️energy/📸️snapshot/🔣️.json',f/'🏗️ifc/🔥️energy'/('energy-'+version+'.ifc')))
cases.append(('2x3',f/'🏗️ifc/🪑️components/📸️snapshot/🔣️.json',f/'🏗️ifc/🪑️components/🪑️components.ifc'))
cases.append(('4',b/'🖼️assets/🏢️office/📸️snapshot.json',f/'🏢️ifc4/🏢️example-office/🏢️example-office.ifc'))
cases.append(('4',b/'🖼️assets/🏡️house/📸️snapshot.json',f/'🏢️ifc4/🏡️example-house/🏡️example-house.ifc'))
def verify(case):
    version,source,target=case
    destination=out/(version+'-'+target.name);args=[version,source,'-']
    runner=out/'jco/_run.js'
    command=['node',runner,*args] if runner.exists() else ['node_modules/.bin/jco','run',wasm,'--jco-dir',out/'jco',*args]
    with destination.open('wb') as output:subprocess.run(command,stdout=output,check=True)
    data=destination.read_bytes()
    assert data==target.read_bytes(),(target,len(data),len(target.read_bytes()))
    print(version,target.name,len(data),hashlib.sha256(data).hexdigest(),flush=True)
verify(cases[0])
with ThreadPoolExecutor(max_workers=2) as jobs:
    list(jobs.map(verify,cases[1:]))
