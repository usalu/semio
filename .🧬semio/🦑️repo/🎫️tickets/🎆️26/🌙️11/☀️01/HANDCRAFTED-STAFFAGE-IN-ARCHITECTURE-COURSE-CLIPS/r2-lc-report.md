# r2-lc: bilingual on-screen labels near staffage

## INCIDENT first: summer-strategy clip was truncated and restored

- At about 20:11 local I truncated `E/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py` to 0 bytes. A probe in my own session opened it with mode `wb` while testing why a scripted write had failed with `EINVAL` (the known transient Windows write failure other agents also hit). My mistake.
- Restored the same session by replaying history, not by guessing:
  - base `git show HEAD:<file>` (607 lines, no staffage);
  - every recorded `Edit` on this file from the subagent transcripts (m4 occupant and blinds, hx tag, wheel timelapse, sc collector edits) and every recorded `sed` / python patch (m4 louvre swap, blind to `staffage_motion`, c1 `icon_glyph`, g2 wheel, exchanger, humidifier and heater cast patch, sc collector size, sun, ray and tag moves);
  - the whole-ticket passes that touch the file: hs `fix_style.py` comment strip (10 lines, same count as hs reported), hc ruff fixes (unused `sys as _sys`, unused `components`), xc cast merge (`cooling-summer-strategy-occupant` is now `shared-person-fanning`).
  - Verification: every line number I and the ct audit had recorded before the loss matches (299 heading, 310 `Sorptionsrad`, 314, 322, 326 heater, 338 `Solarkollektor`, 340, 342, 356 to 359); `ast` parses; ruff F401/F841/F821 clean; staffage_only, cooling, integration `test_summer_occupant_follows_house_scale` pass; Beat2 rendered and frames read (all five devices, flow line, tags correct).
  - Residual risk: an edit by another agent that used neither the Edit tool nor a command naming `summer-strategy` would be missing. `git diff HEAD` for the file is 64 insertions and 69 deletions. Whoever owns this clip (sc, g2) should skim `git diff` once. Scripts: `T/🧰️lc/🔎️history*.py`, `🔁️replay.py`, `🔁️finish.py`, `🔁️install.py`.
- Lesson recorded for the fleet: never probe a file with `open(..., "wb")`. Scripts should write a temp file and `os.replace` (as hs and my patch script do).

## Mechanism

The energy clips had no label-language mechanism: `set_vo_language("de")` only fixes subtitle timing, and the `(key, en, de)` clauses only drive narration. So I ported the BIM one 1:1 (same env var, same explicit-no-default rule) into V next to the watt anchors:

- `V/🐍️.py` `language()` reads `SEMIO_TEACHING_LANGUAGE` (en or de, raises otherwise), `tr(en, de)`, `label_number(value, digits)` (decimal point or comma by language, thin-space thousands), `unit_text(value, unit)` (narrow no-break space `U+202F` before the unit), constant `NARROW_SPACE`.
- `watt_anchor` is now bilingual: `_WATT_DEVICES` stores `(en, de)` names; value, compare line and ratio decimal separator follow the language; unit spacing uses `unit_text`.
- The render runtime already exports `SEMIO_TEACHING_LANGUAGE` (`runtime/🐍️.py:518`), so a normal render needs nothing new. Scripts and tests must set it (`🧪️tests/🐍️scenes.py` now does, from `🔣️scenes.json` `language`).
- Subtitles stay German by design (`set_vo_language("de")`, unchanged). An en render therefore shows English labels with German captions until the series is localized.

## Labels made bilingual (this ticket's added or changed labels near staffage)

| Clip | Labels |
|---|---|
| ❄️cooling/🌤️summer-strategy | Solarkollektor, Solarwärme → Heizregister |
| ❄️cooling/👥️internal-gains | Körperwärme, Gesamtwärme (anchor titles), Einzelperson (120 W) with narrow space |
| ❄️cooling/☀️solar-radiation | Bestrahlungsstärke je m² (anchor title) |
| ❄️cooling/⚖️heating-comparison | anchor title Gerät (Device). "Person" is identical in both languages and left as is |
| 🌬️ventilation/⚙️mechanical | WRG Zentral (Central HRV), 70 s Abluft, (Speichern), 70 s Zuluft, (16 °C) with narrow space |
| 🌬️ventilation/❓️need | Zuluft, Abluft tags |
| 🔥️heating/☀️solar-gains | Raffstore (Venetian blind) |
| 🔥️heating/👥️internal-gains | Personen, Geräte, Licht; anchor title eine sitzende Person; the live readouts (see below) |
| 🔥️heating/🧮️calculation | Speicher, Wärme-/erzeuger, unbeheizter/Keller, beheizt + 20 °C (narrow space), 30 m Rohr |
| 🔥️heating/🧱️conduction | Mensch in Ruhe, Teelicht (device reference labels, both the intro row and the later kinds list); height label number format |
| 🧲️physics/🧲️fundamentals | Teelicht, Staubsauger (power and energy ladders, with `0,15` / `0.15`), Licht/Wärme lane readouts, Licht → Wärme, 60 W Wärme, Heizwasser 35 °C |

Unchanged on purpose: "Toaster", "Laptop", "Thermostat", "Person" (same word in en and de; one term, no `tr`).

### Laptop versus Gerät

`🔥️heating/👥️internal-gains` Beat3 showed `Laptop 60 W` and `Gerät 90 W`. The second readout is the server rack (`PHI_E_DEVICE_W`), not a generic device, and the narration says "laptop and rack equipment". Both readouts now name the object, one term per language: `Laptop` and `Server` (identical in en and de). `Geräte` stays the category label (Devices) in Beat1 and the Beat3 heading.

