#!/usr/bin/env bash
# Render one PF beat at -ql with the layout guard and build a frame contact sheet.
set -e
BEAT="$1"; EVERY="${2:-2}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="${PF_MEDIA:-/tmp/pf_media}"
cd "$ROOT"
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT" tutorial/energy/demand/1_physical_fundamentals/scene_1.py "$BEAT" 2>&1 \
  | grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception|Rendered|warn" | grep -v SyntaxWarning | head -60
MP4="$OUT/videos/scene_1/480p15/$BEAT.mp4"
ffprobe -v error -show_entries format=duration -of csv=p=0 "$MP4"
mkdir -p "$OUT/sheets"
ffmpeg -v error -y -i "$MP4" -vf "fps=1/$EVERY,scale=427:240,tile=4x4" "$OUT/sheets/${BEAT}_%02d.png"
ls "$OUT/sheets" | grep "$BEAT"
