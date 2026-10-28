#!/bin/sh
# usage: sh apply.sh check|apply|revert [--root DIR]   (all docs5 batches in order, the three test batches last-to-first so that the page reads run, envelope, scratch; revert runs in reverse order)
mode="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
names="test-scratch test-envelope test-run providers farm-memo capacity-lines promote verdict readme-tests"
[ "$mode" = revert ] && names="readme-tests verdict promote capacity-lines farm-memo providers test-run test-envelope test-scratch"
status=0
for name in $names; do
  printf '%-16s ' "$name"
  python3 -I "$here/staged/$name/stage.py" "$mode" "$@" | tail -1 || status=$?
done
exit $status
