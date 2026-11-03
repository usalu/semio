# Audit: clip text near staffage calls (ct, r2)

Scope: the 35 modified `🐍️.py` clip files under `🎓️teaching/🏛️architecture` that call `staffage`, `staffage_motion`, `fitted_staffage` or `pendant_lamp`. Read-only, no source edited. Scripts: `T/🧰️ct/🔎️dump.py`, `🔎️onscreen.py`, `🔎️states.py`.

Language mechanisms seen:
- BIM clips: `self.tr(en, de)` driven by `SEMIO_TEACHING_LANGUAGE`. Compliant.
- Energy clips: narration as `(key, en, de)` clauses, subtitles German (`subtitle_text`). On-screen labels are mostly German literals with no `tr`. The memory note "energy German subtitles by design" covers the subtitles, not the labels.

## Findings

| file:line | string | issue | suggested fix |
|---|---|---|---|
| ❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:59, 65 | whole clip (`set_vo_language("de")`, `TITLE_DE` only) | No EN path at all. The clip is German-only, so no `en`/`de` language switch exists. | Add `TITLE_EN` and route every label through one `tr`/language helper, as in BIM. |
| heating-comparison/🐍️.py:236 | `"Lüftungsgerät\nmit Kühlregister"` | Device label (cooler, catalog `split-ac-indoor-unit`: en "Split Air Conditioner Indoor Unit", de "Split-Klimagerät Innenteil"). The label names a ventilation unit, not a split AC, and the string is German-only. | Use the language switch. Either align with the catalog ("Split-Klimagerät Innenteil") or document why the cooler is a ventilation unit. |
| heating-comparison/🐍️.py:228, 230, 231, 232 | `"Außenluft"`, `"Zuluft"`, `"Abluft"`, `"Fortluft"` | Air-path labels next to the cooler. German-only. | Language switch: Outdoor air / Supply air / Exhaust air / Extract air. |
| heating-comparison/🐍️.py:182 | `"Raumtemperatur"` | Thermometer label. German-only. | Language switch: Room temperature. |
| ❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py:310 | `"Sorptionsrad"` | Matches catalog de `Sorptionsrad`. No en. Catalog en is "Desiccant Sorption Wheel". | Language switch: Sorption wheel / Sorptionsrad. |
| summer-strategy/🐍️.py:314 | `"Plattentauscher"` | Catalog `plate-heat-exchanger` de is "Kreuzstrom-Plattenwärmeübertrager", en "Cross-Flow Plate Heat Exchanger". The label is a shortened form with no reason given. Hard-coded German. | Language switch. Use "Plattentauscher" / "Plate exchanger" and say in a comment that it is a shortened label. |
| summer-strategy/🐍️.py:322 | `"Verdunstungskühlung"` | Process name, catalog asset `evaporative-humidifier` ("Verdunstungsbefeuchter"). Deliberately different (process vs device). German-only. | Language switch: Evaporative cooling / Verdunstungskühlung. Reason is fine. |
| summer-strategy/🐍️.py:326 | (no label) `cooling-summer-strategy-heater` = `finned-heat-exchanger` | Device has no label. Acceptable, but the heater is unnamed while the other four are. | Optional: add a label "Heizregister" / "Heating coil" for consistency. |
| summer-strategy/🐍️.py:338 | `"Solarkollektor"` | Catalog `solar-thermal-collector` de "Flachkollektor (Solarthermie)". Short name, German-only. | Language switch: Solar collector / Solarkollektor; or use the catalog title. |
| summer-strategy/🐍️.py:340 | `"Solarwärme → Heizregister"` | Arrow label, German-only. | Language switch: Solar heat → heating coil. |
| summer-strategy/🐍️.py:356, 357, 359 | `"Außenluft"`, `"Zuluft"`, `"Fortluft\nfeucht · warm"` | German-only air-path labels. | Language switch. |
| summer-strategy/🐍️.py:358 | `"Abluft 26 °C"` | German-only, and the number unit uses a plain space. The clip convention for units is a thin space (`\,\mathrm{°C}` in math). | Language switch. Use `"Abluft 26 °C"` (narrow no-break space) or build it with `de_num`. |
| summer-strategy/🐍️.py:299 | `"Sorptionsgestützte Kühlung (DEC)"` | Section heading, German-only. | Language switch. |
| mechanical/🐍️.py:799 | `"WRG Zentral"` | Device label for the central unit (catalog `mvhr-unit`: "Wandmontiertes Lüftungsgerät mit Wärmerückgewinnung"). Abbreviation, German-only, not in the catalog. | Language switch: "Central HRV" / "Zentrale WRG". Reason for abbreviation needed. |
| mechanical/🐍️.py:782 | `"Fensterlüftung"`, `"freie Lüftung ohne WRG"` | German-only labels near the occupant and window. | Language switch. |
| mechanical/🐍️.py:789, 790 | `["Kaltluft", "(0 °C)"]`, `["Warmluft", "(20 °C)"]` | `legend()` (L744) wraps `Text` literals with no language choice. Unit plain space. | Pass `(en, de)` pairs through `tr` in `legend`. Thin space before °C. |
| mechanical/🐍️.py:808, 822 | `["vorgewärmte", "Zuluft (16 °C)"]`, `["70 s Zuluft", "(16 °C)"]` | Same `legend` issue. Plain space before °C. | As above. |
| mechanical/🐍️.py:796 | `"Zentrale WRG"`, `"zentrales Lüftungsgerät"` | Device label German-only. Same as L799. | Language switch. |
| 🌬️ventilation/🎬️clip/❓️need/🐍️.py:614 | `"Luftmenge"`, `"Wärmestrom"`, `"mechanisch"`, `"natürlich"`, `"Übergabe"` | German-only. "mechanisch" describes the mechanical ventilation device. | Language switch. |
| need/🐍️.py:661 | `"gilt im Winter wie im Sommer"` | German-only text beside the staffage call at L631 (`shared-mvhr-normal`). Could not confirm it is on screen. | Confirm it is displayed. If so, language switch. |
| 🔥️heating/🎬️clip/👥️internal-gains/🐍️.py:259, 260, 261 | `"Personen"`, `"Geräte"`, `"Licht"` | Device and source labels next to the staffage (occupant, desk, lamp). German-only. | Language switch: People / Devices / Light. |
| internal-gains/🐍️.py:474 | `rf"\text{{Laptop}}\;{de_num(...)}\,\mathrm{{W}}"` | Mixes English "Laptop" with German "Gerät" (L479) on the same live readout. The power value formatting is correct (`de_num` plus `\,\mathrm{W}`). | Build both lines through the language helper: "Laptop" / "Notebook" in de. Keep `de_num` and `\,\mathrm{W}`. |
| internal-gains/🐍️.py:479 | `rf"\text{{Gerät}}\;{de_num(...)}\,\mathrm{{W}}"` | German-only device label. Same formatting as L474. | Language switch: "Device" / "Gerät". |
| internal-gains/🐍️.py:480 | `"Einzelperson (120 W)"` | German-only. Plain space before "W" (no thin space). | Language switch: "Single person (120 W)" / "Einzelperson (120 W)". Use `\,` or a thin space. |
| 🔥️heating/🎬️clip/☀️solar-gains/🐍️.py:989 | `"Raffstore"` | Device label (catalog `external-venetian-blind`), German-only. | Language switch: Venetian blind / Raffstore. |
| solar-gains/🐍️.py:940 | `"ohne Verschattung"` | German-only label beside the thermal blind cast. | Language switch: without shading / ohne Verschattung. |
| 🧲️fundamentals/🐍️.py:166 | `f'innen {THETA_I} °C'` | German-only, plain space before °C. Near staffage L122/127 (not a device). | Language switch: inside / innen; thin space before °C. |
| 🏗️bim/🎬️clip/🔌️exchange/🐍️.py:883, 884 | `"finishes (layer thickness not modelled)"`, `"imposed load: people and furniture"` | English-only labels, no `tr`. Near staffage L876. "people and furniture" is a staffage-related term. | `self.tr(en, de)`: Oberflächen (Schichtdicke nicht modelliert) / Nutzlast: Personen und Mobiliar. |
| exchange/🐍️.py:906 | `f'{TAG}: tributary cell as in the plan'` | English-only. | `self.tr` pair. |
| 🏗️bim/🎬️clip/🔗️consistency/🐍️.py:291 | `"S = section"` | English-only. Near staffage L276. | `self.tr` pair. |
| 🏭️/certificate (📊️demand/🎬️clip/🏷️certificate/🐍️.py):868, 869 | `"genaue Kosten"`, `"Sanierungsaufwand"` | German-only chips near staffage L846/847. Not devices. | Low priority; language switch. |

## Checked and fine

- BIM `self.tr` labels: Villa Savoye title (`information` L208), `quantity` / `number` formatting (`exchange` L843–849, 881). Compliant.
- Math readouts with `de_num` plus `\,\mathrm{W}`, `\,\mathrm{kWh}`, `\,\mathrm{°C}` (heat-flow L306–342, primary-energy L1115–1152, mechanical L753–754, certificate L821). Decimal comma and thin-space convention followed.
- Narration pairs (German subtitles): wording is grammatically correct, umlauts are real. Examples: system-sizing L202, 208, 359; summer-strategy L281–293; mechanical L257, 426–444; heat-flow L269–275, 438–439, 566–567.
- Device names in catalog: the cast-to-asset mapping for all 35 files resolved; titles listed in `🗑️generated/ct/dump.txt` (deleted at the end).

## Not verified

- No render or runtime check: which labels are on screen in which beat is inferred from code, not from frames.
- `legend` (mechanical L744) and `_live` (internal-gains) helpers were read, not traced end to end.
- Narration text in the EN clause field was not compared word by word against the DE field.
