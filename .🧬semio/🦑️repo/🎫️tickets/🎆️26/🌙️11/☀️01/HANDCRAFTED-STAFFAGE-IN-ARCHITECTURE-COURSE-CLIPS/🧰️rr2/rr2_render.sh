#!/bin/bash
# rr2_render.sh <series-dir-relative-to-energy> <Scene> : one render, ql 15 fps, jobs 1
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/rr2/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/$1/🐍️.py"
mkdir -p "$T/🗑️generated/rr2"
s="$2"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/rr2/media" "$F" "$s" > "$T/🗑️generated/rr2/$s.log" 2>&1
echo "$s exit $?" >> "$T/🗑️generated/rr2/done.log"
