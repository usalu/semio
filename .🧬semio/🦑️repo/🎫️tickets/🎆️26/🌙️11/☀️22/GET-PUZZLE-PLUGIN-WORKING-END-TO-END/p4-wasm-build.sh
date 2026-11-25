set -u
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p4-profile-selectors.txt"
cat /tmp/snap/FROZEN
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
cargo build --offline --locked --lib --profile prototype --target wasm32-unknown-unknown --jobs 3 "${profile[@]}" || exit $?
cd "$OLDPWD"
rm -rf "$T/🗑️generated/p4-pkg"; mkdir -p "$T/🗑️generated/p4-pkg"
"$CARGO_HOME/bin/wasm-bindgen" "$CARGO_TARGET_DIR/wasm32-unknown-unknown/prototype/semio_hub_puzzle_component.wasm" --out-dir "$T/🗑️generated/p4-pkg" --typescript --target web --out-name semio_puzzle
