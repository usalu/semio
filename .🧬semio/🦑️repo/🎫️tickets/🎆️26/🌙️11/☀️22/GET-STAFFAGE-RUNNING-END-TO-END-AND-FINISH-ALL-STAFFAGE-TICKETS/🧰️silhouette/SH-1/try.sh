#!/bin/zsh
# 🧪️ Tries named steps on one person and scale without touching the catalog file: the ops file is saved first (`🗑️generated/…/try/<slug>.json`), `keep.sh <slug>` writes the catalog file from the backup and the ops, `drop.sh <slug>` takes the ops back. try.sh <slug> <scale> < steps
source "${0:A:h}/env.sh"
mkdir -p "$GEN/try"; [[ -f "$W/ops/$1.json" ]] || echo "[]" > "$W/ops/$1.json"
cp "$W/ops/$1.json" "$GEN/try/$1.json"
NOAPPLY=1 "$W/hand.sh" "$1" "$2"
