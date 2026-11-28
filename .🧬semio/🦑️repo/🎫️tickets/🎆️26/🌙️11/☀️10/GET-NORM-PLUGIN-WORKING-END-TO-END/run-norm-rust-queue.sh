#!/usr/bin/env bash
# usage: run-norm-rust-queue.sh   (reruns the fixed main norm plugin workspace targets; nested workspaces are run by nested/run.sh)
cd "$(git rev-parse --show-toplevel)" || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"
export SEMIO_COMPILER_PARALLELISM=1 SEMIO_LANGUAGE=en
P="✏️s/🔌️plugins/📕️norm"
run(){ name="$1"; shift; repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --no-fail-fast --locked --offline "$@" > "$G/q-$name.out" 2> "$G/q-$name.err"; echo "$name rc=$?" >> "$G/queue.status"; }
run plugin-fixed --manifest-path "$P/Cargo.toml" -p semio-s-plugin-norm-app-surface-live-editor -p semio-s-plugin-norm-structural-scene-case-kit -p semio-s-plugin-norm-structural-scene-foundation -p semio-s-plugin-norm-mechanics --lib --test moment-line
echo done >> "$G/queue.status"
