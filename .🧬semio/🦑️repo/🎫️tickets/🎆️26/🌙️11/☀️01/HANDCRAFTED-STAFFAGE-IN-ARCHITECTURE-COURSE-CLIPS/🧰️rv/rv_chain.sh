#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/rv"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
PY=.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe
for spec in "$@"; do
  series="${spec%%:*}"; rest="${spec#*:}"; scene="${rest%%:*}"; fr="${rest#*:}"
  F="🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip/$series/🐍️.py"
  $PY -m manim -ql --fps 15 --disable_caching --media_dir "$G/media" "$F" "$scene" > "$G/$scene.log" 2>&1
  mp4=$(find "$G/media" -name "$scene.mp4" | head -1)
  if [ -n "$mp4" ]; then
    $PY "$T/r2_frames.py" "$mp4" "$G/f" "$fr" > "$G/$scene.frames.log" 2>&1
    rm -rf "$G/media"
    echo "$scene ok" >> "$G/done.log"
  else
    echo "$scene FAILED" >> "$G/done.log"
  fi
done
echo ALLDONE >> "$G/done.log"
