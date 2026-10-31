#!/usr/bin/env bash
# usage: previews.sh <jobs> <lang>...   renders all nine previews through preview.py, <jobs> at a time
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export TMP="$PWD/$T/🗑️generated/tmp" TEMP="$PWD/$T/🗑️generated/tmp" PYTHONUTF8=1
jobs=$1; shift
mkdir -p "$T/🗑️generated/preview" "$TMP"
VIDEOS="🦗️grasshopper 💊️capsule Beat2_Capsule Grasshopper_capsule
🦗️grasshopper 🗼️tower Beat3_Tower Grasshopper_tower
🦗️grasshopper 🌳️trees Beat5_DataTrees Grasshopper_trees
🦗️grasshopper 🫗️casting Beat4_Casting Grasshopper_casting
🦗️grasshopper 🦴️anatomy Beat1_Anatomy Grasshopper_anatomy
📊️excel 🦴️anatomy Beat1_Anatomy Excel_anatomy
📊️excel 📍️references Beat2_References Excel_references
📊️excel 📋️program Beat3_Program Excel_program
📊️excel ⚡️energy Beat4_Energy Excel_energy"
for lang in "$@"; do
  while read -r series topic scene output; do
    while [ "$(jobs -rp | wc -l)" -ge "$jobs" ]; do sleep 5; done
    ( "$PY" "$T/render/preview.py" "$series" "$topic" "$scene" "$output" "$lang" > "$T/🗑️generated/preview/$output-$lang.log" 2>&1
      echo "RESULT $output $lang exit=$? $(date +%T)" ) &
  done <<< "$VIDEOS"
done
wait
echo DONE
