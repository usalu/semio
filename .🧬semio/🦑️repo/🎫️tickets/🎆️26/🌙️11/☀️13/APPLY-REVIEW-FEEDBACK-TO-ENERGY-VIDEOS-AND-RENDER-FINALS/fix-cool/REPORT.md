# Cooling video, careful visual polish pass (agent `fix-cool`)

Date 2026-11-13. COOL = `🎓️teaching/🏛️architecture/⚡️energy/❄️cooling`, clips in `COOL/🎬️clip/`. Scripts in this folder (`sheet.py`, `sheet3.py`, `grab.py`, `grab.sh`, `make_report.py`). Frames and contact sheets in `🗑️generated/fix-cool/` (to be deleted by the ticket owner).

## 1. Method

Source: `final/de/speaker/rendered/Full_Cooling_Demand_speaker_de_1080p30.mp4` (15:36 final, 18:30). One frame every 4 s (278 frames, imageio-ffmpeg 7.1), six per contact sheet (47 sheets), every sheet viewed; suspicious frames re-extracted at 1920x1080 and cropped (`grab.sh`). Mid-animation frames (fades, ramping counters such as `0,16 · 0,00 · 0 ≈ 0 W`, a thermometer at 33 °C) were told from held states by neighbouring frames and by the scene code.

## 2. Findings

Checked and fine: title box vs reference chip (no collision in any chapter; the two-line chip of the winter-gains scene stays clear of the title), captions at most two lines, no raw TeX or `$`, no bare `[W]` (the heating-owned `[Q̇_S] = W` line is the allowed form `[Q] = W`), symbols (Q̇_C, Q̇_T, Q̇_V, Q̇_S, lower-case Q̇_i with i,p / i,e / i,l, F_C, g_⊥, θ_i, "Kühltechnik"), number chains: 1200 + 420 + 400 = 2020 W; 450 + 810 = 1260 W; 680 + 1260 = 1940 W; 30 m² · 0,70 · 0,15 · 0,60 · 800 W/m² = 1512 W shown as 1510 W; 1,51 kW / (1,2 kg/m³ · 1,0 kJ/(kgK) · 8 K) = 0,157 m³/s; 0,157 / 2,5 = 0,0629 m²; 0,157 / 0,061 = 2,6.

Defects found and fixed:

| # | where (time in the old final) | defect | fix |
|---|---|---|---|
| F1 | sizing, airflow comparison, 928-940 s | solar bar fills past its track (scale `full = 0,14 < 0,157`) and runs into the readout `0,157 m³/s` | `full = 0,18` |
| F2 | sizing, duct discs, 940-952 s | label `A = 0,0244 m²` of the hygiene disc sits on the top border of the area equation box | discs 8 per m (was 9), centre y 1,15, labels lifted to y -0,32 / -0,68 (box top at -0,94) |
| F3 | solar irradiance chart, 672-688 s | readout `I_s,max = 478 W/m²` (south peak) disagrees with the `800 W/m²` anchor 8 s earlier and with the 800 W/m² of the next scene and of the sizing chain | design irradiance = maximum over all five curves, curve amplitude 950 to 976 so the horizontal peak is 799,95 W/m²; readout tracks that maximum |
| F4 | sizing caption `load`, 912-936 s | "Aus Teil 4 kennen wir die Last: 30 m² · … = 1510 W" while part 4 showed 1,92 m² and about 97 W | "Aus Teil 4, für alle Fenster des Hauses: 30 m² · …" (en and de) |
| F5 | chapter 1 coil and cool-down, 124-212 s; DEC scene | coil 32 to 19 °C with Δθ = 13 K and room settling at 21 °C, against supply 18 °C, room 26 °C, Δθ 8 K in the sizing chapter; DEC supply label 19 °C | coil 32 to 18 °C, Δθ = 14 K, room settles at 26 °C (thermometer red only above 26,5 °C), DEC supply 18 °C |
| F6 | internal gains beats 1 to 5, sizing beat 3, at every beat start (192-198, 224-228, 316-320, 396-400, 456-460, 840-852 s) | the first caption of the beat is held 6 to 12 s on an empty stage (title, reference, caption only) | first visual built before the intro hold in all six beats (beat 4 shows the three cards under the intro caption, then swaps) |
| F7 | internal gains beat 5 intro | German caption said "Kälteanlage" for the system | "Kühltechnik" (en: "cooling technology") |

