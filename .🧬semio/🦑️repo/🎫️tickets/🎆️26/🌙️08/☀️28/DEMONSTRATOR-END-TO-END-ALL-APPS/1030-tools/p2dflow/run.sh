#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; M=$2; shift 2
rm -f "$C/🗑️generated/1030/p2dflow-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- "$M" --manifest-path "Cargo.toml" "$@" > "$C/🗑️generated/1030/p2dflow-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/p2dflow-$N.rc"
