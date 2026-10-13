#!/bin/bash
# 🔒️ Re-resolves each listed Cargo workspace offline (`cargo metadata` without `--locked`), so its Cargo.lock follows the manifests
# with no version bumps; prints the workspaces whose lockfile changed. Skips workspaces already recorded in the done file.
# usage: 🔒️refresh-locks.sh <workspace-list> <done-file> <log-file>
cd /Users/ueli/Documents/semio || exit 2
LIST="$1"; DONE="$2"; LOG="$3"; touch "$DONE"
while read -r ws; do
  [ -z "$ws" ] && continue
  grep -qxF "$ws" "$DONE" && continue
  before=$(shasum "$ws/Cargo.lock" | cut -d' ' -f1)
  if cargo metadata --offline --format-version 1 --manifest-path "$ws/Cargo.toml" > /dev/null 2>> "$LOG"; then status=ok; else status=FAILED; fi
  after=$(shasum "$ws/Cargo.lock" | cut -d' ' -f1)
  [ "$before" != "$after" ] && echo "changed $ws" >> "$LOG"
  echo "$status $ws" >> "$LOG"
  echo "$ws" >> "$DONE"
done < "$LIST"
