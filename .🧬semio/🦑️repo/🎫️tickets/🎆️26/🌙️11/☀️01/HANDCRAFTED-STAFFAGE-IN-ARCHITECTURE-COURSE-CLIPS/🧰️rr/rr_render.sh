#!/bin/bash
# rr_render.sh <script rel to energy dir> <Scene> <tag> <fractions>
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/rr"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=en SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
PY=.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe
$PY -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$3" "🎓️teaching/🏛️architecture/⚡️energy/$1" "$2" > "$G/$3.log" 2>&1
mp4=$(find "$G/media-$3" -name "$2.mp4" | head -1)
if [ -n "$mp4" ]; then cp "$mp4" "$G/$3.mp4"; $PY "$T/r2_frames.py" "$G/$3.mp4" "$G/$3" "$4"; rm -rf "$G/media-$3"; else tail -15 "$G/$3.log"; fi
