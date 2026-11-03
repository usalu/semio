#!/bin/bash
cd /c/git/semio
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
E="🎓️teaching/🏛️architecture/⚡️energy"
run() { f="$E/$1/🐍️.py"; shift; for s in "$@"; do bash "$T/🧰️fr/probe.sh" "$f" "$s" >> "$T/🗑️generated/fr/chain.log" 2>&1; done; }
mkdir -p "$T/🗑️generated/fr"
run "❄️cooling/🎬️clip/⚖️heating-comparison" Beat2_SummerOverheat Beat3_CoolingSystem
run "❄️cooling/🎬️clip/🌤️summer-strategy" Beat1_LastZuerstSenken Beat2_SorptionsKuehlung Beat3_KomfortStrategie
run "🌬️ventilation/🎬️clip/⚙️mechanical" Beat1_Grundtypen Beat2_Waermerueckgewinnung Beat3_Anlagenvergleich Beat4_Kanalquerschnitt
run "📊️demand/🎬️clip/♨️system-losses" Beat2_UebergabeVerteilungSpeicherung Beat3_Erzeugung Beat4_Anlagenaufwandszahl
run "📊️demand/🎬️clip/🏭️primary-energy" Beat1_DreiBilanzgrenzen Beat2_Primaerenergiefaktor Beat3_Rechenbeispiel Beat4_Referenzgebaeude
run "🔥️heating/🎬️clip/👥️internal-gains" Beat1_WinterInterneGewinne Beat2_PersonenPhiP Beat3_GeraetePhiE Beat4_BeleuchtungPhiL Beat5_SummeUndDichte
run "🔥️heating/🎬️clip/🧱️conduction" Beat1_MakroUndMikro Beat4_WattVergleich Beat5_Gebaeudehuelle Beat6_Waermebruecken
run "🔥️heating/🎬️clip/🧲️fundamentals" Beat9_WaermestromFormel
run "❄️cooling/🎬️clip/👥️internal-gains" Beat2_HumanFactor
run "🧲️physics/🎬️clip/🧲️fundamentals" Beat1_Leistung Beat2_EnergieImAlltag Beat3_Energieerhaltung Beat4_Waermepumpe Beat5_Strahlung Beat6_ThermischeMasse Beat7_SensibelLatent Beat8_Zusammenfassung
run "🔥️heating/🎬️clip/🧮️calculation" ReviewingHeatLosses Scene2 ReviewingHeatGains Scene4 UltimateEnergyBalance AnlagenVerluste FullFinalCalculationVideo
echo CHAIN-DONE >> "$T/🗑️generated/fr/chain.log"
