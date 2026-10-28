#!/bin/sh
# Applies the staged xplat-g patch to the working tree in ONE all-or-nothing step (dry run first), then prints the verify command.
# Never runs inside the measurement window 10:30-11:30 unless XPLAT_G_FORCE=1. Never uses a modifying git command.
set -u
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE" && git rev-parse --show-toplevel) || exit 1
cd "$ROOT" || exit 1
NOW=$(date +%H%M)
if [ "${XPLAT_G_FORCE:-0}" != 1 ] && [ "$NOW" -ge 1030 ] && [ "$NOW" -lt 1130 ]; then
  echo "refused: measurement window 10:30-11:30 (now $(date +%H:%M)); apply after 11:30"; exit 3
fi
PATCH="$HERE/staged/xplat-g.patch"
if ! patch -p1 --dry-run < "$PATCH" > "$HERE/staged/apply-dry-run.txt" 2>&1; then
  echo "dry run failed: a peer edited a patched line. See $HERE/staged/apply-dry-run.txt; nothing was changed."; exit 1
fi
if /usr/bin/grep -q "FAILED\|rejected" "$HERE/staged/apply-dry-run.txt"; then echo "dry run reports rejects; nothing was changed."; exit 1; fi
patch -p1 < "$PATCH" > "$HERE/staged/apply.txt" 2>&1 || { echo "apply failed half way: see $HERE/staged/apply.txt and *.rej"; exit 2; }
echo "applied $(/usr/bin/grep -c '^patching file' "$HERE/staged/apply.txt") files; verify with: sh $HERE/verify.sh"
