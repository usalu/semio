#!/bin/zsh
# 🧹 Fleet disk guard: every 120 s remove cargo incremental session dirs untouched for 10+ min under the shared build dir;
# below 40 GB free also remove per-crate hash dirs older than 6 h that have a newer sibling (unreferenced fingerprints).
setopt nullglob
B="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/prune-incremental.txt"
while true; do
  n=0; h=0
  for d in $(find "$B" -maxdepth 3 -type d -name incremental 2>/dev/null); do
    for s in "$d"/*; do [ -d "$s" ] || continue; if [ -z "$(find "$s" -mmin -10 -print -quit 2>/dev/null)" ]; then rm -rf "$s" && n=$((n+1)); fi; done
  done
  freegb=$(df -k / | tail -1 | awk '{printf "%d",$4/1048576}')
  if [ "$freegb" -lt 40 ]; then
    for c in "$B"/*/build/*/ "$B"/build/*/; do
      [ -d "$c" ] || continue
      newest=$(ls -1t "$c" 2>/dev/null | head -1)
      for x in $(find "$c" -mindepth 1 -maxdepth 1 -type d -mmin +360 2>/dev/null); do
        [ "$(basename "$x")" = "$newest" ] && continue
        rm -rf "$x" && h=$((h+1))
      done
    done
  fi
  echo "$(date '+%H:%M:%S') pruned=$n hashdirs=$h free=${freegb}G" >> "$LOG"
  sleep 120
done
