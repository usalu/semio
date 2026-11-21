"""🦴 Validate each individually posed cervical study within the local memory pool."""
from concurrent.futures import ThreadPoolExecutor
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,shutil,subprocess,sys
t=Path(__file__).resolve().parent;g=t/'🗑️generated';r=t.parents[6]
bun=os.getenv('BUN_EXE') or shutil.which('bun') or next(p for p in r.glob('.cache/b/t/*/bun/*/bun*') if p.name in ('bun','bun.exe'))
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
study=os.environ.get('STAFFAGE_STUDY','atlas')
if study not in ('atlas','passenger','clothing','cat','gull','axle','fittings','hedge'):raise ValueError('Unknown study')
receipts='neck' if study=='atlas' else study
subjects=sys.argv[1:] or list(json.loads((t/'digital-cervical-stations.json').read_bytes()))
def check(pair):
    name,part=pair;input=g/(study+'-'+name+'.json')
    if not input.exists():return
    h=sha256(input.read_bytes()).hexdigest();prefix=receipts+'-'+name+'-'+part;rec=g/(prefix+'.json');log=g/(prefix+'.log')
    if rec.exists():
        prev=json.loads(rec.read_bytes())
        if prev['hash']==h and prev['signature']==sig and prev['complete']:return
    cmd=[t/'r3-validate.ts','--file',input,'--compact'] if part=='document' else [t/'digital-scenes.ts','--file',input] if part=='glb' else [t/'digital-scale-proof.ts',input,part]
    rss=1050 if study=='fittings' else 540 if part in ('document','glb') else 830
    with log.open('w') as stream:
        done=subprocess.run([sys.executable,str(t/'equipment-guard.py'),str(bun),'--smol',*map(str,cmd)],stdout=stream,stderr=stream,env={**os.environ,'EQUIPMENT_RSS_MIB':str(rss),'EQUIPMENT_HEADROOM_PERCENT':'48'})
    out=log.read_text();rows=[json.loads(l) for l in out.splitlines() if l.startswith('{')]
    if part=='document':
        complete=any(l.startswith(('VALID ','INVALID '))for l in out.splitlines())
        problems=[l for l in out.splitlines() if l.startswith(('PRINT ','ASSET ','SCHEMA '))]
    elif part=='glb':
        complete=bool(rows and rows[0].get('scenes')==12)
        problems=rows[0].get('findings',[]) if rows else []
    else:
        complete=bool(rows and len(rows[0].get('rules',[]))==13)
        problems=rows[0].get('issues',[]) if rows else []
    if sha256(input.read_bytes()).hexdigest()!=h:return
    result={'hash':h,'signature':sig,'complete':complete,'pass':complete and done.returncode==0 and not problems,'findings':problems}
    temp=rec.with_suffix('.'+str(os.getpid())+'.partial');temp.write_text(json.dumps(result));os.replace(temp,rec)
    print(name,part,'pass' if result['pass'] else 'interrupted' if not complete else json.dumps(problems,ensure_ascii=False)[:650],flush=True)
parts=os.environ.get('STAFFAGE_PARTS','document,50,100,200,500,glb').split(',')
with ThreadPoolExecutor(max_workers=min(3,int(os.environ.get('STAFFAGE_WORKERS','3')))) as pool:
    list(pool.map(check,[(n,k) for n in subjects for k in parts]))
