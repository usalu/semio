#!/usr/bin/env python3
"""land.py check|apply: lands the files of Z/src that differ from their recorded base onto the live tree. A file whose live copy still equals the base is replaced; a file the live tree changed since is three-way merged (git merge-file, no repository involved) and left untouched when the merge conflicts; a new file is created. The previous live content of every touched file goes to Z/backup. Writes in place (no mv, no rename)."""
import os,sys,json,hashlib,subprocess,shutil
Z=os.path.dirname(os.path.abspath(__file__));LIVE='/Users/Shared/semio'
base=json.load(open(os.path.join(Z,'base.json')))
sha=lambda b:hashlib.sha256(b).hexdigest()
mode=sys.argv[1] if len(sys.argv)>1 else 'check'
changed=[]
for r,_,fs in os.walk(os.path.join(Z,'src')):
    for f in fs:
        rel=os.path.relpath(os.path.join(r,f),os.path.join(Z,'src'));new=open(os.path.join(Z,'src',rel),'rb').read()
        if rel in base and sha(new)==base[rel]:continue
        if rel not in base and not os.path.exists(os.path.join(LIVE,rel)) :changed.append((rel,'new'));continue
        if rel not in base:changed.append((rel,'exists-unrecorded'));continue
        changed.append((rel,'edit'))
report=[]
for rel,kind in sorted(changed):
    live=os.path.join(LIVE,rel);new=open(os.path.join(Z,'src',rel),'rb').read()
    if kind=='new':report.append((rel,'new'));continue
    if kind!='edit':report.append((rel,kind));continue
    cur=open(live,'rb').read()
    report.append((rel,'clean' if sha(cur)==base[rel] else 'live-changed'))
for rel,state in report:print(state.ljust(14),rel.split('/')[-3:])
if mode!='apply':sys.exit(0)
if any(s in('exists-unrecorded',) for _,s in report):sys.exit('unrecorded existing file')
for rel,state in report:
    live=os.path.join(LIVE,rel);new=open(os.path.join(Z,'src',rel),'rb').read()
    if state=='new':
        os.makedirs(os.path.dirname(live),exist_ok=True)
        with open(live,'wb') as f:f.write(new)
        continue
    cur=open(live,'rb').read()
    bk=os.path.join(Z,'backup',rel);os.makedirs(os.path.dirname(bk),exist_ok=True)
    with open(bk,'wb') as f:f.write(cur)
    if state=='clean':
        with open(live,'wb') as f:f.write(new)
    else:
        tmp=os.path.join(Z,'merge.tmp');
        with open(tmp,'wb') as f:f.write(cur)
        r=subprocess.run(['git','merge-file','-p',tmp,os.path.join(Z,'base',rel),os.path.join(Z,'src',rel)],capture_output=True)
        if r.returncode!=0:print('CONFLICT',rel,r.returncode);continue
        with open(live,'wb') as f:f.write(r.stdout)
        print('merged',rel)
print('applied')
