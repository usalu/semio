T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PUZZLE-PLUGIN-WORKING-END-TO-END"
profile=()
while IFS= read -r s; do profile+=(--config "$s"); done < "$T/🗑️generated/p4-profile-selectors.txt"
cd "🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust"
cargo rustc --offline --locked --lib --profile prototype --target wasm32-unknown-unknown --jobs 3 "${profile[@]}" -p semio-s-artifact-stdio-semio-brep-snapshot -- -Zunpretty=expanded > "$OLDPWD/$T/🗑️generated/p4-brep-expanded.rs"
