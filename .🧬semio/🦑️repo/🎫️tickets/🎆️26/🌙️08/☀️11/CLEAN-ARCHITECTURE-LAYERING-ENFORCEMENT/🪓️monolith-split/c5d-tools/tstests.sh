#!/bin/sh
# C5D: runs the plugin TS laws that read the cut sources, one bun process each, logs to the generated folder.
cd /Users/ueli/Documents/semio
ulimit -n 65536
i=0
for f in \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🕰️history/📐️planning/🧪️tests/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️media-slots/🎟️collect/🧪️tests/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏇️mounted-owner/🧪️tests/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-runtime-close-budget/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/♻️retirement/🧪️tests/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧪️tests/🧩️history-partition/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🟦️.ts" \
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/🟦️.ts"
do
  i=$((i+1))
  echo "=== $i $f"
  nice -n 10 perl -e 'alarm 900; exec @ARGV' -- bun test "./$f" 2>&1 | /usr/bin/grep -E '^\((pass|fail|skip)\)|^ *[0-9]+ (pass|fail|skip)|^Ran |^error:|Segmentation|^ +at <anonymous>' | cut -c1-300
done
