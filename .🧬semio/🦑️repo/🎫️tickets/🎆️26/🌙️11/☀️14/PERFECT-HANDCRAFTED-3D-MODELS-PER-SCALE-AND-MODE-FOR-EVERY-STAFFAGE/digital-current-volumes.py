"""🔍 Validate catalog source bytes one scale at a time, retaining measured findings and the exact rule signature."""
from pathlib import Path
from hashlib import sha256
from runpy import run_path
import os,sys,json,subprocess
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';db=g/'canonical-scale-checks';db.mkdir(exist_ok=True)
signature=run_path(str(t/'digital-proof-digest.py'))['module_digest']
def strict():return signature(r,t/'digital-scale-proof.ts')
def combined():return signature(r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
refs=combined();inputs=sorted(r.glob('🎓*/*architecture/*staffage/*catalog/*/*/🔣️.json'),key=lambda p:p.stat().st_size)
partition=next((a for a in sys.argv[1:] if '/' in a),None)
if partition:
 idx,num=map(int,partition.split('/'));inputs=inputs[idx::num]
wanted={a for a in sys.argv[1:] if '/' not in a};inputs=[p for p in inputs if not wanted or p.parent.name in wanted]
for p in inputs:
 name=p.parent.name;file=db/(name+'.json');raw=p.read_bytes();h=sha256(raw).hexdigest();sig=strict();prev=json.loads(file.read_bytes()) if file.is_file() else {}
 if prev.get('hash')!=h or prev.get('signature')!=sig:prev={'hash':h,'signature':sig,'scales':{}}
 if len(prev['scales'])==4 and not any(prev['scales'].values()):continue
 for candidate in g.glob('*-'+name+'.json'):
  test=candidate.with_name(candidate.stem+'-500.json')
  if not test.exists():continue
  tested=candidate.read_bytes();ch=sha256(tested).hexdigest()
  try:equal=json.loads(raw)['asset']==json.loads(tested)['asset']
  except (KeyError,TypeError):continue
  if not equal:continue
  for k in ('50','100','200','500'):
   rec=candidate.with_name(candidate.stem+'-'+k+'.json')
   if not rec.exists():continue
   data=json.loads(rec.read_bytes())
   if data.get('hash')==ch and data.get('signature')==refs and data.get('complete') and data.get('pass'):prev['scales'][k]=[]
 for scale in ('500','200','100','50'):
  if scale not in prev['scales']:
   cmd=[sys.executable,t/'equipment-guard.py',os.environ['BUN_EXE'],'--smol',t/'digital-scale-proof.ts',p,scale]
   log=db/(name+'-'+scale+'.log')
   with log.open('w') as output:subprocess.run(cmd,cwd=r,stdout=output,stderr=output,env={**os.environ,'EQUIPMENT_BACKGROUND':'1','EQUIPMENT_RSS_MIB':'1030' if p.stat().st_size>2000000 else '800','EQUIPMENT_HEADROOM_PERCENT':'48'})
   rows=[json.loads(l) for l in log.read_text().splitlines() if l.startswith('{')]
   if sha256(p.read_bytes()).hexdigest()!=h or strict()!=sig:break
   if len(rows)==1 and len(rows[0].get('rules',[]))==13:
    prev['scales'][scale]=rows[0]['issues']
   else:
    issues=run_path(str(t/'digital-bounded-proof.py'))['measure'](os.environ['BUN_EXE'],p,scale,sig)
    if issues is None:print(name,scale,'interrupted',flush=True);continue
    prev['scales'][scale]=issues
  if sha256(p.read_bytes()).hexdigest()!=h:break
  temp=file.with_suffix('.'+str(os.getpid())+'.partial');temp.write_text(json.dumps(prev,ensure_ascii=False));os.replace(temp,file)
 print(name,len(prev['scales']),sum(len(v) for v in prev['scales'].values()),flush=True)
