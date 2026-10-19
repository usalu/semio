#!/bin/zsh
# usage: swap-run.sh <outPrefix>: runs the react engine config against the ORIGINAL ShellHost roots (host-effects, shell-actions, backbone-worker), then restores the new roots
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit3-tools/paths.sh
T="$C/fnsplit3-tools"; V="$C/🗑️generated/fnsplit3-x/vt"; N="$C/🗑️generated/fnsplit3-x/newroots"
cd $R
restore() { cp "$N/host-effects.ts" "$F7"; cp "$N/shell-actions.ts" "$F2"; cp "$N/backbone-worker.ts" "$F9"; }
trap restore EXIT INT TERM
cp "$T/orig/host-effects.ts" "$F7"; cp "$T/orig/shell-actions.ts" "$F2"; cp "$T/orig/backbone-worker.ts" "$F9"
VT_RES="$V/$1.json" VT_CAP=1500 "$T/vt.sh" "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts" 2>&1 | head -1
restore
python3 "$T/vtsum.py" "$V/$1.json" "$V/$1.sum.json"
