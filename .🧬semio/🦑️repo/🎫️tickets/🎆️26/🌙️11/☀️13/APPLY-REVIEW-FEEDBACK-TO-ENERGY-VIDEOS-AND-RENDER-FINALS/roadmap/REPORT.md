# roadmap report (F06 (1): roadmap first in every video), recorded by the coordinator

All five energy videos start with `Beat1_Fahrplan` as segment 0 of their first chapter.

- Shared builder `🏗️authoring/🗺️roadmap/🐍️.py` (`RoadmapScene`, `build_roadmap`, `layout(count)`,
  `validate_roadmap`, pictogram registry) + tests `🧪️tests/{🐍️.py,🔣️.cases.json}` (11 pass).
- Per family `<family>/🎬️clip/🗺️roadmap/{🐍️.py,🏗️composition/🐍️.py,🔊️speech/🐍️.py}`; no beat renumbered.
- Manifest `🎬️clip/🔣️.json` + hierarchy `🔣️.cases.json` (130 canonical sources, 138 segment
  occurrences). No `poster` declared on the roadmap: chapter poster stays the last segment.

| Video | Cards | Chips |
|---|---|---|
| physical_fundamentals | Leistung/Energie `P,E` · Erhaltung · Wärmepumpe `COP` · Strahlung `g,I_s` · Therm. Masse · Sensibel/latent | none |
| ventilation (moved out of `❓️need`, was `Beat3_Fahrplan`) | Warum lüften `CO₂` · Luftmenge · Wärmestrom `Φ` · natürlich · mechanisch · Übergabe `Q̇_V` | out → Heizung, Kühlung |
| heating | Grundlagen `Φ,Δθ` · Wärmeleitung `U,λ,ψ` · Lüftung `Q̇_V` · interne Gewinne `Q̇_I` · solare Gewinne `Q̇_S,g⊥,F_C` · Bilanz `Q_h` | in `Q̇_V` (card 3), out → Energiebilanz `Q_h` |
| cooling | Heizen/Kühlen · interne Lasten · Transmission `Q̇_T,Q̇_V` · solare Last · Auslegung `Q̇_C,V̇` · Sommerstrategie `Q_c` | in `Q̇_V` (card 3), out → Energiebilanz `Q_c` |
| energy_balance | Bilanz `Q_h,Q_c` · Klima · Leistung · Endenergie `e_p,Q_E` · Primärenergie `f_P,Q_P` · Ausweis · Praxis | in `Q_h`, `Q_c` (card 1) |

Decisions: ventilation has six cards ("sechs Schritten"); the edition-less standards line was
dropped; heating cards follow the manifest order; incoming chips use a raw `←`.

Verification: last frames of all five scenes viewed (direct manim, German preview), in
`🗑️generated/roadmap-out/frames/`. Pipeline render of `Ventilation_01_WarumLueften` plays roadmap,
Beat1, Beat2 in order but aborted at the end on a missing stored poster for
`Ventilation_06_Uebergabe` (since captured by the UI agent). Narration guard clean.
Failing: hierarchy `test_view_hover_then_fullscreen_click_commits_the_chapter` for
`Ventilation_02_Luftmenge` (not confirmed unrelated). Not run: `staffage_only`, output-folder
tests (hung, killed), `🎬️clip/🧪️tests`.

Open: recapture posters (ventilation chapter 1 poster shows the old roadmap frame); no
`timing.json` for the new scenes until speech synth runs; confirm `Q_c`; heating card uses
`Q̇_I` while the heating scenes use `Q̇_i` (closing consistency pass).

Edited besides new files: three ventilation `❓️need` files, READMEs (ventilation, heating,
cooling, demand), `🏗️authoring/🎬️clip/README.md`, `📖️concepts/📝️.md`.
