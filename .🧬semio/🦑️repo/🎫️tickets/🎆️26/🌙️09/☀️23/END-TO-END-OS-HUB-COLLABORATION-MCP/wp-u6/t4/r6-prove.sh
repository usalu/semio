#!/bin/zsh
# 🧪️ Row 6 re-proof on the post-round-2 tree (clone `s14-u6-r6` + `u6-row-target.py --write`), run inside the overlay lane with
# private build/target dirs: both generators write their generated TS (the L1 runbook), the typegen laws read them back, then the
# ui-contract lib serial, runtime + ui(wgpu) per process, the SDK row laws and every touched crate's lib suite (reds re-run alone).
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4
R=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-r6
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=67108864 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-r6build CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-r6target
cd "$R" || exit 2
echo "GEN $(date +%T)"
SEMIO_TYPEGEN_OUT="$R/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts" cargo test --offline -p semio-framework-ui-contract --features typegen --test typegen_export 2>&1 | /usr/bin/grep -E "^error|test result|panicked" | sed 's/^/UI-CONTRACT-GEN /'
SEMIO_TYPEGEN_OUT="$R/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts" cargo test --offline -p semio-framework --features typegen --lib exports_typescript_bindings 2>&1 | /usr/bin/grep -E "^error|test result|panicked" | sed 's/^/MANIFEST-GEN /'
cargo test --offline -p semio-framework --features typegen --lib 2>&1 | tee $L/r6-framework.txt | /usr/bin/grep -E "^error|test result|FAILED" | sed 's/^/FRAMEWORK /'
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-contract --features typegen)
echo "CONTRACT serial: $("$bin" --test-threads 1 2>&1 | /usr/bin/grep -E 'test result|aborting|FAILED' | tr '\n' ' ')"
cargo test --offline -p semio-framework-ui-contract --features typegen --test typegen_export 2>&1 | /usr/bin/grep -E "test result|panicked" | sed 's/^/UI-CONTRACT-LAW /'
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-runtime)
echo "RUNTIME $(zsh $T/each-test.sh "$bin" | tail -1)"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui --features wgpu-engine)
zsh $T/each-test.sh "$bin" | tail -8 | sed 's/^/UI /'
args=(); for p in $(cat $T/crates.txt) semio-framework-plugin semio-s-artifact-stdio-bcf semio-s-artifact-stdio-xlsx semio-s-plugin-playbook-procedural; do args+=(-p $p); done
cargo test --offline --no-run --message-format short --lib $args 2>&1 | tee "$L/r6-bins-raw.txt" | /usr/bin/grep -E "^error" -A3 | head -20
/usr/bin/grep "Executable" "$L/r6-bins-raw.txt" | sed 's/.*(\(.*\))$/\1/' | sort -u > "$L/r6-bins.txt"
/usr/bin/grep -c . "$L/r6-bins.txt" | sed 's/^/BINARIES /'
sdk=$(/usr/bin/grep "/semio_framework_plugin-" "$L/r6-bins.txt" | head -1)
echo "SDK laws: $("$sdk" row_ table_kit home_shaped mounted_document_tree ui_history_panel --test-threads 1 2>&1 | /usr/bin/grep -E 'test result' | tr '\n' ' ')"
for b in $(cat "$L/r6-bins.txt"); do zsh $T/suite.sh "${${b:t}%-*}" "$b"; done
echo "R6-DONE $(date +%T)"
