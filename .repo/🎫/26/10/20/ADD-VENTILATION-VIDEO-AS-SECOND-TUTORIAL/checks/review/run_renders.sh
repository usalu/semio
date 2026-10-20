#!/bin/bash
cd /Users/niloufarghandehariyoon/Developer/semio
T=".repo/🎫/26/10/20/ADD-VENTILATION-VIDEO-AS-SECOND-TUTORIAL/checks/review"
V=tutorial/energy/demand/Ventilation
while read -r folder n beat; do
  LAYOUT_CHECK=1 .venv/bin/manim -ql --media_dir $T/media_$beat $V/$folder/scene_$n.py $beat > $T/logs/${n}_$beat.log 2>&1
  echo "$beat layout=$(grep -c LAYOUT $T/logs/${n}_$beat.log) err=$(grep -c Traceback $T/logs/${n}_$beat.log)" >> $T/logs/summary.txt
done <<LIST
1_warum_lueften 1 Beat2_Lueftungsweg
1_warum_lueften 1 Beat3_Fahrplan
3_waermestrom_luft 3 Beat2_SensibleWaerme
4_natuerliche_lueftung 4 Beat1_Fensterregeln
4_natuerliche_lueftung 4 Beat6_GrenzenDerFreienLueftung
5_mechanische_lueftung 5 Beat4_Kanalquerschnitt
6_uebergabe 6 Beat2_UebergabeHeizungKuehlung
LIST
echo ALLDONE >> $T/logs/summary.txt
