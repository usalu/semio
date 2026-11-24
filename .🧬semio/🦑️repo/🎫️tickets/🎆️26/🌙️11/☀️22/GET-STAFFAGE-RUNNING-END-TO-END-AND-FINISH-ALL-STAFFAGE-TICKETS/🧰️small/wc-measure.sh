#!/bin/sh
# 📐️ Measures one candidate document of the walking commuter, one process after the other: the three-views sweep, the probe of the pinned guards and of the print rule, and the silhouette measurement. `wc-measure.sh <candidate json> [pinned capsule id]`
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$S" || exit 1
timeout 600 "$BUN" "$U/🧰️three-views/sweep-2.ts" walking-commuter --file "$1" 2>&1 | grep -v '^ \|^$\|^[0-9]* |' | tail -${SWEEP_TAIL:-3} | cut -c1-260
timeout 600 "$BUN" "$HERE/wc-probe.ts" "$1" ${2:-shoe-toe-l} 2>&1 | grep '^stride\|^walk\|^print\|Error:' | cut -c1-420
timeout 600 "$BUN" "$U/🧰️three-views/silhouette-2.ts" walking-commuter --file "$1" 2>&1 | grep '^walking-commuter\|Error:' | grep -v ' box ' | awk '{print $2, $3, $5, $7, $NF}' | tr '\n' ';' ; echo
