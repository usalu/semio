#!/bin/bash
# [DEBUG] sequential layout-check renders for phase 2 Cooling (one media dir per render)
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/p2c"
run() { # name scene_file beat
  d="$T/$1/media"; mkdir -p "$T/$1"
  LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir "$d" "$2" "$3" > "$T/$1/$3.full.log" 2>&1
  grep -a "\[LAYOUT\]\|Traceback\|Error" "$T/$1/$3.full.log" > "$T/$1/$3.log"
  echo "[DEBUG] done $3: $(wc -l < "$T/$1/$3.log") lines" >> "$T/progress3.txt"
}
for item in "$@"; do
  IFS=: read -r name file beat <<< "$item"
  run "$name" "$file" "$beat"
done
echo "[DEBUG] ALL DONE" >> "$T/progress3.txt"
