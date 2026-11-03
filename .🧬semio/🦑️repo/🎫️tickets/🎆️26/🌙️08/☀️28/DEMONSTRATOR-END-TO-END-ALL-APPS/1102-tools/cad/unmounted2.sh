#!/bin/bash
# usage: unmounted2.sh <plugin dir>; lists tests/*/🦀️.rs not mounted by path/include and whose dir name is referenced nowhere
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
python3 -I $C/1102-tools/cad/unmounted.py "$1" | while read -r f; do
  d=$(basename "$(dirname "$f")")
  case "$d" in mutate-*|*mutate-*|solves-*|io-*|export-*|simulate-*) continue;; esac
  n=$(/usr/bin/grep -rlF --include='*.rs' --include='Cargo.toml' --include='*.ts' --include='*.json' "$d" "$1" 2>/dev/null | /usr/bin/grep -v "/🧪️tests/$d/" | wc -l | tr -d ' ')
  echo "$n refs  $f"
done
