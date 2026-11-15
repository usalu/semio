# Visual audit: German speaker finals 1080p30 (physical fundamentals, ventilation)

Read-only audit (no source edited). Frames: `🗑️generated/check-a/` (1280 wide, one per 20 s; `phys/p_NNN.png` and `vent/v_NNN.png`, NNN = 20 s index + 1; contact sheets in `sheets/`; spectator and early frames in `spec/` and `early/`).

## Media facts (ffmpeg, bundled imageio binary)

| File | Duration | Video | Audio |
|---|---|---|---|
| Full_Physical_Fundamentals_speaker_de_1080p30.mp4 | 00:18:12.37 | H.264 1920x1080 30 fps | AAC 48 kHz stereo, 2 kb/s |
| Full_Ventilation_speaker_de_1080p30.mp4 | 00:31:09.80 | H.264 1920x1080 30 fps | AAC 48 kHz stereo, 2 kb/s |

Both files have an audio stream, but it is at 2 kb/s and RESUME.md states no speech was synthesized (silent voice track). The ventilation final (16:13) also predates the morph fix that RESUME.md says was needed; the physics final (18:17) is newer.

## Results per item

| # | Item | Result | Frames / evidence |
|---|---|---|---|
| 1 | Opening UI cards show real frames, no empty or broken card | FAIL (physics), PASS (ventilation) | Physics: `early/phys_0_to_3s.png` and `phys/p_t2s.png` (app overview): card "Parametrik" shows the placeholder "Noch keine Videos" (empty card). Other cards show real frames. Ventilation: `vent/v_t2s.png` all six cards show real rendered frames. Spectator overview not checked. |
| 2 | Roadmap (Fahrplan) is first content scene | PARTIAL | Physics `phys/p_001.png` and ventilation `vent/v_001.png` show the roadmap at the 0 s sample. But direct seeks (`early/phys_0_to_3s.png`, 0 to 3 s, and `vent/v_t2s.png`) show a screen recording of the app (chapter overview "Zusammenfassung" / "Lüftung" cards) before the roadmap. The app recording is an intro before the first content scene, so the roadmap is not clean first. Timing of the roadmap start not pinned. |
| 3 | No bare bracket units; numbers carry units; compound units as fractions; readable parentheses | PASS (sampled) | Examples: `phys/p_004.png` ("P in kW", "t in h"), `phys/p_011.png` (Q̇_H in kW, Q_h ≈ 15000 kWh/a), `phys/p_017.png` (1kWh = 1000W · 3600s), `vent/v_015.png` (V̇ = 220 m³/h), `vent/v_028.png` (Q̇_V,sens = V̇ · c_Luft · Δθ in W). No "[W]" seen. Not every frame was checked for unit placement inside equations. |
| 4 | Captions at most two lines; symbols as math; no raw $ or TeX source | PASS (sampled) | Captions wrap to at most two lines in all sampled frames (e.g. `vent/v_029.png`, `vent/v_033.png`). Δθ, Q̇_V, θ, ρ rendered as math. No raw `$`, TeX source or spelled-out symbols seen. |
| 5a | Physics: JAZ reference (VDI 4650) | FAIL (overlap) | `phys/p_030.png` (580 s; top-right frame of `sheets/phys_s029.png`): the "Jahresarbeitszahl (JAZ) JAZ ≈ 3" / "VDI 4650 Blatt 1:2024-02 · DIN EN 14825:2023-10 (SCOP)" block overlaps the "außen"/"innen" axis labels. Reference text itself is present. |
| 5b | Physics: Endenergie / Umweltwärme / Nutzwärme distinction | PASS | `sheets/phys_s025.png` (Q_N, Q_U, Q_E labels on stack bars) and `phys/p_032.png` (Q_E bars, Brennstoff/Strom Endenergie legend). |
| 5c | Physics: shower icon beat (Beat1) | NOT SEEN | Not in the 20 s samples; no shower glyph found. Needs a frame at Beat1 (approx. 0:00 to 1:00 of the Leistung chapter). |
| 5d | Ventilation: "Grenzwert nach Pettenkofer" | PASS | `vent/v_004.png` ("Grenzwert nach Pettenkofer (1858)", 1000 ppm). Reference corner `vent/v_004.png` (Bundesgesundheitsbl. 51 (2008) 1358-1369). |
| 5e | Ventilation: ρ 1,2 / 1,27 | PASS | `vent/v_022.png` ("ρ ≈ 1,2 kg/m³ (20 °C)", "1,27 kg/m³ (5 °C)"). |
| 5f | Ventilation: units slide "Gebäudebezogene Einheiten / SI-Einheiten" | PASS | `vent/v_024.png` (two framed lines, 75 W/K vs 74 W/K). |
| 5g | Ventilation: 8 % reference | PASS | `sheets/vent_s041.png` (frame 42: "Maas (1995), Diss. Univ. Kassel, Kap. 8.2, S. 74", n_Kipp ≈ 1 h⁻¹, 5 min / 60 min ≈ 8 %) and `vent/v_045.png` header. Page number 74 is recorded as estimated in vent-b/REPORT.md. |
| 5h | Ventilation: stack effect (no window glyph), Δρ, Δp, v steps | PASS | `vent/v_049.png` through `vent/v_054.png`: low inlet, high outlet on shaft, no window glyph; Δρ = 0,032 kg/m³, Δp = 0,8 Pa, v = 1,2 m/s steps. |
| 5i | Ventilation: night ventilation with sun and shading | PASS | `vent/v_058.png`, `vent/v_059.png` (sun, "Verschattung", night band on the 48 h axis). |
| 5j | Ventilation: supply low / extract high (no short-circuit) | NOT CONFIRMED | Stack and natural openings show low inlet / high outlet (`vent/v_049.png`). Mechanical room drawings (Beat1 of mechanical chapter) not seen clearly in the samples. |
| 5k | Ventilation: hand-over with nine cards | PASS | `vent/v_085.png`, `vent/v_089.png` (nine cards in the row). Final frame `vent/v_093.png` shows the hand-over slide. |
| 5l | Ventilation: winter latent 770 W and a sum row | PASS | `vent/v_034.png`, `vent/v_036.png` (Q̇_V,lat = 770 W, sum Q̇_V = Q̇_V,sens + Q̇_V,lat), `vent/v_087.png` (2270 W / 1260 W sums). |
| 5m | Ventilation: "Kühltechnik" wording | PASS | `vent/v_036.png`, `vent/v_093.png`. |
| 6 | Overlap, clipped text, text off-screen, empty frames, broken layout | FAIL (several) | See list below. |

