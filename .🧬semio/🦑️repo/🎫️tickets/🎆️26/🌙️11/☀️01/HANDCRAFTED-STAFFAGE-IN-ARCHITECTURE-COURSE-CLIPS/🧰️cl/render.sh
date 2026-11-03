#!/bin/bash
# r2_render.sh <series-dir> <Scene>... : renders scenes one at a time, ql 15 fps, de, speaker
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/cl/out" PYTHONUTF8=1
series="$1"; shift
F="🎓️teaching/🏛️architecture/⚡️energy/$series/🐍️.py"
mkdir -p "$T/🗑️generated/cl"
for s in "$@"; do
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/cl/media" "$F" "$s" > "$T/🗑️generated/cl/$s.log" 2>&1
  echo "$s exit $?" >> "$T/🗑️generated/cl/done.log"
done
