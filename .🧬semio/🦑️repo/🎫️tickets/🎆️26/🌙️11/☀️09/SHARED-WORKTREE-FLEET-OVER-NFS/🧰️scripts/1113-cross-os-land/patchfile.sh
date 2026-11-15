#!/bin/bash
# usage: SRC=.. DST=.. patchfile.sh <relative file>...   applies diff(live-file -> my file) with patch -F0 (fails on any drift)
MOD="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
for f in "$@"; do
  s="$SRC/$MOD/$f"; d="$DST/$MOD/$f"
  if [ ! -e "$d" ]; then mkdir -p "$(dirname "$d")"; cp "$s" "$d"; echo "new $f"; continue; fi
  diff -u "$d" "$s" > /tmp/land.patch
  rc=$?; [ $rc -eq 0 ] && { echo "same $f"; continue; }
  patch -F0 --dry-run "$d" < /tmp/land.patch >/dev/null || { echo "CONFLICT $f"; exit 1; }
  patch -F0 "$d" < /tmp/land.patch >/dev/null && echo "patched $f"
done
