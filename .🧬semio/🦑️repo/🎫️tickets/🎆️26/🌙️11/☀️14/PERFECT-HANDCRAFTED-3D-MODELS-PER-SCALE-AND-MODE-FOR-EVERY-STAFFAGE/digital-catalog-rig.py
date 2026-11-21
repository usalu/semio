"""🦴 Record source-matched whole-catalogue rig, mass-centre and load checks."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,shutil,subprocess,sys
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated/digital-rigs'
files=sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'))
digest=run_path(str(t/'digital-proof-digest.py'))['module_digest']
sig=digest(r,t/'digital-rig-fields.ts',t/'digital-rig-contract.json')
bun=os.getenv('BUN_EXE') or shutil.which('bun') or next(p for p in r.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
def receipt(p):
    f=g/(p.parent.name+'.json');j=json.loads(f.read_bytes()) if f.exists() else {}
    return j if j.get('hash')==sha256(p.read_bytes()).hexdigest() and j.get('signature')==sig else {}
todo=[p.parent.name for p in files if not receipt(p)]
if '--report' not in sys.argv:
    for i in range(0,len(todo),8):
        subprocess.run([sys.executable,str(t/'equipment-guard.py'),str(bun),'--smol',str(t/'digital-rig-fields.ts'),*todo[i:i+8]],env={**os.environ,'STAFFAGE_RIG_SIGNATURE':sig})
rows=[receipt(p) for p in files]
passed=[j for j in rows if j and not j['findings']]
total=lambda k:sum(j[k] for j in passed)
text=['# Staffage rig and structural verification','',f"{len(passed)} of {len(files)} current model documents passed. The passing runs cover {total('poses')} named poses, {total('bones'):,} posed bone frames, {total('members'):,} member diagrams and {total('loads'):,} free-body forces and support reactions.",'','Three.js independently reconstructs parent-to-child rigid transforms and authored mass centres, then sums world forces and lever-arm moments. The checks cover segment endpoints, member diagram station positions and moment components. Folded cloth uses the solved crease basis for endpoint and mass calculations. Flight and deliberately unbalanced poses retain the physical residual in the comparison. Named poses are measured once because all four render scales share the same rig.','','| Model | Poses | Posed bones | Member diagrams | Result |','| --- | ---: | ---: | ---: | --- |']
for p,j in zip(files,rows):
    label='Pending' if not j else 'Pass' if not j['findings'] else ' / '.join(j['findings'][:3])
    text.append(f"| {p.parent.name} | {j.get('poses','—')} | {j.get('bones','—')} | {j.get('members','—')} | {label} |")
(t/'digital-catalog-rig.md').write_text('\n'.join(text)+'\n')
print(text[2])
