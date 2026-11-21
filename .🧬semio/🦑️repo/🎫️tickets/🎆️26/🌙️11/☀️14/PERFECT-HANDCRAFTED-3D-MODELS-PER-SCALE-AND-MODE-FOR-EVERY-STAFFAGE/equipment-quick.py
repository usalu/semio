"""📏 Run six analytical rules across the 43 equipment models in small independent processes."""
from hashlib import sha256
from pathlib import Path
import json,os,re,shutil,subprocess,sys
t=Path(__file__).resolve().parent;root=t.parents[6];c=next((root/'🎓️teaching').glob('*architecture/*staffage/*catalog'));b=shutil.which("bun") or next((root/'.cache/b/t/m/bun').glob('*/bun'))
imports=re.compile(r"""(?:\bfrom\s*|\bimport\s*\(?\s*)["'](\.[^"']+)["']""")
mode="print" if "--print" in sys.argv else "semantic" if "--semantic" in sys.argv else "analytic"
chosen={v for v in sys.argv[1:] if not v.startswith("--")}
script=t/{"print":"equipment-print.ts","semantic":"equipment-semantic.ts","analytic":"equipment-quick.ts"}[mode]
known=set();queue=[script];version=sha256()
while queue:
    source=queue.pop().resolve()
    if source in known or not source.is_file():continue
    known.add(source);queue.extend(source.parent/name for name in imports.findall(source.read_text()))
for source in sorted(known):version.update(str(source.relative_to(root)).encode());version.update(source.read_bytes())
revision=version.hexdigest()
out=t/'🗑️generated/equipment-check'/(mode+'.jsonl');previous={r['asset']:r for row in out.read_text().splitlines() if (r:=json.loads(row))} if out.exists() else {}
with out.open('a') as log:
    for folder in sorted(c.iterdir()):
        if not folder.name.endswith(('components','vehicles','machinery')):continue
        for p in sorted(folder.glob('*/*.json')):
            name=p.parent.name
            if chosen and name not in chosen:continue
            digest=sha256(p.read_bytes()).hexdigest()
            if previous.get(name,{}).get("input")==digest and previous[name].get("code")==revision:continue
            run=subprocess.run([sys.executable,str(t/'equipment-guard.py'),str(b),'--smol',str(script),str(p)],capture_output=True,text=True,env={**os.environ,'EQUIPMENT_RSS_MIB':os.environ.get('EQUIPMENT_RSS_MIB','140')})
            if run.returncode or not run.stdout.startswith('{'):print(name,'guard',run.stderr[:120]);continue
            record=json.loads(run.stdout);record['input']=digest;record['code']=revision
            log.write(json.dumps(record)+'\n');log.flush();print(name,len(record['issues']),flush=True)
