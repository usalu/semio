#!/bin/zsh
# usage: appliances-b-run.sh <slug> [check|insert|validate|all]
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
cd "$T"
slug=$1; mode=${2:-all}
asset="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/$slug/🔣️.json"
mkdir -p "🗑️generated/authoring-appliances-b"
[[ $mode == build || $mode == all ]] && python3 "volumes/appliances/author_$slug.py"
[[ $mode == check || $mode == all ]] && $BUN volume-check.ts "$slug" --volume "volumes/appliances/$slug.json"
[[ $mode == insert || $mode == final ]] && python3 volume-insert.py "volumes/appliances/$slug.json" "$asset"
[[ $mode == validate || $mode == final ]] && { $BUN volume-check.ts "$slug"; $BUN appliances-b-validate.ts "$slug"; }
true
