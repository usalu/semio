# Apply Physical Fundamentals Review Feedback

Goal: `R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS`

Repo MCP was unavailable (`CONNECTION_CLOSED`), so `repo://goals`, `ticket_open` and `ticket_close` could not be called. Work logged here.

Scope: `tutorial/energy/demand/1_physical_fundamentals/` only, plus additive math typesetting in `tutorial/manim_visuals.py`, the PF intro card text, the PF section of `KONZEPTE_UND_FORMELN.md` and the PF launch entry.

## New beat order (`BEATS` in `scene_1.py`)

1. `Beat1_EnergieImAlltag` — kWh at the meter, dropped „h“, 1 kWh = vacuum 1 h = ≈ 3 min shower, energy scale tealight → world, glass/ceiling/openings
2. `Beat2_Leistung` — E = P·t as area, vacuum/kettle/shower same area, 1 W = 1 J/s, power scale tealight → all power plants, heating load vs annual demand
3. `Beat3_Energieerhaltung` — E_ein = E_aus + ΔE_Speicher, lamp 5/95 → 100 % heat, person 100 W, winter/summer bars
4. `Beat4_Waermepumpe` — animated refrigerant cycle, 1 + 3 = 4, COP from bars, COP(T), JAZ, comparison wood chips/oil/gas/electric/air-WP/ground-WP
5. `Beat5_Strahlung` — short-wave through glass, g from counted rays, Q̇_S = g·A·I, long-wave reflected by low-e, greenhouse, external shading
6. `Beat6_ThermischeMasse` — Q = m·c·ΔT from slab geometry, 24 h phase shift/damping, why the ceiling, night purge / solar storage
7. `Beat7_SensibelLatent` — T–Q plateau, saturation curve, dew point, person 70 W + 30 W, condensation/dehumidification
8. `Beat8_Venturi` — continuity, Bernoulli, suction, wind over ridge, opening position/size, ventilation heat loss
9. `Beat9_Kraft` — F = m·g, W = F·s area, animated heart 1 N · 1 m per second → 1 J → 1 W, 1000 hearts = 1 kW, 1 kWh = 3,6·10⁶ J

## Feedback checklist

Regeln
- [x] Alles visuell, Ideen morphen: tank → P–t area (B1→B2), slab → tiles, short-wave → long-wave rays, duct → building, heart → 1000 dots → kWh tank (B9→B1)
Allgemein
- [x] Faktoren 0→1 animiert: kWh meter, tank level, minutes, litres, temperature, P/t/E, 5 %/95 %, COP, g, A, ΔT, φ, φ (rel. humidity), V̇, Q̇_V, mass, force, work, heartbeats
- [x] Teilwerte aus physischer Repräsentation: E from rectangle P×t, COP from stacked bars, g from ray count, A from pane dims, m from slab volume, v₂ from A₂, W from F–s area, P from beat/second counters
- [x] Unverknüpftes entfernt: Hannover, Enercity, pipe/bucket, speedometer, primary-energy teaser, Villa/Block, cheat sheet
- [x] Text sachlich: "Architektur formt Raum … hört nie auf" removed, plain German subtitles (all ≤ 2 caption lines, checks/caption_check.py)
- [x] Architektur + Bauphysik: every concept on a building element (glass, ceiling, openings, room section)
- [x] Formeln korrekt gesetzt: `math_text` / `math_row` (LaTeX subset, real sub/sup/fractions, italic variables, upright units, no TeX install)
Physikalische Grundlagen
- [x] kWh/Arbeit zuerst (B1), Kraft am Schluss (B9)
- [x] Falsche Kurzform „kW“ statt „kWh“ (B1 slip/hour)
- [x] Hannover entfernt
- [x] Herz animiert: 1 N über 1 m, 1×/s → 1 J → 1 W (B9)
- [x] Konkrete Skalen: power tealight 30 W → 9 TW, energy tealight 0,1 kWh → world 1,7·10¹⁴ kWh, 1 kWh = vacuum 1 h, ≈ 3 min shower
- [x] Wärmepumpen-Schema animiert (B4)
- [x] COP-Vergleich Holzhackschnitzel, Öl, Gas, Elektro (+ Luft-/Erd-WP) (B4)
- [x] DIN-Normen — feedback line was cut off ("DIN Normen warden"); interpreted as: cite the norm where the quantity is defined (DIN EN 12831, DIN V 18599, DIN EN 14511, VDI 4650, DIN EN 410, DIN 4108-2/-3, DIN EN ISO 13786, DIN 1946-6)
- [x] Kurz-/langwellige Strahlung, Glas, Low-E (B5)
- [x] Thermische Masse, Phasenverschiebung, Decke (B6)
- [x] Sensibel/latent, Wasseraufnahmekapazität der Luft (B7)
- [x] Venturi, Positionierung, Querschnitte, Lüftungswärmeverlust (B8)

## Numbers used

