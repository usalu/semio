#!/bin/zsh
# 🔬️ chk.sh <artifact-dir>... : runs `new crates --check-applied` per artifact, logs to hdr-chk-<n>.log and prints the verdict line.
cd /Users/ueli/Documents/semio
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
for a in "$@"; do
  tag=$(echo "$a" | sed 's|.*artifacts/||; s|[^A-Za-z0-9]|_|g')
  [ -z "$tag" ] && tag=$(echo "$a" | md5 | cut -c1-8)
  nice bun ./📜️script.ts new crates "$a/📦️packages/🦀️rust" --check-applied --sandbox=/Users/ueli/.cache/hdr/chk-$tag > "$F/🗑️generated/hdr-chk-$tag.log" 2>&1
  rc=$?
  echo "$a rc=$rc $(/usr/bin/grep -o '"onlyCombined": [0-9]*' "$F/🗑️generated/hdr-chk-$tag.log" | tr '\n' ' ')"
done
