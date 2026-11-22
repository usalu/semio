"""🦴 Prepare, measure and publish independently checked human shaft contours from current catalog sources."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json,os,shutil,subprocess,sys
p=Path(__file__).resolve().parent
root=p.parents[6];g=p/'🗑️generated'
bun=os.environ.get('BUN_EXE') or shutil.which('bun') or next(f for f in root.glob('.cache/b/t/*/bun/*/bun*') if f.name in ('bun','bun.exe'))
study=json.loads((p/'digital-human-shaft-case.json').read_bytes())
parts=('document','50','100','200','500','glb')
def native(name,args,tag):
    log=g/('shaft-'+name+'-'+tag+'.log')
    with log.open('w') as out:r=subprocess.run([sys.executable,p/'equipment-guard.py',bun,'--smol',*args],stdout=out,stderr=out,env={**os.environ,'EQUIPMENT_RSS_MIB':'280'})
    return r.returncode,log.read_text()
for name in sys.argv[2:] or study['subjects']:
    cat=next(root.glob('🎓*/*architecture/*staffage/*catalog/*/'+name+'/*.json'));before=sha256(cat.read_bytes()).hexdigest();draft=g/('limb-'+name+'.json')
    if sys.argv[1]=='prepare':
        code,baseline=native(name,[p/'digital-schoolchild-check.ts',cat],'original');line=next((s for s in baseline.splitlines() if s.startswith(name+' ')),None)
        if line is None:print(name,'baseline failed',baseline[-450:],flush=True);continue
        code,err=native(name,[p/'digital-limb-gauge.ts',cat,draft],'author');assert code==0,err[-700:]
        code,fit=native(name,[p/'digital-schoolchild-check.ts',draft],'fitted');row=next((s for s in fit.splitlines() if s.startswith(name+' ')),None)
        if not row or code:print(name,'fitted failed',fit[-450:],flush=True);continue
        assert sha256(cat.read_bytes()).hexdigest()==before
        stats={'source':before,'draft':sha256(draft.read_bytes()).hexdigest(),'original':json.loads(line.split(' ',1)[1]),'fitted':json.loads(row.split(' ',1)[1])}
        (g/('shaft-'+name+'.json')).write_text(json.dumps(stats));print(name,stats['original']['failed'],round(stats['fitted']['maximumDiameterChangeMm'],2),flush=True)
    elif sys.argv[1] in ('verify','publish'):
        if sys.argv[1]=='verify':
            subprocess.run([sys.executable,p/'digital-cervical-measure.py',name],check=True,env={**os.environ,'STAFFAGE_STUDY':'limb','STAFFAGE_WORKERS':'1','STAFFAGE_PARTS':'document,500,glb,50,100,200'})
        h=sha256(draft.read_bytes()).hexdigest();signature=run_path(str(p/'digital-proof-digest.py'))['module_digest'](root,p/'digital-scale-proof.ts',p/'r3-validate.ts',p/'digital-scenes.ts')
        receipts=[json.loads(f.read_bytes()) for part in parts if (f:=g/('limb-'+name+'-'+part+'.json')).is_file()]
        if len(receipts)!=6 or not all(r['hash']==h and r['signature']==signature and r['pass'] for r in receipts):
            print(name,'awaiting verified receipts',flush=True);continue
        same=json.loads(cat.read_bytes())['asset']==json.loads(draft.read_bytes())['asset']
        if not same:subprocess.run([sys.executable,p/'digital-rig-install.py',name,bun,'digital-limb-gauge.ts',draft.name],check=True)
        after=sha256(cat.read_bytes()).hexdigest();code,out=native(name,[p/'digital-schoolchild-check.ts',cat],'canonical');assert code==0,out[-700:]
        subprocess.run([sys.executable,p/'digital-human-review.py',name],check=True)
        assert sha256(cat.read_bytes()).hexdigest()==after;print(name,after,flush=True)
    else:raise ValueError(sys.argv[1])
