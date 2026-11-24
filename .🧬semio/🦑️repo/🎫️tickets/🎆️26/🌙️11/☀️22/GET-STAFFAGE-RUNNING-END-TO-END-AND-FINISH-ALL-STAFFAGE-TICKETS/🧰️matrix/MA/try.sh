#!/bin/zsh
# 🧪️ One dry run of the repair of an asset on a copy: `refit.sh`, then the gate of the copy over every pose, clip instant and transition and the silhouette, validation and three-views sweep of the backup and the copy: try.sh <id> [tag]
BATCH=MA source "${0:A:h}/../MR/env.sh"
a="$1"; tag="${2:-try}"
[[ -n "$NOFIT" ]] || "$W/refit.sh" "$a"
(cd "$T" && timeout 1500 "$B" "$M/gate.ts" "$GEN/try/$a.json" --twins > "$GEN/gate/$a.$tag.log" 2>&1)
grep -E "HARD|^hard" "$GEN/gate/$a.$tag.log" | cut -c1-${CUT:-240}
[[ -n "$NOMEASURE" ]] || (cd "$S/📦️packages/🟦️typescript" && timeout 900 "$B" "$M/measure.ts" "$GEN/backup/$a.json" "$GEN/try/$a.json" 2>&1 | cut -c1-230)
