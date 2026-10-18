#!/bin/zsh
# usage: modvt.sh <suffix> : runs every module/framework vitest config (cwd = dir holding 🧪️tests), writes tsx-vt-<name>-<suffix>
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/tsx-tools/env.sh
SUF="$1"; shift
for line in "$@"; do
  name="${line%%=*}"; rest="${line#*=}"; dir="${rest%%|*}"; cfg="${rest#*|}"
  echo "### $name"
  "$T/vitest.sh" "$name-$SUF" "$R/$dir" "$cfg" 2>&1 | tail -2
done
