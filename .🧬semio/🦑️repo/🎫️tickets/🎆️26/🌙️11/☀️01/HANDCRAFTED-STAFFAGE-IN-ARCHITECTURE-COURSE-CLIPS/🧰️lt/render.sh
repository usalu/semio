#!/bin/bash
# r2_render.sh <series-dir> <Scene>... : renders scenes one at a time, ql 15 fps, de, speaker
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=${LANG_R:-de} SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/r2/out" PYTHONUTF8=1
series="$1"; shift
F="🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/$series/🐍️.py"
mkdir -p "$T/🗑️generated/r2"
for s in "$@"; do
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/r2/media" "$F" "$s" > "$T/🗑️generated/r2/$s.log" 2>&1
  echo "$s exit $?" >> "$T/🗑️generated/r2/done.log"
done
