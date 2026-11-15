#!/usr/bin/env python3
"""Rehearses the R28 landing and its rollback in a scratch root: copies the touched live files, lands with --root, compares with the overlay, rolls back and compares with the originals. usage: rehearse.py <scratch root>"""
import os,sys,shutil,subprocess,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
LIVE="/Users/Shared/semio";SC=os.path.abspath(sys.argv[1])
assert os.path.realpath(SC).startswith("/private/tmp/claude-501/"),SC
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
if os.path.exists(SC):shutil.rmtree(SC)
rels=sorted({e[0] for e in edits.E}|{r for r,_ in edits.REPLACE})
for rel in rels:os.makedirs(os.path.dirname(SC+"/"+rel),exist_ok=True);shutil.copyfile(LIVE+"/"+rel,SC+"/"+rel)
open(SC+"/AGENTS.md","w").close();os.makedirs(SC+"/.cache/f",exist_ok=True);open(SC+"/.cache/f/machine.json","w").write("{}")
before={r:sha(SC+"/"+r) for r in rels}
def land(*args):return subprocess.run([sys.executable,HERE+"/land.py",*args,"--root",SC],capture_output=True,text=True)
dry=land("--dry-run");print(dry.stdout[-600:]);assert "REFUSED" not in dry.stdout,dry.stdout
assert all(sha(SC+"/"+r)==before[r] for r in rels),"dry run wrote"
done=land("--land","--no-tests");print(done.stdout[-400:],done.stderr[-300:]);assert done.returncode==0
changed=[r for r in rels if sha(SC+"/"+r)!=before[r]];print("changed",len(changed),"of",len(rels))
assert len(changed)==len(rels)
for rel,src in edits.REPLACE:assert sha(SC+"/"+rel)==hashlib.sha256(open(HERE+"/"+src,"rb").read()).hexdigest()
again=land("--dry-run");assert "REFUSED" not in again.stdout and "edit " not in again.stdout.replace("same","") ,again.stdout;print("second dry run: nothing left to do")
import glob
bk=sorted(glob.glob(SC+"/.🧬semio/**/🗑️generated/R28-landing-*",recursive=True))
assert bk,"no backup written";rb=land("--rollback",bk[-1]);print(rb.stdout[-200:],rb.stderr[-200:]);assert rb.returncode==0
assert all(sha(SC+"/"+r)==before[r] for r in rels),"rollback did not restore every file"
newfiles=[os.path.join(d,f) for d,_,fs in os.walk(SC) for f in fs if "vouch" in d and "🗑️generated" not in d]
assert not newfiles,("new files left after rollback",newfiles)
print("REHEARSAL OK: land, second dry run clean, rollback restores",len(rels),"files byte for byte and removes the new test files")
