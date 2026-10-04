#!/usr/bin/env bash
# usage: render_m14.sh <m1|m4> <BeatClass>...
here="$(cd "$(dirname "$0")" && pwd)"
cd "$here/../../../../../../.." || exit 1
mod="$1"; shift
case "$mod" in
  m1) file=tutorial/energy/demand/Heating/1_introduction/scene_1.py; stem=scene_1 ;;
  m4) file=tutorial/energy/demand/Heating/4_internal_heat_gain/scene_4.py; stem=scene_4 ;;
esac
media=/tmp/heat_$mod
logdir="$here/logs"
mkdir -p "$media/sheets" "$logdir"
for beat in "$@"; do
  (
    LAYOUT_CHECK=1 .venv/bin/manim -ql --disable_caching --media_dir "$media" "$file" "$beat" > "$logdir/${mod}_${beat}.log" 2>&1
    rm -f "$media/sheets/${beat}_"*.png
    ffmpeg -v error -y -i "$media/videos/$stem/480p15/$beat.mp4" -vf "fps=1/2,scale=427:240,tile=4x4" "$media/sheets/${beat}_%02d.png"
    echo "== $beat"; grep -E "\[LAYOUT\]|Error|Traceback|File \"|line [0-9]+|Exception" "$logdir/${mod}_${beat}.log" | grep -v SyntaxWarning | head -40
  )
done
wait
