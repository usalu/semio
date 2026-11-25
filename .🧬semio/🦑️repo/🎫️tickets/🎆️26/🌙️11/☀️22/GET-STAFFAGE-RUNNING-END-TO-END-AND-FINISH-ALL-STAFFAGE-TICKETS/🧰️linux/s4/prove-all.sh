#!/bin/sh
# ⚖️ Proves asset scripts on copies in a scratch directory outside the checkout's staffage library, one bun process per asset inside one grant of the memory queue; the lines go to 🗑️generated/linux-s4/proof/<id>.txt.
# usage: prove-all.sh <scratch dir> [--no-rules] [asset id ...]   (default: every asset of assets.txt that has a script)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
R=$(cd "$HERE/../../../../../../../../.." && pwd)
SCRATCH="$1"
shift
FLAG=""
[ "${1:-}" = "--no-rules" ] && { FLAG="--no-rules"; shift; }
OUT="$HERE/../../🗑️generated/linux-s4/proof"
mkdir -p "$OUT" "$SCRATCH"
[ $# -gt 0 ] && IDS="$*" || IDS=$(cat "$HERE/assets.txt")
cd "$R"
for id in $IDS; do
  [ -f "$HERE/$id.ts" ] || continue
  code=0
  timeout 600 repo tools exec bun -- "$HERE/prove.ts" "$SCRATCH" "$id" $FLAG > "$OUT/$id.txt" 2> "$OUT/$id.err" || code=$?
  echo "exit $code $(grep -m1 '^PROOF' "$OUT/$id.txt" | cut -c1-90)"
done
