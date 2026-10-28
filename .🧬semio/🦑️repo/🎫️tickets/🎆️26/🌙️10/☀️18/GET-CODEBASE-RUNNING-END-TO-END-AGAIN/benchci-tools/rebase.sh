#!/bin/sh
# 🛂️ Rebuilds batch `acceptance` on the CURRENT live files in one go (the hot files are the bench command, the bench tests, the operations catalogue, the README and the help snapshot): builds the batch without a help snapshot, applies it to the lane's mirror (created when missing), regenerates the help snapshot there with the real renderer, rebuilds with it and applies again. `sh rebase.sh` then `sh staged/acceptance/verify.sh <mirror>`; an anchor that moved stops mkbatch with exit 2 and names it.
set -eu
here=$(cd "$(dirname "$0")" && pwd)
tools=$(cd "$here/.." && pwd)
M=${BC_MIRROR:-$(python3 -I -c 'import os,tempfile;print(os.path.join(os.environ.get("SEMIO_MIRROR_HOME") or os.path.join(tempfile.gettempdir(),"sm"),"bc"))')}
[ -d "$M" ] || python3 -I "$tools/mirror-tools/mirror.py" create bc --only library process launcher .github
python3 -I "$here/mkbatch.py" | tail -1
python3 -I "$here/forceapply.py" "$M"
python3 -I "$here/helpgen_run.py" "$M" "$here/help-snapshot.json"
python3 -I "$here/mkbatch.py" --help-from "$here/help-snapshot.json" | tail -1
python3 -I "$here/forceapply.py" "$M"
echo "rebase OK: sh $here/staged/acceptance/verify.sh $M"
