#!/usr/bin/env bash
W=/c/git/semio
T="$(ls -d "$W"/.🧬semio/🦑️repo/🎫️tickets/*/*/*/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS)"
E="🎓️teaching/🏛️architecture"
export SM_SCENE_TIMEOUT=240 SM_TIMEOUT=900 PYTHONUTF8=1
for pair in "⚡️energy/❄️cooling/🎬️clip/⚖️heating-comparison:Beat2_SummerOverheat,Beat3_CoolingSystem" "⚡️energy/📊️demand/🎬️clip/♨️system-losses:Beat2_UebergabeVerteilungSpeicherung,Beat3_Erzeugung,Beat4_Anlagenaufwandszahl" "⚡️energy/📊️demand/🎬️clip/🏭️primary-energy:Beat3_Rechenbeispiel" "⚡️energy/📊️demand/🎬️clip/🧪️practice:Beat1_BerechnungVsAbrechnung,Beat2_HydraulischerAbgleich,Beat3_SerienAbschluss" "⚡️energy/🔥️heating/🎬️clip/🧮️calculation:FullFinalCalculationVideo" "🎯️optimization/🎬️clip/⚖️index:Beat4_NormalizedIndex"; do
  "$T/🧰️sm/run.sh" "$E/${pair%%:*}/🐍️.py" "${pair##*:}" "" 2>&1 | head -1
done
echo RERUNDONE
