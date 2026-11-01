#!/bin/bash
# usage: MAN=<manifest> w.sh <n> <wasm|native> <cargo args...>  (default root Cargo.toml; framework crates use MAN=🧰️framework/Cargo.toml)
D=$(dirname "$0"); N=$1; K=$2; shift 2
exec "$D/../chk.sh" "p2d-$N" "${MAN:-Cargo.toml}" "$K" "$@"
