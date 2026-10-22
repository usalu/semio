#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/small-fixes"
mkdir -p "$G/info"
cfg="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/tsconfig.json"
tag="$1"
NODE_OPTIONS=--max-old-space-size=8192 nice -n 10 node node_modules/typescript/bin/tsc -p "$cfg" --noEmit --incremental --tsBuildInfoFile "$G/info/$tag.tsbuildinfo" --pretty false > "$G/$tag.txt" 2>&1
echo "rc=$?" >> "$G/$tag.txt"
