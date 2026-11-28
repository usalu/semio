#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/tl"; mkdir -p "$G" /c/t/tl /c/t/tltmp
D="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"
export TMP='C:\t\tltmp' TEMP='C:\t\tltmp' TMPDIR='C:\t\tltmp' CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR='C:\t\tl' CARGO_BUILD_BUILD_DIR='C:\t\tl'
export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= SEMIO_BUILD_MODE=prototype RUST_MIN_STACK=134217728
export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=selected
cd /c/semio/🌎️hub && cargo build --offline -j 3 --manifest-path /c/semio/🌎️hub/Cargo.toml --profile prototype --target wasm32-wasip2 -p semio-os-history-schema --lib --config "$(cygpath -m "$D/native-fem-profile.toml")" > "$G/rlib.log" 2>&1
echo "rc=$?" >> "$G/rlib.log"
