#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/sc2/out" PYTHONUTF8=1
lang="$1"; file="$2"; scene="$3"
export SEMIO_TEACHING_LANGUAGE="$lang"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/sc2/media-$lang" "$file" "$scene" > "$T/🗑️generated/sc2/$scene-$lang.log" 2>&1
echo "$scene $lang exit $?"
