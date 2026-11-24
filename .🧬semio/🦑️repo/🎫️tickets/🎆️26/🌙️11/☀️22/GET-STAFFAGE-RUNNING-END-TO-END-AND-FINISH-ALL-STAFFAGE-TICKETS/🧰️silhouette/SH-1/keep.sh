#!/bin/zsh
# ✅️ Writes the catalog file of people from the backup of the session start and the ops `ops/<slug>.json` (the same result as the ops on the catalog file, without what a dropped trial left): keep.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do [[ -f "$GEN/backup/$a.json" ]] || cp "$CAT/$a/🔣️.json" "$GEN/backup/$a.json"; (cd "$W" && "$B" apply.ts "$a" --in "$GEN/backup/$a.json") | tail -1; done
