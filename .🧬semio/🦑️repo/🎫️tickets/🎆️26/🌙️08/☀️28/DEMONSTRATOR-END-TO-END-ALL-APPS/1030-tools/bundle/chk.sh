#!/bin/bash
# usage: chk.sh <N> — native check of the demonstrator bundle crate incl. its standalone-parity tests
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
rm -f "$C/🗑️generated/1030/bundle-chk$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-demonstrator --lib --tests --keep-going > "$C/🗑️generated/1030/bundle-chk$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/bundle-chk$N.rc"
