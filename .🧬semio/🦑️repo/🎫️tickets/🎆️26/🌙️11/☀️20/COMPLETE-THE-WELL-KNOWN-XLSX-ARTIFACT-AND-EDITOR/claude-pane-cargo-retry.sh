#!/bin/zsh
# [DEBUG] retries the native hub wasm build while other agents' in-flight edits break unrelated crates
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; G=$T/🗑️generated/claude-pane
for i in 1 2 3 4 5 6 7 8 9 10 11 12; do
  $T/claude-pane-cargo.sh > $G/retry-$i.log 2>&1
  if tail -5 $G/retry-$i.log | grep -q "^exit 0"; then echo "RETRY-OK $i" > $G/retry.status; exit 0; fi
  if grep -q 'Finished `semio-hub-stdio-office-component`' $G/retry-$i.log && [[ -s $G/native/target/wasm32-wasip2/wasm-prototype/semio_hub_stdio_component.wasm && -s $G/native/target/wasm32-wasip2/wasm-prototype/semio_hub_stdio_office_component.wasm ]]; then echo "RETRY-OK $i" > $G/retry.status; exit 0; fi
  grep -E "^error" $G/retry-$i.log | grep -v "could not compile" | head -3 | cut -c1-200 > $G/retry-$i.errors
  echo "attempt $i failed" > $G/retry.status; sleep 120
done
echo "RETRY-EXHAUSTED" > $G/retry.status
