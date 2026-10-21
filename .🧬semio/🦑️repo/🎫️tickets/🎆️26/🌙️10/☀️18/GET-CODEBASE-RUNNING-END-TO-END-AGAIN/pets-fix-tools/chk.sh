#!/bin/zsh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/pets-fix"
out="$G/$1.txt"; shift; KG="--keep-going"; [[ "$1" == test ]] && KG=""
SEMIO_CARGO_LANE=petsfix nice -n 10 bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- "$@" --manifest-path Cargo.toml $KG --message-format=short > "$out" 2>&1
echo rc=$?
grep -v '^\[script\]\|^\[capacity\]' "$out" | grep ': error' | sed -E 's/^.*rust\/\.\.\/\.\.\/\.\.\///; s/^.*rust\/\.\.\/\.\.\///' > "$G/errs.txt"
wc -l < "$G/errs.txt"
