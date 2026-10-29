#!/bin/bash
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/1028-tools/sweepplugin"
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
mode=$1; b=$2
case $b in root_*) M=Cargo.toml;; *) M="🧰️framework/Cargo.toml";; esac
A=$(sed 's/^/-p /' "$T/batch_$b" | tr '\n' ' ')
if [ $mode = nat ]; then X="--lib --tests"; else X="--target wasm32-wasip2 --lib"; fi
O="$G/sweepplugin-$mode-$b"
rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" --keep-going $X $A > "$O.txt" 2>&1
echo $? > "$O.rc"
