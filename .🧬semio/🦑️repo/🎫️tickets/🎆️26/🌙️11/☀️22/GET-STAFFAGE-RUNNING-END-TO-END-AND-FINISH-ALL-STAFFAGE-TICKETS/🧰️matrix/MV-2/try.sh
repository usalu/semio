#!/bin/zsh
# 🧪️ Dry run of the repair of one plant: applies `ops/<id>.json` to a copy `$GEN/try/<id>.json` of the catalog file (or of the copy that is there, with KEEP=1) and runs the gate over every pose, clip instant and transition on the copy: try.sh <id> [gate flags]
BATCH=MV-2 source "${0:A:h}/../MR/env.sh"
a="$1"; shift; mkdir -p "$GEN/try"
cd "$T"
if [[ -n "$KEEP" ]]; then "$B" "$W/apply.ts" "$a" --in "$GEN/try/$a.json" --out "$GEN/try/$a.json"; else "$B" "$W/apply.ts" "$a" --out "$GEN/try/$a.json"; fi
timeout 1800 "$B" "$M/gate.ts" "$GEN/try/$a.json" "$@" > "$GEN/gate/$a.try.log" 2>&1
grep -E "HARD|hard findings|^    " "$GEN/gate/$a.try.log" | cut -c1-260 | awk '/^    /{n++; if(n<=6)print; next}{n=0;print}'
