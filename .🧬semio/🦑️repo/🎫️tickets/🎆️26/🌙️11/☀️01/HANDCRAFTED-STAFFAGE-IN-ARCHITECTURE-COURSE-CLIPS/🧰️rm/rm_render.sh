#!/bin/bash
# rm_render.sh <series-path-under-energy> <Scene> : one scene, ql 15 fps, de
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/rm/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/$1/🐍️.py"
mkdir -p "$T/🗑️generated/rm"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/rm/media" "$F" "$2" > "$T/🗑️generated/rm/$2.log" 2>&1
echo "$2 exit $?" >> "$T/🗑️generated/rm/done.log"
