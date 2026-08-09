# Revise Heating Intro Animations

Ticket: `26/08/09/REVISE-HEATING-INTRO-ANIMATIONS`

## Verified defects (baseline render in `baseline/`, `manim -ql`)

The file was already on the five-rule template, so the problems were pedagogical and
compositional rather than structural.

| Beat | Defect |
|------|--------|
| all | Content filled roughly a quarter of the frame, clustered near y ≈ 0…1.5, with large dead bands above and below |
| all | Each transport mode had its own unrelated cartoon (party dots / lone wall box / speaker / sun+roof), so nothing showed that the three cross the *same* Δθ |
| 1 | People and heat were both plain orange dots — indistinguishable. The cold side turned orange, reading as "the party moved outside" rather than "heat arrived" |
| 2 | Seven dots in a row, `Indicate`-flashed in sequence: nothing was handed along, no warm/cold side, no direction |
| 3 | Two bare vertical lines as the "door", ribbons passing through, no room, no inside/outside |
| 4 | Speaker and a `#2B2B2B` roof nearly invisible on the near-black background; the two analogies floated unconnected |
| 6–9 | Travel agency / ticket desk / bouncer metaphors carried the explanation while the physics stayed thin |
| — | Hardcoded `font_size=16` / `34` and a hardcoded hex colour, against skill rules 8 and 4 |

## Shared-helper promotion (prerequisite)

The layout-zone and gauge machinery proven in `Cooling/6` was duplicated-by-copy risk for a
second scene file, so it moved into `tutorial/manim_visuals.py` as the single source:
`SAFE_TOP` / `SAFE_BOTTOM` / `SAFE_BOTTOM_FORMULA`, `fit_band`, `meter` / `set_meter` /
`bind_meter`, `chip`, `dim_chip`, `cross_mark`, `dim_arrow`. `scene_6.py` was switched over to
import them and re-verified (9/9 beats render clean).

`dim_chip`'s docstring now records *why* it does not route through `set_opacity`, and the
`Indicate`-snapshot trap that caused the Cooling Beat 9 bug.

## Fix

**One through-line.** `_section()` draws warm room | wall | cold outside with `innen 20 °C`,
`Δθ = 20 K`, `außen 0 °C`, and beats 1–5 all redraw that identical scaffold, changing only the
mechanism on it. `_route_strip()` shows which of the three a beat is on.

**Heat is no longer a dot.** `_quantum()` draws a small wave; `_person()` draws a body glyph.
The two can never be confused, which was Beat 1's central defect.

1. Warm/cold section, Δθ named as the only driver, packets crossing while both temperatures
   converge (20→19, 0→1, Δθ 20→18 K), then a reverse arrow struck through for the second law.
2. Molecule lattice inside the wall; a pulse travels column by column while one ringed molecule
   visibly oscillates *in place* — "nur die Energie wandert, nicht das Material".
3. The same wall gains an opening; buoyant loop out the top, cold air in below; one tagged
   parcel followed all the way out of the building.
4. Radiation waves cross the gap, then the air is removed entirely: conduction and convection
   are struck through, the waves continue. The distinguishing property, shown rather than told.
5. All three at once, summed in the formula panel as `Q̇_ges = Q̇_k + Q̇_c + Q̇_r`, each term
   ringed against its own visual, then the ISO 6946 bundling.
6. Metaphor dropped: three tokens funnel into one building-element value, and a real four-layer
   wall appears, setting up R and U honestly.
7. Bouncer dropped: a temperature gradient drawn across one layer. Widening d flattens the
   gradient and moves the R and heat-flow gauges; swapping λ (2,1 → 0,035) flattens it far more.
8. `R_ges = R_si + Σ d/λ + R_se` transformed in the same panel into `U = 1 / R_ges`, against two
   real build-ups (Altbau U ≈ 1,4 vs saniert U ≈ 0,15) with matching leak arrows.
9. `Q̇ = U · A · Δθ` made felt: a watt gauge responds as A doubles, Δθ doubles, and finally U
   drops — landing on "only U is a design decision".

Class names unchanged, so `generate_audio.py` and `build_full_video.py` needed no edit
(verified equal by set comparison).

## Verification

Three `manim -ql` passes (`baseline/`, `after/`, `final/`), frames tiled with ffmpeg and read
back each pass. Second pass fixed: heat packets drawn over the occupant row (Beat 1), the
"bleibt am Platz" tag sitting on the molecule lattice and the cold zone recolouring orange
against its own `0 °C` label (Beat 2), the `Öffnung` label inside the opening (Beat 3), and the
lever chip overlapping the watt readout plus an outdoor-temperature restore that never took
effect because it transformed into a `.copy()` of an already-consumed mobject (Beat 9).

Caption widths were measured against `CAPTION_MAX_WIDTH`; three over-wide lines were hand-broken
so no caption is auto-shrunk. 9/9 beats render clean: 44.8, 33.8, 41.0, 35.8, 36.4, 37.8, 49.7,
44.8, 54.9 s — about 6.3 min, against 5.8 min of estimated narration.

## Follow-up (not done here — costs TTS credits)

This topic has **no audio at all** — `beat_*_audio.*` was never generated. Run:

1. `Tutorial: Heating 1 — Generate Audio` (launch.json)
2. `Tutorial: Heating 1 — Build Full Video` (launch.json)
