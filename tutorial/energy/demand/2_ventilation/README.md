# Ventilation — Full Video

Plays the **Ventilation series** in curriculum order, prepended with the series
intro card ``Demo_Intro_Lueftung`` from ``tutorial/intro/intro_scene.py``:

0. Intro — Lüftung
1. Kapitel 1 — Warum lüften?
2. Kapitel 2 — Wie viel Luft?
3. Kapitel 3 — Was die Luft trägt
4. Kapitel 4 — Natürliche Lüftung
5. Kapitel 5 — Mechanische Lüftung und Wärmerückgewinnung
6. Kapitel 6 — Übergabe an Heizung und Kühlung

Entry file: [`full_ventilation_video.py`](./full_ventilation_video.py)

This is the second video in the curriculum, after `1_physical_fundamentals/` and
before `3_heating/`, `4_cooling/`, and `5_energy_balance/`.

---

## Why this video comes before Heating and Cooling

Ventilation affects heating **and** cooling: the air change rate matters in winter
(heat leaves with the air) exactly as in summer (heat and moisture arrive with the
air). It is therefore explained once, season-neutral, and Heating and Cooling only
plug their own season into it:

| Quantity | Defined in | Heating uses it as | Cooling uses it as |
|----------|------------|--------------------|--------------------|
| `V`, `n`, `V̇ = V·n` (= `q_v,R`) | Kapitel 2 | `Φ_V = V·n·c_Luft·Δθ` | `q_v,R` in `Q̇_V = ρ·c_p·Δθ·q_v,R` |
| `c_Luft = ρ·c_p / 3600 ≈ 0,34 Wh/(m³·K)` | Kapitel 3 | winter factor | summer factor |
| `Δθ` (signed), `Δx` | Kapitel 3 | `Δθ = 20 K`, no latent part | `Δθ = 6 K`, `Δx = 4,5 g/kg` |
| `η_WRG = (θ_ZUL − θ_AUL) / (θ_ABL − θ_AUL)` | Kapitel 5 | `Φ_V·(1 − η_WRG)` | cold recovery |
| duct `A = q_v,R / v_m`, `r = √(A/π)` | Kapitel 5 | — | sizing method for the cooling airflow |
| night / window / cross / stack ventilation | Kapitel 4 | Stoß- vs. Kipplüftung | free cooling, night discharge |

Kapitel 6 closes with the hand-over: what Heating takes (`Φ_V` sensible) and what
Cooling takes (sensible + latent, sizing airflow, night ventilation, cold recovery).

---

## The through-line

One house, one set of numbers, from Kapitel 1 to 6:

```
V_e = 580 m³ → V ≈ 440 m³ (0,76·V_e)
n = 0,5 h⁻¹ → V̇ = 220 m³/h = 0,061 m³/s (= q_v,R)
c_Luft ≈ 0,34 Wh/(m³·K)
Winter: Δθ = 20 K → Q̇_V,sens ≈ 1500 W   → mit WRG (η = 0,8) ≈ 300 W
Sommer: Δθ =  6 K → Q̇_V,sens ≈  450 W   → mit WRG (η = 0,8) ≈  90 W
        Δx = 4,5 g/kg → Q̇_V,lat ≈ 810 W (Plattentauscher gewinnt nichts zurück)
Kanal:  v_m = 2,5 m/s → A ≈ 0,0244 m² → r ≈ 0,088 m
```

Where each number lives, so a change is made once:

| Value | Source of truth |
|-------|-----------------|
| `V_e`, `V`, `n`, `V̇` | `2_luftmenge/scene_2.py` |
| `c_Luft`, winter/summer `Δθ`, `Δx`, `Q̇_V,sens`, `Q̇_V,lat` | `3_waermestrom_luft/scene_3.py` |
| `η_WRG`, duct `v_m`, `A`, `r` | `5_mechanische_lueftung/scene_5.py` |

Standards cited per beat via the `_din_ref()` chip: DIN EN 12831-1, DIN V 18599,
DIN 1946-4, DIN 1946-6, DIN EN 16798-1, DIN 4108-2, VDI 2078.

---

## Prerequisites

- Working directory: **semio repo root** (or any cwd — the script resolves paths)
- Repo `.venv` with Manim
- `ffmpeg` on PATH (needed for the reliable concat step)

```bash
source .venv/bin/activate          # macOS / Linux
.venv/bin/manim --version
ffmpeg -version
```

---

## How to run (recommended)

Render **each chapter separately**, then **ffmpeg-concat** (avoids Manim's mega-combine
bug on hundreds of partials / paths with spaces).

```bash
# low quality + open the finished video when done
.venv/bin/python tutorial/energy/demand/2_ventilation/full_ventilation_video.py -q l

# medium / high
.venv/bin/python tutorial/energy/demand/2_ventilation/full_ventilation_video.py -q m
.venv/bin/python tutorial/energy/demand/2_ventilation/full_ventilation_video.py -q h

# render only, do not open the player
.venv/bin/python tutorial/energy/demand/2_ventilation/full_ventilation_video.py -q l --no-play
```

VS Code / Cursor launch: **🛠️dev🎬manim ventilation full** (one beat preview:
**🛠️dev🎬manim ventilation**, intro card: **🛠️dev🎬manim intro ventilation**).

---

## Layout guard

Set `LAYOUT_CHECK=1` and any text that overlaps other text, or leaves the frame, is
printed with its coordinates at the end of every narration clause:

