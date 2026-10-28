#!/bin/sh
# 🛂️ Rebuilds the staged batch and re-applies it to the lane's mirror: revert the old build, run mkbatch, apply the new one. `sh cycle.sh [mkbatch args]`
set -eu
here=$(cd "$(dirname "$0")" && pwd)
M=${BC_MIRROR:-/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/sm/bc}
python3 -I "$here/staged/acceptance/stage.py" revert --root "$M" >/dev/null || true
python3 -I "$here/mkbatch.py" "$@" | tail -3
python3 -I "$here/staged/acceptance/stage.py" apply --root "$M" >/dev/null
echo "cycle OK"
