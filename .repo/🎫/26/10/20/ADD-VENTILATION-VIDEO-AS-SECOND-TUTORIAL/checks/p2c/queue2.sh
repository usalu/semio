#!/bin/bash
# [DEBUG] wait for the first queue, then run the second
T="/Users/niloufarghandehariyoon/Developer/semio/.repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/p2c"
until grep -q "ALL DONE" "$T/progress.txt" 2>/dev/null; do sleep 10; done
"$T/render_seq2.sh" \
 "c6_b1:tutorial/energy/demand/Cooling/6_sommerstrategie/scene_6.py:Beat1_LastZuerstSenken" \
 "c5_b3:tutorial/energy/demand/Cooling/5_systemauslegung/scene_5.py:Beat3_IsolateAirflow" \
 "c5_b2:tutorial/energy/demand/Cooling/5_systemauslegung/scene_5.py:Beat2_VolumeFlowEquation" \
 "c6_b3:tutorial/energy/demand/Cooling/6_sommerstrategie/scene_6.py:Beat3_KomfortStrategie" \
 "c6_b2:tutorial/energy/demand/Cooling/6_sommerstrategie/scene_6.py:Beat2_SorptionsKuehlung" \
 "c3:tutorial/energy/demand/Cooling/3_transmission_humidity/scene_3.py:Beat3_LueftungslastImSommer"
