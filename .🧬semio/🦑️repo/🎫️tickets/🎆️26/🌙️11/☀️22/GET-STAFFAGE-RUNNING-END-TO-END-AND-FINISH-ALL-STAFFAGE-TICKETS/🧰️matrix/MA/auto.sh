#!/bin/zsh
# 🤖️ The automatic pass of one or more assets on trial copies, nothing of the catalog is written: proposals for the bones the first gate reports (`propose.sh`, skipped with KEEP_OPS=1), the settle loop that keeps silhouette, validation and three-views sweep as good as the backup (`settle.ts`), then the gate of the copy; one summary block per asset in `$GEN/auto/<id>.log`: auto.sh <id>…
BATCH=MA source "${0:A:h}/../MR/env.sh"
mkdir -p "$GEN/auto"
for a in "$@"; do
  {
    [[ -n "$KEEP_OPS" ]] || "$W/propose.sh" "$a"
    (cd "$S/📦️packages/🟦️typescript" && BATCH=MA timeout 5400 "$B" "$W/settle.ts" "$a" 2>&1 | cut -c1-400)
    NOFIT=1 NOMEASURE=1 CUT=260 "$W/try.sh" "$a" auto
  } > "$GEN/auto/$a.log" 2>&1
  echo "$a: before $(tail -1 "$GEN/gate/$a.0.log") | after $(tail -1 "$GEN/auto/$a.log") | $(grep -c 'made strict' "$GEN/auto/$a.log") settle steps, $(grep -c 'left open' "$GEN/auto/$a.log") left open"
done
