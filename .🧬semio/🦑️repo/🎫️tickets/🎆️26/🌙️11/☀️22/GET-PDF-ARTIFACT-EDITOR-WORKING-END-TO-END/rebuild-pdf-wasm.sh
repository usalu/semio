#!/bin/sh
set -eu
# 🧠️ Every wasm crate compiles with `-Z threads=1` (the repository's `[target.wasm32-wasip2]` rustflags with only the thread count changed, so the
# component's memory maximum and shadow stack stay the configured ABI): `-Z threads=0` runs the front end on every core and its parallel codegen
# translates the first batch of CGUs (one per thread) into LLVM modules at once, which put `semio-hub-stdio-pdf` (the 20-app monomorphization) at a
# 10 GB footprint within two minutes and got its component SIGKILLed (round 4, 🌐️pdf-browser-journey.md). Cargo appends target rustflags after
# profile rustflags, so a per-package override cannot win against the target table; the whole target is overridden instead.
# 📏️ `opt-level = "z"`: measured against 1 it cuts the component's code from 278 to 185 MB (functions 588k to 534k) at the same 3.5 GB peak and a
# shorter build; thin LTO adds 2 GB for no code bytes, fat LTO and `codegen-units = 1` exceed the 8.2 GB guard (🧠️pdf-model-compile-memory.md, Guest code size).
# 🧩️ The plugin ships as one component per runtime shard (`PackageDescriptor.components`): the three cdylibs build in one cargo run, so
# every shared crate compiles once, and the stage takes the plugin descriptor the native `committed_descriptor` test described.
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
ticket="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
TC=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin 2>/dev/null | head -1 || true)
[ -n "$TC" ] || TC=$HOME/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
target="$ticket/🗑️generated/native-target"
stage="$root/.cache/b/p/m/browser/prototype/modules/📘️stdio-pdf"
work="$ticket/🗑️generated/wasm-transpile"
cd "$root"
wasm_rustflags=$(awk '/^\[target\.wasm32-wasip2\]/ { inside = 1; next } /^\[/ { inside = 0 } inside && /^rustflags/ { sub(/^rustflags *= */, ""); print; exit }' "$root/.cargo/config.toml" | sed 's/"threads=0"/"threads=1"/')
case "$wasm_rustflags" in *'"threads=1"'*) ;; *) echo "rebuild-pdf-wasm: no [target.wasm32-wasip2] rustflags with threads=0 in .cargo/config.toml" >&2; exit 2 ;; esac
NATIVE_CARGO_LANE=w NATIVE_CARGO_DIR="$root" sh "$ticket/native-cargo.sh" --config 'profile.prototype.inherits="dev"' --config 'profile.prototype.opt-level="z"' --config 'profile.prototype.debug=0' --config 'profile.prototype.codegen-units=16' --config 'profile.prototype.incremental=false' --config 'profile.prototype.lto="off"' --config 'profile.prototype.strip="none"' --config "target.wasm32-wasip2.rustflags=$wasm_rustflags" build -j 1 --manifest-path "$root/🌎️hub/Cargo.toml" -p semio-hub-stdio-pdf-1-4-component -p semio-hub-stdio-pdf-1-7-editors-component -p semio-hub-stdio-pdf-1-7-viewers-component --lib --target wasm32-wasip2 --profile prototype
wasm="$target/lane-w/t/wasm32-wasip2/prototype"
described=$(ls -td "$target"/lane-*/*/tmp/stdio-pdf-described 2>/dev/null | head -1)
[ -n "$described" ] || { echo "rebuild-pdf-wasm: no described stdio-pdf descriptor: run run-committed-descriptor.sh first" >&2; exit 3; }
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1 || true)
[ -n "$bun" ] || bun=$(command -v bun)
"$bun" "$ticket/stage-pdf-component.ts" "$wasm" "$work" "$described"
