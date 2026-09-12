#!/bin/zsh
# 🌊 Synthetic host-edit burst: 20 mtime-only saves over 30s across the largest host TS/TSX files.
# mtime-only (touch) keeps concurrent peers' content untouched while producing identical watcher events.
R="/Users/ueli/Documents/semio"
FILES=(
  "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"
  "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx"
  "$R/🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx"
  "$R/🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts"
  "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHelpers/🟦️.tsx"
)
for i in {1..20}; do
  touch "${FILES[$(( (i - 1) % ${#FILES[@]} + 1 ))]}"
  sleep 1.5
done
