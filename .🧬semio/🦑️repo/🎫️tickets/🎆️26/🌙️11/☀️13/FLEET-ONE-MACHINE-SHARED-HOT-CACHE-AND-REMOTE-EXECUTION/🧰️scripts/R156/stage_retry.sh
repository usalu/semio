#!/bin/zsh
cd /private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R156
sed "s#\"../../🧠️memory/📋️slice-report/🟨️.mjs\"#\"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/📋️slice-report/🟨️.mjs\"#" new/retry.mjs > stage/🔁retry/🟨️.mjs
python3 patch_load.py load.orig.ts new/load.ts