### Spacing before units

The clips use `\,` in typeset math (already correct everywhere I touched) and a plain space in plain `Text`. For touched plain-text labels I use `unit_text` (U+202F). Rendered frames show it correctly in the project font (no tofu). Subtitle and narration strings keep plain spaces: they are prose.

## Tests and renders

- New language-agnostic test `V/🧪️tests/🐍️labels.py` + `🔣️labels.json`: `tr` per language, no default language (unset, empty, `fr`, `EN` all raise), numbers per language checked against numpy's own positional float formatting (third-party), narrow space, watt device names authored in both languages. 5 tests OK.
- `🧪️tests/🐍️scenes.py`: the two `_plant_house()` tests now run under an explicit language from the JSON case file (they failed with the new strict `language()` otherwise).
- Results: staffage_only 22 OK, visuals `🧪️tests/🐍️` OK, scenes 12 OK, cooling clip tests OK, cooling internal-gains tests OK, ventilation architecture OK, ventilation volume 2 OK, physics fundamentals OK, authoring clip tests OK, integration `test_summer_occupant_follows_house_scale` OK. The first run had transient failures from other agents' in-flight edits (`contextlib` import in `🧍️staffage/🐍️.py`); a re-run was green. The full `🐍️integration` module takes about 15 minutes and had only the summer cast error caused by the unrestored file, now fixed.
- Renders (ql, 15 fps, one scene at a time), frames read from `T/🗑️generated/lc/` before deletion:
  - `🔥️heating/👥️internal-gains` Beat1 in de: Geräte, Personen, Licht; in en: Devices, People, Light, nothing overlaps, desk label sits left of the desk as before.
  - Same clip Beat3 in de and en: `Laptop 60 W`, `Server 90 W`, `Φe = 150 W`, narrow space renders.
  - `❄️cooling/🌤️summer-strategy` Beat2 in en (also the restoration check): `Solar collector`, `Solar heat → heating coil`; neighbours stay German (see follow-up).

## Follow-up: pre-existing German-only labels (outside this ticket's added or changed lines)

Grouped by clip. Counts are `Text/_label/body_text/chip/section_heading` plus `\text{}` literals with German words.

- ❄️cooling/🌤️summer-strategy (24 + 2): Sorptionsgestützte Kühlung (DEC) heading, Sorptionsrad, Plattentauscher, Verdunstungskühlung, Außenluft, Zuluft, Abluft 26 °C (plain space), Fortluft feucht · warm, `\text{feucht}` / `\text{trocken}` readouts, heating coil has no label, question text near the end.
- ❄️cooling/⚖️heating-comparison (14): whole clip, German-only title; Lüftungsgerät mit Kühlregister (the cooler is a ventilation unit, the catalog asset is a split indoor unit), Außenluft/Zuluft/Abluft/Fortluft, Raumtemperatur, Interne Gewinne, "Person" anchor.
- ❄️cooling/👥️internal-gains (17 + 10): activity names (Schlafen, Büroarbeit, Gehen, Hochleistungssport), Interne Gewinne, remaining `\text{}` readouts.
- ❄️cooling/☀️solar-radiation (6), 💧️transmission-humidity (8), 📐️system-sizing (15 + 1).
- 🌬️ventilation/⚙️mechanical (7 + 4): legends Kaltluft (0 °C), Warmluft (20 °C), vorgewärmte Zuluft (16 °C) (plain-space °C), Fensterlüftung, freie Lüftung ohne WRG, panel title Zentrale WRG and zentrales Lüftungsgerät.
- 🌬️ventilation/❓️need (8 + 5): Luftmenge, Wärmestrom, mechanisch, natürlich, Übergabe, gilt im Winter wie im Sommer.
- 🌬️ventilation/🪟️natural (33 + 10), 🌡️heat-flow (17 + 1), 📦️volume (3 + 12), 🤝️handoff (4 + 5).
- 🔥️heating/👥️internal-gains (8 + 2): section headings (Geräteabwärme, …), DIN labels, Φ readout words.
- 🔥️heating/☀️solar-gains (24 + 4): ohne Verschattung and the other labels beside the blind cast.
- 🔥️heating/🧮️calculation (20 + 3), 🧱️conduction (18 + 8, innen/außen legend), 🧲️fundamentals (31 + 0, e.g. `innen {THETA} °C` with plain space), 🌬️convection (3).
- 🧲️physics/🧲️fundamentals (64 + 47): the biggest. Wasserkocher, Dusche (Durchlauferhitzer), Staubsauger name, Start/An chips (Start is English, An is German), whole ladders (Einfamilienhaus, Auto, Windrad, Kraftwerksblock, ...), `P_{\mathrm{Wärme}}`.
- 📊️demand/🏷️certificate (23 + 12): Monate, witterungsbereinigt, genaue Kosten, Sanierungsaufwand; ♨️system-losses (17 + 7), ⚖️regulations (8 + 3), ⚡️demand-power (13 + 18), 🏭️primary-energy (14 + 2), 🔁️recap (18), 🧪️practice (16 + 6).
- BIM clips from the ct audit (`self.tr` missing, English-only): 🔌️exchange (finishes, imposed load, tributary cell), 🔗️consistency (`S = section`).
- All plain `Text` strings with `20 °C`-style units that I did not touch still use a plain space; convert them with `unit_text` when the clip is localized.

## Open

- The strict `language()` raises without `SEMIO_TEACHING_LANGUAGE`. Any new test or script that builds a scene with these labels must set it (as the runtime does).
- `tr` for en/de only; no manifest-driven label table. Fine while the clips author labels inline, as BIM does.
- The localization of subtitles and the remaining labels above is a separate ticket.
