# vent-a report (ventilation chapters 1–3: need, volume, heat-flow), recorded by the coordinator

Verified with direct manim renders (preview, German); English strings read only. Frames:
`🗑️generated/vent-a-out/direct/frames/`. Pipeline render not run. Narration guard (filter
`ventilation`) clean.

- F04: "Grenzwert nach Pettenkofer" / "Pettenkofer limit". Corner reference
  `Pettenkofer 1858 · Bundesgesundheitsbl. 51 (2008) 1358 · DIN 1946-6` (UBA 2008 paper,
  Bundesgesundheitsblatt 51, 1358-1369, confirmed by web search). DIN EN 16798-1 Table B.9
  NOT cited (could not verify).
- F05: "darum bedarf es immer der Lüftung neben Heizung und Kühlung".
- F06(2): roadmap card 2 shows Φ only (no Δx); card 5 and both hand-over chips show Q̇_V. Φ tied to
  Q̇_V once in Beat2_Lueftungsweg. Scene not moved (roadmap agent does that).
- F07: "verblasst" removed; only the indoor air is exchanged, walls/slabs/roof are not air.
- F08: ρ = 1,2 kg/m³ (20 °C) with second label 1,27 kg/m³ (5 °C); norm uses 0,34 Wh/(m³K).
- F09: unit slide = two framed numeric lines "Gebäudebezogene Einheiten" / "SI-Einheiten"
  (75 W/K vs 74 W/K, rounding 0,335 ≈ 0,34 noted) + note that unit usage varies, not only in the DIN.
- F10: units on every number in all three chapters; "in unit" instead of brackets; fraction bars.
- F11: narration uses `$…$` TeX throughout.
- Winter latent (F17 follow-up): Beat4_WinterUndSommer shows 770 W, Δx = 4,3 g/kg, note
  "per Konvention nicht in der Heizlast"; Q̇_V = Q̇_V,sens + Q̇_V,lat visible in both seasons.

Other: new clause keys `norm`, `variiert` (heat-flow Beat1), `winter_lat`, `humid` (Beat4) –
speech must be re-synthesised. README symbol table + Pettenkofer reference updated (Φ_V entries
at lines 31/35 belong to vent-b's chapters). Volume unit test fixed (2 pass). Two pinned
Beat3_LatenteLast cases updated in `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`.
Integration caption test still fails on other agents' scenes (Beat2_StossOderKipp,
Beat4_Waermepumpe). Quiz unchanged.

Files (`🎓️teaching/🏛️architecture/⚡️energy/`): `🌬️ventilation/🎬️clip/❓️need/🐍️.py`,
`📦️volume/🐍️.py`, `📦️volume/🧪️tests/🐍️.py`, `🌡️heat-flow/🐍️.py`, `README.md`,
`🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`.
