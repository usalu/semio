# Apply Cooling Review Feedback

Goal: `R26-02/UPDATED-DOCS/UPDATED-USER-DOCS/UPDATED-TUTORIALS`

Repo MCP was unavailable (`CONNECTION_CLOSED`), so `repo://goals`, `ticket_open` and `ticket_close` could not be called. Ticket created and logged here by hand, same as `APPLY-HEATING-FEEDBACK`.

Dev instruction: apply the supervisor's Cooling feedback **scene by scene** — finish and render-check one Teil before starting the next. Work spec with the verbatim feedback and the scene mapping: [spec.md](spec.md).

## Teil 1 — Heizwärmebedarf vs. Kühllast ✅

`1_heating_vs_cooling/scene_1.py`

- [x] Mechanische Lüftung zu rudimentär → Beat3 rebuilt: `_build_ahu` draws a Lüftungsgerät at the left facade with rotating fan, blue Kühlregister coil, and the four VDI air paths (Außenluft → Kühlregister → Zuluft, Abluft → Fortluft) as `flow_guides` + `animate_flows` particle streams. Supply stream recolours orange→cyan at the register, extract stream red→orange; the fan spins during both passes and the heat block drains while the streams run.
- [x] Faktoren/Teilwerte: thermometer already counts 35 → 21 °C live during the flow; Kühllast gets its typeset symbol `Q̇_K` pinned to the Fortluft stream at the outro.
- [x] Latex: `Q̇_K` via `math_label` (shared `math_text` helpers from the Heating ticket) — real dot accent + subscript.
- [x] Text: `"Solare Gewinne (Exzessiv)"` → `"Übermäßige solare Gewinne"`; `"bricht der Komfort … zusammen"` → `"bleibt der Raum … zu warm"`. Narration "vent" clause now names Abluft / Kühlregister / Zuluft to match the drawing.
- Render-checked `-ql` all three beats; frames in [checks/](checks/) (`b3_v2_vent.png`, `b3_v2_outro.png`, `b2_v2_excess.png`). Narration is single-sourced from `scene_1.py`, so `generate_audio.py` needs re-synthesis only when VO is next rebuilt (clause keys unchanged).

## Teil 2 — Interne Wärmegewinne ✅

`2_internal_gains/scene_2.py`

- [x] Körperwärme-Aktivitätsskala → Beat2 gains `_activity_figure` poses (Schlafen 80 W auf dem Bett, Büroarbeit 100 W am Stuhl, Gehen 200 W, Hochleistungssport 800 W mit Tempo-Strichen): each pose fades in with growing heat waves while its watt readout counts from 0, then all four `ReplacementTransform` onto one 0–800 W scale with staggered labels; the Büroarbeit point gets ringed and its 100 W flies into `q̇_p` when the formula names it.
- [x] f_N je Nutzung → Beat3 adds three usage cards (Büro 7/10, Serverraum 10/10, Besprechung 3/10): ten device icons per card light up one by one while `f_N` counts live to 0,7 / 1,0 / 0,3 — the lit share *is* the factor.
- [x] f_g 0→1 animiert → during the `fg` clause one of two luminaires switches off, its heat waves fade (fill+stroke — `set_opacity`, not `set_stroke`: the waves are filled ParametricFunctions), and `f_g` counts 1,0 → 0,5 beside the fixtures.
- [x] Latex → all four formula panels typeset via `math_panel` (`Q̇_Pers = n·q̇_p`, `Q̇_Geräte = Σ P_el·f_N`, `Q̇_Licht = Σ P_Licht·f_g`, `Q̇_i = …`), Beat4 card terms via `math_label`.
- [x] Text → "brodelt ein Wärmeproblem", "Wärmefalle … Überstunden", "Kälteanlage kämpfen" plainer.
- Render-checked `-ql` Beats 1–4 + 8; frames `s2b2_*`, `s2b3_*`, `s2b4_*` in [checks/](checks/). `probe_fade.py` documents the fill-opacity gotcha.

## Teil 3 — Transmission & Feuchte ✅

`3_transmission_humidity/scene_3.py`

