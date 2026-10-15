#!/bin/bash
# usage: oracle.sh <tree-root-containing-🧰️framework> <outdir>
ROOT="$1"; OUT="$2"; mkdir -p "$OUT"
LIB="$ROOT/🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🦀️.rs"
cd "$(dirname "$LIB")"
run() { name="$1"; shift; rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --crate-name semio_framework_surface --emit=metadata -o /dev/null "$@" "$LIB" 2> "$OUT/$name.err"; }
run native
run test --test
run wasm32 --target wasm32-unknown-unknown
run wasip2 --target wasm32-wasip2
for f in native test wasm32 wasip2; do echo "$f: $(/usr/bin/grep -c '^error' "$OUT/$f.err") errors"; done
