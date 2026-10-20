#!/usr/bin/env python3
"""TS2742 overlay experiment: usage ts2742-experiment.py <label> [link dir relative to the overlay]... ; always adds the two generated exports when a link or --exports is given.
Re-runs only the unit configs that had TS2742 in the baseline run, fresh compiler state, and counts TS2742 diagnostics."""
import json,glob,re,os,sys,subprocess,hashlib,shutil
T='/private/tmp/conc-ts';OV=T+'/ov-s';RUN=T+'/run-r'
label=sys.argv[1];links=[a for a in sys.argv[2:] if not a.startswith('--')];exports='--exports' in sys.argv
man=OV+'/🧰️framework/🔨️modules/🛂️manifest'
orig=subprocess.run(['git','show','HEAD:🧰️framework/🔨️modules/🛂️manifest/package.json'],capture_output=True,text=True,cwd='/Users/ueli/Documents/semio').stdout
open(man+'/package.json','w').write(orig)
if exports or links:
    d=json.loads(orig);d['exports']['./generated/manifest']='./🤖️generated/🪪️manifest/🟦️.ts';d['exports']['./generated/ui-contract']='./🤖️generated/📜️ui-contract/🟦️.ts'
    open(man+'/package.json','w').write(json.dumps(d,ensure_ascii=False,indent=2))
made=[]
for l in links:
    p=os.path.join(OV,l,'node_modules','@semio-tech');os.makedirs(p,exist_ok=True)
    t=os.path.join(p,'framework-manifest')
    if not os.path.lexists(t): os.symlink(man,t);made.append(t)
units=set()
for f in glob.glob(RUN+'/store/receipts/*/*.json'):
    r=json.load(open(f))
    if 'TS2742' in r['diagnostics']: units.add(r['unit'])
total=0;sites=set()
for u in sorted(units):
    cfg=json.load(open(RUN+'/state/sessions/s1/configs/'+hashlib.sha256(u.encode()+b"\0").hexdigest()[:24]+'.json'))
    cfg['compilerOptions']['outDir']=T+'/x-out';cfg['compilerOptions']['tsBuildInfoFile']=T+'/x.tsbuildinfo'
    xc=RUN+'/state/sessions/s1/configs/x-cfg.json';json.dump(cfg,open(xc,'w'))
    out=subprocess.run(['nice','-n','10','node','--max-old-space-size=2048','/Users/ueli/Documents/semio/node_modules/typescript/bin/tsc','-p',xc,'--pretty','false'],capture_output=True,text=True,cwd=OV).stdout
    for l in out.split('\n'):
        m=re.match(r'(.*?\(\d+),\d+\): error TS2742',l)
        if m: total+=1;sites.add(m.group(1))
    shutil.rmtree(T+'/x-out',ignore_errors=True)
    if os.path.exists(T+'/x.tsbuildinfo'): os.remove(T+'/x.tsbuildinfo')
for t in made: os.remove(t)
print(label,'| units',len(units),'| TS2742 diagnostics',total,'| sites',len(sites))
