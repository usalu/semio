#!/bin/bash
# [DEBUG] wait for the third queue, then render the three composer sections (one media dir each)
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/p2c"
until grep -q "ALL DONE" "$T/progress3.txt" 2>/dev/null; do sleep 10; done
for s in Cooling_03_TransmissionHumidity Cooling_05_Systemauslegung Cooling_06_Sommerstrategie; do
  mkdir -p "$T/sec_$s"
  LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir "$T/sec_$s/media" tutorial/energy/demand/Cooling/full_cooling_video.py "$s" > "$T/sec_$s/full.log" 2>&1
  grep -a "\[LAYOUT\]\|Traceback\|Error" "$T/sec_$s/full.log" > "$T/sec_$s/layout.log"
  echo "[DEBUG] section $s done: $(wc -l < "$T/sec_$s/layout.log") lines" >> "$T/progress4.txt"
done
echo "[DEBUG] ALL DONE" >> "$T/progress4.txt"
