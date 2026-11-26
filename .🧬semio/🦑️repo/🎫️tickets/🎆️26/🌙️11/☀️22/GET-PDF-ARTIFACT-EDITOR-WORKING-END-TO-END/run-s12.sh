#!/bin/sh
# usage: run-s12.sh <log name> <package> <test filters..>  (slice 12: one gated, watched cargo test run over a framework package's lib tests; log in 🗑️generated)
cd "$(dirname "$0")"
root=$(CDPATH= cd -- "$(pwd)/../../../../../../.." && pwd)
name=$1; pkg=$2; shift 2
export NATIVE_CARGO_DIR="$root/🧰️framework"
sh ./run-s4.sh "$name" test -p "$pkg" --lib -- "$@" --test-threads=2 --nocapture
