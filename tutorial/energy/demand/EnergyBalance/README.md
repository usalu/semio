# EnergyBalance — Full Video

Plays the **EnergyBalance series** in curriculum order, prepended with the
series intro card ``Demo_Intro_Energiebilanz`` from ``tutorial/intro/intro_scene.py``:

0. Intro — Energiebilanz
1. Kapitel 1 — Die Energiebilanz des Gebäudes
2. Kapitel 2 — Regelwerk & Klimadaten
3. Kapitel 3 — Bedarf vs. Leistung
4. Kapitel 4 — Von der Nutzenergie zur Endenergie
5. Kapitel 5 — Primärenergie & das GEG
6. Kapitel 6 — Der Energieausweis
7. Kapitel 7 — Theorie vs. Praxis

Entry file: [`full_energy_balance_video.py`](./full_energy_balance_video.py)

This is the fourth video in the curriculum, after `1_physical_fundamentals/`,
`Heating/`, and `Cooling/`.

---

## The through-line

The series is one continuous calculation, not seven topics. Each chapter hands
the next one a number:

| Step | Quantity | Where it comes from |
|------|----------|---------------------|
| Bilanz | `Q_h = Q_Senke − η_h · Q_Quelle` | Kapitel 1, from the Heating/Cooling flows |
| Verfahren | 12 Monatsbilanzen, Klimadaten, `G_t` | Kapitel 2 |
| Leistung ≠ Energie | `Φ_HL [kW]` vs. `Q_h [kWh/a]` | Kapitel 3 |
| Anlagenkette | `Q_E = Q_h · e_p` | Kapitel 4 |
| Vorkette | `Q_P = Q_E · f_p` | Kapitel 5 |
| Ergebnis | Effizienzklasse auf dem Ausweis | Kapitel 6 |
| Realität | Bedarf ≠ Verbrauch | Kapitel 7 |

One worked example runs through Kapitel 4 to 6 unchanged, so the numbers on
screen stay the same building throughout:

```
Q_h  = 100 kWh/(m²·a)
  · e_p = 1,35 (Gaskessel)   → Q_E = 135  → · f_p = 1,1 → Q_P = 149  → Klasse E
  · e_p = 0,29 (Wärmepumpe)  → Q_E =  29  → · f_p = 1,8 → Q_P =  52  → Klasse A+
```

Standards cited per beat via the `_din_ref()` chip: GEG, DIN V 18599-1/-2/-5/-6/-10,
DIN EN 12831-1, DIN 4108-2, VDI 2078, VDI 3807, VDI 4645, VDI 4650, VOB/C DIN 18380.

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

Render **each chapter separately**, then **ffmpeg-concat**. This avoids Manim's
mega-combine bug (`InvalidDataError` on `partial_movie_file_list.txt` when
hundreds of partials / paths with spaces).

From the **semio repo root**:

```bash
# low quality + open the finished video when done
.venv/bin/python tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py -q l
```

Or from this folder with the venv already active (`(.venv)` in the prompt):

```bash
python full_energy_balance_video.py -q l
```

```bash
# medium / high
.venv/bin/python tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py -q m
.venv/bin/python tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py -q h

# render only, do not open the player
.venv/bin/python tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py -q l --no-play
```

VS Code / Cursor launch: **🛠️dev🎬manim energy balance full**

---

## Layout guard

Every beat's layout is checked automatically while it renders. Set
`LAYOUT_CHECK=1` and any text that overlaps other text, or leaves the frame,
is printed with its coordinates:

```bash
LAYOUT_CHECK=1 .venv/bin/manim -ql tutorial/energy/demand/EnergyBalance/1_recap_heizen_kuehlen/scene_1.py Beat1_Bilanzgrenze
```

```
[LAYOUT] Beat1_Bilanzgrenze·bedarf: overlap 'Heizwärme Q_h' @(-1.30,-1.68) × 'Q_Senke' @(1.06,-2.09) by 0.86×0.01 units
```

The check runs inside `hold_for()` — the settled state at the end of every
narration clause — so one render sweeps a whole beat. It lives in
`tutorial/manim_visuals.py` (`layout_conflicts` / `check_layout`) and applies to
every tutorial series, not just this one. A clean render prints nothing.

The layout primitives those beats are built from live in the same file:
`note_line`, `stat_card`, `card_grid`, `side_labels` (dodges callout labels
apart before drawing), `stacked_bar`, and `labeled_axes` (manual axes plus a
normalized-coordinate mapper — never `Axes(include_numbers=True)`, which would
pull in LaTeX).

---

## Where the video is written

Default media root is next to this file: `tutorial/energy/demand/EnergyBalance/media/`

| Quality | Output |
|---------|--------|
| `-q l` | `media/videos/full_energy_balance_video/480p15/FullEnergyBalanceVideo.mp4` |
| `-q m` | `media/videos/full_energy_balance_video/720p30/FullEnergyBalanceVideo.mp4` |
| `-q h` | `media/videos/full_energy_balance_video/1080p60/FullEnergyBalanceVideo.mp4` |

Open that `.mp4` in any player.

