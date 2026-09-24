#!/bin/zsh
# 🧹 Disk guard: every 5 min, when free < 80 GiB, prune only cargo incremental sessions idle > 60 min (safe while builds run).
root="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo"
log="/Users/ueli/Documents/semio/.tmp-ticket/🗑️generated/disk-guard.txt"
mkdir -p "${log:h}"
while true; do
  free_gib=$(df -g /System/Volumes/Data | awk 'NR==2{print $4}')
  if [ "$free_gib" -lt 80 ]; then
    before=$free_gib
    find "$root" -type d -name incremental -prune 2>/dev/null | while read -r inc; do
      find "$inc" -mindepth 1 -maxdepth 1 -type d -mmin +60 -exec rm -rf {} + 2>/dev/null
    done
    echo "$(date '+%F %T') free ${before} GiB -> $(df -g /System/Volumes/Data | awk 'NR==2{print $4}') GiB" >> "$log"
  fi
  sleep 300
done
