"""🧠 Verifies one human and one job per child under the shared memory guard, preserving each complete result."""
from pathlib import Path
import os,sys,subprocess,json,hashlib,time
t=Path(__file__).resolve().parent
out=t/'🗑️generated'/'hu-review'
hum=next(t.parents[6].glob('🎓️teaching/*/🧍️staffage/🗂️catalog/👤️humans'))
deadline=time.monotonic()+float(os.environ.get('HU_ROUND_SECONDS','inf'))
for name in (sys.argv[2:] or sorted(p.name for p in hum.iterdir() if p.is_dir())):
    mode=sys.argv[1]
    if time.monotonic()>=deadline:break
    asset=next((hum/name).glob('*.json'))
    def digest():return hashlib.sha256(asset.read_bytes()).hexdigest()
    sha=digest()
    receipt=out/(name+'-export.json')
    if mode=='export' and receipt.exists():
        prev=json.loads(receipt.read_bytes())
        if prev['sha256']==sha and prev.get('current') and not prev.get('findings'):continue
    args=['hu-checked.ts',name,mode] if mode!='export' else ['digital-scenes.ts',name]
    with (out/('fresh-'+mode+'-'+name+'.log')).open('w') as log:
        result=subprocess.run([sys.executable,str(t/'equipment-guard.py'),os.environ.get('BUN_EXE','bun'),'--smol',*args],cwd=t,stdout=log,stderr=log)
    print(name,result.returncode,flush=True)
    if result.returncode:sys.exit(result.returncode)
    if mode=='export':
        rows=[json.loads(line) for line in (out/('fresh-'+mode+'-'+name+'.log')).read_text().splitlines() if line.startswith('{')]
        assert len(rows)==1 and rows[0]['id']==name and rows[0].get('scenes')==12,rows
        proof={**rows[0],'sha256':sha,'current':digest()==sha}
        receipt.write_text(json.dumps(proof))
        assert proof['current'] and not proof['findings'],proof

