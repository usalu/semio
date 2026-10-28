#!/bin/sh
# usage: sh apply.sh check|apply|revert [--root DIR]   (all docs4 round-2 batches in order; revert runs in reverse order)
mode="$1"; shift
here="$(cd "$(dirname "$0")" && pwd)"
names="memo-serve plan-walk dashboard rust-lanes disk-memo sealed-capacity status"
[ "$mode" = revert ] && names="status sealed-capacity disk-memo rust-lanes dashboard plan-walk memo-serve"
status=0
for name in $names; do
  printf '%-14s ' "$name"
  python3 -I "$here/staged2/$name/stage.py" "$mode" "$@" | tail -1 || status=$?
done
exit $status
