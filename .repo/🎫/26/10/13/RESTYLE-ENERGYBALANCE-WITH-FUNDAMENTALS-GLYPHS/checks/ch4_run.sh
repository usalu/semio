#!/bin/bash
# usage: ch4_run.sh BeatClass [step] [cols]
cd "$(dirname "$0")"
B=$1; STEP=${2:-2}; COLS=${3:-2}
SCENE=/Users/niloufarghandehariyoon/Developer/semio/tutorial/energy/demand/EnergyBalance/4_system_losses/scene_4.py
LAYOUT_CHECK=1 /Users/niloufarghandehariyoon/Developer/semio/.venv/bin/manim -ql --media_dir ch4_media $SCENE $B > ch4_log_$B.txt 2>&1
grep -E "LAYOUT|Error|Traceback|rror:|warn|File " ch4_log_$B.txt | head -60
/Users/niloufarghandehariyoon/Developer/semio/.venv/bin/python ch4_sheet.py ch4_media/videos/scene_4/480p15/$B.mp4 sheets/ch4_${B} $STEP $COLS
