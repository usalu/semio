"""🌿 Checkpoint the single guarded living-asset worker and yield on the next item boundary."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time

ticket=Path(__file__).resolve().parent
repo=Path.cwd()
staff=next(repo.glob('*teaching/*architecture/*staffage'))
kernel=next(repo.glob('*framework/*modules/*3d/*model'))
out=ticket/'🗑️generated/living-current'
out.mkdir(exist_ok=True,parents=True)
pause=out/'pause'
timing=out/'timings.jsonl'
p=argparse.ArgumentParser()
p.add_argument('--bun',required=True)
p.add_argument('--list',action='store_true')
p.add_argument('--max-jobs',type=int)
p.add_argument('--mode',choices=['glb','strict'])
p.add_argument('--by-scale',action='store_true')
p.add_argument('subjects',nargs='*')
args=p.parse_args()
def digest(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def raw_version(mode):
    roots=[kernel/'🔺️tessellation',kernel/'✅️validation',kernel/'🧮️math',kernel/'🩻️integrity',kernel/'📐️bounds',staff/'🔨️modules/🧊️volume',staff/'🔨️modules/🦴️rig']
    if mode=='glb':roots+=[kernel/'📤️export/🧊️glb',kernel/'📏️edges',staff/'🔨️modules/📤️export/🧊️glb',staff/'🧪️tests/🧊️model-catalog/🔍️oracles']
    else:roots+=[staff/'🔨️modules/🗂️catalog/🧊️volumes',staff/'🔨️modules/🗂️catalog/🧊️volume',staff/'🔨️modules/🧮️evaluation',staff/'🔨️modules/🎨️depiction/🏗️structural-view']
    files=[ticket/'living-current.ts',kernel/'🧬️schema/🟦️.ts',staff/'🧬️schema/🟦️.ts',*(f for root in roots for f in sorted(root.rglob('*.ts')))]
    h=hashlib.sha256()
    for f in files:h.update(f.name.encode());h.update(f.read_bytes())
    return h.hexdigest()
def version(mode):
    return subprocess.check_output([args.bun,'--smol',str(ticket/'living-run-signature.ts'),mode],text=True).strip()
alias_file=out/'code-aliases.json'
aliases=json.loads(alias_file.read_text()) if alias_file.exists() else {raw_version(m):version(m) for m in ('glb','strict')}
if not alias_file.exists():alias_file.write_text(json.dumps(aliases))
snapshot={m:version(m) for m in ('glb','strict')} if args.list else {}
def result(path,source,code):
    if not path.exists():return None
    data=json.loads(path.read_text())
    observed=data.get('code')
    return data if data.get('hash')==source and aliases.get(observed,observed)==code else None
def worker(name,mode,code,scale=None):
    cmd=[sys.executable,str(ticket/'equipment-guard.py'),args.bun,'--smol',str(ticket/'living-current.ts'),name,mode]
    if scale is not None:cmd.append(scale)
    env={**os.environ,'LIVING_AUDIT_CODE':code}
    return subprocess.run(cmd,env=env).returncode
def rule_groups(name,scale,sha,code,target):
    summaries=[]
    for group in ('source','bounds','elevation','front','plan','heat','skeleton','pair'):
        if pause.exists():return 75
        path=out/f'{name}-strict-{scale}-{group}.json'
        data=result(path,sha,code)
        if data is None:
            script='living-silhouette-view.ts' if group in ('elevation','front','plan') else 'living-strict-part.ts'
            cmd=[sys.executable,str(ticket/'equipment-guard.py'),args.bun,'--smol',str(ticket/script),name,scale,group]
            rc=subprocess.run(cmd,env={**os.environ,'LIVING_AUDIT_CODE':code}).returncode
            if rc:return rc
            data=result(path,sha,code)
        if data is None:return 75
        summaries.append(data)
    if version('strict')!=code or digest(assets[name])!=sha:return 75
    summary={'id':name,'scale':int(scale),'code':code,'hash':sha,'rules':[k for s in summaries for k in s['rules']],'findings':[f for s in summaries for f in s['findings']]}
    temp=target.with_suffix('.tmp');temp.write_text(json.dumps(summary));temp.replace(target)
    return 0
def record(info):
    with timing.open('a') as f:f.write(json.dumps(info)+'\n')
    print('[DEBUG]',info,flush=True)
catalog=next(staff.glob('*catalog'))
assets={p.parent.name:p for category in ('*vegetation','*animals') for p in catalog.glob(category+'/*/*.json')}
order=args.subjects or ['bamboo-clump','pigeon','agave','domestic-cat','english-oak','red-fox','multi-stem-birch','goat','lombardy-poplar','sheep','small-leaved-lime','pig','london-plane-pollarded','dairy-cow','scots-pine','hedgehog','japanese-maple','labrador-dog','hosta-clump','red-squirrel','ivy-on-trellis','herring-gull','miscanthus-grass','horse','weeping-willow','mallard-duck','hornbeam-hedge','roe-deer','date-palm','apple-tree']
history=[json.loads(line) for line in timing.read_text().splitlines()] if timing.exists() else []
sharded={row['asset'] for row in history if row['mode']=='strict' and row['exit'] in (-15,75,143,241)}
done=0
for name in order:
    for mode in ([args.mode] if args.mode else ['glb','strict']):
        if not args.list and pause.exists() or args.max_jobs is not None and done>=args.max_jobs:sys.exit(0)
        file=assets[name];code=snapshot.get(mode) or version(mode);sha=digest(file);target=out/(name+'-'+mode+'.json')
        if result(target,sha,code) is not None:continue
        if args.list:
            print(name,mode,sha[:10],code[:10]);continue
        start=time.monotonic()
        print('[DEBUG] starting',name,mode,flush=True)
        rc=75 if mode=='strict' and (args.by_scale or name in sharded) else worker(name,mode,code)
        data=None
        if mode=='strict' and rc in (-15,75,241):
            findings=[];parts=[]
            for scale in ('50','100','200','500'):
                if pause.exists():sys.exit(0)
                part=out/(name+'-strict-'+scale+'.json')
                old=result(part,sha,code)
                if old is None:
                    split=out/f'{name}-strict-{scale}.split'
                    rc=75 if split.exists() else worker(name,mode,code,scale)
                    if rc in (-15,75,241):
                        split.touch()
                        rc=rule_groups(name,scale,sha,code,part)
                    if rc:break
                    old=result(part,sha,code)
                if old is None:break
                findings.extend(old['findings']);parts.append(scale)
            if len(parts)==4 and digest(file)==sha and version(mode)==code:
                data={'id':name,'hash':sha,'code':code,'scales':parts,'findings':findings}
                temp=target.with_suffix('.tmp');temp.write_text(json.dumps(data));temp.replace(target);rc=0
        current=digest(file)==sha and version(mode)==code
        if current:data=result(target,sha,code)
        record(dict(asset=name,mode=mode,hash=sha,code=code,seconds=round(time.monotonic()-start,1),exit=rc,current=current,findings=len(data['findings']) if data else None,bytes=data.get('bytes') if data else None))
        done+=1

