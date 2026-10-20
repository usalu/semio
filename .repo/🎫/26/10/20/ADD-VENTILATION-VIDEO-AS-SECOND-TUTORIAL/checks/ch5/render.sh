#!/bin/bash
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/ch5"
S=tutorial/energy/demand/Ventilation/5_mechanische_lueftung/scene_5.py
for B in "$@"; do
  LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir "$T/media_$B" $S $B > "$T/$B.log" 2>&1
  echo "done $B rc=$?" >> "$T/status.log"
done
