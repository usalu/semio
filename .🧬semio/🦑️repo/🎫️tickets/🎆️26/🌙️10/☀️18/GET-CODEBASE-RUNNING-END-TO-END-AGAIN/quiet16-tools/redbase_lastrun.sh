#!/bin/sh
# 🔴 quiet16: runs law files of a mirror with batch `lastrun` against the BASE forecast (the forecast rule, the policy type, schema and policy with the two knobs, the four policy-carrying fixtures and the estimates oracle swapped to their base bytes = `hold`), keeping the regenerated oracle fixtures and the changed laws, then swaps the batch back: sh redbase_lastrun.sh <mirror root> <law path>...
here=$(cd "$(dirname "$0")" && pwd); root=${1:?root}; shift
python3 -I - "$here" "$root" base <<'PY'
import json,os,shutil,sys
here,root,which=sys.argv[1:4];stage=os.path.join(here,"staged","lastrun");m=json.load(open(os.path.join(stage,"manifest.json"),encoding="utf8"))
for index,(rel,row) in enumerate(m.items()):
    if index<=8:shutil.copyfile(os.path.join(stage,"base",row["file"]),os.path.join(root,rel))
print("[DEBUG] base forecast in place")
PY
(cd "$root" && bun test "$@" 2>&1 | grep -a -E "^\(fail\)|^ *[0-9]+ (pass|fail)|Ran ")
python3 -I "$here/staged/lastrun/stage.py" check --root "$root" >/dev/null; python3 -I - "$here" "$root" <<'PY'
import json,os,shutil,sys
here,root=sys.argv[1:3];stage=os.path.join(here,"staged","lastrun");m=json.load(open(os.path.join(stage,"manifest.json"),encoding="utf8"))
for index,(rel,row) in enumerate(m.items()):
    if index<=8:shutil.copyfile(os.path.join(stage,"files",row["file"]),os.path.join(root,rel))
print("[DEBUG] batch restored")
PY
