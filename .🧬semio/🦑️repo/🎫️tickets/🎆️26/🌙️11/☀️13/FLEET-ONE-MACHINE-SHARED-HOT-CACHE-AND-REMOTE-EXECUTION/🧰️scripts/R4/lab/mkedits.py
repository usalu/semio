#!/usr/bin/env python3
"""R4 edit generator: pairs the before/after copies of the edited files (a scratch dir holding files.txt with one worktree-relative path per line, before/N.old and after/N.new) and writes ../edits.json (anchored literal replacements from the changed line runs, identical ones merged with a count) and ../baseline.json (sha256 of the before copies, which must equal the live files).
usage: mkedits.py <scratch dir>"""
import os,sys,json,hashlib,difflib
OUT=os.path.dirname(os.path.dirname(os.path.abspath(__file__)));S=os.path.abspath(sys.argv[1])
files=[l.rstrip("\n") for l in open(S+"/files.txt",encoding="utf8") if l.strip()]
edits=[];base={}
for n,rel in enumerate(files,1):
    old=open(S+"/before/%d.old"%n,"rb").read();new=open(S+"/after/%d.new"%n,"rb").read();base[rel]=hashlib.sha256(old).hexdigest()
    a=old.decode("utf8").splitlines(True);b=new.decode("utf8").splitlines(True);found={}
    for tag,i1,i2,j1,j2 in difflib.SequenceMatcher(None,a,b,autojunk=False).get_opcodes():
        if tag=="equal":continue
        if i1==i2:key=(a[i1-1],a[i1-1]+"".join(b[j1:j2]))
        else:key=("".join(a[i1:i2]),"".join(b[j1:j2]))
        found[key]=found.get(key,0)+1
    for (o,w),c in found.items():
        total=old.decode("utf8").count(o)
        if total!=c:sys.exit("%s: old chunk occurs %d times in the file but %d times as a change: %r"%(rel,total,c,o[:80]))
        edits.append({"path":rel,"kind":"literal","old":o,"new":w,"count":c})
json.dump(edits,open(OUT+"/edits.json","w",encoding="utf8"),ensure_ascii=False,indent=1);json.dump(base,open(OUT+"/baseline.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(edits),"edits in",len(files),"files")
