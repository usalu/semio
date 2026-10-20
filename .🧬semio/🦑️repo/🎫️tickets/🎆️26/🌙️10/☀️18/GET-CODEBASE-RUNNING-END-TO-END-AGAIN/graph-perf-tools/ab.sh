#!/bin/sh
# usage: ab.sh N SCRATCH_DIR variant... ; variants: live | showperf | gp ; WARM=1 additionally runs a disk-warm pass per variant. Metric of record: cpuSeconds.
HERE="$(cd "$(dirname "$0")" && pwd)"
N=${1:-100}; S=${2:-/private/tmp/gp/ab}; shift 2 2>/dev/null
ORDER=${*:-live gp}
mkdir -p "$S"
for v in $ORDER; do
  VARIANT=$v nice perl -e 'alarm 900; exec @ARGV' -- bun "$HERE/gp-bench.ts" "$N" 0 observe "$S/run-$v" 2>&1 | grep -E 'EQUIV|^\{|rror' | cut -c1-520 | sed "s/^/[$v cold] /"
  if [ -n "$WARM" ]; then KEEP=1 NOTOP=1 VARIANT=$v nice perl -e 'alarm 900; exec @ARGV' -- bun "$HERE/gp-bench.ts" "$N" 0 observe "$S/run-$v" 2>&1 | grep -E '^\{|rror' | cut -c1-420 | sed "s/^/[$v warm] /"; fi
done
