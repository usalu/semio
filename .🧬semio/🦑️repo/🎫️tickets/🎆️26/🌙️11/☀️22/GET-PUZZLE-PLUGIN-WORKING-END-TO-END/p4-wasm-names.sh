T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p4-profile-selectors.txt"
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
cargo rustc --lib --profile prototype --target wasm32-unknown-unknown --locked --offline --jobs 3 "${profile[@]}" -- -C strip=none ${P4_EXTRA:-}
cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm" "$OLDPWD/$T/🗑️generated/p4-module-names.wasm"
