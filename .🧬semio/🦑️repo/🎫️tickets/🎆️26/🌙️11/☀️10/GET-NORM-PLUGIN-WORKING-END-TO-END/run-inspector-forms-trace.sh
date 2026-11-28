#!/usr/bin/env bash
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated"
export SEMIO_COMPILER_PARALLELISM=1 SEMIO_LANGUAGE=en RUST_BACKTRACE=1
repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --locked --offline --manifest-path "✏️s/🔌️plugins/📕️norm/Cargo.toml" -p semio-s-plugin-norm-inspector-forms --lib paid::static_source::tests::original_static_definitions -- --nocapture > "$G/trace-forms.out" 2> "$G/trace-forms.err"
echo "trace-forms rc=$?" >> "$G/trace.status"
