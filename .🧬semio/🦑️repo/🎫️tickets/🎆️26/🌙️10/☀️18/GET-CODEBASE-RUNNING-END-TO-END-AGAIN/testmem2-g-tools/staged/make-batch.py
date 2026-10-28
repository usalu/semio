#!/usr/bin/env python3
"""[testmem2-g] builds batch.json (anchored line edits with old and new text, base and after sha256 per existing file) and new-files.tgz from the workbench (a clone of the process module, see ../mkwb.sh) and ./files (edited copies of library test files, flat: manifest.json + f<NNN>), against the REAL tree as it is now.
usage: make-batch.py <workbench dir>"""
import difflib,hashlib,json,os,sys,tarfile
R="/Users/ueli/Documents/semio/";HERE=os.path.dirname(os.path.abspath(__file__));WB=sys.argv[1].rstrip("/")+"/"
PROC="🧰️framework/🔨️modules/🏃️process/"
sha=lambda s:hashlib.sha256(s.encode("utf8")).hexdigest()
def walk(base,sub):
    for d,_,fs in os.walk(base+sub):
        for f in fs: yield os.path.relpath(os.path.join(d,f),base)
pairs=[]  # (rel, afterPath)
MINE=[PROC+p for p in ["🎛️owned-execution/📥️capture/🟦️.ts","🎛️owned-execution/🟦️.ts","🧪️testing/🗂️suites/🏃️run/🟦️.ts","🧪️testing/🟨️bun/📁️files/🟦️.ts"]]
for rel in walk(WB,PROC):
    if rel in MINE or not os.path.exists(R+rel): pairs.append((rel,WB+rel))
FLAT=HERE+"/files";FILES={rel:FLAT+"/"+e["file"] for rel,e in json.load(open(FLAT+"/manifest.json",encoding="utf8")).items()}
for rel,path in FILES.items(): pairs.append((rel,path))
edited=[];new=[]
for rel,after in sorted(pairs):
    if os.path.islink(after): continue
    if os.path.exists(R+rel) and open(R+rel,"rb").read()==open(after,"rb").read(): continue
    a=open(after,encoding="utf8").read()
    if not os.path.exists(R+rel): new.append(rel);continue
    b=open(R+rel,encoding="utf8").read()
    if a==b: continue
    bl,al=b.splitlines(keepends=True),a.splitlines(keepends=True);edits=[]
    for tag,i1,i2,j1,j2 in difflib.SequenceMatcher(None,bl,al,autojunk=False).get_opcodes():
        if tag=="equal": continue
        k=1 if tag=="insert" else 0
        while True:
            if i1-k<0: raise SystemExit("no unique anchor: "+rel)
            old="".join(bl[i1-k:i2]);nw="".join(al[j1-k:j2])
            if b.count(old)==1 and a.count(nw)==1: break
            k+=1
        edits.append({"old":old,"new":nw})
    edited.append({"file":rel,"baseSha256":sha(b),"afterSha256":sha(a),"edits":edits})
json.dump({"files":edited},open(HERE+"/batch.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
with tarfile.open(HERE+"/new-files.tgz","w:gz") as tar:
    for rel in new: tar.add(WB+rel if os.path.exists(WB+rel) else FILES[rel],arcname=rel)
json.dump(new,open(HERE+"/new-files.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(edited),"edited files,",sum(len(e["edits"]) for e in edited),"edits,",len(new),"new files")
for e in edited: print("  edit",e["file"][-80:],len(e["edits"]))
for n in new: print("  new ",n[-80:])
