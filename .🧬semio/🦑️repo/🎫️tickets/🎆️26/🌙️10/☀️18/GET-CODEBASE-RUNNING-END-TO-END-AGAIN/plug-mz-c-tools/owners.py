#!/usr/bin/env python3
"""usage: owners.py DIR FILE... : prints the crates (Cargo.toml dirs under DIR) whose #[path] module tree mounts each FILE (relative paths from the repo root)."""
import os,re,sys,subprocess
os.chdir("/Users/ueli/Documents/semio")
d=sys.argv[1]; want=[os.path.realpath(t) for t in sys.argv[2:]]
sys.argv=[sys.argv[0],"/dev/null"]
exec(open(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/plug-mz-c-tools/modscan2.py").read().split("tot=0")[0])
for cf in subprocess.run(["rg","--files","-g","Cargo.toml",d],capture_output=True,text=True).stdout.split("\n"):
    if not cf: continue
    t=open(cf).read()
    if "[package]" not in t: continue
    seen=set()
    for k,tp in targets(cf,t)[:1]: walk(tp,seen)
    seen={os.path.realpath(x) for x in seen}
    for x in want:
        if x in seen: print(re.search(r'name\s*=\s*"([^"]+)"',t).group(1),"mounts",x[-70:])
