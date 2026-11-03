#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/op"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/🎯️optimization/🎬️clip/$1/🐍️.py"; shift
for s in "$@"; do
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media" "$F" "$s" > "$G/$s.log" 2>&1
  echo "$s exit $?" >> "$G/done.log"
done
