"""📏 Recheck the 43 tracked documents under a small resident-set limit."""
from hashlib import sha256
from pathlib import Path
import json,os,shutil,subprocess,sys
t=Path(__file__).resolve().parent;root=t.parents[6];catalog=next((root/'🎓️teaching').glob('*architecture/*staffage/*catalog'));target=t/'🗑️generated/equipment-check/catalog-schema.jsonl'
schema=sha256((catalog.parent/'🧬️schema/🔣️.json').read_bytes()).hexdigest()
cache={(r["id"],r["sha256"]) for row in target.read_text().splitlines() if (r:=json.loads(row)).get("schemaSha256")==schema and r["valid"]}
b=shutil.which('bun') or next((root/'.cache/b/t/m/bun').glob('*/bun'))
files=sorted((p for d in catalog.iterdir() if d.name.endswith(('components','vehicles','machinery')) for p in d.glob('*/*.json')),key=lambda p:p.stat().st_size)
with target.open('a') as ledger:
    for p in files:
        name=p.parent.name;digest=sha256(p.read_bytes()).hexdigest()
        if (name,digest) in cache:continue
        child=subprocess.run([sys.executable,str(t/'equipment-guard.py'),str(b),'--smol',str(t/'equipment-schema.ts'),name],capture_output=True,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':'140'})
        if child.returncode or not child.stdout.startswith('{'):
            print('[DEBUG]',name,child.stderr,flush=True);break
        rec=json.loads(child.stdout)
        if rec['sha256']!=digest or rec['schemaSha256']!=schema or not rec['valid']:raise SystemExit(child.stdout)
        ledger.write(json.dumps(rec,separators=(',',':'))+'\n');ledger.flush();print(name,flush=True)
