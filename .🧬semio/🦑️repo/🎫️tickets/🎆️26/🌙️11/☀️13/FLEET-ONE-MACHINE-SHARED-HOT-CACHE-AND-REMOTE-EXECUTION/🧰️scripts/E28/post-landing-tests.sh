#!/bin/zsh
# runs the runnable member tests in the live checkout after the landing and prints pass/fail counts
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; TL="$L/⚡️caching/🚀️bootstrap/🛠️tools"
cd /Users/ueli/Documents/semio || exit 2
for f in "$TL/🔏️authorizers/📦️bundle/🧪️tests/🟦️.ts" "$TL/🧪️tests/🔖️bundle-isolation/🟦️.ts" "$L/⚡️caching/🩹️patches/🧪️tests/🔭️loader/🟦️.ts" "$TL/📦️sdk/🪪️identity/🧪️tests/🧮️keys/🟦️.ts" "$TL/🔗️activation/🧪️tests/🟦️.ts" "$TL/🔏️authorizers/📦️runtime/🧪️tests/🟦️.ts" "$TL/🔏️authorizers/📦️snapshot/🧪️tests/🟦️.ts" "$TL/🧹️retention/📥️physical/🧪️tests/🎯️selection/🟦️.ts" "$TL/🧹️retention/📦️generation/🧪️tests/🟦️.ts" "$TL/🧹️retention/🎚️store-bound/🧪️tests/🟦️.ts" "$TL/📦️sdk/📥️source/🧊️pristine/🧪️tests/🧮️restore/🟦️.ts" "$TL/📦️sdk/🎟️capacity/🧪️tests/🧾️sealed-plan/🟦️.ts" "$TL/📦️sdk/🎟️capacity/🧪️tests/📏️artefact-scale/🟦️.ts"; do
  echo -n "${f##*tools/}: "; repo test-file --json "$f" 2>&1 | grep -E '^ +[0-9]+ (pass|fail)$' | tr '\n' ' '; echo
done
