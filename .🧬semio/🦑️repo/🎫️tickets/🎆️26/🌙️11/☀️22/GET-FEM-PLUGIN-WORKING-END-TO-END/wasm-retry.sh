#!/usr/bin/env bash
# 🔁️ Rebuilds the FEM wasm component until it links, waiting out peer breakage, then republishes the browser module.
D="$(cd "$(dirname "$0")" && pwd)"; G="$D/🗑️generated"; mkdir -p "$G"
for i in $(seq 1 12); do
  bash "$D/native-fem-dev.sh" wasm; rc=$(tail -1 "$G/nfd-wasm.log" | sed -n 's/^rc=//p'); rc=${rc:-1}
  if [ $rc -eq 0 ]; then (cd "$D" && bun native-fem-materialize.ts C:/t/wd/wasm32-wasip2/prototype/semio_hub_fem_component.wasm > "$G/materialize.log" 2>&1); echo "published rc=$?"; exit 0; fi
  cp "$G/nfd-wasm.log" "$G/nfd-wasm-fail$i.log"; echo "attempt $i failed"; sleep 120
done
exit 1
