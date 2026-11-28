#!/usr/bin/env bash
set -eu

ticket="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$ticket/../../../../../../.." && pwd)"
generated="$ticket/🗑️generated"
output="$generated/lguest-browser"
component="$generated/lguest-wasip2/t/wasm32-wasip2/prototype/semio_hub_puzzle_component.wasm"
base=semio_hub_puzzle_component
bun=(/home/deck/.cache/semio/runtimes/bun/*/bun)
if [[ "${1:-}" == "--await-build" ]]; then
  build_log="$2"
  printf '%s\n' '[DEBUG] Waiting for the fresh Puzzle component'
  while [[ ! "$component" -nt "$generated/guest-frozen-component.wasm" ]]; do
    if rg -q '^error:' "$build_log"; then
      printf '%s\n' '[DEBUG] Cargo stopped before producing the component'
      exit 1
    fi
    sleep 20
  done
fi
test "$component" -nt "$generated/guest-frozen-component.wasm"
mkdir -p "$output"

cd "$root"
node node_modules/@bytecodealliance/jco/dist/jco.js transpile "$component" -o "$output" --name "$base" --instantiation async --map 'semio:framework/pure=./🟨️.js' 'semio:framework/host-async=./🟨️.js'
python "$ticket/guest-wasm-locals.py" "$component" 50000
python "$ticket/guest-wasm-stack.py" "$output/$base.core.wasm" --expected-stack 8388608 --expected-memory-maximum 8192
"${bun[0]}" "$ticket/lguest-browser-probe.ts" prepare current
node --v8-pool-size=1 --experimental-wasm-jspi --input-type=module --eval "$(cat "$output/probe-source.mjs")" "$output/$base.js" > "$output/probe.b64"
"${bun[0]}" "$ticket/lguest-browser-probe.ts" finalize current
"${bun[0]}" build --target=node --outfile "$output/editor-probe.mjs" "$ticket/lguest-editor-probe.ts"

failed=0
for dimension in 2 3 5; do
  app="s.puzzle.puzzle${dimension}d@1/*#editor"
timeout 170 node --v8-pool-size=1 --experimental-wasm-jspi "$output/editor-probe.mjs" "$output/🌉️bridge.js" "$app" --edit > "$output/editor-${dimension}d.log" 2>&1 || failed=1
done
exit "$failed"
