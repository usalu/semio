#!/bin/zsh
# usage: chk.sh LOG pkgs...  (foreground check in lane rustsync)
cd /Users/ueli/Documents/semio || exit 2
log=$1; shift
export SEMIO_CARGO_LANE=rustsync
args=(); for p in "$@"; do args+=(-p "$p"); done
nice -n 10 bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path 🧰️framework/Cargo.toml $args --lib --keep-going --message-format=short > "$log" 2>&1
echo "rc=$?" >> "$log"
