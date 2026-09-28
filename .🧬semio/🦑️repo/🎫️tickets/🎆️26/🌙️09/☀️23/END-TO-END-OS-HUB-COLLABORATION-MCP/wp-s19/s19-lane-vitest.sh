#!/bin/zsh
# 🧪️ S19 vitest suites in ONE `overlay`-lane hold: the flow-extensions host laws inside the scratch overlay (kernel
# `scopeContributionsJson`, engine contributions push / window fault / wgpu extension dispatch) and the live-tree
# spawned-program-session law (contributions receiver). `SEMIO_TEST_LEVEL=standard`: the engine config includes its
# listed suites only above `fundamental`/`quick`. usage: zsh s19-lane-vitest.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"; T="/Users/ueli/Documents/semio"
V="$T/node_modules/.bin/vitest"; M="$T/.tmp-ticket/📜️fleet-mutex.sh"
ENGINE="🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript"
echo "START $(date '+%T')" > "$capture"
export SEMIO_TEST_LEVEL=standard
zsh $M overlay s19 -- zsh -c "
cd '$O/🧰️framework/🔨️modules/🎠️kernel' && nice -n 15 '$V' run --config '🧪️tests/🎚️config/🟦️.ts' scope-contributions; echo OVERLAY_KERNEL_RC=\$?
cd '$O/$ENGINE' && nice -n 15 '$V' run --config '../../🧪️tests/🎚️config/🟦️.ts' contributions-push window-fault wgpu-extension-dispatch; echo OVERLAY_ENGINE_RC=\$?
cd '$T/$ENGINE' && nice -n 15 '$V' run --config '../../🧪️tests/🎚️config/🟦️.ts' spawned-program-session; echo LIVE_SPAWNED_RC=\$?
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