## Item 6 detail (overlaps, errors, empty frames)

| Severity | Video | Frame (20 s grid) | Finding |
|---|---|---|---|
| Fail | Physics | `phys/p_030.png` (580 s) | JAZ/VDI reference block overlaps "außen"/"innen" labels. |
| Fail | Physics | `phys/p_041.png` (800 s) | "24" overlaps "t in h" at the end of the x-axis of the thermal-mass chart. |
| Fail | Physics | `phys/p_051.png` (1000 s) | Blank scene with only the caption (empty frame). |
| Fail | Ventilation | `vent/v_017.png` (320 s) | Blank scene with only the header (empty frame). |
| Fail | Ventilation | `vent/v_045.png` (880 s) | "Stoßlüften 340 Wh" label overlapped by a second bar; the value text is hidden. |
| Fail | Ventilation | `vent/v_057.png` (1120 s) | "h = 4,4m" overlapped by the winter info box text ("ρ(θ_i) = 1,204 kg/m³"). |
| Fail | Ventilation | `vent/v_061.png` (1200 s) | Legend boxes at bottom left ("Nacht ... öffnen", "Tag ... verschatten"): garbled overlapping text. |
| Fail | Ventilation | `vent/v_030.png` vs `vent/v_031.png` | Values do not agree: outside 14,4 g/kg (and inside 10,1 g/kg) in the chart, while the caption and next frame say 15,0 and 10,5 g/kg (Δx 4,5 g/kg). |
| Fail | Ventilation | `vent/v_044.png` | Kipplüftung bar sums to "827 Wh" with an unlabelled segment; later frame `vent/v_045.png` says "≈830 Wh" with labelled segments 340 + 300 + 190. |
| Fail | Ventilation | `vent/v_055.png` | Arithmetic: 0,6 · 0,1 m² · 1,2 m/s = 0,072 m³/s, shown as 0,069 m³/s (≈250 m³/h). |
| Fail | Ventilation | `vent/v_027.png` vs `vent/v_028.png` | V̇ shown as 219 m³/h in one frame (v_027) and 220 m³/h in the next (v_028). |
| Fail | Ventilation | `vent/v_090.png` (top-left, 1800 s) | Stale element "Q̇_V,sens = 386 W" next to the Heizung box (conflicts with 1500 W). |
| Minor | Ventilation | `vent/v_005.png` (80 s) | "Grenzwert nach Pettenkofer (1858)" label collides with the Feuchte/Geruch column headers. |
| Minor | Ventilation | `vent/v_054.png` | Formula box for v = √(2Δp/ρ) overlaps the house drawing. |
| Minor | Ventilation | `vent/v_065.png` | "ungenutzt" label touches the left frame edge. |
| Minor | Ventilation | `vent/v_072.png` | "-360 W" label partly hidden behind the bar. |
| Minor | Ventilation | `vent/v_077.png` | Fensterlüftung and Zentrale WRG boxes overlap at their border. |
| Minor | Ventilation | `vent/v_079.png` | Caption text faint (low contrast). |
| Minor | Physics | `phys/p_053.png`-`phys/p_055.png` | Final frame `phys/p_055.png` (1080 s) is a screen recording of the app (Physikalische Grundlagen tab), not a scene. |
| Note | Ventilation | `vent/v_056.png` | "2,7-fache" in the caption not verified from the frame. |

## Not checked

- Spectator versions: only `spec/phys_spec_t0s.png` viewed (full-screen summary, no overlap). `spec/*_t2s`, `_t6s`, `_t12s`, `_t19s` and the ventilation spectator frames were extracted but not viewed.
- Speaker 2/6/12 s frames: `phys/p_t2s.png` and `vent/v_t2s.png` viewed (app overview). `p_t6s`, `p_t12s`, `v_t6s`, `v_t12s` not viewed.
- Items 5c (shower), 5j (supply/extract) need a targeted re-check.

## Frame index used

- Physics: `phys/p_001.png` ... `phys/p_055.png` (20 s grid; 55 frames). Contact sheets `sheets/phys_s001.png` ... `sheets/phys_s053.png` (4 frames each).
- Ventilation: `vent/v_001.png` ... `vent/v_093.png` (20 s grid; 93 frames). Contact sheets `sheets/vent_s001.png` ... `sheets/vent_s093.png`.
- Speaker 2/6/12 s: `phys/p_t2s.png` etc. and `vent/v_t2s.png` etc. Spectator: `spec/*.png`. Early seek: `early/phys_0_to_3s.png`.
- Note: sheet names may use a 4-frame index; the frame number in each name is the first frame of the sheet.

Note on frame numbering: `vent/v_NNN` numbers in the table above are the 20 s grid index (t = (NNN-1) * 20 s). Where a row says `vent/v_0xx.png` in a sheet, the frame was checked in the contact sheet `sheets/vent_s0xx.png`.
