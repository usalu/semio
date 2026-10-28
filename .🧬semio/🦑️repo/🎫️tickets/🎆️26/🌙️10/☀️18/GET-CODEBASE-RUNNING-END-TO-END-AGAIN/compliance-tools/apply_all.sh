#!/bin/sh
# 🏭️ Applies every compliance batch to a root in order and verifies it: sh apply_all.sh <root> [apply|revert]   (a mirror root: run through mirror.py run cp-main -- sh apply_all.sh <root>)
set -e
here=$(cd "$(dirname "$0")" && pwd)
root=$1
verb=${2:-apply}
for b in fwprocess fwmodules libother libtools libbuild libgraph lang names; do
  [ -f "$here/staged/$b/stage.py" ] || continue
  python3 -I "$here/staged/$b/stage.py" "$verb" --root "$root" | tail -1
done
