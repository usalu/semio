#!/usr/bin/env bash
# Render one Modul 3 beat at -ql with the layout guard and build a frame contact sheet.
BEAT="$1"; EVERY="${2:-2}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT="${M3_MEDIA:-/tmp/heat_m3/$BEAT}"; SHEETS=/tmp/heat_m3/sheets
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="$HERE/m3_${BEAT}.log"
cd "$ROOT"
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT" tutorial/energy/demand/Heating/3_convection/scene_3.py "$BEAT" > "$LOG" 2>&1
grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception|caption_bar wrapped" "$LOG" | grep -v SyntaxWarning | head -40
MP4="$OUT/videos/scene_3/480p15/$BEAT.mp4"
ffprobe -v error -show_entries format=duration -of csv=p=0 "$MP4"
mkdir -p "$SHEETS"
rm -f "$SHEETS/${BEAT}_"*.png
ffmpeg -v error -y -i "$MP4" -vf "fps=1/$EVERY,scale=427:240,tile=4x4" "$SHEETS/${BEAT}_%02d.png"
ls "$SHEETS" | grep "$BEAT"
