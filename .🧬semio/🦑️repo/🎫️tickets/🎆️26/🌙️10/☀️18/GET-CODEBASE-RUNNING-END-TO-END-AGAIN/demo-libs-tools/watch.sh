#!/bin/sh
cd /Users/ueli/Documents/semio || exit 1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
snap(){ { stat -f '%m' "$L/📥️source/🔏️closure/🟦️.ts"; /usr/bin/grep -A8 "export function observedRequireResolve" "$L/👁️observation/🟨️.mjs"; /usr/bin/grep -A12 "export async function nativeCommandInputs" "$L/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs"; } | cksum; }
base=$(snap)
for i in $(seq 1 ${1:-55}); do [ "$(snap)" != "$base" ] && { echo CHANGED; exit 0; }; sleep 10; done
echo UNCHANGED
