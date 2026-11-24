#!/bin/zsh
# ⏪️ Puts the catalog file of a vehicle back to its state before this batch (the copy in 🗑️generated/silhouette/R3-V/pristine), for a repair whose ops changed in a way a second apply does not undo; the ops are applied anew afterwards: restore.sh <id>
source "${0:A:h}/env.sh"
[[ -f "$GEN/pristine/$1.json" ]] || { echo "no pristine copy of $1"; exit 1 }
cp "$GEN/pristine/$1.json" "$CAT/$1/🔣️.json" && echo "$1 restored"
