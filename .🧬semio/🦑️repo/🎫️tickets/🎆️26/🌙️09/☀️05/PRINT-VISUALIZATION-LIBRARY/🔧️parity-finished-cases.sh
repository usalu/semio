#!/usr/bin/env bash
# 🧪 Runs `parity quick` per case for the cases the finished agents own, one at a time, so one
# case's timeout cannot hide the verdict of every case after it in the alphabet.
set -u
TEST="C:/git/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
CASES="${*:-annotation-placement catalog-coverage charts-bar-layout charts-box-violin charts-evaluation-curves charts-financial charts-heatmap-matrix charts-histogram-density charts-kpi-gauge charts-line-area charts-pie-donut charts-polar-radar charts-quadrant-table charts-scatter-trend charts-timeline composition-concat-inset coordinate-polar-ternary data-csv facet-layout format-number format-time guide-axis-ticks guide-legend mark-geometry probe-protocol scale-color scale-continuous scale-discrete scale-temporal shape-arc-pie shape-curves shape-links-ribbons shape-symbols transform-bin transform-stack transform-statistics}"
export SEMIO_CMD_BUDGET_MS=300000
cd "$TEST" || exit 1
for c in $CASES; do
  printf '%-28s ' "$c"
  out=$(bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print" --case "$c" 2>&1)
  line=$(printf '%s' "$out" | grep -a 'level=quick' | tail -1)
  if [ -n "$line" ]; then
    echo "$line"
    printf '%s' "$out" | grep -a 'parity failed' | sed 's/^/    /'
  else
    printf '%s' "$out" | grep -a -m 1 'error:' | cut -c1-160
    echo
  fi
done
