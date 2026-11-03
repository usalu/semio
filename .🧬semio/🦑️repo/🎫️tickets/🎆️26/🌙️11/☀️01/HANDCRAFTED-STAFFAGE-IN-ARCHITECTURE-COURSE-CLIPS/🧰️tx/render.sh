#!/bin/bash
# render.sh <lang> <clip-py> <Scene>
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=$1 SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/tx/out" PYTHONUTF8=1
mkdir -p "$T/🗑️generated/tx"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/tx/media-$1" "$2" "$3" > "$T/🗑️generated/tx/$1-$3.log" 2>&1
echo "$1 $3 exit $?"
