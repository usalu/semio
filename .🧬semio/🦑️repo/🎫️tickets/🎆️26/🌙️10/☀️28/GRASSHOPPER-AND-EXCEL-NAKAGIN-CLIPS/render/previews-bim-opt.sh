#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp" PYTHONUTF8=1
mkdir -p "$T/🗑️generated/preview" "$TMP"
while read -r script scene output; do
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do sleep 5; done
  ( "$PY" "$T/render/preview_any.py" "$script" "$scene" "$output" en > "$T/🗑️generated/preview/$output-en.log" 2>&1
    echo "RESULT $output exit=$? $(date +%T)" ) &
done < "$T/render/bim-opt.list"
wait; echo DONE
