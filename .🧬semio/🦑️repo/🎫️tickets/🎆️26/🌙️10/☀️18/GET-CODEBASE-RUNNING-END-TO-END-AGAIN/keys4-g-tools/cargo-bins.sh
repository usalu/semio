#!/bin/sh
# cargo-bins.sh <tag> <extra cargo test args...> -- <crate...> : builds the test executables of the crates ONCE through the repo (`bun repo cargo`, shared store, prototype profile), JSON messages to <tag>.jsonl in the generated folder, exit + wall seconds to <tag>.done.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/keys4-g"
TAG="$1"; shift
EXTRA=""; while [ "$1" != "--" ]; do EXTRA="$EXTRA $1"; shift; done; shift
ARGS=""; for crate in "$@"; do ARGS="$ARGS -p $crate"; done
START=$(date +%s)
# shellcheck disable=SC2086
bun repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --no-run --message-format=json $EXTRA $ARGS > "$G/$TAG.jsonl" 2> "$G/$TAG.err"
echo "$? $(( $(date +%s) - START ))" > "$G/$TAG.done"
