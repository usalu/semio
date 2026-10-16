#!/bin/zsh
cd /Users/ueli/Documents/semio
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
tag=$1; list=$2
files=("${(@f)$(cat $list)}")
perl -e 'alarm 1500; exec @ARGV' -- node node_modules/typescript/bin/tsc --noEmit --strict --allowImportingTsExtensions --module esnext --moduleResolution bundler --target esnext --resolveJsonModule --skipLibCheck --types bun --noErrorTruncation "${files[@]}" > "$G/rcc-tsc-$tag.txt" 2>&1
echo "exit=$?" > "$G/rcc-tsc-$tag-status.txt"
