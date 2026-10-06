#!/usr/bin/env bash
# Render every Heating section at -ql with the layout guard, then concat the six section videos (no intro card: tutorial/intro/assets is not in the repo).
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../.." && pwd)"
cd "$ROOT"
MEDIA=tutorial/energy/demand/Heating/media
OUT=$MEDIA/videos/full_heating_video/480p15
SECTIONS="${*:-Heating_01_Introduction Heating_02_Conduction Heating_03_Convection Heating_04_InternalGains Heating_05_Solar Heating_06_FinalCalculation}"
for S in $SECTIONS; do
  LOG="$HERE/full_ql_$S.log"
  LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$MEDIA" tutorial/energy/demand/Heating/full_heating_video.py "$S" > "$LOG" 2>&1
  echo "== $S: $(grep -c '\[LAYOUT\]' "$LOG") layout, $(grep -c Traceback "$LOG") tracebacks"
  grep "\[LAYOUT\]" "$LOG" | head -8
done
LIST="$HERE/full_ql_concat.txt"
: > "$LIST"
for S in Heating_01_Introduction Heating_02_Conduction Heating_03_Convection Heating_04_InternalGains Heating_05_Solar Heating_06_FinalCalculation; do
  echo "file '$ROOT/$OUT/$S.mp4'" >> "$LIST"
done
mkdir -p tutorial/energy/demand/Heating/rendered
ffmpeg -v error -y -f concat -safe 0 -i "$LIST" -c:v copy -an "$OUT/FullHeatingDemandVideo.mp4"
cp "$OUT/FullHeatingDemandVideo.mp4" tutorial/energy/demand/Heating/rendered/Full_Heating_Demand_NoAudio_480p15.mp4
ffprobe -v error -show_entries format=duration -of csv=p=0 "$OUT/FullHeatingDemandVideo.mp4"
