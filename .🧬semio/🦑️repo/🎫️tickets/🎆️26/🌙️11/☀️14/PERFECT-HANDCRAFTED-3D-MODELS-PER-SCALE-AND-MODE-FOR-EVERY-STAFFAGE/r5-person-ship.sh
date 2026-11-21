#!/bin/sh
# 🚢 Ships one or both round 5 human assets of this agent: rebuild and commit through the asset's run script, measure every scale against the anthropometric references, run the product volume check and the whole-asset validation. Usage: sh r5-person-ship.sh [umbrella] [white-cane]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for a in "$@"; do
  W=r5-person-with-$a; G="🗑️generated/$W"
  case $a in umbrella) M="--stature 1.70 --sole 0.022 --cloth 0.02 --hem 0.11 --calf 0.13";; white-cane) M="--stature 1.68 --sole 0.022 --arm arm --cloth 0.01 --hem 0.08";; esac
  ./$W-run.sh --commit > "$G/run.log" 2>&1; echo "== $a"; grep -E "VALID|invalid" "$G/run.log"
  for s in 50 100 200 500; do echo "### 1:$s"; "$BUN" r5-person-measure.ts humans/person-with-$a --scale $s $M; done > "$G/measure.md"
  grep -E "###|[3-9]\.[0-9] %|[1-9][0-9]\.[0-9] %" "$G/measure.md"
  "$BUN" volumes-check.ts humans/person-with-$a --product > "$G/check.log" 2>&1; grep -E "FAIL:|PASS:" "$G/check.log"; grep -oE "!areaDeviation:[a-z-]+=[0-9.]{5}" "$G/check.log"; grep -E "^FAIL" "$G/check.log" | cut -c1-60
  "$BUN" r3-validate.ts humans/person-with-$a --volumes | tail -2
done