- Shower: 30 kg · 4,19 kJ/(kg·K) · 28 K = 3 520 kJ ≈ 0,98 kWh
- EFH: heating load 8 kW, 15 000 kWh/a (≈ 1 875 full-load hours)
- Germany final energy ≈ 2 340 TWh (AGEB 2023), world primary ≈ 620 EJ ≈ 1,7·10¹⁴ kWh
- World installed power plant capacity ≈ 9 TW
- Air at 20 °C / 50 %: x ≈ 7,3 g/kg, dew point ≈ 9,3 °C (Magnus)
- Person: 45 g/h · 2 450 J/g ≈ 30 W latent
- Ventilation: 0,34 Wh/(m³K) · 100 m³/h · 20 K = 680 W

## Verification

- Every beat rendered at `-ql` with `LAYOUT_CHECK=1` (`checks/render_beat.sh`, log `checks/final_render.log`); frames reviewed as contact sheets.
- `checks/math_probe.py` renders the typesetting probe.
- VO manifests reset (`vo_timing.json`, `vo_trace.json`) — VO must be re-synthesized with `generate_audio.py` → `--trace` → `--align`.
- Intro card (`Demo_Intro_PhysikalischeGrundlagen`) could not be rendered here: `tutorial/intro/assets/welfenschloss (1) (1).png` is not in the repository (pre-existing).
- Final pass: all nine beats render with zero `[LAYOUT]` issues (Beat1 and Beat5 re-rendered after tag moves, `checks/final_render_beat1.log`).

## Files

Updated
- `tutorial/energy/demand/1_physical_fundamentals/scene_1.py` (rewritten: 9 beats, shared `BEATS` order)
- `tutorial/energy/demand/1_physical_fundamentals/full_physical_fundamentals_video.py` (playlist from `BEATS`)
- `tutorial/energy/demand/1_physical_fundamentals/generate_audio.py` (imports `BEATS`)
- `tutorial/energy/demand/1_physical_fundamentals/build_full_video.py` (scene list from `BEATS`)
- `tutorial/energy/demand/1_physical_fundamentals/vo_timing.json`, `vo_trace.json` (reset — old beat keys)
- `tutorial/manim_visuals.py` (additive `math_text` / `math_row`; `formula_panel` tags the whole row family)
- `tutorial/intro/intro_scene.py` (PF intro subtitle)
- `tutorial/energy/demand/KONZEPTE_UND_FORMELN.md` (PF section, overview node, norms row)
- `.vscode/launch.json` (PF preview entry → `Beat1_EnergieImAlltag`)

Created (ticket)
- `checks/math_probe.py`, `checks/caption_check.py`, `checks/render_beat.sh`, `checks/final_render.log`, `checks/final_render_beat1.log`

Status: closed 2026-10-03 (manually — repo MCP unavailable).

## Reopened 2026-10-09 — formula on the p-V graph (Beat9_Kraft)

Dev report (screenshot at 14:33 of the full video): the `1 W = 1 J / 1 s` box sat on the p-V diagram's V axis (`130`, `V [ml]`).

- Cause: `w_panel` used the shared bottom formula slot, but the p-V axis origin sits at y = −1.25 and reaches into that slot.
- Fix: the panel is stacked under the `P = n·W/t ≈ 1 W` line in the right result column (`next_to(p_read, DOWN, aligned_edge=LEFT)`), clear of the graph.
- Same beat, flagged by the layout guard: the weight arrow `F_G` started at the block centre and ran through `102 g`, and the `F_G` label sat on the floor line → arrow now starts at the block underside (0,55 units), label beside the arrow below the block.
- Verified: `LAYOUT_CHECK=1` render of Beat9 reports nothing; contact sheets `checks/overlap/pf_b9v2_*.png` reviewed.
- `full_physical_fundamentals_video.py` had the same stale-clip bug as the Cooling compose script (skipped any beat whose mp4 existed): beats are now reused only when newer than `scene_1.py`, `vo_timing.json`, the script and the shared `manim_visuals.py` / `manim_fonts.py`; the freshly rendered clip is returned from the script's own output folder.

- Guard pass over beats 1–8 before the full HQ render (`checks/overlap/pf_guard_b1_8.log`), all fixed and screenshot-verified (`pf_b1v2`, `pf_b5v2`, `pf_b8v2`, `pf_b8v3_zuluft`):
  - Beat1: the flying `h` paused on the thought-cloud outline → now pauses above the cloud; `kW`→`h` gap widened.
  - Beat5: `DIN EN 410` ran into `A = …` → tucked under `g = 0,50`; `kurzwellig` sat on the sun rays → moved below the ray fan.
  - Beat8: `Sog −` sat on the lowest wind streamline → leeward of the eave below it; `Zuluft` sat on the inflow particles → left of where the stream starts (stream now starts at x = −5,6).
- Full video re-rendered at 1080p60 (`full_physical_fundamentals_video.py -q h`, log `checks/full_render_qh.log`); all other Physical Fundamentals video files deleted on the dev's request — only `rendered/Full_Physical_Fundamentals_1080p60.mp4` kept.

Status: closed 2026-10-09 (manually — repo MCP unavailable).
