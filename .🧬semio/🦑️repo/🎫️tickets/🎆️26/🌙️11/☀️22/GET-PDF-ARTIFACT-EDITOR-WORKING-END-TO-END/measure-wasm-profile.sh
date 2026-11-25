#!/bin/sh
# usage: measure-wasm-profile.sh <tag> <opt-level> <lto> <codegen-units>  (one gated wasm32-wasip2 build of `semio-hub-stdio-pdf-component` in its own profile `pv<tag>`
# — so the staged browser build is neither replaced nor invalidated — with the 5 s rustc sampler; writes 🗑️generated/wasm-<tag>.{out,mem.log,peaks.txt,size.json,done})
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
G="$here/🗑️generated"
tag=${1:?tag}; opt=${2:?opt-level}; lto=${3:?lto}; units=${4:?codegen-units}
profile="pv$tag"
stop="$G/wasm-$tag.stop"
rm -f "$stop" "$G/wasm-$tag.done"
case "$opt" in [0-9]) level="$opt" ;; *) level="\"$opt\"" ;; esac
wasm_rustflags=$(awk '/^\[target\.wasm32-wasip2\]/ { inside = 1; next } /^\[/ { inside = 0 } inside && /^rustflags/ { sub(/^rustflags *= */, ""); print; exit }' "$root/.cargo/config.toml" | sed 's/"threads=0"/"threads=1"/')
sh "$here/sample-rustc-memory.sh" "$G/wasm-$tag.mem.log" "$stop" w &
sampler=$!
start=$(date +%s)
status=0
NATIVE_CARGO_LANE=w NATIVE_CARGO_DIR="$root/🌎️hub" sh "$here/native-cargo.sh" --config "profile.$profile.inherits=\"dev\"" --config "profile.$profile.opt-level=$level" --config "profile.$profile.debug=0" --config "profile.$profile.codegen-units=$units" --config "profile.$profile.incremental=false" --config "profile.$profile.lto=\"$lto\"" --config "profile.$profile.strip=\"none\"" --config "target.wasm32-wasip2.rustflags=$wasm_rustflags" build -j 1 -p semio-hub-stdio-pdf-component --lib --target wasm32-wasip2 --profile "$profile" > "$G/wasm-$tag.out" 2>&1 || status=$?
end=$(date +%s)
touch "$stop"
wait "$sampler" 2>/dev/null || true
python3 "$here/peak-rustc-memory.py" "$G/wasm-$tag.mem.log" > "$G/wasm-$tag.peaks.txt"
bun=$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)
component=$(ls -t "$here"/🗑️generated/native-target/lane-w/t/wasm32-wasip2/"$profile"/semio_hub_stdio_pdf_component.wasm 2>/dev/null | head -1)
[ -n "$component" ] && "$bun" "$here/wasm-code-size.mjs" "$component" > "$G/wasm-$tag.size.json"
echo "exit $status seconds $((end - start)) opt=$opt lto=$lto codegen-units=$units" > "$G/wasm-$tag.done"
