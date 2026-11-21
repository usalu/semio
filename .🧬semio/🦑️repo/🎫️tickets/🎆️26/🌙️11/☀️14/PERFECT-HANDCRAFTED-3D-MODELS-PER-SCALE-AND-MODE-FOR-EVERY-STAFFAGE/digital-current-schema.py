"""📑 Check small groups and retain only results corresponding to the exact current document and schema."""
import hashlib,json,os,sys,subprocess
from pathlib import Path
T=Path(__file__).resolve().parent;R=T.parents[6];S=next(R.glob('🎓*/*/*staffage'))
out=T/'🗑️generated/digital-current-schema.log';catalog={p.parent.name:p for p in next(S.glob('🗂*')).glob('*/*/🔣️.json')}
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
schema=digest(S/'🧬️schema/🔣️.json')
records={}
if out.exists():
    for line in out.read_text().splitlines():
        if line.startswith('{'):
            row=json.loads(line);records[row['id']]=row
pending=[id for id,path in catalog.items() if records.get(id,{}).get('schemaSha256')!=schema or records.get(id,{}).get('sha256')!=digest(path)]
bun=(__import__('shutil').which('bun') or next(R.glob('.cache/b/t/*/bun/*/bun')))
for i in range(0,len(pending),8):
    proc=subprocess.run([sys.executable,str(T/'equipment-guard.py'),str(bun),'--smol',str(T/'equipment-schema.ts'),*pending[i:i+8]],capture_output=True,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':'350'})
    if proc.returncode:raise SystemExit(proc.stderr)
    for line in proc.stdout.splitlines():
        row=json.loads(line);records[row['id']]=row
    out.write_text(''.join(json.dumps(r,ensure_ascii=False)+'\n' for r in records.values()))
assert all(records[k]['schemaSha256']==schema and records[k]['sha256']==digest(v) and records[k]['valid'] for k,v in catalog.items())
print(len(catalog),'current documents conform to',schema[:12])
