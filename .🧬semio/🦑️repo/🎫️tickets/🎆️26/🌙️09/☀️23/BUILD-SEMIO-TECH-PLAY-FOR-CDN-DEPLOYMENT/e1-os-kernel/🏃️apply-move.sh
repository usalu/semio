#!/bin/bash
# 🏃️ Applies one E1 store move end to end: move-items, compile-driven fixes in every configuration (native, test,
# sync, wasm32-wasip2), unused-import pruning, preamble tidying, and a final fix pass. Stops on the first red config.
# usage: 🏃️apply-move.sh <spec.json> <owners.json> [--finish]   (--finish skips the move and only fixes, prunes, tidies)
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"; REPO=/Users/ueli/Documents/semio; cd "$REPO" || exit 2
SPEC=$1; OWNERS=$2
STORE="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store"
TARGET="$STORE/$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['target'])" "$SPEC")"
[ "${3:-}" = "--finish" ] || { python3 "$HERE/🚚️move-items.py" "$SPEC" --owners "$OWNERS" --write | grep -v '^   ' || exit 1; }
loop() { python3 "$HERE/🔁️fix-loop.py" --files "$TARGET" --rounds 14 -- "$@" 2>&1 | tail -1; }
for cfg in "" "--profile test" "--features sync,ureq" "--target wasm32-wasip2" "--target wasm32-unknown-unknown"; do
  r=$(loop $cfg); echo "[$cfg] $r"; case "$r" in *" 0 errors"*) ;; *) exit 1;; esac
done
python3 "$HERE/🧹️prune-imports.py" "$TARGET" -- --features sync,ureq
python3 "$HERE/🧽️tidy-imports.py" "$TARGET"
for cfg in "" "--profile test" "--features sync,ureq" "--target wasm32-wasip2" "--target wasm32-unknown-unknown"; do
  r=$(loop $cfg); echo "[$cfg] $r"; case "$r" in *" 0 errors"*) ;; *) exit 1;; esac
done
