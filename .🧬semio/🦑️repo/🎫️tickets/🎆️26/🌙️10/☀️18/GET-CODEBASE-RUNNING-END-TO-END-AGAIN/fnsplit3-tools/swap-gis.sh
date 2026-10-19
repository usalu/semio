#!/bin/zsh
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit3-tools/paths.sh
T="$C/fnsplit3-tools"; V="$C/🗑️generated/fnsplit3-x/vt"; N="$C/🗑️generated/fnsplit3-x/newroots"
cd $R
cp "$F4" "$N/gis-worker.ts"
restore() { cp "$N/gis-worker.ts" "$F4"; }
trap restore EXIT INT TERM
cp "$T/orig/gis-worker.ts" "$F4"
VT_RES="$V/base-gis2.json" VT_CAP=1700 "$T/vt.sh" "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🎚️config/🟦️.ts" 2>&1 | head -1
restore
python3 "$T/vtsum.py" "$V/base-gis2.json" "$V/base-gis2.sum.json"
python3 "$T/vtdiff.py" "$V/base-gis2.sum.json" "$V/new-gis2.sum.json" | cut -c1-300