```bash
LAYOUT_CHECK=1 .venv/bin/manim -ql tutorial/energy/demand/2_ventilation/1_warum_lueften/scene_1.py Beat1_Lueftungsbedarf
```

A clean render prints nothing. See `5_energy_balance/README.md` for the shared layout
primitives in `tutorial/manim_visuals.py`.

---

## Where the video is written

Default media root is next to this file: `tutorial/energy/demand/2_ventilation/media/`

| Quality | Output |
|---------|--------|
| `-q l` | `media/videos/full_ventilation_video/480p15/FullVentilationVideo.mp4` |
| `-q m` | `media/videos/full_ventilation_video/720p30/FullVentilationVideo.mp4` |
| `-q h` | `media/videos/full_ventilation_video/1080p60/FullVentilationVideo.mp4` |

The deliverable is also copied to `rendered/Full_Ventilation_<quality>.mp4`.

---

## Chapter scenes and beats

| Scene class | Content |
|-------------|---------|
| `Ventilation_01_WarumLueften` | Kapitel 1 |
| `Ventilation_02_Luftmenge` | Kapitel 2 |
| `Ventilation_03_WaermestromLuft` | Kapitel 3 |
| `Ventilation_04_NatuerlicheLueftung` | Kapitel 4 |
| `Ventilation_05_MechanischeLueftung` | Kapitel 5 |
| `Ventilation_06_Uebergabe` | Kapitel 6 |

| Chapter | Beat | Idea |
|---------|------|------|
| 1 | `Beat1_Lueftungsbedarf` | CO₂, Feuchte, Schadstoffe — der hygienische Bedarf, ganzjährig |
| 1 | `Beat2_Lueftungsweg` | Luft durch die Öffnungen, Richtung kehrt mit der Jahreszeit um |
| 1 | `Beat3_Fahrplan` | Luftmenge → Wärmestrom → natürlich → mechanisch → Übergabe |
| 2 | `Beat1_Innenvolumen` | Brutto- zu Nettovolumen `V ≈ 0,76·V_e` |
| 2 | `Beat2_Luftwechselrate` | `n`, `V̇ = V·n`, Richtwerte von Lager bis OP |
| 2 | `Beat3_Volumenstrom` | ein Volumenstrom, zwei Schreibweisen (`V·n` = `q_v,R`) |
| 3 | `Beat1_Luftwaerme` | 1 m³ Luft, `c_Luft ≈ 0,34 Wh/(m³·K)` |
| 3 | `Beat2_SensibleWaerme` | `Q̇_V,sens = V̇·c_Luft·Δθ`, Winter und Sommer |
| 3 | `Beat3_LatenteLast` | `Q̇_V,lat = ρ·r·Δx·q_v,R` |
| 3 | `Beat4_WinterUndSommer` | eine Formel, zwei Jahreszeiten |
| 4 | `Beat1_Fensterregeln` | das Fenster als Regler |
| 4 | `Beat2_StossOderKipp` | 340 Wh gegen ≈ 780 Wh |
| 4 | `Beat3_Querlueftung` | zwei Öffnungen, `A_eff` |
| 4 | `Beat4_Auftrieb` | `Δp = h·g·(ρ_a − ρ_i)` |
| 4 | `Beat5_Nachtlueftung` | den Speicher entladen |
| 4 | `Beat6_GrenzenDerFreienLueftung` | Wetter, Filter, Regelung, keine Rückgewinnung |
| 5 | `Beat1_Grundtypen` | Abluft, Zuluft, Zu-/Abluft |
| 5 | `Beat2_Waermerueckgewinnung` | `η_WRG`, Winter 1500 → 300 W, Sommer Kälterückgewinnung |
| 5 | `Beat3_Anlagenvergleich` | Fenster, zentral, dezentral; Passivhaus |
| 5 | `Beat4_Kanalquerschnitt` | `V̇ → A → r` aus dem hygienischen Volumenstrom |
| 6 | `Beat1_Zusammenfassung` | die Größen `V, n, V̇, c_Luft, Δθ, Δx, η` als eine Kette |
| 6 | `Beat2_UebergabeHeizungKuehlung` | was Heizung und Kühlung übernehmen |

```bash
.venv/bin/manim -pql tutorial/energy/demand/2_ventilation/1_warum_lueften/scene_1.py Beat1_Lueftungsbedarf
```

---

## Notes

- Captions come from each beat's `NARRATION`.
- Synced German VO: run each chapter's `generate_audio.py` (synth), then re-render with
  `VO_TRACE=1` so `hold_for` uses measured clause lengths; align speech onto traced
  subtitle starts and mux — same pipeline as `4_cooling/README.md` describes. Renaming a
  beat invalidates that chapter's `vo_timing.json` / `vo_trace.json` keys, so re-run
  `--synth` and `--trace`.
- The house/sun/person/air glyphs, the `_din_ref()` citation chip and the
  `math_panel` formula style are the Physical Fundamentals vocabulary from
  `tutorial/manim_visuals.py`, shared with all other series.
- The ventilation beats were moved here from `3_heating/3_convection`,
  `4_cooling/3_transmission_humidity`, `4_cooling/5_systemauslegung` and
  `4_cooling/6_sommerstrategie`; Heating and Cooling link back to this video.
- Do not mix Heating/Cooling scenes into this playlist.
