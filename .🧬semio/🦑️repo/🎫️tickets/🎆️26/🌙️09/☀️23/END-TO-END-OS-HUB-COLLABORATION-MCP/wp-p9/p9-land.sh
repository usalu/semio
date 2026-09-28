#!/bin/zsh
# 🛬️ P9 window-3 landing of `patches/p9-agent-lane.py`: `write` = dry run on the live tree → back up the 24 files (new ones as
# absent) → write; `revert` = restore that backup byte-exactly (created files removed). Checks run separately:
# native `p9-native-cargo.sh land-native-<n> test …` and wasm32 `📜️fleet-mutex.sh wasm p9 -- zsh wp-w4/w4-wasm-check.sh …`.
# Usage: p9-land.sh write | revert
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-p9/patches || exit 2
BACKUP="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-p9-land-backup"
ROOT=/Users/ueli/Documents/semio
case "$1" in
  write)
    python3 p9-agent-lane.py --dry-run > /dev/null || { python3 p9-agent-lane.py --dry-run; exit 1; }
    [ -e "$BACKUP" ] && { echo "backup $BACKUP exists; revert or remove it first"; exit 1; }
    mkdir -p "$BACKUP"
    python3 p9-agent-lane.py --dry-run | sed -n 's/^   //p' > "$BACKUP/files.txt"
    while IFS= read -r rel; do
      if [ -e "$ROOT/$rel" ]; then mkdir -p "$BACKUP/tree/${rel:h}"; cp -p "$ROOT/$rel" "$BACKUP/tree/$rel"; else echo "$rel" >> "$BACKUP/created.txt"; fi
    done < "$BACKUP/files.txt"
    python3 p9-agent-lane.py --write
    echo "landed $(wc -l < "$BACKUP/files.txt" | tr -d ' ') file(s) $(date '+%H:%M:%S'); backup $BACKUP"
    ;;
  revert)
    [ -f "$BACKUP/files.txt" ] || { echo "no backup"; exit 1; }
    while IFS= read -r rel; do [ -f "$BACKUP/tree/$rel" ] && cp -p "$BACKUP/tree/$rel" "$ROOT/$rel"; done < "$BACKUP/files.txt"
    [ -f "$BACKUP/created.txt" ] && while IFS= read -r rel; do rm -f "$ROOT/$rel"; rmdir "$ROOT/${rel:h}" 2>/dev/null; done < "$BACKUP/created.txt"
    python3 p9-agent-lane.py --dry-run | head -2
    echo "reverted $(date '+%H:%M:%S')"
    ;;
  *) echo "usage: p9-land.sh write | revert"; exit 2 ;;
esac
