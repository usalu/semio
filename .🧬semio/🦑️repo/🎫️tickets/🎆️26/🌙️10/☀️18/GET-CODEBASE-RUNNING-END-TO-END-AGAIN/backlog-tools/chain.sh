#!/bin/sh
# 🔗 [backlog] applies batch ids in order onto a mirror (land.py), stops at the first refusal. usage: chain.sh <mirror> <id>...
M="${TMPDIR%/}/sm/$1"; shift; T="$(cd "$(dirname "$0")" && pwd)"
for b in "$@"; do
  out=$(python3 -I "$T/land.py" "$b" apply --root "$M" 2>&1); rc=$?
  if [ $rc -ne 0 ]; then echo "FAIL $b rc=$rc"; printf '%s\n' "$out" | tail -8 | cut -c1-220; exit $rc; fi
  echo "ok   $b: $(printf '%s\n' "$out" | tail -1 | cut -c1-100)"
done
