#!/bin/bash
# 🎞️ render.sh <lang> <relative clip dir under energy> <Scene> : one scene, ql 15 fps, speaker audience
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/lc"
lang="$1"; dir="$2"; scene="$3"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE="$lang" SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out-$lang" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/$dir/🐍️.py"
mkdir -p "$G"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$lang" "$F" "$scene" > "$G/$scene-$lang.log" 2>&1
echo "$scene $lang exit $?" >> "$G/render-done.log"
