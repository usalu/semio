#!/bin/zsh
# 🔁️ R2 — re-verification driver for fem2d/fem3d/energy and the nine A7 apps.
# Usage: 🐚️r2-verify.sh <lane> <crate> [feature]
#   lane = laws | full | wasm
# Logs land under 🗑️generated/r2/.
set -u
cd /Users/ueli/Documents/semio || exit 1
export DEVELOPER_DIR=/Library/Developer/CommandLineTools
export CARGO_INCREMENTAL=0
export CARGO_PROFILE_WASM_DEV_DEBUG=false
OUT=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ARTIFACT-TREE-VIRTUALISED-STREAMING/🗑️generated/r2"
mkdir -p "$OUT"

lane="$1"; crate="$2"; feat="${3:-}"
args=(-p "$crate")
[[ -n "$feat" ]] && args+=(--features "$feat")

case "$lane" in
  laws)
    log="$OUT/laws-$crate.txt"
    cargo test "${args[@]}" -- stamps materialises granularity slice >"$log" 2>&1
    ;;
  full)
    log="$OUT/full-$crate.txt"
    cargo test "${args[@]}" >"$log" 2>&1
    ;;
  wasm)
    log="$OUT/wasm-$crate.txt"
    cargo check "${args[@]}" --target wasm32-wasip2 >"$log" 2>&1
    ;;
  *) echo "unknown lane $lane"; exit 2;;
esac
rc=$?
echo "$lane $crate rc=$rc"
tail -3 "$log"
exit $rc
