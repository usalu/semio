#!/bin/sh
# usage: measure.sh [step ...]   steps: infer-live infer-new scale-live-1200 scale-new-1200 scale-new-3735 scale-new-2400 scale-live-2400 (default all)
# before/after measurements, one heavy process at a time, niced, guarded by the load average; results in OUT (default /private/tmp/graph-mem/measure)
HERE=$(cd "$(dirname "$0")" && pwd); STEPS="$*"
OUT=${OUT:-/private/tmp/graph-mem/measure}; mkdir -p "$OUT"
STAGED="$HERE/staged/bench.json"
run() { name=$1; shift; case " ${STEPS:-all} " in *" all "*|*" $name "*) ;; *) return;; esac; sh "$HERE/guard.sh"; env "$@" nice -n 10 bun "$HERE/${BENCH}.ts" 2>&1 | grep -v "input-precision" | cut -c1-6000 > "$OUT/$name.txt"; }
export EXCLUDE="🧑‍💻dev"
BENCH=bench;       run infer-live  N=394 MIX=0.15 SERIES=1 EVERY=50 RSSMAX_MB=3800 WALLMAX_S=600 SCRATCH=/private/tmp/graph-mem/m-live
BENCH=bench;       run infer-new   VARIANT="$STAGED" N=394 MIX=0.15 SERIES=1 EVERY=50 RSSMAX_MB=3800 WALLMAX_S=600 SCRATCH=/private/tmp/graph-mem/m-new
BENCH=scale-bench; run scale-live-1200 S=1200 SCRATCH=/private/tmp/graph-mem/ms-live
BENCH=scale-bench; run scale-new-1200  VARIANT="$STAGED" S=1200 SCRATCH=/private/tmp/graph-mem/ms-new
BENCH=scale-bench; run scale-new-3735  VARIANT="$STAGED" S=3735 RSSMAX_MB=3900 SCRATCH=/private/tmp/graph-mem/ms-new-full
BENCH=scale-bench; run scale-new-2400  VARIANT="$STAGED" S=2400 RSSMAX_MB=3900 SCRATCH=/private/tmp/graph-mem/ms-new-2400
BENCH=scale-bench; run scale-live-2400 S=2400 RSSMAX_MB=3900 SCRATCH=/private/tmp/graph-mem/ms-live-2400
echo done > "$OUT/done"
