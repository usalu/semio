#!/bin/zsh
# 🧪️ U6 T4 re-proof after the disabled-row-action addition and the owned-red fixes (run inside the overlay lane): ui-contract lib
# as ONE serial process, ui(wgpu) per-process, the SDK suite, and the lib suites of every crate the first proof did not run.
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-u6/t4
L=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-u6-logs
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-contract --features typegen)
echo "CONTRACT serial: $("$bin" --test-threads 1 2>&1 | /usr/bin/grep -E 'test result|aborting|FAILED' | tr '\n' ' ')"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui-runtime)
echo "RUNTIME $(zsh $T/each-test.sh "$bin" | tail -1)"
bin=$(zsh $T/lib-bin.sh -p semio-framework-ui --features wgpu-engine)
zsh $T/each-test.sh "$bin" | tail -8 | sed 's/^/UI /'
cargo test --offline --no-run --message-format short --lib -p semio-framework-plugin -p semio-s-artifact-energy-model -p semio-s-artifact-raster-raster -p semio-s-plugin-procedural -p semio-s-artifact-procedural-generation3d -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-sourcing-curation -p semio-s-artifact-stdio-bcf -p semio-s-artifact-stdio-xlsx -p semio-s-plugin-playbook-procedural 2>&1 | tee "$L/bins2-raw.txt" | /usr/bin/grep -E "^error" -A3 | head -10
/usr/bin/grep "Executable" "$L/bins2-raw.txt" | sed 's/.*(\(.*\))$/\1/' > "$L/bins2.txt"
/usr/bin/grep -c . "$L/bins2.txt" | sed 's/^/BINARIES /'
for b in $(cat "$L/bins2.txt"); do zsh $T/suite.sh "${${b:t}%-*}" "$b"; done
echo PROVE2-DONE
