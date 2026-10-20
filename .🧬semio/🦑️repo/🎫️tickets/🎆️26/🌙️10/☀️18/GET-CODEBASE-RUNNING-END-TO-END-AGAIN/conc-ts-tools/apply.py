#!/usr/bin/env python3
"""Applies the staged TypeScript check-unit module (create-only, all or nothing).
Default is a dry run. `--apply` writes every file via temp + rename, backs up nothing (create-only), and rolls back every created file on any error.
`--root DIR` applies to a copy of the tree instead (verification overlay). `--undo` removes the created files if they still equal the staged bytes. Idempotent: a file that already equals its staged bytes is skipped; a different existing file refuses the whole set (drift)."""
import os,sys,shutil,hashlib
HERE=os.path.dirname(os.path.abspath(__file__)); S=os.path.join(HERE,'staged','🧩️units')
R=os.path.abspath(os.path.join(HERE,*['..']*8))
if '--root' in sys.argv: R=sys.argv[sys.argv.index('--root')+1]
L=os.path.join(R,'🧰️framework','🛍️products','🦑️repo','🔨️modules','📚️library')
T=os.path.join(L,'🏗️build','🟦️typescript','🧩️units')
def files():
    for r,_,fs in os.walk(S):
        for f in sorted(fs): yield os.path.relpath(os.path.join(r,f),S)
def digest(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
def plan():
    ops=[];refuse=[]
    for rel in sorted(files()):
        src=os.path.join(S,rel);dst=os.path.join(T,rel)
        if os.path.exists(dst):
            if digest(dst)==digest(src): ops.append(('same',rel))
            else: refuse.append(rel)
        else: ops.append(('create',rel))
    return ops,refuse
def main():
    args=sys.argv[1:]
    if not os.path.isdir(L): print('library not found:',L);return 2
    if '--undo' in args:
        n=0
        for rel in sorted(files(),reverse=True):
            dst=os.path.join(T,rel)
            if os.path.exists(dst) and digest(dst)==digest(os.path.join(S,rel)): os.remove(dst);n+=1
        for r,d,f in sorted(os.walk(T),reverse=True):
            if os.path.isdir(r) and not os.listdir(r): os.rmdir(r)
        print('undone',n);return 0
    ops,refuse=plan()
    for k,rel in ops: print(k,rel)
    for rel in refuse: print('DRIFT (exists, differs)',rel)
    print(sum(1 for k,_ in ops if k=='create'),'to create,',sum(1 for k,_ in ops if k=='same'),'already applied,',len(refuse),'refused')
    if refuse: return 1
    if '--apply' not in args: print('dry run: pass --apply to write');return 0
    made=[]
    try:
        for k,rel in ops:
            if k!='create': continue
            dst=os.path.join(T,rel);os.makedirs(os.path.dirname(dst),exist_ok=True)
            tmp=dst+'.tmp';shutil.copyfile(os.path.join(S,rel),tmp);os.replace(tmp,dst);made.append(dst)
    except Exception as e:
        for p in made: os.remove(p)
        print('rolled back:',e);return 1
    print('applied',len(made));return 0
sys.exit(main())
