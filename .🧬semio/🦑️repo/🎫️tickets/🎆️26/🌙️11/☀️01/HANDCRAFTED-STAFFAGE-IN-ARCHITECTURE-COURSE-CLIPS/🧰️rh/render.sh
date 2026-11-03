#!/bin/bash
# render.sh <series-dir> <Scene>... : one scene at a time, ql 15 fps, de, speaker; then 8-frame sheet
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/rh2"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
PY=.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe
series="$1"; shift
F="🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/$series/🐍️.py"
mkdir -p "$G"
for s in "$@"; do
  rm -rf "$G/media"
  $PY -m manim -ql --fps 15 --disable_caching --media_dir "$G/media" "$F" "$s" > "$G/$s.log" 2>&1
  code=$?
  mp4=$(find "$G/media" -name "$s.mp4" | head -1)
  [ -n "$mp4" ] && cp "$mp4" "$G/$s.mp4" && $PY "$T/🧰️rh/sheet.py" "$G/$s.mp4" "$G/$s" > "$G/$s.sheet.log" 2>&1
  echo "$s exit $code" >> "$G/done.log"
done
rm -rf "$G/media"
