#!/bin/zsh
# 🧪️ S19 flow-extensions host laws inside the scratch overlay through the `overlay` lane: kernel `scopeContributionsJson`
# + the engine suites of the contributions push, window fault and wgpu extension dispatch. usage: zsh s19-vitest-overlay.sh <capture> [<root>]
capture="$1"; R="${2:-/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay}"
V=/Users/ueli/Documents/semio/node_modules/.bin/vitest
M=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
echo "START $(date '+%T') root=$R" > "$capture"
zsh $M overlay s19 -- zsh -c "cd '$R/🧰️framework/🔨️modules/🎠️kernel' && nice -n 15 $V run --config '🧪️tests/🎚️config/🟦️.ts' scope-contributions; echo KERNEL_RC=\$?; cd '$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript' && nice -n 15 $V run --config '../../🧪️tests/🎚️config/🟦️.ts' contributions-push window-fault wgpu-extension-dispatch; echo ENGINE_RC=\$?" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
