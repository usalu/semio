#!/bin/bash
# usage: preview.sh <builder.mjs> <asset> <view> <scale> <out-sheet-name> <pose...>
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/a1"
B="$1"; A="$2"; V="$3"; SC="$4"; N="$5"; shift 5
[ "$B" != "none" ] && repo tools exec bun -- "$T/🧰️a1/$B" 2>&1 | grep DEBUG
rm -rf "$G/$N"
for p in "$@"; do "$T/🧰️a1/render.sh" "$A" "$p" "$V" "$SC" "$G/$N" >/dev/null; done
repo tools exec bun -- "$T/🧰️a1/sheet.mjs" "$G/$N.png" 800 2 "$G/$N/"*.svg 2>&1 | grep -v "^\["
echo done
