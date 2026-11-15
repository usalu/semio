#!/usr/bin/env bash
TK="/c/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/APPLY-REVIEW-FEEDBACK-TO-ENERGY-VIDEOS-AND-RENDER-FINALS"
R="$TK/cooling/render_scene.sh"
for pair in \
  "⚖️heating-comparison Beat3_CoolingSystem" \
  "💧️transmission-humidity Beat3_LueftungslastImSommer" \
  "💧️transmission-humidity Beat1_TransmissionOpaque" \
  "📐️system-sizing Beat2_VolumeFlowEquation" \
  "📐️system-sizing Beat3_IsolateAirflow" \
  "☀️solar-radiation Beat1_SolarIrradiance" \
  "☀️solar-radiation Beat5_SolarCoolingLoad" \
  "👥️internal-gains Beat2_HumanFactor" \
  "👥️internal-gains Beat3_DevicesLighting" \
  "👥️internal-gains Beat4_CumulativeLoad" \
  "⚖️heating-comparison Beat2_SummerOverheat" \
  "🌤️summer-strategy Beat2_SorptionsKuehlung"; do
  set -- $pair
  bash "$R" "$1" "$2"
done
echo ALLDONE
