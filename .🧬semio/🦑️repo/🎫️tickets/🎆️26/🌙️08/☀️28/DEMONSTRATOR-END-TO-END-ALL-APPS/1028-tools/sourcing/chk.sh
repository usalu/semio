#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1028"
N="$1"; shift
for a in $(seq 1 12); do
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check "$@" --target wasm32-wasip2 --keep-going > "$G/sourcing-$N.txt" 2>&1
  rc=$?
  [ $rc -eq 0 ] && break
  /usr/bin/grep -q 'failed to load manifest\|Source refused before compiler\|Source extent or boundary\|cannot be resolved' "$G/sourcing-$N.txt" || break
  cp "$G/sourcing-$N.txt" "$G/sourcing-$N.attempt$a.txt"; sleep 60
done
echo $rc > "$G/sourcing-$N.rc"
