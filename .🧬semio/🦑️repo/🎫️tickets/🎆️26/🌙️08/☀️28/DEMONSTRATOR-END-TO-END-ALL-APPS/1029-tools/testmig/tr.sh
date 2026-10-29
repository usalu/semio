#!/bin/bash
# usage: tr.sh <name> <workspace-dir> [extra cargo test args...]   (compile + run, keep-going)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N="$1"; D="$2"; shift 2
rm -f "$C/🗑️generated/1029/testmig-$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path "$D/Cargo.toml" "$@" > "$C/🗑️generated/1029/testmig-$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/testmig-$N.rc"
