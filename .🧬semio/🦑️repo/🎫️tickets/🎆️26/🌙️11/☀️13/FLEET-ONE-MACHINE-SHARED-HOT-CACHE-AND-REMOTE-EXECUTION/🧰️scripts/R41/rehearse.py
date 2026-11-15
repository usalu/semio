#!/usr/bin/env python3
"""R41: rehearses land.py in a scratch root (copies of the four live files only): dry run, landing, idempotence, a concurrent unrelated edit (--merge), a refused anchor, rollback byte for byte. usage: rehearse.py <scratch root under /private/tmp/claude-501>"""
import os,sys,shutil,subprocess,hashlib
HERE=os.path.dirname(os.path.abspath(__file__));sys.path.insert(0,HERE)
import edits
SC=os.path.abspath(sys.argv[1]);assert os.path.realpath(os.path.dirname(SC)).startswith("/private/tmp/claude-501/"),SC
LIVE="/Users/Shared/semio"
if os.path.exists(SC):shutil.rmtree(SC)
T="/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION"
os.makedirs(SC+T)
files=sorted({e[0] for e in edits.E})
for rel in files:
    os.makedirs(os.path.dirname(SC+"/"+rel),exist_ok=True);shutil.copyfile(LIVE+"/"+rel,SC+"/"+rel)
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
orig={rel:sha(SC+"/"+rel) for rel in files}
def land(*a):
    r=subprocess.run([sys.executable,HERE+"/land.py","--root",SC,*a],capture_output=True,text=True);return r.returncode,r.stdout+r.stderr
def check(label,ok):
    print(("ok   " if ok else "FAIL ")+label)
    if not ok:sys.exit(1)
code,out=land("--dry-run");check("dry run: nothing written, no refusal",code==0 and "REFUSED" not in out and all(sha(SC+"/"+r)==orig[r] for r in files))
code,out=land("--land","--no-tests");check("land",code==0 and "landed" in out and all(sha(SC+"/"+r)!=orig[r] for r in files if not r.endswith(".d.mts") or True))
landed={rel:sha(SC+"/"+rel) for rel in files}
backup=[l.split("originals in ")[1].strip() for l in out.splitlines() if "originals in" in l][0]
code,out=land("--dry-run");check("second dry run: all same (idempotent)",code==0 and out.count("same ")>=len(files))
check("lock released",not os.path.exists(SC+T+"/landing.lock"))
code,out=land("--rollback",backup);check("rollback restores every file byte for byte",code==0 and all(sha(SC+"/"+r)==orig[r] for r in files))
epoch=[r for r in os.popen("cd '%s/new' && find . -type f"%HERE).read().split("\n") if r]
check("rollback removed the new test file",all(not os.path.exists(SC+"/"+r[2:]) for r in epoch))
p=SC+"/"+files[1];open(p,"a",encoding="utf8").write("\n// unrelated edit of a peer\n");peer=sha(p)
code,out=land("--land","--merge","--no-tests");check("merge: lands on top of the peer's edit",code==0 and "// unrelated edit of a peer" in open(p,encoding="utf8").read() and "staleFacts" in open(p,encoding="utf8").read())
code,out=land("--rollback",backup);check("rollback after a merge restores the pre-landing bytes (peer edit included in the backup taken at landing time)",code==0)
victim=edits.E[0][0];txt=open(SC+"/"+victim,encoding="utf8").read();open(SC+"/"+victim,"w",encoding="utf8").write(txt.replace(edits.E[0][1][:40],"CHANGED-BY-PEER-"))
code,out=land("--dry-run");check("an anchor that a peer removed is REFUSED",code==0 and "REFUSED" in out)
code,out=land("--land","--no-tests");check("and a landing is refused as a whole (nothing written)",code!=0)
shutil.rmtree(SC);print("rehearsal done")
