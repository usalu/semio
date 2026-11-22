"""📐 Run the whole rule suite in bounded, checkpointed named groups and report individual assertions."""
from pathlib import Path
import json,os,re,sys,subprocess
from hashlib import sha256
from runpy import run_path
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';staff=next(r.glob('🎓*/*architecture/*staffage'));case=next(staff.glob('*tests/*volumes-rules/🟦️.ts'));vectors=next(staff.glob('*fixtures/*volumes-rules/🔣️.json'));oracle=next(case.parent.glob('*.py'));bun=sys.argv[1]
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,case,vectors,oracle)
tests=json.loads((g/'current-rule-tests.json').read_bytes());names=[x['name'] for x in tests];record=g/('rule-test-results-'+sig[:12]+'.json')
done=json.loads(record.read_bytes()) if record.exists() else {}
ordered=[n for n in names if done.get(n,{}).get('status')!='passed']
def run(batch):
 key=sha256(repr(batch).encode()).hexdigest()[:16];pst=g/('rule-test-'+key+'.json');log=pst.with_suffix('.log')
 args=[sys.executable,str(t/'equipment-guard.py'),bun,'--smol','node_modules/vitest/vitest.mjs','run','--config',str(t/'vs-vitest.config.ts'),'--maxWorkers','1','--pool=threads','--reporter=json','--outputFile='+str(pst),'-t','(?:'+'|'.join(re.escape(n.split(' > ')[-1]) for n in batch)+')$']
 if pst.exists():pst.unlink()
 with log.open('w') as output:subprocess.run(args,cwd=r,stdout=output,stderr=output,env={**os.environ,'VS_TEST':str(case.relative_to(r)),'EQUIPMENT_RSS_MIB':'1080','EQUIPMENT_HEADROOM_PERCENT':'48'})
 rows=[q for s in json.loads(pst.read_bytes())['testResults'] for q in s['assertionResults'] if q['status']!='pending'] if pst.exists() else []
 if not rows and len(batch)>1:
  cut=len(batch)//2;run(batch[:cut]);run(batch[cut:]);return
 for n in batch:
  hits=[q for q in rows if q['title']==n.split(' > ')[-1]]
  done[n]={'status':hits[0]['status'],'messages':hits[0].get('failureMessages',[])[:1]} if hits else {'status':'interrupted'}
 record.write_text(json.dumps(done,ensure_ascii=False));print(len(done),len(names),{x:sum(q['status']==x for q in done.values()) for x in ('passed','failed','interrupted')},flush=True)
for i in range(0,len(ordered),6):run(ordered[i:i+6])
