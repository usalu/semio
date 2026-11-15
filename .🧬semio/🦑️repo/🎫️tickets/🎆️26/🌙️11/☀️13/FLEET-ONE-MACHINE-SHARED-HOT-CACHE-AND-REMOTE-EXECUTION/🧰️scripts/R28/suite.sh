#!/bin/zsh
# usage: suite.sh <scratch dir> [before|after]  -- runs every touched-module test file; "after" serves the edited files from the overlay <scratch>/ov (built by build-overlay.py), "before" runs the live tree
HERE=${0:A:h}; S=$1; MODE=${2:-after}; LIVE=/Users/Shared/semio
BUN=/Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
G="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph"
DIRS=("$G/🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🧪️tests" "$G/🕰️generations/✂️cutoff/🧪️tests" "$G/🕰️generations/✂️cutoff/🧪️tests/🧾️vouch" "$G/🧮️derived-nodes/🧪️tests" "$G/🕰️generations/🧪️tests" "$G/🕰️generations/🔌️load/🧪️tests")
OVERLAID=("$G/🧮️derived-nodes/🧩️closure-kinds/📒️manifest/🧪️tests" "$G/🕰️generations/✂️cutoff/🧪️tests" "$G/🕰️generations/✂️cutoff/🧪️tests/🧾️vouch")
for d in $DIRS; do
  if [[ $MODE == after && ${OVERLAID[(Ie)$d]} -eq 0 ]]; then
    out=$(R5_RUN_IN=$LIVE zsh "$HERE/run-overlay.sh" "$S/ov" "$d" 🟦️.ts 2>&1)
  elif [[ $MODE == after ]]; then
    (cd "$LIVE/$d" 2>/dev/null && find . -type f | while read f; do [ -e "$S/ov/$d/$f" ] || { mkdir -p "$(dirname "$S/ov/$d/$f")"; cp "$f" "$S/ov/$d/$f"; }; done)
    mkdir -p "$S/ov/$d"; printf 'preload = ["%s/overlay-preload.ts"]\n' "$HERE" > "$S/ov/$d/bunfig.toml"
    out=$(zsh "$HERE/run-overlay.sh" "$S/ov" "$d" 🟦️.ts 2>&1)
  else
    [[ -d "$LIVE/$d" ]] || { echo "SKIP $d (not in the live tree)"; continue; }
    out=$(cd "$LIVE/$d" && env -u SEMIO_FLEET_GROUP -u SEMIO_REMOTE_CACHE_TOKEN BUN_RUNTIME_TRANSPILER_CACHE_PATH=0 $BUN test ./🟦️.ts --timeout 180000 2>&1)
  fi
  echo "== $MODE ${d#$G/}"; echo "$out" | grep -E "^\(fail\)|^ *[0-9]+ (pass|fail)" | cut -c1-160
done
