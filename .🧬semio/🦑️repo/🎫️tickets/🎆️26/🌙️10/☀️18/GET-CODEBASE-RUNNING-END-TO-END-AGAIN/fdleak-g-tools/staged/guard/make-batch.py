#!/usr/bin/env python3
"""[fdleak-g] builds batch.json (anchored line edits with old and new text, base and after sha256) and new-files.tgz from the workbench mirror of the guard batch.
Edits of live files are anchored on the live text; the two files of the bun test ceiling (new in census2-g's batch) are anchored on census2-g's version (extracted from its new-files.tgz) and marked prereq.
usage: make-batch.py <workbench dir> <census2 extracted dir>"""
import difflib,hashlib,json,os,sys,tarfile
R="/Users/ueli/Documents/semio/";HERE=os.path.dirname(os.path.abspath(__file__));WB=sys.argv[1].rstrip("/")+"/";X=sys.argv[2].rstrip("/")+"/"
PROC="🧰️framework/🔨️modules/🏃️process/";BUN=PROC+"🧪️testing/🟨️bun/"
sha=lambda s:hashlib.sha256(s.encode("utf8")).hexdigest()
LIVE=["bunfig.toml",PROC+"🧪️testing/🏖️sandbox/🛡️guard/🍞️bun/🔌️preload/🟦️.ts",PROC+"🧰️tools/🏃️run/🟦️.ts"]
CENSUS=[BUN+"🧯️ceiling/🟦️.ts",BUN+"🧯️ceiling/🧪️tests/🟦️.ts"]
NEWDIRS=[BUN+"🪦️orphans/",BUN+"🚧️operands/"]
def walk(base,sub):
    for d,_,fs in os.walk(base+sub):
        for f in fs: yield os.path.relpath(os.path.join(d,f),base)
def edits_of(b,a,rel):
    bl,al=b.splitlines(keepends=True),a.splitlines(keepends=True);edits=[]
    for tag,i1,i2,j1,j2 in difflib.SequenceMatcher(None,bl,al,autojunk=False).get_opcodes():
        if tag=="equal": continue
        k=1
        while True:
            if i1-k<0 or i2+k>len(bl): raise SystemExit("no unique anchor: "+rel)
            old="".join(bl[i1-k:i2+k]);nw="".join(al[j1-k:j2+k])
            if b.count(old)==1 and a.count(nw)==1 and nw not in b and old not in a: break
            k+=1
        edits.append({"old":old,"new":nw})
    return edits
edited=[];new=[]
for rel in LIVE+CENSUS:
    a=open(WB+rel,encoding="utf8").read();b=open((R if rel in LIVE else X)+rel,encoding="utf8").read()
    if a==b: continue
    row={"file":rel,"baseSha256":sha(b),"afterSha256":sha(a),"edits":edits_of(b,a,rel)}
    if rel in CENSUS: row["prereq"]="census2-g"
    edited.append(row)
for d in NEWDIRS:
    for rel in sorted(walk(WB,d)):
        if not os.path.islink(WB+rel): new.append(rel)
GUIDE="🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
EN="   - **Test processes** are guarded three ways (`🧪️testing/🟨️bun`): a `bun test` operand without a leading `./` is refused (bun reads it as a filter and scans the whole checkout, holding every directory open: 122,880 files and 2 GB in one measured case); the preload `🪦️orphans` caps the open files of every `bun test` process (8192, `SEMIO_TEST_OPEN_FILES_CEILING`) and ends the process and everything it started when its runner is gone; the owned runners stop a tree at 90 % of the cap by name and stop and name every process a test file left running. `SEMIO_TEST_OPERAND_GUARD=off` and `SEMIO_TEST_ORPHAN_GUARD=off` lift them.\n"
DE="   - **Testprozesse** werden dreifach bewacht (`🧪️testing/🟨️bun`): ein `bun test`-Operand ohne führendes `./` wird abgelehnt (bun liest ihn als Filter und durchsucht den ganzen Checkout, jedes Verzeichnis bleibt offen: 122.880 Dateien und 2 GB in einem gemessenen Fall); der Preload `🪦️orphans` begrenzt die offenen Dateien jedes `bun test`-Prozesses (8192, `SEMIO_TEST_OPEN_FILES_CEILING`) und beendet den Prozess samt allem, was er gestartet hat, sobald sein Läufer weg ist; die eigenen Läufer halten einen Baum bei 90 % der Grenze namentlich an und beenden und nennen jeden Prozess, den eine Testdatei zurückgelassen hat. `SEMIO_TEST_OPERAND_GUARD=off` und `SEMIO_TEST_ORPHAN_GUARD=off` heben sie auf.\n"
guide=[]
for anchor,text in(("   - **Test runners** start the workers one job of their grant carries",EN),("   - **Testläufer** starten so viele Worker, wie ein Job ihrer Zusage trägt",DE)):
    guide.append({"old":anchor,"new":text+anchor})
edited.append({"file":GUIDE,"baseSha256":"census2-g version","afterSha256":"anchored","edits":guide,"anchoredOnly":True})
json.dump({"files":edited},open(HERE+"/batch.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
with tarfile.open(HERE+"/new-files.tgz","w:gz") as tar:
    for rel in new: tar.add(WB+rel,arcname=rel)
json.dump(new,open(HERE+"/new-files.json","w",encoding="utf8"),ensure_ascii=False,indent=1)
print(len(edited),"edited files,",sum(len(e["edits"]) for e in edited),"edits,",len(new),"new files")
for e in edited: print("  edit",e["file"][-70:],len(e["edits"]),e.get("prereq",""))
