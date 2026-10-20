#!/bin/sh
# A/B benchmark: live vs fixed on the same sample, sequential. usage: ab.sh N SCRATCH_DIR_SHORT [order...]
# CPU seconds are the comparable metric under fleet load; scratch must be a SHORT path (closure manifest staging path limit 256).
HERE="$(cd "$(dirname "$0")" && pwd)"
N=${1:-150}; S=${2:-/private/tmp/show-perf-bench}; shift 2 2>/dev/null
ORDER=${*:-live fixed live fixed}
mkdir -p "$S"
for v in $ORDER; do
  if [ "$v" = fixed ]; then P=fixed; else P=; fi
  PATCHES=$P nice perl -e 'alarm 900; exec @ARGV' -- bun "$HERE/bench.ts" "$N" 0 observe "$S/run-$v" 2>&1 | grep -E 'EQUIV|^\{|rror' | cut -c1-520 | sed "s/^/[$v] /"
done
