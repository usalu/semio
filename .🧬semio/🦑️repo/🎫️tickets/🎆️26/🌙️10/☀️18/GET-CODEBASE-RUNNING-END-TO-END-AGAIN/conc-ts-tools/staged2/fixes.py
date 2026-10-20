#!/usr/bin/env python3
"""Group 2: anchored source fixes that make exported declarations nameable (declaration boundary law). Data: fixes.json.
usage: fixes.py [--root DIR] [--only ID[,ID]] [--skip-missing] [--data FILE] [--apply | --undo]   (dry run by default)
Every `old` text must occur exactly `count` times (default 1) in the CURRENT file; a file whose ops are all already applied is skipped (idempotent);
anything else refuses the whole set (drift). --apply writes all files or none (temp + rename, rollback on error, backups in <ticket>/🗑️generated/conc-ts/fixes-backup).
--root points the same operations at an overlay copy (verification) instead of the repository."""
import json,os,sys,shutil,hashlib
HERE=os.path.dirname(os.path.abspath(__file__))
REPO=os.path.abspath(os.path.join(HERE,*['..']*9))
def main():
    a=sys.argv[1:];root=a[a.index('--root')+1] if '--root' in a else REPO
    only=set(a[a.index('--only')+1].split(',')) if '--only' in a else None
    data=a[a.index('--data')+1] if '--data' in a else os.path.join(HERE,'fixes.json')
    fx=[f for f in json.load(open(data)) if not only or f['id'] in only]
    bk=os.path.join(HERE,'..','..','🗑️generated','conc-ts','fixes-backup',hashlib.sha256(root.encode()).hexdigest()[:8])
    if '--undo' in a:
        n=0
        if os.path.isdir(bk):
            for r,_,fs in os.walk(bk):
                for f in fs:
                    rel=os.path.relpath(os.path.join(r,f),bk);shutil.copyfile(os.path.join(r,f),os.path.join(root,rel));n+=1
        print('restored',n);return 0
    todo=[];bad=[]
    for f in fx:
        p=os.path.join(root,f['file'])
        if not os.path.exists(p):
            if '--skip-missing' in a: print('absent ',f['id']);continue
            bad.append((f['id'],'missing file'));continue
        s=open(p,encoding='utf8').read();state=[]
        for i,op in enumerate(f['ops']):
            c=op.get('count',1);o=s.count(op['old']);n=s.count(op['new'])
            if op['old'] in op['new']: st='done' if n==c else ('todo' if o==c and n==0 else 'DRIFT')
            else: st='todo' if o==c else ('done' if o==0 and n>=1 else 'DRIFT')
            state.append(st)
            if st=='DRIFT': bad.append((f['id'],'op %d: %d occurrences of old (expected %d)'%(i,o,c)))
        if 'DRIFT' in state: continue
        if all(x=='done' for x in state): print('done   ',f['id']);continue
        if any(x=='done' for x in state): bad.append((f['id'],'partially applied'));continue
        t=s
        for op in f['ops']: t=t.replace(op['old'],op['new'])
        todo.append((f,p,s,t));print('would  ',f['id'],f['file'][-70:])
    for i,m in bad: print('REFUSED',i,m)
    print(len(todo),'to apply,',len(bad),'refused')
    if bad: return 1
    if '--apply' not in a: print('dry run: pass --apply');return 0
    done=[]
    try:
        for f,p,s,t in todo:
            b=os.path.join(bk,f['file']);os.makedirs(os.path.dirname(b),exist_ok=True)
            if not os.path.exists(b): open(b,'w',encoding='utf8').write(s)
            tmp=p+'.concts.tmp';open(tmp,'w',encoding='utf8').write(t);os.replace(tmp,p);done.append((p,s))
    except Exception as e:
        for p,s in done: open(p,'w',encoding='utf8').write(s)
        print('rolled back',e);return 1
    print('applied',len(done));return 0
sys.exit(main())
