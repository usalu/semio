#!/bin/bash
cd /c/git/semio
R=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🧰️lc3/render_en.sh"
while read -r clip scene tag; do
  bash "$R" "$clip" "$scene" "$tag"
done <<'EOF'
❄️cooling/🎬️clip/🌤️summer-strategy Beat2_SorptionsKuehlung ss2
❄️cooling/🎬️clip/🌤️summer-strategy Beat3_KomfortStrategie ss3
❄️cooling/🎬️clip/💧️transmission-humidity Beat3_LueftungslastImSommer th3
❄️cooling/🎬️clip/👥️internal-gains Beat2_HumanFactor ig2
❄️cooling/🎬️clip/☀️solar-radiation Beat1_SolarIrradiance sr1
🌬️ventilation/🎬️clip/🌡️heat-flow Beat3_LatenteLast hf3
🌬️ventilation/🎬️clip/📦️volume Beat3_Volumenstrom vo3
🌬️ventilation/🎬️clip/🤝️handoff Beat2_UebergabeHeizungKuehlung ho2
🌬️ventilation/🎬️clip/❓️need Beat3_Fahrplan ne3
🔥️heating/🎬️clip/🌬️convection Beat1_LueftungImWinter co1
🔥️heating/🎬️clip/🧮️calculation Scene4 ca4
🔥️heating/🎬️clip/🧲️fundamentals Beat6_VonWegenZuZahlen fu6
📊️demand/🎬️clip/🏭️primary-energy Beat1_DreiBilanzgrenzen pe1
📊️demand/🎬️clip/🏷️certificate Beat1_BedarfVsVerbrauch ce1
📊️demand/🎬️clip/🧪️practice Beat1_BerechnungVsAbrechnung pr1
📊️demand/🎬️clip/♨️system-losses Beat4_Anlagenaufwandszahl sl4
📊️demand/🎬️clip/⚖️regulations Beat1_GEGundNormen re1
📊️demand/🎬️clip/⚡️demand-power Beat2_Normheizlast dp2
EOF
echo ALLDONE >> ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS/🗑️generated/lc3/done.log"
