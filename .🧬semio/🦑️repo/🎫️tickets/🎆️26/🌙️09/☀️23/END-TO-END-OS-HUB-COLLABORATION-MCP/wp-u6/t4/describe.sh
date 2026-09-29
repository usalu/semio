#!/bin/zsh
# 🛂️ Regenerates one plugin's descriptor pair (`🛂️.descriptor.semio` + `🔣️.json`) INSIDE the overlay, exactly as the inferred Nx
# `describe` target does (component-dev → `describe component --manifest`), but with the overlay's PRIVATE build-dir/target:
# links the wasm32-wasip2 component (`wasm-dev`), stages it at `<crate>/dist/component-dev/`, then runs the describe emitter.
# Run inside the overlay lane: ocargo.sh <capture> 'zsh describe.sh <plugin dir under ✏️s/🔌️plugins>…'
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay
cd "$O" || exit 2
for plugin in "$@"; do
  manifest="✏️s/🔌️plugins/$plugin/📦️packages/🦀️rust/Cargo.toml"
  package=$(/usr/bin/grep -m1 '^name' "$manifest" | sed 's/.*"\(.*\)".*/\1/')
  wasm="${package//-/_}.wasm"
  start=$(date +%s)
  if ! cargo rustc --offline -p "$package" --lib --crate-type cdylib --target wasm32-wasip2 --profile wasm-dev 2>&1 | /usr/bin/grep -E "^error" -A5 | head -20; then :; fi
  built="$CARGO_TARGET_DIR/wasm32-wasip2/wasm-dev/$wasm"
  if [ ! -f "$built" ]; then echo "DESCRIBE $plugin NO-COMPONENT"; continue; fi
  staged="$(dirname "$manifest")/dist/component-dev"
  mkdir -p "$staged" && cp "$built" "$staged/$wasm"
  bun "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts" component --manifest "$manifest" 2>&1 | tail -2 | sed "s|^|DESCRIBE $plugin |"
  echo "DESCRIBE $plugin wall=$(( $(date +%s) - start ))s"
done
