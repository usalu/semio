#!/bin/zsh
# 🧪️ R10 item 5: typecheck every TS program importing the files whose dead deprecated aliases were removed (rule 20).
R=/Users/ueli/Documents/semio
for dir in "🧰️framework/📦️packages/🟦️typescript" "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript"; do
  echo "=== $dir $(date '+%H:%M:%S')"
  (cd "$R/$dir" && nice -n 10 bun ./📜️script.ts typecheck 2>&1 | tail -60)
  echo "=== rc=$? $(date '+%H:%M:%S')"
done
