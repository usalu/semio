#!/bin/bash
# usage: native-wasm.sh <surface|editor|node-graph|puzzle|flow|canvas|site> ; runs the target natively in release mode with a genuine build-caller envelope
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd); ROOT="$(cd "$HERE/../../../../../../.." && pwd)"; G="$HERE/🗑️generated"; mkdir -p "$G"
run_bun() { if [[ -n "${BUN:-}" ]]; then "$BUN" "$@"; else repo tools exec bun -- "$@"; fi; }
export SEMIO_BUILD_MODE=release
F="$ROOT/🧰️framework"; RCPTENV=$(python3 -c "import json;print(json.load(open('$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🧬️schema/🔣️.json'))['definitions']['Policy']['const']['environment'])")
case "$1" in
 surface) id=@semio-tech/framework-surface-rs:wasm; cwd="$F/🔨️modules/🗺️surface/📦️packages/🦀️rust"; cmd=(./📜️script.ts wasm);;
 editor) id=@semio-tech/framework-editor-rs:wasm; cwd="$F/🔨️modules/✍️editor/📦️packages/🦀️rust"; cmd=(./📜️script.ts wasm);;
 node-graph) id=@semio-tech/framework-os-node-graph-rs:wasm; cwd="$F/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📦️packages/🦀️rust"; cmd=(./📜️script.ts wasm);;
 puzzle) id=@semio-tech/puzzle-plugin:wasm; cwd="$ROOT/🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust"; cmd=(./📜️script.ts wasm);;
 flow) id=semio-framework-os-flow-browser-package:wasm; cwd="$F/🛍️products/💻️os/🔨️modules/🌊️flow/🎁️browser-package/📦️packages/🦀️rust"; cmd=("../../🏗️build/📜️script.ts" wasm);;
 canvas) id=semio-framework-canvas:stage-runtime-assets; cwd="$F/🔨️modules/🖼️canvas/📦️packages/🦀️rust"; cmd=(./📜️script.ts stage-runtime-assets);;
 site) id=@semio-tech/mit-bestand-demonstrator:build; cwd="$ROOT/♻️mit-bestand/🧺️demonstrator"; cmd=(./🔨️modules/📦️site/📜️script.ts build);;
 gen:*) spec=${1#gen:}; id=${spec%%=*}; rel=${spec#*=}; cwd="$ROOT/$rel"; cmd=(./📜️script.ts "${id##*:}");;
 *) echo "unknown $1"; exit 2;;
esac
out=$(cd "$ROOT" && run_bun "$HERE/native-wire.ts" "$id"); wire=$(printf %s "$out"|sed -n 1p); prov=$(printf %s "$out"|sed -n 2p)
tgt=${id##*:}; proj=${cwd#$ROOT/}; started=$(($(date +%s)*1000-1000))
issued=$(run_bun "$HERE/native-receipt.mjs" issue "$proj" "$tgt")
rcpt=$(printf %s "$issued" | python3 -c 'import json,sys;print(next(iter(json.load(sys.stdin).values()),""))')
cd "$cwd"
if env SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_SCRIPT_PROVIDERS="$prov" SEMIO_TASK_PROJECT="${id%%:*}" SEMIO_TASK_TARGET="$tgt" ${rcpt:+$RCPTENV="$rcpt"} repo capacity run --memory="${MEM:-3000}" -- repo "${cmd[@]}" > "$G/native-${1//[:=@\/]/_}.log" 2>&1; then
  run_bun "$HERE/native-receipt.mjs" settle "$proj" "$tgt" "$issued" 1 "$started"; echo "OK $1"
else
  run_bun "$HERE/native-receipt.mjs" settle "$proj" "$tgt" "$issued" 0 "$started"; echo "FAIL $1"; tail -30 "$G/native-${1//[:=@\/]/_}.log" | cut -c1-300; exit 1
fi
