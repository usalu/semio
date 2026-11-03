#!/usr/bin/env bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/ne"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql -s --fps 15 --disable_caching --media_dir "$G/media-still" "$T/🧰️rx/rx_plan_probe.py" PlanProbe > "$G/still.log" 2>&1
find "$G/media-still" -name "*.png"
tail -3 "$G/still.log"
