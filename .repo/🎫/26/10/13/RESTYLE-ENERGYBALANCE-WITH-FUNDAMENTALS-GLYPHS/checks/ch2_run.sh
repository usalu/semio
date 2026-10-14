#!/bin/bash
# usage: ch2_run.sh BeatClass [step] [cols]
cd "$(dirname "$0")"
B=$1; STEP=${2:-3}; COLS=${3:-2}
SCENE=/Users/niloufarghandehariyoon/Developer/semio/tutorial/energy/demand/EnergyBalance/2_regulatory_framework/scene_2.py
LAYOUT_CHECK=1 /Users/niloufarghandehariyoon/Developer/semio/.venv/bin/manim -ql --media_dir ch2_media $SCENE $B > ch2_log_$B.txt 2>&1
grep -E "LAYOUT|Error|Traceback|rror:|warn" ch2_log_$B.txt | head -60
/Users/niloufarghandehariyoon/Developer/semio/.venv/bin/python ch2_sheet.py ch2_media/videos/scene_2/480p15/$B.mp4 sheets/ch2_${B} $STEP $COLS
