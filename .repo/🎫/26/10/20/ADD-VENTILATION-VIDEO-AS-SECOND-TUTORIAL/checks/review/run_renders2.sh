#!/bin/bash
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/review"
V=tutorial/energy/demand/Ventilation
while pgrep -f run_renders.sh >/dev/null; do sleep 5; done
while read -r folder n beat; do
  rm -rf $T/media_$beat
  LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir $T/media_$beat $V/$folder/scene_$n.py $beat > $T/logs/${n}_$beat.log 2>&1
  echo "R2 $beat layout=$(grep -c LAYOUT $T/logs/${n}_$beat.log) err=$(grep -c Traceback $T/logs/${n}_$beat.log)" >> $T/logs/summary.txt
done <<LIST
3_waermestrom_luft 3 Beat2_SensibleWaerme
3_waermestrom_luft 3 Beat3_LatenteLast
3_waermestrom_luft 3 Beat4_WinterUndSommer
4_natuerliche_lueftung 4 Beat6_GrenzenDerFreienLueftung
1_warum_lueften 1 Beat3_Fahrplan
2_luftmenge 2 Beat1_Innenvolumen
2_luftmenge 2 Beat2_Luftwechselrate
2_luftmenge 2 Beat3_Volumenstrom
5_mechanische_lueftung 5 Beat2_Waermerueckgewinnung
LIST
echo ALLDONE2 >> $T/logs/summary.txt
