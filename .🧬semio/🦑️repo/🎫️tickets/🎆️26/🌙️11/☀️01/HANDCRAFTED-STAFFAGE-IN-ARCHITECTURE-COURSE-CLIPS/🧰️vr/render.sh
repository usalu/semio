#!/bin/bash
# render.sh <lang> <clip-dir relative to energy/> <Scene>... : one scene at a time, ql 15 fps, speaker
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/vr"
lang="$1"; dir="$2"; shift 2
export PYTHONPATH=/c/git/semio SEMIO_TEACHING_LANGUAGE=$lang SEMIO_TEACHING_AUDIENCE=speaker SEMIO_TEACHING_OUTPUT_ROOT="$G/out" PYTHONUTF8=1
F="🎓️teaching/🏛️architecture/⚡️energy/$dir/🐍️.py"
mkdir -p "$G"
for s in "$@"; do
  .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe -m manim -ql --fps 15 --disable_caching --media_dir "$G/media-$lang" "$F" "$s" > "$G/$lang-$s.log" 2>&1
  echo "$lang $s exit $?" >> "$G/done.log"
done
