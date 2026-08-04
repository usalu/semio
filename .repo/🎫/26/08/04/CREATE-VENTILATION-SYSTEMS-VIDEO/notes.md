# Ventilation Systems — Cooling Part 6

- Goal: `🎯r2602🎯updateddocs🎯updateduserdocs🎯updatedtutorials`
- Repo MCP was unavailable in this session; ticket folder created manually.

## Coverage audit of the existing Cooling videos

| Part | Covered |
| --- | --- |
| 1 heating_vs_cooling | why winter gains become summer loads |
| 2 internal_gains | `Q̇_Personen`, `P_el · f_N`, `P_Licht` |
| 3 transmission_humidity | `Q̇_T` with `ΔT_äq`, thermal mass phase lag, `Q̇_sens` + `Q̇_lat` |
| 4 solar_radiation | `I_S,max` per orientation, `F_F`, `F_V` (Raffstore), `g_tot` |
| 5 systemauslegung | `Q̇_V = ρ·c·Δθ·q_V,R` → `q_V,R` → duct `A` → `r` |

Gap that motivated this ticket: part 5 sizes **one** Zu-/Abluftsystem and never says
it is one option among several. Nothing explained free vs. mechanical ventilation,
the three mechanical base types, heat recovery, or air distribution.

## Still uncovered after this ticket (candidates for parts 7+)

- h,x-Diagramm (Mollier): Kühlen mit Entfeuchtung, Taupunkt, Nacherwärmung —
  part 3 introduces the latent load but never shows the psychrometric process.
- Kühllast vs. Wärmeeintrag per VDI 2078: Speicherwirkung, Gleichzeitigkeit,
  Auslegungstag, when the peak actually occurs.
- Kälteerzeugung: Kompressions- vs. Absorptionskältemaschine, EER/SEER,
  freie Kühlung, Rückkühlwerk.
- Flächenkühlung: Kühldecke, Bauteilaktivierung (TABS) and why air alone is a poor
  cooling medium — connects back to the 40 W/m² limit shown in beat 5.
- Zentral vs. dezentral, and the hygienic minimum flow per person (CO₂-based).

## Beats built

1. `Beat1_Systemuebersicht` — taxonomy tree, free vs. fan-assisted, mechanical branch highlighted.
2. `Beat2_FreieLueftung` — single-sided, cross, stack (`Δp = h·g·(ρa − ρi)`), night purge, then the limits.
3. `Beat3_MechanischeGrundtypen` — Abluft (−), Zuluft (+), Zu-/Abluft (=) side by side with pressure regimes.
4. `Beat4_Waermerueckgewinnung` — counterflow plate exchanger, 32/26 °C summer case, `Φ = (θZUL − θAUL)/(θABL − θAUL) ≈ 0,8`.
5. `Beat5_Luftfuehrung` — Mischlüftung vs. Quelllüftung, stratification, ~40 W/m² cap.
6. `Beat6_RLTFunktionen` — Filtern/Heizen/Kühlen/Be-/Entfeuchten, Lüftungs- vs. Teil- vs. Vollklimaanlage, `n = q_V,R / V_Raum`.
7. `Beat7_Zusammenfassung` — two-column closing card: six-point recap on the left,
   normative references on the right.

## Normative references cited in beat 7

| Reference | Covers |
| --- | --- |
| DIN 1946-6 | Lüftung von Wohnungen — system types, Lüftungskonzept |
| DIN EN 16798-3 | Lüftung von Nichtwohngebäuden — RLT systems (replaced DIN EN 13779) |
| DIN EN 16798-1 | Eingangsparameter Innenraumklima — design air flow rates |
| DIN EN 308 | Prüfverfahren WRG — defines the Rückwärmzahl / temperature ratio Φ |
| DIN EN ISO 7730 | Thermische Behaglichkeit — PMV/PPD and draught rate, the Quelllüftung limit |
| VDI 2078 | Berechnung der thermischen Lasten — the cooling load behind parts 1–5 |
| VDI 6022 | Hygieneanforderungen an RLT-Anlagen (footnote) |
| DIN EN ISO 16890 | Luftfilterklassen ePM1/ePM10 (footnote, replaced EN 779) |

VDI 2078 and VDI 6022 are Richtlinien, not DIN standards — hence the column heading
"Normen und Richtlinien". Cited without year on purpose, with an on-screen note to
check the current Ausgabestand.

