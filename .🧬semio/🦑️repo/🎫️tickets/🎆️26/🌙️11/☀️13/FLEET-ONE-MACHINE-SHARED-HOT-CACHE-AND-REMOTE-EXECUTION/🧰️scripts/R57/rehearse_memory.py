#!/usr/bin/env python3
"""R57 rehearsal: runs the staged footprint module and its staged tests outside the worktree. Relative imports that leave the footprint directory are rewritten to absolute paths of the live tree; the staged module, schema and tests keep their own relative layout inside the scratch directory. Usage: rehearse.py <stage dir with base/ and work/> <bun test filter or empty>"""
import os,re,sys,shutil,subprocess
R="/Users/ueli/Documents/semio"
F=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory"
BUN=os.path.expanduser("~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun")
stage=sys.argv[1];variant="base" if os.environ.get("R57_BASE")=="1" else "work";flt=sys.argv[2] if len(sys.argv)>2 else ""
run=os.path.join(stage,"run");shutil.rmtree(run,ignore_errors=True)
assert os.path.realpath(run).startswith(os.path.realpath(stage))
def fix(text,origin,keep):
    def sub(m):
        spec=m.group(2)
        if not (spec.startswith("../") or spec.startswith("./")) or spec in keep:return m.group(0)
        return m.group(1)+os.path.normpath(os.path.join(origin,spec))+m.group(3)
    return re.sub(r'((?:from|import)\s*\(?\s*")([^"]+)(")',sub,text)
for src_path,rel,keep,origin in [(os.path.join(stage,variant,"🟨️.mjs"),"🟨️.mjs",{"./🧬️schema/🔣️.json"},F),(os.path.join(stage,"laws-standalone.ts"),"🧪️tests/🟦️.ts",{"../🟨️.mjs"},F+"/🧪️tests")]:
    src=open(src_path,encoding="utf8").read()
    out=os.path.join(run,rel);os.makedirs(os.path.dirname(out),exist_ok=True)
    open(out,"w",encoding="utf8").write(fix(src,origin,keep))
os.makedirs(run+"/🧬️schema",exist_ok=True);shutil.copy(os.path.join(stage,variant,"🧬️schema/🔣️.json"),run+"/🧬️schema/🔣️.json")
env={k:v for k,v in os.environ.items() if not k.startswith(("SEMIO_FLEET_","SEMIO_REMOTE_CACHE_"))}
args=[BUN,"test","./🟦️.ts","--timeout","180000"]+(["-t",flt] if flt else [])
sys.exit(subprocess.run(args,cwd=run+"/🧪️tests",env=env).returncode)
