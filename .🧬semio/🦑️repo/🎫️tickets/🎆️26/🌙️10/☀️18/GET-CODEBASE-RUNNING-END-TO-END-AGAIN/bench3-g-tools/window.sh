#!/bin/sh
# bench3-g measurement stages, one official run at a time, edits reverted byte-exact (suffix strip keeps peer edits).
# usage: window.sh <tag> <stage...>   stages: noop-course noop-print noop-dashboard edit-course
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TAG=$1; shift
O="$C/🗑️generated/bench3-g/$TAG"
M="env -u SEMIO_SCRIPT_STALL_MS python3 -I $C/bench3-g-tools/measure.py"
R=/Users/ueli/Documents/semio
export BENCH_WINDOW=1
mkdir -p "$O"
cd $R || exit 1
for stage in "$@"; do
  echo "[$(date +%H:%M:%S)] stage $stage" >> "$O/chain.log"
  case $stage in
    noop-course) $M noop --product course --root $R --tool bazel --out "$O/noop" --prime --samples 3 --tag $TAG;;
    noop-print) $M noop --product print --root $R --tool bazel --out "$O/noop" --prime --samples 3 --tag $TAG;;
    noop-dashboard) $M noop --product dashboard --root $R --tool bazel --out "$O/noop" --prime --samples 3 --tag $TAG;;
    edit-course) $M edit --product course --root $R --tool bazel --out "$O/edit" --samples 1 --revert --tag $TAG;;
  esac
done
echo "[$(date +%H:%M:%S)] WINDOW-DONE $TAG" >> "$O/chain.log"
