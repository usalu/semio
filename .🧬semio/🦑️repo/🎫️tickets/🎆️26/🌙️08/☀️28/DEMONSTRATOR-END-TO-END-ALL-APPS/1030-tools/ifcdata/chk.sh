#!/bin/bash
# usage: chk.sh <tag> <cargo check args...>  (ifc workspace, native)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1; shift
rm -f "$C/🗑️generated/1030/ifcdata-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml" --keep-going "$@" > "$C/🗑️generated/1030/ifcdata-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1030/ifcdata-$N.rc"
