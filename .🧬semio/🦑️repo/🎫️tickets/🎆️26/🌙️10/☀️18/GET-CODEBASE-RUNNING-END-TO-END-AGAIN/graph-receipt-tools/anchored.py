"""Anchored, idempotent, atomic edits: apply(path, [(anchor, replacement)]) refuses unless each anchor occurs exactly once (or the replacement is already present and the anchor is gone)."""
import os,sys,tempfile
def apply(path, ops, dry=False):
    data=open(path,encoding="utf8").read()
    out=data
    for anchor,repl in ops:
        n=out.count(anchor)
        if n==1: out=out.replace(anchor,repl,1)
        elif n==0 and repl in out: continue
        else: raise SystemExit(f"ANCHOR {'MISSING' if n==0 else 'AMBIGUOUS('+str(n)+')'} in {os.path.basename(os.path.dirname(path))}/{os.path.basename(path)}: {anchor[:90]!r}")
    if out==data: print("unchanged",path[-60:]); return False
    if dry: print("would write",path[-60:]); return True
    st=os.stat(path)
    fd,tmp=tempfile.mkstemp(dir=os.path.dirname(path),prefix=".gr-")
    with os.fdopen(fd,"w",encoding="utf8") as f: f.write(out)
    os.chmod(tmp,st.st_mode); os.replace(tmp,path); print("written",path[-60:]); return True
