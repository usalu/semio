#!/bin/sh
# usage: sh apply.sh check|apply|revert [--root DIR]   (all docs4 batches in order; revert runs in reverse order)
mode="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
names="graph-memo plan-reuse action-env lock-order capacity rust-path measuring readme-bazel g34"
[ "$mode" = revert ] && names="g34 readme-bazel measuring rust-path capacity lock-order action-env plan-reuse graph-memo"
status=0
for name in $names; do
  printf '%-14s ' "$name"
  python3 -I "$here/staged/$name/stage.py" "$mode" "$@" | tail -1 || status=$?
done
exit $status
