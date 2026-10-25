#!/bin/sh
# usage: cargo-check.sh <log> <cargo args...> : graph-free short per-crate cargo run through the repo binary.
cd /Users/ueli/Documents/semio || exit 1
L="$1"; shift
export SEMIO_SCRIPT_STALL_MS=14400000
exec nice -n 10 bun repo cargo --store=shared --configuration=prototype -- "$@" > "$L" 2>&1
