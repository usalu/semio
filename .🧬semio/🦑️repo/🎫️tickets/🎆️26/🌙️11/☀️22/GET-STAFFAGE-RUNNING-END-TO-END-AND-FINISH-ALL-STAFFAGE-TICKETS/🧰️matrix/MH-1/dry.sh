#!/bin/zsh
# 🧯️ Dry run of the ops of an asset on a copy: fit.ts with ops/<id>.json into $GEN/try/<id>.json, the listing in $GEN/try/<id>.fit.log, printed without the spans: dry.sh <id>
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"
"$W/q.sh" 1000 1800 "$T" "$B" "$W/fit.ts" "$1" --ops "$W/ops/$1.json" --out "$GEN/try/$1.json" > "$GEN/try/$1.fit.log" 2>&1
sed -E 's/span \{[^}]*\}//; /^written/d' "$GEN/try/$1.fit.log" | cut -c1-${COLS:-220}
