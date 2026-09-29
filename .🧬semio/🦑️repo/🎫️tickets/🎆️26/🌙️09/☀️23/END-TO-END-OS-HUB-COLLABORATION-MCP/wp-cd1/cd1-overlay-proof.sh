#!/bin/zsh
# 🧪️ CD1 overlay proof chain (one lane ticket per cargo run, sequential): stdio-semio lib tests, the flow bridge law, the CAD typology laws. cd1-overlay-proof.sh <round>
r="$1"; S="/Users/ueli/Documents/semio/.tmp-ticket/wp-cd1/cd1-overlay-cargo.sh"
zsh "$S" "proof-$r-stdio-semio" test --offline --no-fail-fast -p semio-s-artifact-stdio-semio --lib
zsh "$S" "proof-$r-flow-brep-invoke" test --offline --no-fail-fast -p semio-framework-os-flow --test flow_brep_invoke
zsh "$S" "proof-$r-cad-typology" test --offline --no-fail-fast -p semio-s-artifact-cad-cad --lib typology
echo "CHAIN DONE $(date '+%H:%M:%S')" > "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-cd1-work/proof-$r-done.txt"
