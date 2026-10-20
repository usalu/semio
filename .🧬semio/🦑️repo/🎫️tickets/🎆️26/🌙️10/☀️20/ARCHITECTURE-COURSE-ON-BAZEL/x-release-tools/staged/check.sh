#!/bin/sh
# Usage: check.sh [live|patched]   (live = red run against the unpatched adapter, patched = green run against the staged patch)
# REGRESSION="📥️fingerprint 📥️actions ..." overrides the existing adapter suites that run beside the new 📥️mode suite.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../../../../../../../.." && pwd)
TICKET=$(cd "$HERE/../.." && pwd)
OUT="$TICKET/🗑️generated/x-release/check"
MODE=${1:-patched}
SUITES=${REGRESSION:-"📥️fingerprint 📥️actions 📥️coverage"}
REVISION="$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs"
cd "$ROOT"
PRELOAD=""
LINKS=$(stat -f %l "$REVISION" 2>/dev/null || stat -c %h "$REVISION")
if [ "$LINKS" != "1" ]; then
  echo "[check] the Source revision reader has $LINKS links (a hardlink mirror exists); preloading the nlink-free stand-in"
  PRELOAD="--preload $TICKET/x-release-tools/no-nlink-revision.ts"
fi
rm -rf "$OUT/$MODE"
# shellcheck disable=SC2086
python3 -I "$HERE/emit.py" "$MODE" "$OUT/$MODE" $SUITES
FILES="$OUT/$MODE/🧪️tests/📥️mode/🟦️.ts"
for suite in $SUITES; do FILES="$FILES $OUT/$MODE/🧪️tests/$suite/🟦️.ts"; done
# shellcheck disable=SC2086
bun test $PRELOAD $FILES
X_RELEASE_MODE=$MODE bun test $PRELOAD "$HERE/planner-parity.ts"
python3 -I "$HERE/apply.py" --check
