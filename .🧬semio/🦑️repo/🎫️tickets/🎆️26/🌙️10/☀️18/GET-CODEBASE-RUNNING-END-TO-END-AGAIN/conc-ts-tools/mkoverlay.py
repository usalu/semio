#!/usr/bin/env python3
"""Frozen overlay of a program: copies the program's workspace files, ancestor package.json / tsconfig files and rebuilds node_modules as symlink directories
(third-party -> real store, workspace packages -> overlay copies). usage: mkoverlay.py <listed.txt> <family tsconfig (repo-relative)> <overlay dir>"""
import os,sys,shutil,json
R='/Users/ueli/Documents/semio'
listed,family,ov=sys.argv[1],sys.argv[2],sys.argv[3]
shutil.rmtree(ov,ignore_errors=True); os.makedirs(ov)
files=[l.strip() for l in open(listed) if l.strip().startswith(R+'/') and '/node_modules/' not in l]
rel=sorted(set(f[len(R)+1:] for f in files))
need=set(rel)
def ancestors(p):
    d=os.path.dirname(p)
    while True:
        for n in ('package.json','tsconfig.json'):
            q=os.path.join(d,n) if d else n
            if os.path.exists(os.path.join(R,q)): need.add(q)
        if not d: break
        d=os.path.dirname(d)
for p in rel: ancestors(p)
need.add(family)
c=json.load(open(os.path.join(R,family)))
for p in list(need):
    if p.endswith('tsconfig.json'):
        pass
# follow extends chain
cur=family
while True:
    j=json.load(open(os.path.join(R,cur)));e=j.get('extends')
    if not e: break
    cur=os.path.normpath(os.path.join(os.path.dirname(cur),e)); need.add(cur)
for p in sorted(need):
    src=os.path.join(R,p)
    if not os.path.isfile(src): continue
    dst=os.path.join(ov,p); os.makedirs(os.path.dirname(dst),exist_ok=True); shutil.copyfile(src,dst)
pkgdirs={os.path.dirname(p) for p in need if p.endswith('package.json')}
def link_modules(d):
    real=os.path.join(R,d,'node_modules') if d else os.path.join(R,'node_modules')
    if not os.path.isdir(real): return
    out=os.path.join(ov,d,'node_modules'); os.makedirs(out,exist_ok=True)
    def one(name,path,dst):
        t=os.path.realpath(path)
        if t.startswith(R+'/') and '/.cache/' not in t:
            tr=t[len(R)+1:]
            if tr in pkgdirs or os.path.isdir(os.path.join(ov,tr)): t=os.path.join(ov,tr)
        if not os.path.lexists(dst): os.symlink(t,dst)
    for name in sorted(os.listdir(real)):
        p=os.path.join(real,name)
        if name.startswith('@') and os.path.isdir(p):
            os.makedirs(os.path.join(out,name),exist_ok=True)
            for n2 in sorted(os.listdir(p)): one(n2,os.path.join(p,n2),os.path.join(out,name,n2))
        else: one(name,p,os.path.join(out,name))
for d in sorted(pkgdirs|{''}): link_modules(d)
print('copied',len(need),'files; package dirs',len(pkgdirs),'overlay',ov)
