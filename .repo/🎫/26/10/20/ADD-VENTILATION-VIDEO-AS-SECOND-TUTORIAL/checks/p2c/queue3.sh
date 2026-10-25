#!/bin/bash
# [DEBUG] wait for the second queue, then re-render Beat1 of part 6 after the chip text change
T="/Users/niloufarghandehariyoon/Developer/semio/.repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/p2c"
until grep -q "ALL DONE" "$T/progress2.txt" 2>/dev/null; do sleep 10; done
"$T/render_seq3.sh" "c6_b1r:tutorial/energy/demand/Cooling/6_sommerstrategie/scene_6.py:Beat1_LastZuerstSenken"
