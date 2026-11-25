#!/usr/bin/env bash
# 🛂️ Native fallback: builds the current semio-plugin-describe, describes the fresh FEM component into its owner folder, and republishes the browser module.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; G="$T/🗑️generated"; mkdir -p "$G" /c/t/desc /c/t/desctmp
export TMP='C:\t\desctmp' TEMP='C:\t\desctmp' CARGO_TARGET_DIR='C:\t\desc' CARGO_BUILD_BUILD_DIR='C:\t\desc' CARGO_INCREMENTAL=0 RUST_MIN_STACK=134217728
M="/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml"
[ -x /c/t/desc/release/semio-plugin-describe.exe ] && [ -z "$REBUILD" ] || cargo build -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' --manifest-path "$(cygpath -m "$M")" --release --bin semio-plugin-describe > "$G/describe-build.log" 2>&1 || { echo "build rc=$?"; tail -20 "$G/describe-build.log"; exit 1; }
W='C:\t\wd\wasm32-wasip2\prototype\semio_hub_fem_component.wasm'
OWN="/c/semio/🌎️hub/🧩️compositions/🏗️fem/🧩️component"
/c/t/desc/release/semio-plugin-describe.exe describe "$W" --out "$(cygpath -w "$OWN")" > "$G/describe-run.log" 2>&1 || { echo "describe rc=$?"; tail -5 "$G/describe-run.log"; exit 1; }
cd "$T" && bun native-fem-materialize.ts "$W" > "$G/describe-materialize.log" 2>&1 || { echo "materialize rc=$?"; tail -5 "$G/describe-materialize.log"; exit 1; }
echo "described and published"
