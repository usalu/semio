#!/bin/sh
cd /Users/ueli/Documents/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun)
for s in "🧊️volumes-rules" "🧊️model-silhouette" "✅️asset-validation" "🧊️volume-schema" "🪤️volume-traps"; do
  VS_TEST="🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/$s/🟦️.ts" nice $BUN x vitest run --config "$T/vs-vitest.config.ts" > "$T/🗑️generated/vs/after-swap-16-$s.log" 2>&1
done