- [x] Spitzenlast später Nachmittag → Beat2 "peak" narration now says "typisch in den späten Nachmittag, selten erst in den Abend"; the clock advances ~5 h instead of two full turns and the sun sinks low and dim instead of vanishing below the horizon.
- [x] Latex → all four panels typeset via `math_panel` (`Q̇_T = U·A·Δθ_eq` twice, `Q̇_L = Q̇_sens + Q̇_lat`, `Q̇_sens`/`Q̇_lat` product chains), unit rows via `math_label` (`kg/m³`, `kJ/(kg K)`, `m³/s` with real super-/subscripts), fly-in tokens via `math_label`.
- [x] Captions free of raw `ρ_a · c_p,a` strings — spelled out as prose.
- Render-checked `-ql` all four beats; frames `s3b1_formula.png`, `s3b2_peak.png`, `s3b4_sens.png`, `s3b4_lat.png` in [checks/](checks/).

## Teil 4 — Solare Einstrahlung ✅

`4_solar_radiation/scene_4.py`

- [x] Sonnenleistungskurven aus 3D-Sonnenvisualisierung → Beat1 rebuilt: the facade morphs into a cavalier-projected cube house (`_cav_proj`, dashed hidden edges, N/O/S/W tags, compass); the sun shrinks onto a dashed 3D arc Ost→Süd→West and sweeps 6–18 h while the four facades glow with their live direct-beam share and the chart draws all five orientation curves from the same solar model (`_sun_angles` + `_facade_irradiance` with air-mass attenuation `_sun_atten` — the curves are computed from the sun, not hand-tuned bells). A time cursor links sun position and chart.
- [x] Faktoren 0→1: Beat2 adds `F_F = 1,00 → 0,70` live readout while the frame eats the glass share; Beat3 already drives `F_V` 1,0 → 0,15 on its scale (labels now typeset).
- [x] Latex → all five formula panels via `math_panel` (`A_eff = A·F_F`, `I_red = I_S,max·F_V`, `g_tot = τ_e + q_i`, `Q̇_S,tr = A·F_F·F_V·g_tot·I_S,max`), `τ_e`/`q_i`/scale labels via `math_label`; watt-anchor title and all captions free of raw `_` strings.
- Render-checked `-ql` all five beats; frames `s4b1_v2_*`, `s4b2_v2_ffread.png`, `s4b3_fv.png`, `s4b5_eq.png` in [checks/](checks/).

## Teil 5 — Systemauslegung ✅

`5_systemauslegung/scene_5.py` (+ shared `tutorial/manim_visuals.py`)

- [x] Latex → shared math typesetting extended with `\sqrt{…}` (stretched radical + overbar) and `\pi` (additive, `#region Math typesetting`; probe `checks/probe_sqrt.py`). All five beats typeset: `Q̇_V = ρ_a·c_p,a·Δθ·q_v,R`, the rearranged `q_v,R` as a **real fraction** with coloured numerator/denominator, `A = q_v,R/v_m` as a fraction, `r = √(A/π)` with radical, plus every θ/ρ/c/q label, card value, balance pan, bridge line, tip and the Beat5 chain tokens via `math_label`/`math_row`.
- [x] Highlight rings keep working on fraction parts by addressing the frac box (`items["frac"][0]` → numerator `[0]`, denominator `[2]`).
- [x] Beat5's taller fraction panels rescaled/respread so the stacked chain no longer collides with the token row.
- Render-checked `-ql` all five beats; frames `s5b2_cards.png`, `s5b3_frac.png`, `s5b5_v2_chain.png` in [checks/](checks/). (The "Mechanische Lüftung am Anfang" item was Teil 1's Beat 3 — fixed there; Teil 5 Beat 1's system view already shows RLT unit, ducts and grilles.)

## Teil 6 — Lüftungssysteme ✅

`6_lueftungssysteme/scene_6.py` (+ playlist, audio scripts, vo_timing, KONZEPTE_UND_FORMELN.md)

