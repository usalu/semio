"""🖨️ Check each authored band endpoint and nominal drawing with a fresh bounded print worker."""
from pathlib import Path
from hashlib import sha256
import os,json,subprocess,sys
ticket=Path(__file__).resolve().parent;out=ticket/'🗑️generated'
for file in sys.argv[2:]:
    p=out/file;raw=p.read_bytes();digest=sha256(raw).hexdigest();a=json.loads(raw)['asset'];del raw
    samples=[(d,k) for d,r in a['representations'].items()if r['status']=='authored'for k in sorted({int(d),r['validRange']['minDenominator'],r['validRange']['maxDenominator']})]
    del a
    for d,k in samples:
        target=out/(p.stem+'-print-'+d+'-'+str(k)+'.json')
        if target.exists() and json.loads(target.read_text()).get('source')==digest:continue
        run=subprocess.run([sys.executable,str(ticket/'equipment-guard.py'),sys.argv[1],'--smol',str(ticket/'digital-paper-candidate.ts'),str(p),d+'/'+str(k)],text=True,capture_output=True)
        lines=[json.loads(l)for l in run.stdout.splitlines()if l.startswith('{')]
        if lines:
            result=lines[0];target.write_text(json.dumps(dict(source=digest,**result)));print(p.stem,d,k,result['suitable'],[v['shapes'] for v in result['violations']],flush=True)
        else:print(p.stem,d,k,run.returncode,run.stderr[-85:],flush=True)
