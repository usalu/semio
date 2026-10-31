#!/bin/bash
# usage: chk.sh <N> <cargo check args...> — generation3d workspace, native
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
M="✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml"
N=$1; shift
mkdir -p "$C/🗑️generated/1030"
rm -f "$C/🗑️generated/1030/stepctx-chk$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$M" --keep-going "$@" > "$C/🗑️generated/1030/stepctx-chk$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/stepctx-chk$N.rc"
