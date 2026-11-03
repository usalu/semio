#!/bin/bash
# render.sh <series-dir> <Scene>...: renders scenes one at a time (ql, 15 fps, de) then 8-frame sheets; appends to done.log
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/rd2"
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=${LANGX:-de} SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
PY=.🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe
series="$1"; shift
F="🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/$series/🐍️.py"
mkdir -p "$G"
for s in "$@"; do
  rm -rf "$G/media"
  $PY -m manim -ql --fps 15 --disable_caching --media_dir "$G/media" "$F" "$s" > "$G/$s.log" 2>&1
  mp4=$(find "$G/media" -name "$s.mp4" | head -1)
  if [ -n "$mp4" ]; then
    mkdir -p "$G/mp4"; cp "$mp4" "$G/mp4/$s.mp4"
    $PY "$T/🧰️rd2/sheet.py" "$G/mp4/$s.mp4" "$G/sheets" ${N:-8} >> "$G/done.log" 2>&1
    echo "$s ok" >> "$G/done.log"
  else
    echo "$s FAILED" >> "$G/done.log"; tail -5 "$G/$s.log" >> "$G/done.log"
  fi
  rm -rf "$G/media"
done
