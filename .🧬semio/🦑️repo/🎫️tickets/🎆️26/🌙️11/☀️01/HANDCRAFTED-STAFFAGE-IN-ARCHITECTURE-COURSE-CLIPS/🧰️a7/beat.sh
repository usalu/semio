#!/bin/bash
# usage: beat.sh <scene-file> <class> <name>
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/a7"
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
mkdir -p "$G/bin" "$G/media"
cp ".🧬semio/🎓️teaching/⚡️cache/python/Lib/site-packages/imageio_ffmpeg/binaries/ffmpeg-win-x86_64-v7.1.exe" "$G/bin/ffmpeg.exe"
export PATH="$PWD/$G/bin:$PATH" PYTHONUTF8=1 PYTHONIOENCODING=utf-8 PYTHONDONTWRITEBYTECODE=1
"$PY" -m manim render -ql --fps 15 --disable_caching --media_dir "$G/media" -o "$3" "$1" "$2" > "$G/$3.log" 2>&1
echo "exit $?" >> "$G/$3.log"
