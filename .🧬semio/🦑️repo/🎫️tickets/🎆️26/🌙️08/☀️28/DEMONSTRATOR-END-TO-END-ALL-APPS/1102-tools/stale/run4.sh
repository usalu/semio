#!/bin/bash
# stale lane W3b: wait for law2, then fresh-turn law and verfolgen smoke
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"

run(){ n=$1; shift; rm -f "$G/stale-$n.rc"; SEMIO_LANGUAGE=en "$S" "W-stale-$n" repo cargo --store=shared --configuration=prototype -- "$@" > "$G/stale-$n.txt" 2>&1; echo $? > "$G/stale-$n.rc"; }
run fresh4 test --manifest-path "$PWD/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/📦️packages/🦀️rust/Cargo.toml" --no-fail-fast --lib -- fresh_turn_laws
run verfolgen4 test --manifest-path "$PWD/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" --no-fail-fast -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen_pane
echo done > "$G/stale-all4.rc"
