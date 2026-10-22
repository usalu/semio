#!/bin/zsh
# usage: chk.sh LOG check|test -p crate ... [-- extra]
cd /Users/ueli/Documents/semio || exit 1
L=$1; shift; MODE=$1; shift
export SEMIO_CARGO_LANE=wirefix CARGO_TARGET_DIR=/private/tmp/wire-fix/target
rm -f "$L"
if [[ $MODE == check ]]; then X=(--lib --tests); else X=(--lib --no-fail-fast); fi
nice -n 10 bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- $MODE --manifest-path Cargo.toml "$@" $X --keep-going --message-format=short > "$L" 2>&1
echo rc=$? >> "$L"
