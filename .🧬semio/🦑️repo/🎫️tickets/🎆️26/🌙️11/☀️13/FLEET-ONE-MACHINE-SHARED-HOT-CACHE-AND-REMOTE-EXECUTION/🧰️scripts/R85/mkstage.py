#!/usr/bin/env python3
"""R85 stage builder: process containment of a readiness stage (roster with fast sampling and heirs, verified stop with survivors named in ready.json, orphan reaping through the lease registry at stage start and while a stage waits on a holder). Base = the live file at the time the scratch copy was taken (unchanged since, checked by the mtimes), new = the scratch file. Usage: mkstage.py SCRATCH_ROOT STAGE_DIR [--first] (landing b: every file exists, base = live)"""
import os,shutil,sys
R="/Users/ueli/Documents/semio/";S=sys.argv[1].rstrip("/")+"/";D=sys.argv[2].rstrip("/")+"/"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";RD=L+"🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/";C=RD+"🌲️process-tree/🧹️containment/"
shutil.rmtree(D,ignore_errors=True);os.makedirs(D+"base");os.makedirs(D+"new")
mods=[RD+"🟦️.ts",RD+"🧬️schema/🔣️.json",RD+"🧪️tests/🟦️.ts",RD+"🌲️process-tree/🟦️.ts",RD+"🌲️process-tree/🧬️schema/🔣️.json"]
news=[C+"🟦️.ts",C+"🧬️schema/🔣️.json",C+"🧪️tests/🟦️.ts",C+"🧪️tests/🔣️.json",C+"🧪️tests/🐍️.py"]
rows=[]
for n,rel in enumerate(mods+news):
    i="f%d"%n;kind="mod" if rel in mods or os.path.exists(R+rel) else "new"
    shutil.copyfile(S+"w/"+rel,D+"new/"+i)
    if kind=="mod":
        base=S+"ready.before.ts" if rel==RD+"🟦️.ts" and "--first" in sys.argv else R+rel
        shutil.copyfile(base,D+"base/"+i)
    rows.append("%s %s %s"%(i,rel,kind))
open(D+"index.txt","w",encoding="utf8").write("\n".join(rows)+"\n")
SC=L+"🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/"
tests=[C+"🧪️tests/🟦️.ts",RD+"🌲️process-tree/🧪️tests/🟦️.ts",RD+"🧪️tests/🟦️.ts",SC+"🔁️restart/🟦️.ts",SC+"🧮️closure/🟦️.ts"]
open(D+"tests.txt","w",encoding="utf8").write("\n".join(tests)+"\n")
open(D+"before.txt","w",encoding="utf8").write("\n".join(tests[:3])+"\n")
open(D+"note.txt","w",encoding="utf8").write("""Files: ready `🚦️ready/🟦️.ts` + schema (`containment` in ready.json, EN+DE), `🌲️process-tree` (program names in the table rows, `taskkill /PID <pid> /T /F`, no sampleMilliseconds), tests; new `🌲️process-tree/🧹️containment` (code, schema, fixture, Python oracle, tests).
- Containment: the roster of a stage is sampled every second during the first minute and every 5 s after (was 30 s); a process whose parent was a roster member that exited joins the roster (Windows keeps the dead parent pid); a stop ends the roster (Windows `taskkill /PID <pid> /T /F` per member with its recorded creation time, POSIX SIGKILL plus the process group), reads the table again, repeats up to 4 passes, names survivors (program name) in `ready.json` `containment` EN+DE and keeps them in the roster; a roster of a stage with a non-zero exit is kept and every roster is stopped (verified) at the next stage start.
- Orphans: at stage start and while a stage waits on a named holder every lease registry entry that is alive with its registered start (pid and start time), belongs to this checkout (lease directory below `<checkout>/.cache/`, command `<64 hex> <verb>` or a file of the checkout), has no agent session, started inside the window of a stopped try (stalled, stage-timeout, interrupted; 60 s grace), outside the working window of a person (`human.json`) and not in or below the current stage tree is stopped and verified; each reap is a bilingual log line, tail line and `containment` entry.
""")
print(len(rows),"files")