Not changed (judged correct): held house plus thermometer at 124-136 s is a narration hold with live ripples; `F_C = 0,00` at 712-716 s is the heating scene's ideal-blind step before it settles at 0,15; the solar chapter shows `Q̇_S = A · F_F · F_C · g_⊥ · I_s,max` without `F_S`, `F_W`, which is the cooling simplification pinned to 1510 W by the running example.

## 3. What changed (file:line in `COOL/🎬️clip/`)

- `👥️internal-gains/🐍️.py`: 207-216 (beat 1), 289-297 (beat 2), 590-608 (beat 3), 797-838 (beat 4), 915-990 (beat 5), 894-895 narration.
- `📐️system-sizing/🐍️.py`: 388-389 narration `load` (en, de); 449-461 balance built before the intro hold; 522 `full = 0,18`; 555-568 duct disc geometry.
- `☀️solar-radiation/🐍️.py`: 174 amplitude 976; 327-331 and 376-383 `peak` over all curves.
- `⚖️heating-comparison/🐍️.py`: 187 colour rule; 398-402 narration en/de (18 °C, 14 K, 26 °C); 448 and 455 targets.
- `🌤️summer-strategy/🐍️.py`: 357 supply label 18 °C.
- Tests: `📐️system-sizing/🧪️tests/🐍️.py` new class `SizingChain` (independent arithmetic of 1510 W, 0,157 m³/s, 0,0629 m² and chart peak = 800 W/m²), `🧪️tests/🔣️.cases.json` block `chain`. Cooling tests pass (sizing 4, clip 4, internal gains 3).

## 4. Verified by eye

Preview renders (Cooling_05, 02, 04, 01) and then the new 1080p final (frames every 4 s, sheets in `🗑️generated/fix-cool/new`): bars inside tracks (924-940 s), disc labels clear of the equation box (940-944 s), balance visible from 844 s, `I_s,max = 800 W/m²` with the chart (676-680 s), `Δθ = 14 K` and 18 °C coil, room settling at 26 °C, overview card of chapter 1 shows 26 °C and 18 °C, no empty stage at the starts of internal gains beats 1 (196 s), 2 (228 s), 4 (396 s), 5 (460 s) and sizing beat 3, new load caption in two lines (916-920 s), "Kühltechnik" caption (460 s). Beat 3 start (316 s) still shows one half-faded frame, which is the normal fade-in.

Final render (profile final, German, `--jobs 3`): speaker and spectator finals were rebuilt and both printed "Ready: ...Full_Cooling_Demand_speaker_de_1080p30.mp4" (speaker, 01:07, 336 MB, 18:22) and "Ready: ...Full_Cooling_Demand_spectator_de_1080p30.mp4" (02:17). Two earlier attempts failed at assembly ("Chapter inputs changed while assembling the lesson", "...before publication: Cooling_04_SolarRadiation") because other agents edited physics fundamentals and `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` (imported by cooling solar) during the run; the third run, without edits in between, succeeded. Fixed spots viewed in both finals: 26 °C and 18 °C (chapter 1), balance of `I_s,max = 800 W/m²`, disc labels and bars (sizing), "Kühltechnik" caption (speaker only; the spectator cut has no captions by design).

## 5. Shared follow-ups (not touched, outside `❄️cooling/`)

1. `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json` lines 401-402 pin the sizing caption `load` (en, de) with the old text; update to the new clauses or that test fails.
2. Stored posters of Cooling_01, 02, 04, 06 are stale after these edits (render warns "stale stored posters"); recapture with `poster-ensure.py`.
3. Thermal-mass scene (physics `🧲️fundamentals/🐍️.py:2604`): the reference chip `DIN EN ISO 13786:2018-04 · Wärmespeicherung` sits tight under the room slab at bottom right (readable, about 27 px from the frame edge); it belongs in the top-right corner like everywhere else.
4. Pattern for the other energy videos: "first caption held on an empty stage" appears wherever `hold_for(..., "intro", used=BEAT_SUBTITLE_FADE + 0.3)` directly follows `caption_bar` before any visual; grep for it.
5. The first complete final failed at its very end ("Chapter inputs changed while assembling the lesson") because the physics fundamentals source was edited by another agent during the run; the render had to be repeated (about 1.5 h lost).

## 6. Remaining

- Speech for the changed clauses (sizing `load`, chapter 1 `coil` and `cool_down`, internal gains beat 5 `intro`) is synthesised by the final render; not listened to.
- The new final was checked at the fixed spots and at chapter starts, not frame by frame again.

