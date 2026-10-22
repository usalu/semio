#!/bin/zsh
# usage: lib.sh LOG -p crate ...   (check --lib only)
cd /Users/ueli/Documents/semio || exit 1
L=$1; shift
export SEMIO_CARGO_LANE=wirefix CARGO_TARGET_DIR=/private/tmp/wire-fix/target
rm -f "$L"
nice -n 10 bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --manifest-path Cargo.toml "$@" --lib --keep-going --message-format=short > "$L" 2>&1
echo rc=$? >> "$L"
