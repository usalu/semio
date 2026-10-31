#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
rm -f "$C/🗑️generated/1030/pui-m$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value-retained-map "$@" > "$C/🗑️generated/1030/pui-m$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/pui-m$N.rc"
