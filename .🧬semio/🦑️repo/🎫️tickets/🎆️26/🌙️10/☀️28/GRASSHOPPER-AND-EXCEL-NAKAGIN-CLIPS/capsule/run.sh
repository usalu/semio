#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️28/GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS"
L=${1:-en}
shift
PYTHONIOENCODING=utf-8 .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe "$T/capture.py" 🦗️grasshopper 💊️capsule Beat2_Capsule "$L" "$@" > "$T/🗑️generated/capsule/run-$L.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/capsule/run-$L.log"
