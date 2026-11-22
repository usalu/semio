"""🏠 Check a catalog asset per process and record both input/output evidence and its input digest."""
from pathlib import Path
import json,os,sys,subprocess,shutil,hashlib
T=Path(__file__).resolve().parent;R=T.parents[6];S=next(R.glob('🎓*/*/*staffage'));out=T/'🗑️generated/digital-house-scenes.jsonl'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
from runpy import run_path
module_digest=run_path(str(T/"digital-proof-digest.py"))["module_digest"]
def version():return module_digest(R,T/"digital-scenes.ts")
catalog=sorted(next(S.glob('🗂*')).glob('*/*/🔣️.json'))
requested={s for s in sys.argv[1:] if not s.startswith('--')}
unknown=requested-{p.parent.name for p in catalog}
if unknown:raise SystemExit('Unknown staffage: '+', '.join(sorted(unknown)))
assets=[p for p in catalog if (p.parent.name in requested if requested else '--all' in sys.argv or p.parent.parent.name.endswith(('objects','furniture','appliances')))]
partition=next((opt for opt in sys.argv[1:] if opt.startswith('--shard=')),None)
if partition:
    index,count=map(int,partition.split('=',1)[1].split('/'))
    if not 0<=index<count:raise ValueError(partition)
    assets=sorted(assets,key=lambda p:(-p.stat().st_size,p.parent.name))[index::count]
old={(r['id'],r['sha256'],r['validator']):r for r in ([json.loads(line) for line in out.read_text().splitlines()] if out.exists() else [])}
bun=os.environ.get('BUN_EXE') or shutil.which('bun') or next(p for p in R.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
os.environ["EQUIPMENT_BACKGROUND"]="1"
for p in assets:
    name=p.parent.name;key=(name,digest(p),version())
    if key in old and old[key].get('findings')==[]:continue
    result=subprocess.run([sys.executable,str(T/'equipment-guard.py'),str(bun),'--smol',str(T/'digital-scenes.ts'),'--file',str(p)],capture_output=True,text=True)
    rows=[json.loads(x) for x in result.stdout.splitlines() if x.startswith('{')]
    record=rows[-1] if rows else {'id':name,'paused':result.returncode,'message':result.stderr[-280:]}
    if key!=(name,digest(p),version()):
        print('[DEBUG] source or exporter changed during',name,flush=True);continue
    record.update(sha256=key[1],validator=key[2])
    with out.open('a') as f:f.write(json.dumps(record,ensure_ascii=False)+'\n')
    print(name,record.get('bytes',record.get('paused')),len(record.get('findings',[])),flush=True)
    if 'Host headroom below' in result.stderr:break
