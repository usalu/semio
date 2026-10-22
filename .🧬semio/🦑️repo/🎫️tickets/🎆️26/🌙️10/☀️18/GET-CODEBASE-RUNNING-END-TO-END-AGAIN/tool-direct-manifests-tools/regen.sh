#!/bin/sh
# 🔁️ Rebuilds the staged increment from a tree that already holds round 6, checks it against that tree and runs the self-test.
# usage: sh regen.sh [BASE_ROOT]   (default: the live checkout; pass an overlay to rehearse)
set -eu
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
nice -n 10 python3 -I mkliterals.py
if [ $# -gt 0 ]; then
  base=$1
  list=$(mktemp)
  (cd "$base" && nice -n 10 git ls-files -co --exclude-standard -z > "$list")
  nice -n 10 python3 -I make.py --root "$base" --list "$list"
  rm -f "$list"
  nice -n 10 python3 -I apply.py check --root "$base" || true
else
  nice -n 10 python3 -I make.py
  nice -n 10 python3 -I apply.py check || true
fi
nice -n 10 python3 -I selftest.py
nice -n 10 python3 -I law/checker.py --cases law/fixtures.json
