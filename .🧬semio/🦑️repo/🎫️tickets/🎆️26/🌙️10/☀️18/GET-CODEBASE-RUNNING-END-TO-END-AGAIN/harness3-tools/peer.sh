#!/bin/sh
# 🧪 harness3: the peer-edit scenario chain on course:build under the declared 16 GB machine (official runs, sequential).
# Usage: sh peer.sh [name]  -> output below 🗑️generated/harness3/<name>, summary.md at the end (python3 -I harness3-tools/render.py peer <dir>)
HERE=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
C=$(dirname "$HERE")
O="$C/🗑️generated/harness3/${1:-peer1}"
mkdir -p "$O"
BENCH_ENV='{"SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES":"16384","SEMIO_HOST_CAPACITY_CORES":"8"}' exec python3 -I "$C/peer-g-tools/scenario.py" run "$O" \
  control:course control:course \
  peer:p1:course:graph control:course peer:p1:course:bazel control:course \
  peer:p3:course:graph control:course peer:p3:course:bazel control:course \
  peer:p2:course:graph control:course
