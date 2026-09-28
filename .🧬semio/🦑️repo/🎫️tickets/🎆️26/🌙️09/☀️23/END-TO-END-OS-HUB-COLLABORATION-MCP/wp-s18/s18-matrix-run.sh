#!/bin/zsh
# 🧮️ S18: program-matrix via the dev script verb against serve 6540; args after the tag go to `verify matrix`.
D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"
TAG="$1"; shift
cd "$D" && echo "[s18] START $TAG $(date '+%F %T')" && NX_DAEMON=false nice -n 10 bun "$D/📜️script.ts" verify matrix --serve http://127.0.0.1:6540/ --tag "$TAG" "$@"; echo "[s18] END $TAG rc=$? $(date '+%F %T')"
