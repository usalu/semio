#!/usr/bin/env bash
T="$(ls -d /c/git/semio/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
R="$T/m9_render.sh"
D="📊️demand/🎬️clip"
for s in 1_BerechnungVsAbrechnung 2_HydraulischerAbgleich 3_SerienAbschluss; do
  bash "$R" "$D/🧪️practice/🐍️.py" "Beat$s" "rd1-pr${s%%_*}" "1,5,10,20" 
done
for s in 1_DreiBilanzgrenzen 2_Primaerenergiefaktor 3_Rechenbeispiel 4_Referenzgebaeude; do
  bash "$R" "$D/🏭️primary-energy/🐍️.py" "Beat$s" "rd1-pe${s%%_*}" "1,5,10,20"
done
echo ALLDONE
