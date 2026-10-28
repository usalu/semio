#!/usr/bin/env python3
"""[testmem2-g] applies, reverts or checks the testmem2-g batch (anchored edits in batch.json, new files in new-files.tgz).
usage: stage.py check|apply|revert|selftest [--root DIR] [--only PREFIX]    exit 0 when every file ends in the requested state, 1 on a conflict.
A file is `after` when every edit's new text is present, `base` when every edit's old text is present (and none of the new), `conflict` otherwise (a peer edited the anchor). apply/revert replace each edit once and only when its anchor occurs exactly once; new files are never overwritten when they differ."""
import json,os,sys,tarfile,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));args=sys.argv[1:];mode=args[0]
root=args[args.index("--root")+1].rstrip("/")+"/" if "--root" in args else "/Users/ueli/Documents/semio/"
only=args[args.index("--only")+1] if "--only" in args else ""
real=os.path.realpath(root)+os.sep
batch=json.load(open(HERE+"/batch.json",encoding="utf8"));batch["files"]=[e for e in batch["files"] if e["file"].startswith(only)];sha=lambda s:hashlib.sha256(s.encode("utf8")).hexdigest()
def state(text,edits):
    news=[e["new"] in text for e in edits];olds=[e["old"] in text and not e["new"] in text for e in edits]
    return "after" if all(news) else "base" if all(olds) else "conflict"
def convert(text,edits,direction):
    for e in (edits if direction=="apply" else reversed(edits)):
        a,b=(e["old"],e["new"]) if direction=="apply" else (e["new"],e["old"])
        if b in text and a not in text: continue
        if text.count(a)!=1: return None
        text=text.replace(a,b,1)
    return text
failed=0
for entry in batch["files"]:
    path=root+entry["file"]
    if os.path.exists(path) and not os.path.realpath(path).startswith(real): print("REFUSED symlinked path "+entry["file"]);failed+=1;continue
    text=open(path,encoding="utf8").read() if os.path.exists(path) else "";now=state(text,entry["edits"])
    if mode=="selftest":
        base=convert(text,entry["edits"],"revert");again=convert(base,entry["edits"],"apply") if base is not None else None
        ok=base is not None and sha(base)==entry["baseSha256"] and again is not None and sha(again)==entry["afterSha256"]
        print(("OK   " if ok else "FAIL ")+entry["file"][-70:]+" roundtrip base/after");failed+=not ok;continue
    if mode=="check":
        print(("%-8s"%now)+entry["file"][-70:]+"  sha="+("after" if sha(text)==entry["afterSha256"] else "base" if sha(text)==entry["baseSha256"] else "other"));failed+=now=="conflict";continue
    want="after" if mode=="apply" else "base"
    if now==want: print("already "+want+"  "+entry["file"][-60:]);continue
    out=convert(text,entry["edits"],mode)
    if out is None: print("CONFLICT "+entry["file"]);failed+=1;continue
    open(path,"w",encoding="utf8").write(out);print(mode+"d  "+entry["file"][-60:])
names=[n for n in json.load(open(HERE+"/new-files.json",encoding="utf8")) if n.startswith(only)]
if mode=="apply":
    with tarfile.open(HERE+"/new-files.tgz") as tar:
        for member in tar.getmembers():
            if not member.name.startswith(only): continue
            target=root+member.name;data=tar.extractfile(member).read()
            if os.path.lexists(target) and not os.path.realpath(target).startswith(real): print("REFUSED symlinked path "+member.name);failed+=1;continue
            if os.path.exists(target):
                if open(target,"rb").read()!=data: print("CONFLICT new file differs "+member.name[-60:]);failed+=1
                continue
            os.makedirs(os.path.dirname(target),exist_ok=True);open(target,"wb").write(data)
    print("new files present:",len(names))
elif mode=="revert":
    for n in names:
        if os.path.exists(root+n): os.remove(root+n)
    for n in names:
        d=os.path.dirname(root+n)
        while d.startswith(root.rstrip("/")+"/"):
            try: os.rmdir(d)
            except OSError: break
            d=os.path.dirname(d)
    print("new files removed:",len(names))
else:
    print("new files present:",sum(os.path.exists(root+n) for n in names),"of",len(names))
sys.exit(1 if failed else 0)