Total runtime at `-ql`: ~141 s.

## Revision pass — typography and airflow

- **Font.** Pango's generic `"Serif"` alias is not an installed face; it collapses word
  spaces ("Institutfür", "Luftüberhaupt"). Verified by rendering the same string in
  7 families. Georgia/Charter/PT Serif are correct. Now resolved centrally through
  `tutorial/manim_fonts.py` → `apply_body_font()`. Scenes 1–5 still call
  `Text.set_default(font="Serif")` and carry the same bug.
- **Type sizes** raised throughout: title 26→30, subtitle 18→21, step 16→19,
  headers 18→21, body notes 13–15→15–17.
- **Equations** moved from the hand-rolled `_equation_row` Text hack to `MathTex`
  with terms passed separately, so fragments stay indexable for colouring and
  `Indicate` while getting real math typesetting.
- **`_ensure_latex_paths()`**: Homebrew's dvisvgm bundles a kpathsea that cannot find
  `texmf.cnf` in a Homebrew TeX Live, so `MathTex` failed with a misleading
  "update dvisvgm to at least 2.4". Setting `TEXMFROOT`/`TEXMFDIST`/`TEXMFCNF`
  (derived from `kpsewhich`) fixes it; no-op on a healthy install.
- **Airflow** rewritten: `_stream` (one dot per path, one pass) → `_flow`
  (`UpdateFromAlphaFunc`, several looping waves per path, fade in/out at the path
  ends, optional `color_end` so a particle changes colour as it gains or loses heat)
  plus `_guides` streamlines with arrowheads so the route reads between particles.
  `_smooth_path` now returns a `TipableVMobject` — a bare `VMobject` has no `add_tip`.
- Beat 2 night purge now tells the heat balance: blue 16 °C air in → orange out,
  Speichermasse 26 °C → 21 °C, room wash red → blue.
- Beat 3 colour-codes treated vs. untreated air with a legend, and runs all three
  panels together at the end for comparison.
- Beat 4 redrawn as two parallel counterflow channels with downward `Wärmestrom`
  arrows and an explicit takeaway line, instead of curves crossing a rotated square.
- Beat 5 gained a temperature stratification (20/23/26 °C bands) in the displacement
  room and a full recirculation loop to the return grille in the mixing room.

## Chapter-header unification across all Cooling videos

`tutorial/manim_fonts.py` gained a `#region Scene title` holding the single
definition — `TITLE_FONT_SIZE = 30`, `TITLE_COLOR = "#FFFFFF"`,
`TITLE_EDGE_BUFF = 0.35`, `TITLE_RUN_TIME = 1.4`, plus `scene_title()` and
`play_scene_title()`. Every top-middle title in parts 1–6 now goes through it.

Before, across 26 scenes: sizes 26/28/30/32/36, colours white/cyan/orange,
edge buffs 0.26–0.40, and three different intro animations (`Write`,
`FadeIn(shift=DOWN)`, bare `self.add`).

- Part 1 — 2 titles; the beat-4 `Transform` target now shares the same spec.
- Part 2 — 11 titles; `play_typed_title()` kept as a thin delegate so existing
  call sites stay valid. This file also carries its own `_NOWIGETIT_TEXT_LAYOUT_FIX_V7`
  font machinery, which duplicates `manim_fonts` and should be collapsed later.
- Part 3 — 6 titles; `FadeIn(shift=DOWN)` intros became `Write`.
- Parts 4/5/6 — `_main_title()` now returns `scene_title(MAIN_TITLE)`.

Beats that *continue* a previous beat still use `self.add(title, …)` rather than
re-animating the header every clip.

Verified by rendering all 26 scenes and stacking the title crops:
`preview/titles/all_titles.png`, `preview/titles/s2_titles.png`.

## Implementation notes

- `_stream()` must call `scene.remove(dots, *dots)` — `Scene.play` registers each
  animated dot individually, so removing only the `VGroup` handle leaves every
  airflow dot frozen on screen. This littered all six beats on the first pass.
- `_equation_row()` gained a `"sub"` tag on a fragment: renders it at `sub_scale`,
  bottom-aligned via `arrange(..., aligned_edge=DOWN)`, and tucks it left with a
  cumulative offset so later fragments do not leave a hole.
- Narration is English over German on-screen labels, matching parts 2–5.
- Preview stills: `preview/Beat*_end.png`.
