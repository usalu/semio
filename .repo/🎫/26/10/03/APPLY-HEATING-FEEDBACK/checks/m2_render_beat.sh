#!/usr/bin/env bash
# Render one Modul 2 beat at -ql with the layout guard and build a frame contact sheet.
BEAT="$1"; EVERY="${2:-2}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="/tmp/heat_m2"
LOG="$(cd "$(dirname "$0")" && pwd)/m2_render_${BEAT}.log"
cd "$ROOT"
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT" tutorial/energy/demand/Heating/2_conduction/scene_2.py "$BEAT" > "$LOG" 2>&1
grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception|caption_bar wrapped" "$LOG" | grep -v SyntaxWarning | head -60
MP4="$OUT/videos/scene_2/480p15/$BEAT.mp4"
ffprobe -v error -show_entries format=duration -of csv=p=0 "$MP4"
mkdir -p "$OUT/sheets"
rm -f "$OUT/sheets/${BEAT}_"*.png
ffmpeg -v error -y -i "$MP4" -vf "fps=1/$EVERY,scale=427:240,tile=4x4" "$OUT/sheets/${BEAT}_%02d.png"
ls "$OUT/sheets" | grep "$BEAT"
