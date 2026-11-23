#!/bin/sh
# 🔎 Compact findings of one validation: `sh rework-machinery-findings.sh <category>/<slug> [r3-validate arguments]` (W=<columns> widens the lines).
cd "$(dirname "$0")" || exit 2
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-validate.ts "$@" 2>&1 | grep -v "^ASSET volumes" | sed -E 's/^VOLUMES volumes-[a-z-]+ (1:[0-9]+ )?//; s/volumes-feature volumes\.//; s/ \(the smallest feature.*//; s/ Members meet at their joints.*//; s/ Join every part.*//; s/through touching solids, nor.*worst:/worst:/; s/touch(es)? neither the rest.*worst:/worst:/; s/reach(es)? neither a support.*worst:/worst:/' | cut -c1-${W:-360}
