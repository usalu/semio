#!/bin/zsh
# ♻️ W1: `activate <variant> react dev` over the staged modules, one wasm hold. usage: zsh w1-activate.sh <variant>
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
OUT=/Users/ueli/Documents/semio/.tmp-ticket/wp-w1/generated
zsh "$MUTEX[1]" wasm w1 -- zsh -c "cd '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript' && bun ./📜️script.ts activate $1 react dev" > "$OUT/activate-$1.txt" 2>&1
echo "[w1-activate] $1 rc=$? $(tail -1 "$OUT/activate-$1.txt") $(date '+%F %T')"
