#!/usr/bin/env bash
# usage: native-wasm.sh <surface|editor|node-graph|puzzle|flow|canvas|site> ; runs the target natively in release mode with a genuine build-caller envelope
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd); ROOT="$(cd "$HERE/../../../../../../.." && pwd)"; G="$HERE/🗑️generated"; mkdir -p "$G"
run_bun() { if [[ -n "${BUN:-}" ]]; then "$BUN" "$@"; else repo tools exec --light bun -- "$@"; fi; }
export SEMIO_BUILD_MODE=release
F="$ROOT/🧰️framework"
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
receipt_env=$(printf %s "$issued" | python3 -c 'import json, sys; [(key, value)] = json.load(sys.stdin).items(); print(key + "=" + value)')
log="$G/native-${1//[:=@\/]/_}.log"
if [[ "$1" == site ]]; then
  log="${NATIVE_SITE_RUN_LOG:-$(mktemp "$G/native-site-run.XXXXXX")}"
fi
cd "$cwd"
if env SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" SEMIO_SCRIPT_PROVIDERS="$prov" SEMIO_TASK_PROJECT="${id%%:*}" SEMIO_TASK_TARGET="$tgt" "$receipt_env" repo capacity run --memory="${MEM:-3000}" -- repo "${cmd[@]}" > "$log" 2>&1; then
  run_bun "$HERE/native-receipt.mjs" settle "$proj" "$tgt" "$issued" 1 "$started"
  if [[ "$1" == site ]]; then
    python3 "$HERE/captured-site.py" "$ROOT" "$log" >/dev/null
    published=$(mktemp "$G/native-site-publish.XXXXXX")
    cp "$log" "$published"
    mv -f "$published" "$G/native-site.log"
  fi
  echo "OK $1"
else
  run_bun "$HERE/native-receipt.mjs" settle "$proj" "$tgt" "$issued" 0 "$started"; echo "FAIL $1"; tail -30 "$log" | cut -c1-300; exit 1
fi
