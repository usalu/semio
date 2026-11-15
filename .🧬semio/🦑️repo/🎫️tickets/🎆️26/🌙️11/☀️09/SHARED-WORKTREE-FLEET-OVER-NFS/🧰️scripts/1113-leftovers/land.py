#!/usr/bin/env python3
"""usage: land.py BASE WORK LIVE [--dry]
Lands every file of WORK that differs from BASE into LIVE: a file is replaced only while the live file is byte-identical to BASE (atomic tmp + rename);
otherwise a three-way `git merge-file` (base, live, work) runs and the landing stops on a conflict. Prints `path before after` (16 hex) per file."""
import hashlib,os,subprocess,sys,tempfile,shutil
base,work,live=sys.argv[1:4]; dry="--dry" in sys.argv
def sha(p):
    return hashlib.sha256(open(p,"rb").read()).hexdigest()[:16] if os.path.exists(p) else "-"
changed=[]
for root,_,names in os.walk(work):
    for n in names:
        p=os.path.join(root,n); rel=os.path.relpath(p,work); b=os.path.join(base,rel)
        if not os.path.exists(b) or open(b,"rb").read()!=open(p,"rb").read(): changed.append(rel)
for root,_,names in os.walk(base):
    for n in names:
        rel=os.path.relpath(os.path.join(root,n),base)
        if not os.path.exists(os.path.join(work,rel)): print("DELETED IN WORK (not landed):",rel); sys.exit(2)
def depth(rel):
    first=rel.split("/")[0]
    return (0 if "🧬️schema" in rel or "📖️texts" in rel or "fixtures" in rel else 1 if "🧪️tests" not in rel and rel!="🔁️replica/🟦️.ts" and rel!="README.md" else 2 if rel=="🔁️replica/🟦️.ts" else 3,rel)
changed.sort(key=depth)
for rel in changed:
    b=os.path.join(base,rel); l=os.path.join(live,rel); w=os.path.join(work,rel)
    before=sha(l)
    if os.path.exists(b) and os.path.exists(l) and open(b,"rb").read()!=open(l,"rb").read():
        merged=tempfile.mkstemp()[1]; shutil.copy(l,merged)
        r=subprocess.run(["git","merge-file","-p",l,b,w],capture_output=True)
        if r.returncode!=0: print("CONFLICT",rel,r.returncode); sys.exit(3)
        data=r.stdout
    elif not os.path.exists(b) and os.path.exists(l):
        print("LIVE HAS A FILE THAT WORK ADDS:",rel); sys.exit(4)
    else:
        data=open(w,"rb").read()
    if not dry:
        os.makedirs(os.path.dirname(l),exist_ok=True)
        tmp=l+".landing"; open(tmp,"wb").write(data); os.replace(tmp,l)
    print(rel,before,hashlib.sha256(data).hexdigest()[:16])
