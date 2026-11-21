"""🦴 Check the physical member mesh in every authored scale and persist source-matched results."""
from concurrent.futures import ThreadPoolExecutor
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,subprocess,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated'/'digital-members';g.mkdir(exist_ok=True);r=t.parents[6]
signature=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts')
assets=sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'),key=lambda p:p.stat().st_size)
def check(asset):
    name=asset.parent.name;record=g/(name+'.json');h=sha256(asset.read_bytes()).hexdigest()
    previous=json.loads(record.read_bytes()) if record.exists() else {}
    if previous.get('hash')!=h or previous.get('signature')!=signature:previous={'hash':h,'signature':signature,'scales':{}}
    for scale in ('50','100','200','500'):
        if scale in previous['scales']:continue
        cmd=[sys.executable,t/'equipment-guard.py',os.environ['BUN_EXE'],'--smol',t/'digital-scale-proof.ts',asset,scale,'volumes-structure-coverage']
        done=subprocess.run(cmd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':'840','EQUIPMENT_HEADROOM_PERCENT':'48'})
        rows=[json.loads(s) for s in done.stdout.splitlines() if s.startswith('{')]
        if not rows or sha256(asset.read_bytes()).hexdigest()!=h:return
        previous['scales'][scale]=rows[-1]['issues']
        temp=record.with_suffix('.partial');temp.write_text(json.dumps(previous));os.replace(temp,record)
    print(name,sum(map(len,previous['scales'].values())),flush=True)
with ThreadPoolExecutor(max_workers=2) as pool:list(pool.map(check,assets))
rows=[]
for asset in assets:
    name=asset.parent.name;p=g/(name+'.json');data=json.loads(p.read_bytes()) if p.exists() else {}
    exact=data.get('hash')==sha256(asset.read_bytes()).hexdigest() and data.get('signature')==signature
    scales=data.get('scales',{}) if exact else {}
    rows.append((name,len(scales),sum(map(len,scales.values()))))
with (t/'digital-member-coverage.md').open('w') as stream:
    stream.write('# Physical coverage of structural members\n\nEach measured scale compares the actual member mesh with the projected body in elevation, front and plan. A direct triangle distance resolves vertices past the raster tolerance on subpixel geometry.\n\n')
    stream.write(f"{sum(n for _,n,_ in rows)} of {len(rows)*4} exact-source scales measured. {sum(i for _,_,i in rows)} findings.\n\n| Model | Scales | Findings |\n| --- | ---: | ---: |\n")
    for n,count,issues in rows:stream.write(f'| {n} | {count} | {issues} |\n')
