#!/bin/zsh
cd /Users/ueli/Documents/semio
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/9e1e818a-6033-494e-beec-7c9a689f4b82/scratchpad
until [ -f "$S/target-p3d/.seeded" ]; do sleep 20; done
export CARGO_TARGET_DIR="$S/target-p3d"
export RUSTC_WRAPPER=""
for attempt in {1..60}; do
  echo "[attempt $attempt] component-release $(date)"
  if bun nx run @semio-tech/puzzle-plugin:component-release; then
    for p in $(pgrep -f "serve-release-direct.sh|script.ts serve puzzle3d react release|vite --configLoader"); do kill $p 2>/dev/null; done; sleep 2
    for p in $(pgrep -f "vite --configLoader"); do kill -9 $p 2>/dev/null; done
    echo "[attempt $attempt] serving $(date)"
    exec "$S/serve-release-direct.sh"
  fi
  echo "[attempt $attempt] failed $(date)"; sleep 240
done
echo "[rebuild] gave up $(date)"
