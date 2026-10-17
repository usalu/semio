#!/bin/bash
# 🏃️ Runs the POS oracle (read-only on the repo) for the given rsl family numbers, lib and test, label $LABEL. usage: runfam.sh <label> <n>...
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label="$1"; shift
for n in "$@"; do
  cd "$F/pos-tools" && POS_PREFIX=rsl nice python3 pos.py run "$F/rsl-tools/families/rsl$n.json" --cfgs "${CFGS:-lib,test}" --label "$label" > "$F/🗑️generated/rsl-fam$n-$label-console.log" 2>&1
  echo "rsl$n rc=$?" >> "$F/🗑️generated/rsl-runfam-$label.status"
done
