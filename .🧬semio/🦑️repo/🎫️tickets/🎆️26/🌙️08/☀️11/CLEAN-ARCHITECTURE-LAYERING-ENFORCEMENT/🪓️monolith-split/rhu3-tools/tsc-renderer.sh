#!/bin/sh
# usage: tsc-renderer.sh <label>
cd /Users/ueli/Documents/semio || exit 2
perl -e 'alarm 3000; exec @ARGV' -- node node_modules/typescript/bin/tsc -p '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json' --noEmit --incremental false --pretty false > ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/rhu3-tsc-$1.log" 2>&1
echo $? > ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated/rhu3-tsc-$1.status"
