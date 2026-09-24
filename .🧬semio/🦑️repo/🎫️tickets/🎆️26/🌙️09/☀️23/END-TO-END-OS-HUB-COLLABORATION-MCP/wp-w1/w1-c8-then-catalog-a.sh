#!/bin/zsh
# 🤝️ W1 for C8 + hub 7800: stage gis and draw (describe → materialize-dev), activate the s react dev lane over the
# staged modules, then publish catalog A (stdio,gis,note,draw,writer,puzzle) into w1-catalog-a.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-w1 || exit 1
MUTEX=(/Users/ueli/Documents/semio/.tmp-ticket/*fleet-mutex.sh)
zsh w1-stage-all.sh @semio-tech/gis-plugin @semio-tech/draw-plugin
echo "[w1-c8] activate s react dev $(date '+%F %T')"
zsh "$MUTEX[1]" wasm w1 -- zsh -c 'cd "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript" && bun ./📜️script.ts activate s react dev' > generated/activate-s-c8.txt 2>&1
echo "[w1-c8] activate rc=$? $(tail -1 generated/activate-s-c8.txt) $(date '+%F %T')"
W1_PACKAGES=stdio,gis,note,draw,writer,puzzle W1_DATA_NAME=w1-catalog-a zsh w1-catalog.sh publish
