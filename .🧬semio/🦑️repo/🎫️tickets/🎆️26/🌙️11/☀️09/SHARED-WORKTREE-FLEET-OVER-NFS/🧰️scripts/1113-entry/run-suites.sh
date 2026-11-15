#!/bin/bash
# usage: run-suites.sh <module-root> [substring]  -- runs EVERY test file of the module (all 🧪️tests directories) with SEMIO_* unset, prints pass/fail per file
M="$1"; ONLY="$2"
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
UNSET=$(env | grep '^SEMIO_' | cut -d= -f1 | sed 's/^/-u /' | tr '\n' ' ')
cd "$M" || exit 1
find . -path '*🧪️tests*' -name '*.ts' -not -path '*node_modules*' | sort | while read -r f; do
  case "$f" in *🌪️harness.ts|*🌪️sweep.ts) continue;; esac
  [ -n "$ONLY" ] && [[ "$f" != *"$ONLY"* ]] && continue
  d=$(dirname "$f"); b=$(basename "$f")
  out=$(cd "$M/$d" && env $UNSET "$BUN" test "./$b" --timeout 180000 2>&1)
  echo "$f: $(echo "$out" | grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' ' | tr '\n' ' ')"
  echo "$out" | grep -E "^\(fail\)|^error:" | head -8
done
