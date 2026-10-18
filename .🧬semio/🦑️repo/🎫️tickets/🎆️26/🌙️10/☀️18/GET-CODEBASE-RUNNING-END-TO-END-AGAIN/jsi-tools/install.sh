#!/bin/zsh
# usage: install.sh <stage-dir>  (installs the staged component tree over the build inference module; refuses when the monolith changed since it was split)
set -e
cd /Users/ueli/Documents/semio
STAGE="$1"
I="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference"
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
if [ -d "$I/🗂️components" ]; then echo "already split"; else
  [ "$(shasum -a 256 "$I/🟨️.mjs" | cut -d' ' -f1)" = "$(shasum -a 256 "$C/jsi-tools/orig/inference-orig.mjs" | cut -d' ' -f1)" ] || { echo "monolith changed since the split: re-run the split"; exit 1; }
fi
for d in "$STAGE"/*/; do name="$(basename "$d")"; mkdir -p "$I/$name"; cp -R "$d". "$I/$name/"; done
cp "$STAGE/🟨️.mjs" "$I/🟨️.mjs"
echo installed
