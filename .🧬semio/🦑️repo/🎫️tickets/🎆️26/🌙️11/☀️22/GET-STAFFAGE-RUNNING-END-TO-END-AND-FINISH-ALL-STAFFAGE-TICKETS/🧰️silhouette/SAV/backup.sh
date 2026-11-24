#!/bin/zsh
# 💾️ Copies the catalog files of assets of batch SAV to 🗑️generated/silhouette/SAV/backup/<id>.json unless a backup exists (the state before this fixer): backup.sh <id>…
source "${0:A:h}/env.sh"
mkdir -p "$GEN/backup"
for a in "$@"; do f="$(ls "$S"/🗂️catalog/*/"$a"/🔣️.json)"; [[ -f "$GEN/backup/$a.json" ]] || cp "$f" "$GEN/backup/$a.json"; done
