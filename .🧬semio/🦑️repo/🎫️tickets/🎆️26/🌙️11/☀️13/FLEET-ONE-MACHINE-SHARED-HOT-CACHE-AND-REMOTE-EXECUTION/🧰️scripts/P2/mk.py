#!/usr/bin/env python3
import sys,os,hashlib,importlib.util
S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/P2"
LIVE=os.environ.get("P2_LIVE","/Users/Shared/semio")
spec=importlib.util.spec_from_file_location("patches",S+"/tools/patches.py");m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
def apply(text,edits,path):
    for old,new in edits:
        n=text.count(old)
        if n!=1: raise SystemExit(f"anchor found {n} times in {path}: {old[:90]!r}")
        text=text.replace(old,new,1)
    return text
def build(files=None):
    for rel,edits in m.PATCHES.items():
        if files and rel not in files: continue
        src=open(os.path.join(LIVE,rel),encoding="utf8").read()
        out=apply(src,edits,rel)
        for old,new in getattr(m,"MIRROR_ONLY",{}).get(rel,[]):
            if old not in out: raise SystemExit("mirror anchor missing in "+rel)
            out=out.replace(old,new)
        dst=os.path.join(S,"mirror",rel);os.makedirs(os.path.dirname(dst),exist_ok=True);open(dst,"w",encoding="utf8").write(out)
    for rel,(src) in m.NEW.items():
        text=open(os.path.join(S,"new",rel),encoding="utf8").read()
        for old,new in getattr(m,"MIRROR_ONLY",{}).get(rel,[]): text=text.replace(old,new)
        dst=os.path.join(S,"mirror",rel);os.makedirs(os.path.dirname(dst),exist_ok=True);open(dst,"w",encoding="utf8").write(text)
def land():
    for rel,edits in m.PATCHES.items():
        p=os.path.join(LIVE,rel);src=open(p,encoding="utf8").read();out=apply(src,edits,rel)
        print(rel,hashlib.sha256(src.encode()).hexdigest()[:12],"->",hashlib.sha256(out.encode()).hexdigest()[:12])
        open(p,"w",encoding="utf8").write(out)
    for rel in m.NEW:
        p=os.path.join(LIVE,rel)
        if os.path.exists(p): raise SystemExit("exists: "+rel)
        os.makedirs(os.path.dirname(p),exist_ok=True);open(p,"w",encoding="utf8").write(open(os.path.join(S,"new",rel),encoding="utf8").read())
if __name__=="__main__":
    cmd=sys.argv[1]
    if cmd=="build": build(sys.argv[2:] or None)
    elif cmd=="check":
        for rel,edits in m.PATCHES.items(): apply(open(os.path.join(LIVE,rel),encoding="utf8").read(),edits,rel)
        print("anchors ok")
    elif cmd=="land": land()
