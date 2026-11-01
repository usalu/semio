#!/bin/bash
# usage: t.sh <n> <extra test args after -->
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
O="$C/🗑️generated/1101/uicust-$N"; rm -f "$O.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml --no-fail-fast -p semio-framework-ui-runtime-presentation --test native_resource_custody -- "$@" > "$O.txt" 2>&1
echo $? > "$O.rc"
