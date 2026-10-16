"""Install the staged split into the repo: refuses when the live target changed since the snapshot or a child path exists."""
import os, sys, json
sys.path.insert(0, os.path.dirname(__file__))
from pqslib import *

name = sys.argv[1]
T = TARGETS[name]
sd = stage_dir(name)
snap = json.loads(read(f"{W}/{name}/snap.json"))
live = sha(read(ROOT + "/" + T["file"]))
installed_marker = f"{W}/{name}/installed.json"
if os.path.exists(installed_marker):
    print("already installed; use --reinstall to overwrite staged files over previously installed ones")
    if "--reinstall" not in sys.argv:
        sys.exit(1)
    prev = json.loads(read(installed_marker))
    assert live == prev["after_sha"], "live parent differs from what this tool installed: peer edit, merge by hand"
else:
    assert live == snap["sha"], f"live target changed since snapshot ({live[:12]} vs {snap['sha'][:12]})"
files = []
for dp, dn, fn in os.walk(sd):
    for f in fn:
        p = os.path.join(dp, f)
        files.append(os.path.relpath(p, sd))
for rel in files:
    dst = os.path.join(ROOT, rel)
    if os.path.exists(dst) and rel != T["file"] and not os.path.exists(installed_marker):
        raise SystemExit(f"refusing to overwrite existing {rel}")
for rel in files:
    dst = os.path.join(ROOT, rel)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    write(dst, read(os.path.join(sd, rel)))
after = sha(read(ROOT + "/" + T["file"]))
write(installed_marker, json.dumps(dict(files=sorted(files), after_sha=after), ensure_ascii=False, indent=1))
print("installed", len(files), "files")
