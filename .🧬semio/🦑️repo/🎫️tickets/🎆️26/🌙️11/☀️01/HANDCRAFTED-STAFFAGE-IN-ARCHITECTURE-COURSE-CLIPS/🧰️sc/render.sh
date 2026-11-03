#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/sc/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/🗑️generated/sc/media" "$F" Beat2_SorptionsKuehlung > "$T/🗑️generated/sc/render.log" 2>&1
echo "exit $?" >> "$T/🗑️generated/sc/render.log"
