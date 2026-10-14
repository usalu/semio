#!/bin/bash
# usage: ch3_run.sh BeatClass [step] [cols]
cd "$(dirname "$0")"
B=$1; STEP=${2:-3}; COLS=${3:-2}
SCENE=/Users/niloufarghandehariyoon/Developer/semio/tutorial/energy/demand/EnergyBalance/3_bedarf_vs_leistung/scene_3.py
LAYOUT_CHECK=1 /Users/niloufarghandehariyoon/Developer/semio/.venv/bin/manim -ql --media_dir ch3_media $SCENE $B > ch3_log_$B.txt 2>&1
grep -E "LAYOUT|Error|Traceback|rror:|warn" ch3_log_$B.txt | head -60
/Users/niloufarghandehariyoon/Developer/semio/.venv/bin/python ch3_sheet.py ch3_media/videos/scene_3/480p15/$B.mp4 sheets/ch3_${B} $STEP $COLS
