#!/bin/sh
set -eu
# usage: measure-hub-memory.sh <tag> [crate] [package]  (one gated wasm32-wasip2 `cargo rustc` of a hub crate with the repository's component rustflags at `-Z threads=1`, the
# prototype profile of rebuild-pdf-wasm.sh, `-Ztime-passes` and `-Zdump-mono-stats`; samples the crate's footprint through measure-model-memory.sh; 🗑️generated/model-mem-<tag>, mono-<tag>)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}
crate=${2:-semio_hub_stdio_pdf}
package=${3:-semio-hub-stdio-pdf}
mono="$here/🗑️generated/mono-$tag"
mkdir -p "$mono"
wasm_rustflags=$(awk '/^\[target\.wasm32-wasip2\]/ { inside = 1; next } /^\[/ { inside = 0 } inside && /^rustflags/ { sub(/^rustflags *= */, ""); print; exit }' "$root/.cargo/config.toml" | sed 's/"threads=0"/"threads=1"/')
NATIVE_CARGO_LANE=w NATIVE_CARGO_DIR="$root/🌎️hub" CRATE="$crate" sh "$here/measure-model-memory.sh" "$tag" --config 'profile.prototype.inherits="dev"' --config 'profile.prototype.opt-level=1' --config 'profile.prototype.debug=0' --config 'profile.prototype.codegen-units=16' --config 'profile.prototype.incremental=false' --config 'profile.prototype.lto="off"' --config 'profile.prototype.strip="none"' --config "target.wasm32-wasip2.rustflags=$wasm_rustflags" rustc -j 1 -p "$package" --lib --target wasm32-wasip2 --profile prototype -- -Ztime-passes -Zdump-mono-stats="$mono" -Zdump-mono-stats-format=json
