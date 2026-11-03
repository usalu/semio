#!/bin/bash
cd /c/git/semio
T=$(ls -d .🧬semio/*repo/*tickets/*26/*11/*01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)
G=$(cd "$T" && ls -d *generated*)
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$T/$G/rd/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/🧮️calculation/🐍️.py"
mkdir -p "$T/$G/rd/media"
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$T/$G/rd/media" "$F" AnlagenVerluste > "$T/$G/rd/av.log" 2>&1
echo "exit $?" >> "$T/$G/rd/av.log"
