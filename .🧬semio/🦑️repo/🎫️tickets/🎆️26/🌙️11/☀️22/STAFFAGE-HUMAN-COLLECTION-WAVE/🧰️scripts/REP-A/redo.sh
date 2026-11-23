#!/bin/zsh
# 🔁️ People whose automatic ops broke the three-views guard: prune.ts removes the breaking ops, the catalog file is written again, then the gates and the three-views findings are printed. redo.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do
  (cd "$W" && $B prune.ts $a "$GEN/base/$a.json") > "$GEN/[DEBUG]-prune-$a.log" 2>&1; tail -1 "$GEN/[DEBUG]-prune-$a.log"
  "$W/ship.sh" $a
  (cd "$W" && $B views.ts "$CAT/$a/🔣️.json") | tail -1
done
