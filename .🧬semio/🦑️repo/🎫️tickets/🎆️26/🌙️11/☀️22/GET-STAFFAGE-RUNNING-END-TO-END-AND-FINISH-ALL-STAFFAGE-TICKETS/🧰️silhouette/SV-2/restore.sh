#!/bin/zsh
# ⏪️ Puts the catalog file of plants back to the state before this repair (the copy in 🗑️generated/silhouette/SV-2/backup), so that the ops of `repair/<slug>.json` can be replayed on it: restore.sh <slug>…
source "${0:A:h}/env.sh"
for a in "$@"; do cp "$GEN/backup/$a.json" "$CAT/$a/🔣️.json" && echo "$a restored"; done
