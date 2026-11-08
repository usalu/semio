#!/bin/bash
# usage: chk.sh <label> <crate> : cargo check --tests --lib for one crate, all errors
exec "$(dirname "$0")/../chk.sh" "$1" 🧰️framework/Cargo.toml native -p "$2" --lib --tests
