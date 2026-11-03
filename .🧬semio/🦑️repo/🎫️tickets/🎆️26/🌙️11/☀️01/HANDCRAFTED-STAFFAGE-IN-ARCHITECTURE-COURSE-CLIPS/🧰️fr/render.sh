#!/bin/bash
# render.sh <clip dir under energy> <Scene>: one ql 15 fps render (jobs 1), de, speaker
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/fr/out" PYTHONUTF8=1
mkdir -p "$T/🗑️generated/fr"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/fr/media" "🎓️teaching/🏛️architecture/⚡️energy/$1/🐍️.py" "$2" > "$T/🗑️generated/fr/r_$2.log" 2>&1
echo "$2 exit $?"
find "$T/🗑️generated/fr/media" -name "$2.mp4"
