#!/bin/zsh
# 🧊️ Gate-then-run for this lane's wgpu check on 6118.
#
# 🐛️ The gate matches `^bun .*wgpu-` — the RUNNING probe — not the probe's NAME. A name-shaped gate
# (`pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e'`) matches every peer agent's own gate shell, because
# those shells carry the literal `wgpu-battery.mjs` in their command lines: five such shells were
# waiting on each other on 2026-09-15 with the port completely free, for over three hours.
# `project-puzzle3d-battery-method`'s self-excluding port gate, one layer further out.
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END"
cd "$TICKET" || exit 1
while [ -n "$(pgrep -f '^bun .*wgpu-')" ]; do sleep 5; done
date
bun 🐍️wgpu-battery.mjs --only=frame-loop,examples
echo "EXIT=$?"
date
