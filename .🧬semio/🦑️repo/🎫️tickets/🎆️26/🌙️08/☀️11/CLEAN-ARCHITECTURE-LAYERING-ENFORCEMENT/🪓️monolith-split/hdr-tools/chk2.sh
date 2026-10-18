#!/bin/zsh
# 🔬️ chk2.sh <label>=<artifact-dir>... : like chk.sh with an explicit unique label per artifact (log hdr-chk-<label>.log, sandbox chk-<label>); runs sequentially.
cd /Users/ueli/Documents/semio
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
for spec in "$@"; do
  label="${spec%%=*}"; a="${spec#*=}"
  rm -rf "/Users/ueli/.cache/hdr/chk-$label-base" "/Users/ueli/.cache/hdr/chk-$label-combined"
  nice bun ./📜️script.ts new crates "$a/📦️packages/🦀️rust" --check-applied --samples=20 --sandbox=/Users/ueli/.cache/hdr/chk-$label > "$F/🗑️generated/hdr-chk-$label.log" 2>&1
  rc=$?
  echo "$label $a rc=$rc $(/usr/bin/grep -o '"onlyCombined": [0-9]*' "$F/🗑️generated/hdr-chk-$label.log" | tr '\n' ' ')"
done
