# Improve Lüftungssysteme Animations

Ticket: `26/08/09/IMPROVE-LUEFTUNGSSYSTEME-ANIMATIONS`
Follows `26/08/09/PASSIVE-NATURAL-VENT`, which set the passive-house beat map but left
poster-style visuals and text sitting on room outlines.

## Verified defects (baseline render in `baseline/`, `manim -ql`)

| Beat | Defect |
|------|--------|
| 2 | `Fenster = einstellbares Ventil` printed on top of the room floor line; `Außenluft` badge touching the window frame |
| 3 | `Öffnungsanteil` badge sitting on the room floor line; `Passivhaus …` tip sitting on the room top edge; the "dose both openings" step invisible (windows resized by a few pixels) |
| 4 | `tiefe Zuluft · hohe Abluft` overlapping the room bottom edge and clipped by the formula box; shaft read as two stray orange lines; `h` line floating unconnected at the frame edge |
| 5 | `Nacht` / `Tag` badges overlapping each other, the `21 °C` label, and the floor line |
| all | Static badge posters — no quantity ever moved, so nothing was actually explained |

## Fix

Rewrote `scene_6.py` around measurable, animated quantities instead of badges.

**Layout zones** (`#region Layout zones`) — `SAFE_TOP` / `SAFE_BOTTOM` /
`SAFE_BOTTOM_FORMULA`, plus `_fit_band()`, a no-op guard every beat runs its scaffold
through. This is what structurally prevents the collisions above rather than relying on
hand-tuned constants staying correct.

**New shared motifs** — `_meter` / `_set_meter` / `_bind_meter` (`ValueTracker`-driven
gauge), `_dim_arrow` (measured height), `_chip`, `_park`, `_dim`.

**Beat rework**

1. `Beat1_PassivhausIdee` — cooling-load bar shrinking 100 % → 55 % → 20 % as envelope,
   then natural ventilation, are applied; mechanics gets the ringed remainder.
2. `Beat2_Fensterregeln` — window sash opens against an air-change gauge with a comfort
   band; outdoor 18 °C → open wide / flush, outdoor 32 °C → throttle.
3. `Beat3_Querlueftung` — wind pressure Luv/Lee; new formula
   `A_eff = 1 / √(1/A_1² + 1/A_2²)`; shrinking the outlet visibly collapses the flow
   (series effect), then recovers.
4. `Beat4_Auftrieb` — stratification legend `ρ_a` / `ρ_i`, `Δp` gauge, and the shaft
   physically growing 1,70 m → 3,00 m of `h` with `Δp` 1,2 → 2,1 Pa and faster particles.
5. `Beat5_Nachtlueftung` — 48 h `Axes` profile: outdoor (yellow) vs sealed indoor (red)
   vs night-purged indoor (cyan), night bands, comfort band, `≈ 4 K` peak reduction.
6. `Beat6_KomfortStrategie` — decision flow chart with a travelling token: outdoor cooler?
   → natural / shade → mechanical reserve → comfort.

`Axes` numbers are hand-placed `Text`: CE `DecimalNumber` defaults to `mob_class=MathTex`,
so `include_numbers` / `add_coordinates` would pull in LaTeX (skill rule 3).

## Mechanical-ventilation block (second pass)

Three beats inserted between night purge and the summary, rebuilding the mechanical
animations that `PASSIVE-NATURAL-VENT` deleted (recovered from `git show HEAD:…scene_6.py`)
on the current template — `equation_row` instead of `MathTex`, `NARRATION` triples, zones.

6. `Beat6_GrenzenDerFreienLueftung` — the bridge: four gaps free ventilation cannot close
   (air too hot/humid, no filter, flow follows the weather, no recovery), each paired with
   an in-room event and a checklist row; ends handing over to fan-assisted ventilation.
7. `Beat7_MechanischeGrundtypen` — Abluft- / Zuluft- / Zu-Abluftanlage in three panels with
   fans, passive ports, particle flow and −/+/= pressure states.
8. `Beat8_Waermerueckgewinnung` — counterflow plate exchanger, 32→27 °C supply against
   26→31 °C exhaust, heat-flow arrows, `Φ = (θ_ZUL − θ_AUL)/(θ_ABL − θ_AUL) = 5 K/6 K ≈ 0,8`.

`Beat6_KomfortStrategie` → `Beat9_KomfortStrategie`; its reserve node now reads
`Reserve: Zu-/Abluft mit WRG`. `generate_audio.py` and `build_full_video.py` updated to nine
beats; `launch.json` needed no change (it only targets Beat1).

## Caption sizing

Measuring every subtitle against `CAPTION_MAX_WIDTH` showed 32 of them overflowing and being
silently auto-shrunk by `caption_bar`, so caption size varied beat to beat — the anti-pattern
the skill names. All are now hand-broken to ≤ 2 lines that fit at full `CAPTION_FONT_SIZE`
(budget ≈ 77 characters per line at font 22). Because a two-line bar is taller, `SAFE_BOTTOM`
moved -2.82 → -2.60; `_fit_band` picked up the one beat that then overran (Beat 9).

## Verification

`manim -ql` × 6 beats, three passes (`baseline/`, `after/`, `after2/`), frames tiled with
ffmpeg and read back each pass. Second pass fixed: doubled `A_1`/`A_2` during the Beat 3
symbol morph, Beat 4 outlet ring cutting through `Abluft`, Beat 5 outdoor/indoor curves
being near-identical pinks, Beat 6 token covering node labels.

Final pass: 9/9 beats render clean. Rendered lengths 35.5, 50.9, 49.3, 64.1, 61.1, 56.3,
48.1, 51.8, 48.1 s — about 7.8 min total, against 7.1 min of estimated narration.
Class-name sets in `scene_6.py`, `generate_audio.py` and `build_full_video.py` verified equal.

Bugs caught by frame readback rather than by the renderer, worth remembering:

- `Indicate` snapshots a mobject's state at animation start and restores it on finish, so a
  same-play `set_opacity`/`set_stroke` change is silently reverted. Un-dim in a prior play.
- `caption_bar` shrinks over-wide text instead of erroring, so caption size drift is invisible
  unless measured.

## Follow-up (not done here — costs TTS credits, overwrites assets)

Narration changed and there are now nine beats, so `beat_*_audio.wav` are stale and beats 7–9
have no audio at all. Re-run in order:

1. `Tutorial: Cooling 6 — Generate Audio` (launch.json)
2. `Tutorial: Cooling 6 — Build Full Video` (launch.json)

Available if wanted, from the same recovered file: `Beat5_Luftfuehrung` (Mischlüftung vs
Quelllüftung, with the temperature-stratification animation) and `Beat6_RLTFunktionen`
(Filtern · Heizen · Kühlen · Be-/Entfeuchten). Left out to keep the mechanical block from
outweighing the natural one.
