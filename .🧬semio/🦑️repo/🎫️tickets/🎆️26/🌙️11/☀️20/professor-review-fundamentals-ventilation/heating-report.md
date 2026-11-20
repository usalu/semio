# Heating review report (air-movement arrows, Q̇ notation)

Interpretation: the "arrow of the air movement" treatment of Ventilation is ticket HIDE-VENT-PARTICLE-PATH-ARROWS: tipped streamline guides (`flow_guides`) of air paths are hidden (`visible=False`), moving particles stay, plus the explicit air arrows set to opacity 0. The same is applied to Heating, together with the K/N through-flow idea and the Q̇ decision. Renders: German, spectator, 854x480 at 15 fps, source `🎓️teaching/🏛️architecture/⚡️energy/🔥️heating/🎬️clip/`. Screenshots: `🖼️screenshots/H<NN>_<slug>_before|after.png`.

| # | Change | Scene / file | Screenshots | LAYOUT_CHECK |
|---|---|---|---|---|
| H01 | Roadmap card "Grundlagen" symbol Φ → Q̇ (and narration) | 🗺️roadmap Beat1_Fahrplan | H01_roadmap-symbol | clean |
| H02 | Winter ventilation: two single-window loops replaced by ONE through-flow (intake low at lower window, exhaust high at upper window); path guide hidden; particle count raised (waves 6) | 🌬️convection Beat1_LueftungImWinter | H02_convection-through-flow (t=14 s) | only untouched flags (°C label over house t=3 s, card text t=25 s) |
| H03 | Φ_k → Q̇_k | 🧲️fundamentals Beat2_Waermeleitung | H03_conduction-heat-flow-k | flag at t=3.4 s on untouched example stage |
| H04 | Rising radiator air: tipped guide hidden, particles kept | 🧲️fundamentals Beat3_Konvektion | H04_radiator-air-guide-hidden (t=14 s) | untouched stage flag only |
| H05 | Φ_c → Q̇_c token | Beat3_Konvektion | H05_convection-symbol-qc (t=41 s) | same |
| H06 | Φ_r → Q̇_r | Beat4_Strahlung | H06_radiation-symbol-qr (t=57 s) | pre-existing spectrum-strip flags at t=20.7 s |
| H07 | Φ = U·A·Δθ → Q̇ = U·A·Δθ ("Q̇ in W" kept) | Beat9_WaermestromFormel | H09_formula-q-dot-w | clean |
| H08 | Conduction: Q̇ = U·A·Δθ panel, Q̇_T = Σ Q̇ panel, readouts | 🧱️conduction Beat4/Beat5 | H07_conduction-formula-q-dot, H08_conduction-sum-q-dot | pre-existing "3×/14×", "m/K" line flags |
| H09 | Facade wind guides hidden (summary beat) | Beat5_Zusammenfassung | H10_summary-wind-guides-hidden (t=30 s) | pre-existing title/DIN-ref overlap t=48.7 s, others untouched |
| H10 | Narration only (invisible in spectator render, no before frame): convection "heat flow Φ" removed, calculation "Φ als Last" → Q̇_T = Σ U·A·Δθ, internal gains "Wärmestrom Φ" → "ein Wärmestrom" | convection, 🧮️calculation ReviewingHeatLosses, 👥️internal-gains Beat2 | none | n/a |

Also: README (Q̇ notation, units rule, hidden air guides), internal-gains helper `_phi` renamed `_q_dot`, lexicon cases for Q̇, Q̇_k, Q̇_c,si, new tests `🔥️heating/🎬️clip/🧪️architecture/` (hidden air guides, no Φ in heating scenes).

## Arrows: kept vs removed
Removed (hidden, geometry kept): convection ventilation guide arrows (intake/exhaust, now one through-flow); rising warm-air guide at radiator (Fundamentals Beat3); facade-wind guides (Beat5 summary).
Kept: heat-flow arrows through the wall (convection `wall_arrows`, conduction/summary lanes, loss paths in conduction/solar), radiation rays, energy lanes, dimension arrows, axes; the closed room-air loop outline (no arrow tip) and all moving particles (rising air at the radiator, plume above person in internal gains, ventilation particle streams); calculation window icons keep in/out particles in one single window symbol (no guides exist there).

## Tests
- Heating architecture tests (2) pass; lexicon tests (12, with new Q̇ cases) pass.
## Unverified / open
- Before/after for narration-only changes missing (captions are not in the spectator render; originals not reproducible without git restore).
- LAYOUT_CHECK flags were not compared against a before run (before renders were without it); all listed flags sit on objects I did not touch.
- Scene ids Beat2_PersonenPhiP / Beat3_GeraetePhiE / Beat4_BeleuchtungPhiL still carry "Phi" (renaming touches curriculum JSON, composition, speech, caches); not done.
- Units: panels already carry units; the substitution row in conduction Beat4 (U·25 m²·20 K) lacks U's unit, not changed (readouts show W).
- Scratchpad render outputs under scratchpad/h remain (not deleted).