- [x] Sorptionsgestützte Kühlung → new `Beat9_SorptionsKuehlung` (DEC): Sorptionsrad (rotating spokes), Plattentauscher, two Verdunstungskühler, Solar-Register with sun + dashed Solarwärme duct. Supply particles recolour along the chain (32 °C feucht → 48 °C trocken → 24 °C → 19 °C Zuluft, typeset state readouts); exhaust runs back through humidifier → exchanger → solar heater → wheel regeneration → "Fortluft feucht · warm". Nachteil clause: microbe panel — temperature/humidity flashes, common germs die, resistant red ones survive and duplicate ("Selektionsdruck: resistente Keime überleben").
- [x] Natürliche Lüftung vs. Passivhaus → Beat8 gains a "passivhaus" clause: the η panel gives way to a crossed-out open window with `η = 0` and the line "Passivhaus ⇒ Zu-/Abluft mit WRG — Fensterlüftung entfällt als Hauptweg"; narration states the contradiction and why (Lüftungswärmeverluste without recovery).
- [x] Old Beat9 renamed `Beat10_KomfortStrategie` everywhere (scene, playlist, generate_audio, build_full_video, vo_timing key kept with its measured durations).
- [x] Latex → Beat3 `A_eff = 1/√(1/A₁² + 1/A₂²)` as real nested fraction+radical (highlight rings address the frac internals), Beat4 `Δp = h·g·(ρ_a − ρ_i)`, Beat8 `η` as double fraction; `A_1`/`A_2`/`ρ_a`/`ρ_i`/`A_eff ≈ …` labels via `math_label`; captions free of `_`.
- Render-checked `-ql` Beats 3, 4, 8, 9, 10; frames `s6b3_eq.png`, `s6b8_v3_ph.png`, `s6b9_*` in [checks/](checks/).

## Formelsammlung

`tutorial/energy/demand/KONZEPTE_UND_FORMELN.md` — Teil 2 gains the activity watt scale and the f_N-per-usage table (+ f_g note); Teil 6 gains the DEC process chain with the microbial drawback and the Passivhaus-contradiction paragraph.

## Series check

Full playlist re-render `-q l` (`full_cooling_video.py`, silent 480p concat incl. new Beat9/Beat10): log `checks/full_render_ql_v3.log`, 1 814 s, all seven clips rendered 2026-10-08 16:51–17:04. Spot frames from the merged file: `checks/overlap/merged_sheet.png`.

Root-cause fix in `full_cooling_video.py`: sections were skipped whenever an mp4 existed, so a merge silently reused pre-fix clips (`full_render_ql.log` / `_v2.log` concatenated the 2026-10-07 clips). `_section_sources` + `_is_current` now reuse a clip only when it is newer than its scene files, `vo_timing.json`, `full_cooling_video.py`, `manim_visuals.py` and `manim_fonts.py`; the intro likewise against `intro_scene.py`.

Note for parallel guard runs: two Manim processes in the same scene folder race on `media/texts/*.svg` (`FileNotFoundError`) — run one process per folder.

## Overlap audit (reopened 2026-10-07, dev: "no text on text or objects — verify with screenshots")

Method, per Teil: strict layout guard (`LAYOUT_CHECK=1`, `--dry_run`) → fix → real `-ql` render → contact sheets (`checks/contact_sheet.py`, one frame every 2 s, 12 per sheet, in `checks/overlap/`) reviewed by eye → fix → repeat until both are clean.