---

## Optional: single Manim scene (fragile)

```bash
.venv/bin/manim -pql tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py FullEnergyBalanceVideo
```

This one scene can render all animations and then **fail while combining**
partials. Prefer the Python script above.

Chapter scenes (for Sideview / debugging one chapter):

| Scene class | Content |
|-------------|---------|
| `EnergyBalance_01_Energiebilanz` | Kapitel 1 |
| `EnergyBalance_02_RegelwerkKlima` | Kapitel 2 |
| `EnergyBalance_03_BedarfVsLeistung` | Kapitel 3 |
| `EnergyBalance_04_NutzZuEndenergie` | Kapitel 4 |
| `EnergyBalance_05_Primaerenergie` | Kapitel 5 |
| `EnergyBalance_06_Energieausweis` | Kapitel 6 |
| `EnergyBalance_07_TheorieVsPraxis` | Kapitel 7 |

Example:

```bash
.venv/bin/manim -pql tutorial/energy/demand/EnergyBalance/full_energy_balance_video.py EnergyBalance_01_Energiebilanz
```

---

## Beats

Individual beats (for Sideview on one idea at a time) live in each numbered
chapter folder's `scene_N.py`.

| Chapter | Beat | Idea |
|---------|------|------|
| 1 | `Beat1_Bilanzgrenze` | Bilanzgrenze & Energieerhaltung |
| 1 | `Beat2_Verlustseite` | `Q_T + Q_V`, Wärmebrückenzuschlag |
| 1 | `Beat3_Gewinnseite` | `Q_S + Q_I`, Ausnutzungsgrad `η_h` |
| 1 | `Beat4_WinterUndSommer` | dieselbe Bilanz, umgekehrtes Vorzeichen |
| 2 | `Beat1_GEGundNormen` | GEG → DIN V 18599 → Nachbarnormen |
| 2 | `Beat2_Monatsbilanz` | warum kein Jahresmittel |
| 2 | `Beat3_Klimadaten` | `θ_e`, `I_S`, Testreferenzjahr |
| 2 | `Beat4_Gradtagszahl` | Heizgrenze, `G_t`, Witterungsbereinigung |
| 3 | `Beat1_LeistungUndEnergie` | Höhe vs. Fläche einer Leistungskurve |
| 3 | `Beat2_Normheizlast` | DIN EN 12831-1, Gewinne bewusst ignoriert |
| 3 | `Beat3_Jahresdauerlinie` | Spitze, Fläche, Vollbenutzungsstunden |
| 3 | `Beat4_RichtigAuslegen` | Unter- und Überdimensionierung, Takten |
| 4 | `Beat1_Verlustkette` | vier Stufen vom Raum zum Zähler |
| 4 | `Beat2_UebergabeVerteilungSpeicherung` | Vorlauftemperatur, Rohr- und Speicherverluste |
| 4 | `Beat3_Erzeugung` | Wirkungsgrad, Brennwert, Jahresarbeitszahl |
| 4 | `Beat4_Anlagenaufwandszahl` | `e_p = e_ü · e_d · e_s · e_g` |
| 5 | `Beat1_DreiBilanzgrenzen` | Raum / Zähler / Quelle |
| 5 | `Beat2_Primaerenergiefaktor` | `f_p` je Energieträger |
| 5 | `Beat3_Rechenbeispiel` | ein Gebäude, zwei Anlagen |
| 5 | `Beat4_Referenzgebaeude` | der Grenzwert ist das eigene Gebäude |
| 6 | `Beat1_BedarfVsVerbrauch` | berechnet vs. gemessen |
| 6 | `Beat2_DieSkala` | A+ … H, beide Beispielgebäude verortet |
| 6 | `Beat3_PflichtangabenUndFolgen` | Inserat, Effizienzklasse, Empfehlungen |
| 7 | `Beat1_BerechnungVsAbrechnung` | genormte Randbedingungen, Prebound/Rebound |
| 7 | `Beat2_HydraulischerAbgleich` | Durchfluss folgt Widerstand, nicht Bedarf |
| 7 | `Beat3_SerienAbschluss` | die vier Videos als eine Rechnung |

```bash
.venv/bin/manim -pql tutorial/energy/demand/EnergyBalance/1_recap_heizen_kuehlen/scene_1.py Beat1_Bilanzgrenze
```

---

## Notes

- Captions come from each beat's `NARRATION`.
- Synced German VO: run each chapter's `generate_audio.py` (synth), then
  re-render with `VO_TRACE=1` so `hold_for` uses measured clause lengths;
  align speech onto traced subtitle starts and mux — same pipeline as
  `Cooling/README.md` describes. Renaming a beat invalidates that chapter's
  `vo_timing.json` / `vo_trace.json` keys, so re-run `--synth` and `--trace`.
- The house/sun motifs, the `_din_ref()` citation chip, and the
  `equation_row` / `formula_panel` / `highlight_param` formula style match the
  Cooling series (`Cooling/3_transmission_humidity/scene_3.py`) for visual
  continuity across all four videos.
- Do not mix Heating/Cooling scenes into this playlist.
