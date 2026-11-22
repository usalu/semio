#!/usr/bin/env bash
set -euo pipefail

ticket=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$ticket/../../../../../../.." && pwd)
output="$ticket/🗑️generated"
label="${1:?Specify the native compiler run label}"
[[ "$label" =~ ^[a-z0-9-]+$ ]]
candidate="$output/wasm-candidate-$label"
compiler_log="$output/wasm-final-native-$label.log"
raw="$output/w/t/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm"
bindgen="$root/.cache/b/r/m/cargo/home/bin/wasm-bindgen"
package="$root/🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust/pkg"

cd "$root"
rg -q '^    Finished .*prototype' "$compiler_log"
SEMIO_LANGUAGE=en repo tools exec --light bun -- "$ticket/wasm-source-provenance.mjs" --require-complete >"$output/wasm-source-provenance-$label.json"
test "$("$bindgen" --version)" = 'wasm-bindgen 0.2.129'
mkdir "$candidate"
"$bindgen" "$raw" --out-dir "$candidate" --typescript --target web --out-name semio_puzzle
cp "$package/package.json" "$candidate/package.json"
node "$ticket/wasm-staged-node.mjs" "$candidate"
sha256sum "$raw" "$candidate/semio_puzzle_bg.wasm"
