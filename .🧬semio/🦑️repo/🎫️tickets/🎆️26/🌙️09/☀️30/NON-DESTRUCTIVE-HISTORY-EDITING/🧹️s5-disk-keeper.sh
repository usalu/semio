#!/bin/zsh
# 🧹️ Disk keeper: every two minutes, when the data volume has fewer than 24 GiB free, runs `🧹️s5-stale-unit-prune.py` (two newest units
# per package stay; lock-aware; lock file idle) over every cargo `debug/build` / `wasm-dev/build` directory of this ticket's train
# targets and the shared build dir (other tickets' folders are NOT touched since 23:05) — idle bound 3 h,
# 90 min below 14 GiB, 60 min below 10 GiB. One line per pass in `🗑️generated/coord/disk-keeper.txt`.
setopt no_bg_nice
ticket="${0:A:h}"
peer="${ticket:h}/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/🗑️generated"
shared="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build"
log="$ticket/🗑️generated/coord/disk-keeper.txt"
free() { df -g /System/Volumes/Data | awk 'NR>1{print $4}' }
while true; do
  if [ "$(free)" -lt 24 ]; then
    hours=3; [ "$(free)" -lt 14 ] && hours=1.5; [ "$(free)" -lt 10 ] && hours=1
    before="$(free)"
    for dir in ${(f)"$(find "$ticket/🗑️generated/coord" "$shared" -maxdepth 4 -type d -name build \( -path '*debug*' -o -path '*wasm-dev*' \) 2>/dev/null)"}; do
      python3 "$ticket/🧹️s5-stale-unit-prune.py" "$dir" "$hours" --apply > /dev/null 2>&1
    done
    print -r -- "$(date '+%F %T') idle>${hours}h: free ${before} → $(free) GiB" >> "$log"
  fi
  sleep 120
done
