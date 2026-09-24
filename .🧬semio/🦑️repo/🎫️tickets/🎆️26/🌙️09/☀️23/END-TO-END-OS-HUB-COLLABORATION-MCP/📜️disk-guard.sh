#!/bin/zsh
# 🧹 Disk guard: every 5 min; below 100 GiB free prune idle incremental sessions (> 60 min); below 80 GiB also prune superseded semio-* build units (> 12 h, package has a newer unit, crate not compiling now).
root="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
log="/Users/ueli/Documents/semio/.tmp-ticket/🗑️generated/disk-guard.txt"
mkdir -p "${log:h}"
free_gib() { df -g /System/Volumes/Data | awk 'NR==2{print $4}' }
while true; do
  before=$(free_gib)
  if [ "$before" -lt 100 ]; then
    find "$root" -type d -name incremental -prune 2>/dev/null | while read -r inc; do
      find "$inc" -mindepth 2 -maxdepth 2 -type d -mmin +60 -exec rm -rf {} + 2>/dev/null
    done
    if [ "$(free_gib)" -lt 80 ]; then
      active=" $(ps -axo command | /usr/bin/grep '[r]ustc' | /usr/bin/grep -oE -- '--crate-name [a-z_0-9]+' | awk '{print $2}' | tr '_' '-' | sort -u | tr '\n' ' ') "
      for b in "$root"/debug/build "$root"/wasm32-wasip2/*/build "$root"/wasm32-unknown-unknown/*/build; do
        [ -d "$b" ] || continue
        for pkg in "$b"/semio-*(N/); do
          case "$active" in *" ${pkg:t} "*) continue;; esac
          [ "$(find "$pkg" -mindepth 1 -maxdepth 1 -type d | wc -l)" -lt 2 ] && continue
          find "$pkg" -mindepth 1 -maxdepth 1 -type d -mmin +720 -exec rm -rf {} + 2>/dev/null
        done
      done
    fi
    echo "$(date '+%F %T') free ${before} GiB -> $(free_gib) GiB" >> "$log"
  fi
  sleep 300
done
