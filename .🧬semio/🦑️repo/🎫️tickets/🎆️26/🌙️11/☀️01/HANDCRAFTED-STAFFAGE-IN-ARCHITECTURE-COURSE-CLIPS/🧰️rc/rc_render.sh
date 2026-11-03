#!/bin/bash
# rc_render.sh <series-dir> <Scene>... : renders cooling scenes one at a time, ql 15 fps, de, speaker; keeps the mp4
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/rc"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=de SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
series="$1"; shift
F="🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/🎬️clip/$series/🐍️.py"
mkdir -p "$G"
for s in "$@"; do
  rm -rf "$G/media-$s"
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$s" "$F" "$s" > "$G/$s.log" 2>&1
  code=$?
  mp4=$(find "$G/media-$s" -name "$s.mp4" | head -1)
  [ -n "$mp4" ] && cp "$mp4" "$G/$s.mp4"
  rm -rf "$G/media-$s"
  echo "$s exit $code" >> "$G/done.log"
done
