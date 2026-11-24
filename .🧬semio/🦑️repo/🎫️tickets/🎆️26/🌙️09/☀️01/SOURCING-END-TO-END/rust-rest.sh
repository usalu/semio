#!/bin/zsh
# 🦀️ Oracle, bridge and hub sourcing suites through the native wrapper.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"; S="✏️s/🔌️plugins/🪵️sourcing"
$T/native-cargo.sh oracle "$S/🧩️extensions/🧪️tests/🔬️unit/🔮️oracle/📦️packages/🦀️rust" test --no-fail-fast -- --test-threads=1
$T/native-cargo.sh bridge "$S/🏭️bridge" test --no-fail-fast -- --test-threads=1
$T/native-cargo.sh hub "🌎️hub" test -p semio-hub-sourcing -p semio-hub-sourcing-component --no-fail-fast -- --test-threads=1
$T/native-cargo.sh hub-wasm "🌎️hub" check --target wasm32-wasip2 -p semio-hub-sourcing-component
