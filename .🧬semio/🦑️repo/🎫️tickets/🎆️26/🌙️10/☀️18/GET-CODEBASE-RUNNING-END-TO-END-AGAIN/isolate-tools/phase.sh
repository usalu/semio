#!/bin/sh
# 🔁 [isolate] prepares mirror is1 for one verification phase of a batch: `base` copies the live bytes of every batch file into the mirror and turns the fence off; `new` converts the batch from live into the mirror and turns the fence on. usage: phase.sh base|new <batch>
C="$(cd "$(dirname "$0")/.." && pwd)"; M=/private/tmp/m/is1; L=/Users/ueli/Documents/semio; b="$2"
case "$1" in
 base) while read -r f; do [ -n "$f" ] && cp "$L/$f" "$M/$f"; done < "$C/isolate-tools/batches/$b.txt"; sh "$C/isolate-tools/fence.sh" off "$M";;
 new) python3 -I "$C/isolate-tools/convert.py" "$b" "$L" "$M" | grep -cv "^same" ; sh "$C/isolate-tools/fence.sh" on "$M";;
esac
