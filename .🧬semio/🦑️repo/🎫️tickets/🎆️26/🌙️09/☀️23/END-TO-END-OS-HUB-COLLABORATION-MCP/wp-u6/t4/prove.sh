#!/bin/zsh
# 🧪️ U6 T4 re-proof on the rebased overlay (run INSIDE the overlay lane via ocargo.sh): typegen regen check, the ui-contract lib as
# ONE serial process (proves the poisoned-arena fix), runtime + ui(wgpu) per-process, the SDK row laws, then every touched plugin
# lib suite through suite.sh (reds re-run alone). Each block prints its own verdict line.
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-overlay
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
GEN="$O/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts"
SEMIO_TYPEGEN_OUT="$O/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts" cargo test --offline -p semio-framework --features typegen --lib exports_typescript_bindings 2>&1 | /usr/bin/grep -E "test result|panicked" | sed 's/^/MANIFEST-GEN /'
cargo test --offline -p semio-framework --features typegen --lib exports_typescript_bindings 2>&1 | /usr/bin/grep -E "test result|panicked" | sed 's/^/MANIFEST-LAW /'
cp "$GEN" "$L/gen-before.ts"
SEMIO_TYPEGEN_OUT="$GEN" cargo test --offline -p semio-framework-ui-contract --features typegen --test typegen_export 2>&1 | /usr/bin/grep -E "test result|panicked"
cmp -s "$L/gen-before.ts" "$GEN" && echo "TYPEGEN unchanged" || echo "TYPEGEN CHANGED"
rm -f "$L/gen-before.ts"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-contract --features typegen)
echo "CONTRACT serial: $("$bin" --test-threads 1 2>&1 | /usr/bin/grep -E 'test result|aborting|FAILED' | tr '\n' ' ')"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-runtime)
echo "RUNTIME $(zsh $T/each-test.sh "$bin" | tail -1)"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui --features wgpu-engine)
zsh $T/each-test.sh "$bin" | tail -8 | sed 's/^/UI /'
cargo test --offline --no-run --message-format short --lib -p semio-framework-plugin -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-3d -p semio-s-artifact-puzzle-5d -p semio-s-plugin-puzzle -p semio-s-artifact-cad-cad -p semio-s-artifact-process-process3d -p semio-s-plugin-cad -p semio-s-plugin-process -p semio-s-artifact-space-home -p semio-s-artifact-space-space -p semio-s-artifact-stdio-contract -p semio-s-artifact-stdio-csv -p semio-s-artifact-stdio-tsv -p semio-s-artifact-stdio-wav -p semio-s-artifact-flow-flow -p semio-s-artifact-forms-forms -p semio-s-artifact-lowpoly-lowpoly -p semio-s-artifact-energy-model -p semio-s-artifact-raster-raster 2>&1 | tee "$L/bins-raw.txt" | /usr/bin/grep -E "^error" | head -5
/usr/bin/grep "Executable" "$L/bins-raw.txt" | sed 's/.*(\(.*\))$/\1/' > "$L/bins.txt"
/usr/bin/grep -c . "$L/bins.txt" | sed 's/^/BINARIES /'
sdk=$(/usr/bin/grep "/semio_framework_plugin-" "$L/bins.txt" | head -1)
echo "SDK laws: $("$sdk" row_ table_kit home_shaped mounted_document_tree ui_history_panel --test-threads 1 2>&1 | /usr/bin/grep -E 'test result' | tr '\n' ' ')"
for b in $(cat "$L/bins.txt"); do zsh $T/suite.sh "${${b:t}%-*}" "$b"; done
rm -f "$L/bins.txt"
echo PROVE-DONE