Guard extensions in `tutorial/manim_visuals.py` `#region Layout guard` (probes `checks/probe_guard.py`, `checks/probe_caption_formula.py`):
- checks after **every** `Scene.play` (not only clause holds), each issue printed once with scene time;
- labels must keep a 0,02 gap (touching counts), parts of one typeset label never collide with each other;
- **line through label** (sampled bezier strokes) and **filled object under label** (unless it is the label's backdrop);
- caption/formula frames cut through foreign labels, and any drawn object touching a formula frame / caption box is flagged;
- caption wrapping beyond 2 lines is flagged as a layout issue (it pushes the caption box into the formula panel).
- `\sqrt` radical: more head-room and gap so highlight rings on the radicand never touch the √.

Fixes:
- Teil 1: "Interne Gewinne" below the house (was on the light bulb — the dev's screenshot), summer label two lines, Q̇_K moved off the airflow labels.
- Teil 2: activity scale — Schlafen below the axis; 100 W stops above the formula box; laptop/rack no longer drop through the title; f_N cards taller; Beat 4 cards taller with icon centred between label and term; Beat 8 percentages count live and the two sliders sit further apart.
- Teil 3: content lowered; U/A tokens start apart and morph into the formula; Q̇_sens/Q̇_lat tokens rise into free sky; window bands thinner with arrow heads at their ends; sun rays only on sun-facing surfaces; headers raised, Δx label moved left of the gauge.
- Teil 4: watt badge beside the facade; compass in the free corner; only S and O face tags; path tags clear of the sun and faded before the formula; curve labels off the curves; caption shortened to two lines; sun morphs straight into `I_S,max`; Beat 3 formula panel at standard height, shifted right of the section; "Restanteil" in the free corner; τ_e/q_i glow kept between the tokens; Beat 4 panel at standard height, glazing detail tucked under the pane.
- Teil 5: Beat 2 room shorter so the θ cards clear the formula frame; air blob morphs straight into `q_v,R`; Beat 5 stacked fraction panels scaled.
- Teil 6: Beat 2 opening label left of the window; Beat 3 and 8 captions shortened to two lines; Beat 4 "Abluft" beside the outlet (no longer rides into the subtitle); Beat 5 "≈ 4 K" below the bracket; Beat 6 meter moved off the ✕ list; Beat 7 sign rows spread and the selection frame encloses all labels; Beat 8 "Wärmestrom" off the exchanger and the channel note fades before the formula; Beat 9 sun below the heater, labels spread, microbe strip compact between component labels and caption; Beat 10 decision chart fitted into the inner band and diamond enlarged, ja/nein beside the links.

## Offen

- German VO re-synthesis (`generate_audio.py` per Teil) is needed before the next muxed deliverable: new/changed clauses have no measured timings yet (hold budgets fall back to word-rate estimates; clause keys unchanged where text was only toned down).

## Reopened 2026-10-09 — remaining review gaps

Repo MCP still unavailable. Same ticket, because the review was only partly in the picture.

- Teil 1: the cooling coil counts 32 → 18 °C while the air moves through it.
- Teil 2: `n` counts the people drawn (3 at the formula, 50 in the hall). Heavy lines (biological heater, toaster, light-bulb simile) are gone.
- Teil 3: the clock lands on 17 Uhr, später Nachmittag, with 21 Uhr Abend dim beside it. ΔΘ counts from the thermometer falling 30 → 20 °C. Live °C and r.F. readouts are typeset.
- Teil 4: `F_F` is the drawn glass area over the rough opening (0,49). `F_V` counts 1,00 → 0,15. `g_tot` counts 0 → 0,50 → 0,60 as transmission then secondary heat appear. The south curve’s own peak is `I_S,max`. The solar-load beat draws frame, slats and glass and counts each factor. The Argon / Low-E catalogue line is gone. The axis unit is typeset.
- Teil 5: the coil counts 30 → 18 °C. Δθ counts `25 − 18 = 7 K`. Supply and room temperatures are typeset. The “thermodynamic product” line is plain.
- Teil 6: the opening no longer spends the load on open windows. An open window is a heat-loss path (`Q̇_L`); the reserve is supply/exhaust with recovery. η counts from the supply air falling 32 → 27 °C. The sorption drawback shows the germs mutate and the harmful ones remain. Temperature badges are typeset.

Low-quality full film (silent): `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_480p15.mp4` — 854×480, 29:02.

## Reopened 2026-10-10 — wipe renders + HQ full film

Dev: delete every rendered Cooling video, then render the full series again at highest quality.

- Deleted 3 915 mp4s under `tutorial/energy/demand/Cooling/` (section `media/`, shared `media/`, `rendered/`). Log: `checks/deleted_renders.txt`.
- Force HQ: `full_cooling_video.py -q h --force --no-play` → log `checks/full_cooling_qh_force.log`.
- Delivered: `tutorial/energy/demand/Cooling/rendered/Full_Cooling_Demand_1080p60.mp4` — 1920×1080 @ 60 fps, silent, ~28:50 (intro + Teile 1–6). Same path for `Full_Cooling_Demand_NoAudio_1080p60.mp4`.
