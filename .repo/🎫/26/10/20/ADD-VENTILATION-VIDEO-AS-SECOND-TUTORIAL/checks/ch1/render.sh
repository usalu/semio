#!/bin/zsh
# usage: render.sh BeatClass  -> renders at -ql with layout guard, prints [LAYOUT] lines, builds contact sheets
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/ch1"
B=$1
LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir "$T/media" tutorial/energy/demand/Ventilation/1_warum_lueften/scene_1.py $B 2>&1 | tee "$T/$B.log" > /dev/null
grep -n "LAYOUT\|Error\|Warning" "$T/$B.log"
"$T/sheet.sh" $B ${2:-0.7}
