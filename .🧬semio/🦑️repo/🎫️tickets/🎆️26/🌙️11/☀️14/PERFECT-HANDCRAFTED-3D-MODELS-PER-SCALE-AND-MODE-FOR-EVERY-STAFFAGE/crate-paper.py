"""🧺 Measure all thirteen paper sizes of each crate drawing, one guarded process per size."""
from pathlib import Path
from hashlib import sha256
import json,math,os,shutil,subprocess,sys

ticket=Path(__file__).resolve().parent
root=ticket.parents[6]
asset=next((root/'🎓️teaching').glob('*architecture/*staffage/*catalog/*/storage-crate-stack/*.json'))
data=asset.read_bytes();digest=sha256(data).hexdigest();reps=json.loads(data)['asset']['representations']
log=ticket/'🗑️generated/equipment-check/crate-sizes.jsonl';log.parent.mkdir(exist_ok=True,parents=True)
rows={(r['band'],r['den']):r for line in (log.read_text().splitlines() if log.exists() else []) if (r:=json.loads(line)).get('sha256')==digest}
bun=shutil.which('bun') or next((root/'.cache/b/t/m/bun').glob('*/bun'))
for band,rep in reps.items():
    span=rep['validRange'];lo=span['minDenominator'];hi=span['maxDenominator']
    for i in range(13):
        den=str(math.floor(lo+(hi-lo)*i/12+.5))
        if (band,den) in rows:continue
        proc=subprocess.run([sys.executable,str(ticket/'equipment-guard.py'),str(bun),'--smol',str(ticket/'equipment-paper.ts'),str(asset),band,den],capture_output=True,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':os.environ.get('EQUIPMENT_RSS_MIB','175')})
        if not proc.stdout.strip():raise SystemExit(proc.stderr or proc.returncode)
        r=json.loads(proc.stdout);r['sha256']=digest
        with log.open('a') as f:f.write(json.dumps(r)+'\n')
        print(band,den,len(r['issues']),flush=True)
assert asset.read_bytes()==data
