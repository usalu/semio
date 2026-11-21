"""🧮 Measure dense candidate scales with per-rule process lifetimes under the same memory ceiling."""
from pathlib import Path
from hashlib import sha256
from runpy import run_path
import sys,json,os,subprocess
t=Path(__file__).resolve().parent;r=t.parents[6];g=t/'🗑️generated';study,name,*scales=sys.argv[1:];candidate=g/(study+'-'+name+'.json');h=sha256(candidate.read_bytes()).hexdigest()
sig=run_path(str(t/'digital-proof-digest.py'))['module_digest'](r,t/'digital-scale-proof.ts',t/'r3-validate.ts',t/'digital-scenes.ts')
groups=['availability,feature,bones,solids,copied,slender,monotonic','bounds','silhouette,views-disagree','cross-scale','thermal-coverage','structure-coverage']
for scale in scales:
    allrules=[];issues=[]
    for idx,group in enumerate(groups):
        receipt=g/f'{study}-{name}-{scale}-part{idx}.json';rules=['volumes-'+n for n in group.split(',')]
        old=json.loads(receipt.read_bytes()) if receipt.exists() else {}
        if old.get('hash')!=h or old.get('signature')!=sig:
            log=receipt.with_suffix('.log')
            with log.open('w') as stream:
                subprocess.run([sys.executable,t/'equipment-guard.py',os.environ['BUN_EXE'],'--smol',t/'digital-scale-proof.ts',candidate,scale,','.join(rules)],env={**os.environ,'EQUIPMENT_RSS_MIB':'1050','EQUIPMENT_HEADROOM_PERCENT':'48'},stdout=stream,stderr=stream)
            rows=[json.loads(l) for l in log.read_text().splitlines() if l.startswith('{')]
            if len(rows)!=1 or sorted(rows[0]['rules'])!=sorted(rules):print(name,scale,group,'interrupted',flush=True);break
            old={**rows[0],'hash':h,'signature':sig};receipt.write_text(json.dumps(old))
        allrules+=old['rules'];issues.extend(old['issues'])
    if len(set(allrules))!=13 or sha256(candidate.read_bytes()).hexdigest()!=h:continue
    p=g/f'{study}-{name}-{scale}.json';out=p.with_suffix('.parts.tmp')
    out.write_text(json.dumps({'hash':h,'signature':sig,'complete':True,'pass':not issues,'findings':issues}));os.replace(out,p)
    print(name,scale,'pass' if not issues else str(issues)[:320],flush=True)
