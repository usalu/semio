#!/bin/sh
# bench2-g measurement sequence, one official run at a time, edits reverted byte-exact (suffix strip keeps peer edits).
# usage: [BENCH_QUIET=1] [BENCH_TOTAL_MB=16384] window.sh <tag> <stage...>   stages: noop edit peer concurrent-noop concurrent-edit
# BENCH_QUIET=1: coordinator-declared quiet window, no wait for gate-g; BENCH_TOTAL_MB: simulated host size (host-total.patch).
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
TAG=$1; shift
O="$C/🗑️generated/bench2-g/$TAG"
M="env -u SEMIO_SCRIPT_STALL_MS python3 -I $C/bench2-g-tools/measure.py"
R=/Users/ueli/Documents/semio
export BENCH_WINDOW=1
mkdir -p "$O"
cd $R || exit 1
for stage in "$@"; do
  echo "[$(date +%H:%M:%S)] stage $stage" >> "$O/chain.log"
  case $stage in
    noop) for p in course dashboard print; do $M noop --product $p --root $R --tool bazel --out "$O/noop" --prime --samples 3 --tag $TAG; done;;
    edit) for p in course dashboard print; do $M edit --product $p --root $R --tool bazel --out "$O/edit" --samples 1 --revert --tag $TAG; done;;
    noop-print) $M noop --product print --root $R --tool bazel --out "$O/noop" --prime --samples 3 --tag $TAG;;
    edit-course) $M edit --product course --root $R --tool bazel --out "$O/edit" --samples 1 --revert --tag $TAG;;
    peer) $M peer --product course --peer scheduling --root $R --tool bazel --out "$O/peer" --samples 1 --revert --tag $TAG
          $M peer --product course --peer unrelated-ts --root $R --tool bazel --out "$O/peer" --samples 1 --revert --tag $TAG;;
    concurrent-noop) $M concurrent --root $R --tool bazel --out "$O/concurrent" --mode noop;;
    concurrent-edit) $M concurrent --root $R --tool bazel --out "$O/concurrent" --mode edit;;
  esac
done
echo "[$(date +%H:%M:%S)] WINDOW-DONE $TAG" >> "$O/chain.log"
