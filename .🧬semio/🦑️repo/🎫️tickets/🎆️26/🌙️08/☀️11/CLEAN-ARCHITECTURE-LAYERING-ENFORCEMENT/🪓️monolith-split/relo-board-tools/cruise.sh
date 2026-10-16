#!/bin/zsh
# 🕸️ dependency-cruiser before/after over the slice D TS paths: CR0 = sandbox with the pre-slice-D files rebuilt, CR1 = sandbox of the current tree.
R=/Users/ueli/Documents/semio
T=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/relo-board-tools
G=$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated
SCR=$1
TREES=("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer" "🧰️framework/🔨️modules/🖱️ui/🎬️scene" "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm" "🧰️framework/🔨️modules/🕹️interaction" "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📖️stories")
PATHS=("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👕️canvas-presence/🟦️.ts" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🧪️tests/👻️catalogue-drop/🟦️.tsx" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts" "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts" "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🪪️session-factory/🟦️.ts" "🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts" "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📖️stories/🎭️graph-wasm-canvas/🧪️.story.tsx")
for side in CR0 CR1; do
  rm -rf "$SCR/$side"; mkdir -p "$SCR/$side"
  for tree in $TREES; do mkdir -p "$SCR/$side/$tree"; rsync -a --exclude node_modules --exclude target --exclude dist "$R/$tree/" "$SCR/$side/$tree/"; done
  ln -s "$R/node_modules" "$SCR/$side/node_modules"
  cp "$R/tsconfig.json" "$SCR/$side/tsconfig.json"
done
python3 -I "$T/baseline.py" "$SCR/CR0" > /dev/null
for side in CR0 CR1; do
  (cd "$SCR/$side" && perl -e 'alarm 1200; exec @ARGV' -- node "$R/node_modules/dependency-cruiser/bin/dependency-cruise.mjs" $PATHS --config "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs" --output-type err > "$G/relo-board-depcruise2-$side.log" 2>&1; echo "$side exit $?")
done
