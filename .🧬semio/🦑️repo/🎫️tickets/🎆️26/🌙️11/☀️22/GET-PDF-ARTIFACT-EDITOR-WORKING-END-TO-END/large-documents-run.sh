#!/bin/sh
cd "$(dirname "$0")"
export PATH="$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH"
log="$1"; shift
for attempt in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18; do
  sh native-cargo.sh test -p semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests --lib -- "$@" --test-threads=1 --nocapture > "🗑️generated/$log.log" 2>&1
  if grep -aq "failed to load manifest\|workspace.dependencies\|^error\[E0433\]\|^error: could not compile" "🗑️generated/$log.log" && ! grep -aq "natural-file\|controlled\|export/🦀️\|cursor/🦀️\|typed-operations/🦀️\|artifact-app-laws\|testing/🎟️caller" "🗑️generated/$log.log"; then sleep 90; else break; fi
done
