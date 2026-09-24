#!/bin/zsh
# 🤝️ W1 for C8 after p5's structural pack-schema hash: restage the collab guests on the post-06:12 tree, then activate.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
zsh w1-stage-all.sh "$@"
echo "[w1-c8] activate s react dev $(date '+%F %T')"
zsh "$MUTEX[1]" wasm w1 -- zsh -c 'cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" && bun ./📜️script.ts activate s react dev' > generated/activate-s-c8.txt 2>&1
echo "[w1-c8] activate rc=$? $(tail -1 generated/activate-s-c8.txt) $(date '+%F %T')"
