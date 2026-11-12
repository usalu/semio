#!/usr/bin/env bash
# Sequential dry-run smoke (skip_animations, no media) of every demand Beat scene in both profiles; one python process at a time. Log: $T/🗑️generated/e2b-smoke.log
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️11/DECLARATIVE-TEACHING-MORPHS-AND-FAST-PREVIEWS"
cd /c/git/semio
LOG="$T/🗑️generated/e2b-smoke.log"
mkdir -p "$T/🗑️generated"
: > "$LOG"
run() {
  profile="$1"; clip="$2"; shift 2
  echo "== $profile $clip $*" >> "$LOG"
  bash "$T/clips-e2b/smoke.sh" "$profile" "$clip" "$@" 2>&1 | grep -v "^\[staffage\]" >> "$LOG"
  echo "exit ${PIPESTATUS[0]}" >> "$LOG"
}
for profile in preview final; do
  run $profile 🏷️certificate Beat1_BedarfVsVerbrauch Beat2_DieSkala Beat3_PflichtangabenUndFolgen
  run $profile ⚡️demand-power Beat1_LeistungUndEnergie Beat2_Normheizlast Beat3_Jahresdauerlinie Beat4_RichtigAuslegen
  run $profile ⚖️regulations Beat1_GEGundNormen Beat2_Monatsbilanz Beat3_Klimadaten Beat4_Gradtagszahl
  run $profile ♨️system-losses Beat2_UebergabeVerteilungSpeicherung Beat3_Erzeugung Beat4_Anlagenaufwandszahl
  run $profile 🏭️primary-energy Beat1_DreiBilanzgrenzen Beat2_Primaerenergiefaktor Beat3_Rechenbeispiel Beat4_Referenzgebaeude
  run $profile 🧪️practice Beat1_BerechnungVsAbrechnung Beat2_HydraulischerAbgleich Beat3_SerienAbschluss
  run $profile 🔁️recap Beat1_Bilanzgrenze Beat2_Verlustseite Beat3_Gewinnseite Beat4_WinterUndSommer
done
echo "ALL DONE" >> "$LOG"
