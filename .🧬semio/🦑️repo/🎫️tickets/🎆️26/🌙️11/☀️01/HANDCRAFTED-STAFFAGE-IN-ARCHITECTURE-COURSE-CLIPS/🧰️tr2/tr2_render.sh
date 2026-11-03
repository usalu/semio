#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/tr2"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
PY=.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe
F="🎓️teaching/🏛️architecture/$1"; shift
mkdir -p "$G"
for s in "$@"; do
  rm -rf "$G/media-$s"
  $PY -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$s" "$F" "$s" > "$G/$s.log" 2>&1
  code=$?
  mp4=$(find "$G/media-$s" -name "$s.mp4" | head -1)
  [ -n "$mp4" ] && { cp "$mp4" "$G/$s.mp4"; $PY "$T/r2_frames.py" "$G/$s.mp4" "$G/png" 0.25,0.55,0.85 >> "$G/$s.log" 2>&1; }
  rm -rf "$G/media-$s"
  echo "$s exit $code" >> "$G/done.log"
done
