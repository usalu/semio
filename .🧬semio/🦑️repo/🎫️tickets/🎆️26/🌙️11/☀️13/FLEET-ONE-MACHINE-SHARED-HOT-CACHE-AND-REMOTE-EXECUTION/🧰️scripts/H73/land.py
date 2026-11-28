#!/usr/bin/env python3
"""H73 landing (slot): `land.py check` verifies every anchor and writes nothing; `land.py apply` takes the area locks
(caching-cargo, process; alphabetical), copies the new `🌱️seed` module from `stage/`, applies the schema edits and the anchored
patch once each (temp + rename) and leaves the locks HELD (typecheck + dangling-imports first, then `land.py release`)."""
import os,subprocess,sys,time
HERE=os.path.dirname(os.path.abspath(__file__));T=os.path.normpath(os.path.join(HERE,"..",".."))
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
UC=L+"⚡️caching/🦀️cargo/🗃️unit-cache";LAUNCH=L+"🏃️process/🦀️cargo/🚪️launcher/🟦️.ts"
LOCKS=["landing.lock.caching-cargo","landing.lock.process"]
mode=sys.argv[1]
def run(*a):return subprocess.run(a,capture_output=True,text=True)
if mode=="release":
    for name in reversed(LOCKS):
        p=os.path.join(T,name)
        try:
            if "H73" in open(os.path.join(p,"owner")).read():os.remove(os.path.join(p,"owner"));os.rmdir(p);print("released",name)
        except OSError as e:print("skip",name,e)
    raise SystemExit(0)
if mode=="check":
    r=run("python3",os.path.join(HERE,"patch.py"),"check",UC,LAUNCH);s=run("python3",os.path.join(HERE,"schema_edits.py"),UC,"check");print(r.stdout,r.stderr,"schema",s.returncode,s.stderr);raise SystemExit(r.returncode or s.returncode)
for name in LOCKS:
    p=os.path.join(T,name)
    try:os.mkdir(p)
    except FileExistsError:print("HELD",name,open(os.path.join(p,"owner")).read() if os.path.exists(os.path.join(p,"owner")) else "");raise SystemExit(3)
    open(os.path.join(p,"owner"),"w").write("H73 %s landing lane seed (schema, 🌱️seed module, fleet client, wrapper, launcher)\n"%time.strftime("%H:%M"))
r=run("python3",os.path.join(HERE,"patch.py"),"check",UC,LAUNCH);print(r.stdout,r.stderr)
if r.returncode:print("anchors failed, nothing written; locks held: run `land.py release`");raise SystemExit(1)
r=run("rsync","-a",os.path.join(HERE,"stage","🌱️seed")+"/",UC+"/🛰️fleet/🌱️seed/");print("rsync",r.returncode,r.stderr)
r=run("python3",os.path.join(HERE,"schema_edits.py"),UC,"check");print("schema",r.stdout,r.stderr)
if r.returncode:print("schema anchors failed; locks held: run `land.py release`");raise SystemExit(1)
r=run("python3",os.path.join(HERE,"schema_edits.py"),UC);print(r.stdout,r.stderr)
if r.returncode:raise SystemExit(1)
r=run("python3",os.path.join(HERE,"patch.py"),"apply",UC,LAUNCH);print(r.stdout,r.stderr)
raise SystemExit(r.returncode)
