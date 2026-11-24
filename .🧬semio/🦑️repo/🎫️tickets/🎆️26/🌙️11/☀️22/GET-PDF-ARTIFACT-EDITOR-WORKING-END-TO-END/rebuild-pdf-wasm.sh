#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname -- "$0")/../../../../../../.." && pwd)
ticket="$root/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-PDF-ARTIFACT-EDITOR-WORKING-END-TO-END"
TC=$(ls -d "$root"/.cache/b/t/m/r/*/toolchains/nightly-2026-07-20-*/bin 2>/dev/null | head -1 || true)
[ -n "$TC" ] || TC=$HOME/.rustup/toolchains/nightly-2026-07-20-aarch64-apple-darwin/bin
target="$ticket/🗑️generated/native-target"
stage="$root/.cache/b/p/m/browser/prototype/modules/📘️stdio-pdf"
work="$ticket/🗑️generated/wasm-transpile"
cd "$root"
NATIVE_CARGO_LANE=w NATIVE_CARGO_DIR="$root" sh "$ticket/native-cargo.sh" --config 'profile.prototype.inherits="dev"' --config 'profile.prototype.opt-level=1' --config 'profile.prototype.debug=0' --config 'profile.prototype.codegen-units=16' --config 'profile.prototype.incremental=false' --config 'profile.prototype.lto="off"' --config 'profile.prototype.strip="none"' build -j 1 --manifest-path "$root/🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/📘️pdf/🧩️component/📦️packages/🦀️rust/Cargo.toml" -p semio-hub-stdio-pdf-component --lib --target wasm32-wasip2 --profile prototype
component=$(ls -t "$target"/lane-*/t/wasm32-wasip2/prototype/semio_hub_stdio_pdf_component.wasm | head -1)
bun=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun 2>/dev/null | head -1 || true)
[ -n "$bun" ] || bun=$(command -v bun)
"$bun" "$ticket/stage-pdf-component.ts" "$component" "$work"
