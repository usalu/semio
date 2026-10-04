#!/usr/bin/env bash
# Render one Heating Modul 5 beat at -ql with the layout guard and build a 2 s contact sheet.
BEAT="$1"; EVERY="${2:-2}"
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
OUT=/tmp/heat_m5
LOG="$(dirname "$0")/m5_${BEAT}.log"
cd "$ROOT"
LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT/$BEAT" tutorial/energy/demand/Heating/5_solar_heat_gain/scene_5.py "$BEAT" > "$LOG" 2>&1
grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception|caption_bar wrapped" "$LOG" | grep -v SyntaxWarning | head -40
MP4="$OUT/$BEAT/videos/scene_5/480p15/$BEAT.mp4"
echo "duration: $(ffprobe -v error -show_entries format=duration -of csv=p=0 "$MP4")"
mkdir -p "$OUT/sheets"
rm -f "$OUT/sheets/${BEAT}_"*.png
ffmpeg -v error -y -i "$MP4" -vf "fps=1/$EVERY,scale=427:240,tile=4x4" "$OUT/sheets/${BEAT}_%02d.png"
ls "$OUT/sheets" | grep "$BEAT"
