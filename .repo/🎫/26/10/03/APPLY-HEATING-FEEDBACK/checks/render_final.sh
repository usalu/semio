#!/usr/bin/env bash
# Render one final-calculation scene at -ql with the layout guard and build a contact sheet.
BEAT="$1"; EVERY="${2:-2}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="/tmp/heat_final"
cd "$ROOT"
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT" tutorial/energy/demand/Heating/final_calculation/merged_scenes.py "$BEAT" 2>&1 \
  | grep -E "\[LAYOUT\]|\[DEBUG\]|Error|Traceback|File \"|line [0-9]+|Exception|warn" | grep -v SyntaxWarning | head -60
MP4="$OUT/videos/merged_scenes/480p15/$BEAT.mp4"
ffprobe -v error -show_entries format=duration -of csv=p=0 "$MP4"
mkdir -p "$OUT/sheets"
rm -f "$OUT/sheets/${BEAT}_"*.png
ffmpeg -v error -y -i "$MP4" -vf "fps=1/$EVERY,scale=427:240,tile=4x4" "$OUT/sheets/${BEAT}_%02d.png"
ls "$OUT/sheets" | grep "^${BEAT}_"
