#!/bin/sh
# usage: redo.sh <batch>...: reverts the batch on the mirror (when applied), rebuilds it from the sources, applies it again and runs the docheck and the law
here=$(cd "$(dirname "$0")" && pwd)
. "$here/env.sh"
for b in "$@"; do
  if [ -f "$here/staged/$b/stage.py" ]; then python3 -I "$here/staged/$b/stage.py" revert --root "$MR" >/dev/null 2>&1 || true; fi
  python3 "$here/batches/$b.py" || exit 1
  python3 -I "$here/staged/$b/stage.py" apply --root "$MR" | tail -1
  python3 -I "$here/docheck.py" "$here/staged/$b" "$MR" | tail -2
done
sh "$here/../red-g-tools/mrun.sh" sc-a plain "$L/🏗️build/🧪️tests/🧬️policy-literals/🟦️.ts"
