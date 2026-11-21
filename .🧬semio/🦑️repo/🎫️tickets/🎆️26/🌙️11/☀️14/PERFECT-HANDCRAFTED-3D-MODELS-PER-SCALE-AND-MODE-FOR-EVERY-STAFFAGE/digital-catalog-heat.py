"""🌡️ Checkpoint per-model thermal field measurements and render a source-matched catalogue report."""
from pathlib import Path
from hashlib import sha256
from runpy import run_path
import json,sys,os,subprocess,shutil
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated/digital-climate';g.mkdir(exist_ok=True,parents=True)
files=sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'),key=lambda p:p.stat().st_size)
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-heat-fields.ts',t/'digital-heat-fields.json');bun=os.getenv('BUN_EXE') or shutil.which('bun') or next(p for p in r.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
def receipt(p):
    f=g/(p.parent.name+'.json');j=json.loads(f.read_bytes()) if f.exists() else {}
    return j if j.get('hash')==sha256(p.read_bytes()).hexdigest() and j.get('signature')==sig else {}
todo=[p.parent.name for p in files if not receipt(p)]
if '--report' not in sys.argv:
    for i in range(0,len(todo),4):
        result=subprocess.run([sys.executable,str(t/'equipment-guard.py'),str(bun),'--smol',str(t/'digital-heat-fields.ts'),*todo[i:i+4]],env={**os.environ,'STAFFAGE_HEAT_SIGNATURE':sig})
        if result.returncode:print('[DEBUG] pending climate batch:',todo[i:i+4],flush=True)
rows=[receipt(p) for p in files];passed=[j for j in rows if j and not j['findings']]
text=['# Staffage thermal field verification','',f"{len(passed)} of {len(files)} current assets passed for all their named poses in the authored environment and at 14 °C in shade. These runs cover {sum(j['poses'] for j in passed)} poses, {sum(j['climates'] for j in passed)} pose and climate combinations, and {sum(j['fields'] for j in passed):,} surface fields.",'','The product solver supplies surface temperatures and heat fluxes. Decimal.js independently computes blackbody emission from the SI defining constants and the emissivity of the outer material. The comparisons also check radiation to sky, ground and surroundings and the exposed area used for convection and radiation. The heat solver is shared across display scales. Digital geometry at each scale is verified separately.','','| Asset | Poses | Region fields | Findings |','| --- | ---: | ---: | --- |']
for p,j in sorted(zip(files,rows),key=lambda a:a[0].parent.name):
    text.append(f"| {p.parent.name} | {j.get('poses','—')} | {j.get('fields','—')} | "+('; '.join(j['findings'][:4]) if j.get('findings') else 'Pass' if j else 'Pending')+' |')
(t/'digital-catalog-heat.md').write_text('\n'.join(text)+'\n')
print(text[2])
