#!/bin/bash
# render_en.sh <clip path relative to energy dir> <Scene> <tag> : one en render, ql 15 fps, jobs 1
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/lc3"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/$1/🐍️.py"
mkdir -p "$G"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$3" "$F" "$2" > "$G/$3.log" 2>&1
echo "$3 exit $?" >> "$G/done.log"
