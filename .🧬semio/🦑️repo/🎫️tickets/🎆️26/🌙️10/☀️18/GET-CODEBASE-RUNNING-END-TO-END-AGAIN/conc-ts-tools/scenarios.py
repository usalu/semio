#!/usr/bin/env python3
"""Edit scenarios on the overlay: prints which file each scenario edits. usage: scenarios.py <s1|s2|s3|revert>"""
import json,re,sys,os,shutil
OV='/private/tmp/conc-ts/ov-lib'; B='/private/tmp/conc-ts/scenario-backup'
plan=json.load(open('/private/tmp/conc-ts/run-lib4/plan.json'))
proc=[u for u in plan['units'] if u['id']=='group:🧰️framework/🔨️modules/🏃️process'][0]
cands=[f for f in proc['files'] if '/🧪️tests/' not in f and f.endswith('🟦️.ts')]
def pick():
    for f in cands:
        s=open(os.path.join(OV,f)).read()
        if re.search(r'throw Error\("[A-Za-z ]{12,}"\)',s): return f
f=pick(); leaf='🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts'
os.makedirs(B,exist_ok=True)
def save(p):
    d=os.path.join(B,p.replace('/','_'))
    if not os.path.exists(d): shutil.copyfile(os.path.join(OV,p),d)
cmd=sys.argv[1]
if cmd=='revert':
    for n in os.listdir(B):
        pass
    for p in (f,leaf):
        d=os.path.join(B,p.replace('/','_'))
        if os.path.exists(d): shutil.copyfile(d,os.path.join(OV,p))
    print('reverted');sys.exit()
if cmd=='s1':
    save(f);s=open(os.path.join(OV,f)).read();m=re.search(r'throw Error\("([A-Za-z ]{12,})"\)',s)
    open(os.path.join(OV,f),'w').write(s.replace(m.group(0),m.group(0).replace(m.group(1),m.group(1)+" again"),1));print('s1 body edit',f)
if cmd in('s1b','s1c'):
    save(f);s=open(os.path.join(OV,f)).read();open(os.path.join(OV,f),'w').write(s.replace(' again"',' again twice"',1) if cmd=='s1b' else s.replace(' twice"',' thrice"',1));print(cmd,'body edit',f)
if cmd=='s2':
    save(f);open(os.path.join(OV,f),'a').write('\nexport function concTsProbe(n:number){return n+1}\n');print('s2 signature edit',f)
if cmd=='s3':
    save(leaf);open(os.path.join(OV,leaf),'a').write('\n// conc-ts probe\nconst concTsProbe=1;\n');print('s3 leaf test edit',leaf)
