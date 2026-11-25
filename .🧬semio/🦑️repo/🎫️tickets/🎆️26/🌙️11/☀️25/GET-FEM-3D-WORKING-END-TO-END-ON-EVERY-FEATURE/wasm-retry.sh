#!/usr/bin/env bash
# 🔁️ Rebuilds the FEM wasm until it succeeds (peers' half-done edits and transient Cargo.lock mappings break single runs). Usage: wasm-retry.sh [attempts]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; G="$M/🗑️generated"
for i in $(seq 1 "${1:-30}"); do
  "$M/native-fem-dev.sh" wasm; cp "$G/nfd-wasm.log" "$G/nfd-wasm-last.log"
  tail -1 "$G/nfd-wasm.log" | grep -q "^rc=0$" && { echo "attempt $i rc=0"; exit 0; }
  if grep -q "failed to write .*Cargo.lock" "$G/nfd-wasm.log"; then sleep 10; continue; fi
  echo "attempt $i failed: $(grep -m3 -E '^error(\[E[0-9]+\])?:' "$G/nfd-wasm.log" | tr '\n' ' ')"; sleep 90
done; exit 1
