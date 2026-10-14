#!/bin/zsh
# usage: ch5_run.sh BeatClass fps per
cd /Users/niloufarghandehariyoon/Developer/semio
CK=".repo/🎫/26/10/13/RESTYLE-ENERGYBALANCE-WITH-FUNDAMENTALS-GLYPHS/checks"
LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir "$CK/ch5_media" tutorial/energy/demand/EnergyBalance/5_primary_energy/scene_5.py $1 2>&1 | grep -E "LAYOUT|Error|error|Traceback|warn|Warn" 
.venv/bin/python "$CK/ch5_sheet.py" $1 ${2:-1} ${3:-6}
