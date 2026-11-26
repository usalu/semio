#!/bin/zsh
# 🧪️ Gate (hard findings) and guard (1:50) of trial documents in one grant: g.sh <json>…
source "${0:A:h}/env.sh"
for f in "$@"; do echo "== $f"; (cd $T && $B $M/gate.ts $f --twins 2>&1 | tail -1); (cd $S/📦️packages/🟦️typescript && $B $M/measure.ts $f --scales 50 2>&1 | head -1 | cut -c1-300); done
