#!/bin/zsh
# 🏗️ Rebuilds the catalog file of people from the state before this repair (the backup of 2026-11-23 16:30 in 🗑️generated/silhouette/SH-2/backup) plus their repair ops, so a taken-back op leaves no trace. rebuild.sh <slug>…
source "${0:A:h}/env.sh"; cd "$W"
for a in "$@"; do cp "$GEN/backup/$a.json" "$CAT/$a/🔣️.json" && ./apply.sh "$a"; done
