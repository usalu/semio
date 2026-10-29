#!/bin/bash
# usage: testbins.sh <N> <package-dir> <target>...   (one cargo test, several --test targets)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N="$1"; P="$2"; shift 2; ARGS=""; for t in "$@"; do ARGS="$ARGS --test $t"; done
rm -f "$C/🗑️generated/1029/puzzlefin-tb$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path "$P/Cargo.toml" $ARGS > "$C/🗑️generated/1029/puzzlefin-tb$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/puzzlefin-tb$N.rc"
