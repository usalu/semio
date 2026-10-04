#!/usr/bin/env bash
# Render Modul 1 Beats 1–5 (realistic wall section) one after another with the layout guard; contact sheets per beat.
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../.." && pwd)"
cd "$ROOT"
BEATS="${*:-Beat1_DreiWegeDerWaerme Beat2_Waermeleitung Beat3_Konvektion Beat4_Strahlung Beat5_Zusammenfassung}"
mkdir -p /tmp/heat_m1r/sheets
for B in $BEATS; do
  OUT="/tmp/heat_m1r/$B"; LOG="$HERE/m1r_$B.log"
  LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$OUT" tutorial/energy/demand/Heating/1_introduction/scene_1.py "$B" > "$LOG" 2>&1
  echo "== $B: $(grep -c '\[LAYOUT\]' "$LOG") layout, $(grep -c 'Traceback' "$LOG") tracebacks"
  grep -E "\[LAYOUT\]|Error|Exception" "$LOG" | grep -v SyntaxWarning | head -12
  MP4="$OUT/videos/scene_1/480p15/$B.mp4"
  rm -f /tmp/heat_m1r/sheets/${B}_*.png
  [ -f "$MP4" ] && ffmpeg -v error -y -i "$MP4" -vf "fps=1/2,scale=427:240,tile=4x4" "/tmp/heat_m1r/sheets/${B}_%02d.png"
done
ls /tmp/heat_m1r/sheets
