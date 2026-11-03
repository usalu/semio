#!/bin/bash
# render.sh <domain> <clip> <Scene> [lang]: one scene, ql 15 fps, speaker audience
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/lc2"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE="${4:-en}" SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/$1/🎬️clip/$2/🐍️.py"
mkdir -p "$G"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media" "$F" "$3" > "$G/$3.log" 2>&1
echo "$3 exit $?"
tail -5 "$G/$3.log"
