#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
out="$1"; shift
export NCT_AREA="${(j:|:)@}"
perl -e 'alarm 570; exec @ARGV' -- node --max-old-space-size=8192 node_modules/dependency-cruiser/bin/dependency-cruise.mjs "$@" --config "$C/tax2b-tools/cruise/area-config.cjs" --output-type json --output-to "$out"
echo "rc=$? $out"
