#!/bin/sh
# 🔄️ Puts the base bytes of a rebuilt batch over its files in a mirror (the batch was rebuilt, its old applied state is stale): sh rebase_mirror.sh <batch> <mirror root>   (run through mirror.py run <name> -- sh ...)
here=$(cd "$(dirname "$0")" && pwd)
python3 -I - "$here/staged/$1" "$2" <<'PY'
import json, os, shutil, sys
batch, root = sys.argv[1], sys.argv[2]
for rel, row in json.load(open(os.path.join(batch, "manifest.json"), encoding="utf8")).items():
    target = os.path.join(root, rel)
    if row["base"] is None:
        if os.path.exists(target): os.remove(target)
    else:
        src = os.path.join(batch, "base", row["file"])
        if os.path.exists(src): shutil.copyfile(src, target)
PY
