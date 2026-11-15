# 🔎 Notation audit of the energy course clips

Read-only audit for FEEDBACK items F00, F10, F11, F22, F01/F03/F12, F06, F28 (global consequences) across the energy course.

## Scope and method

- Root: `🎓️teaching/🏛️architecture/⚡️energy/` (short: `E/`). All paths below are relative to `E/`.
- Scope: every `🐍️.py` under `{🧲️physics,🌬️ventilation,🔥️heating,❄️cooling,📊️demand}/🎬️clip/**` (including `🔊️speech`, `🧪️tests`, `🏗️composition`), `🏗️authoring/🎨️visuals/🐍️.py`, `🏗️authoring/🎨️theme/🐍️.py`, `🏗️authoring/📏️running-example/` (`🐍️.py`, `🔣️.json`), `*/❓️quiz/🔣️.json`, and the manifest `🎬️clip/🔣️.json`.
- Method: exhaustive ripgrep passes over the scope for each notation family (units in brackets, unit-less factors, spelled-out symbols, citation helpers, symbol families), then targeted reads of the lines that carry the findings. Line numbers are from ripgrep and refer to the files as they are on disk now.
- Not verified: no web access was used, so no citation was checked against the source. Statements about edition or content of Bohne 2022 / Willems 2024 are open questions, not findings.
- Out of scope but noticed: `🏗️authoring/🎬️clip/**` framework code, BIM / Grasshopper / Excel / optimization manifest entries.

Key facts that drive the fixes:

1. **Displayed text and spoken text are the same string.** `🏗️authoring/🎨️visuals/🐍️.py` defines `Clause = (section, text_en, text_de)`. `subtitle_text()` returns `tr(text_en, text_de)` for the caption. `🏗️authoring/🔊️speech/🐍️.py:88-98` (`subtitle_clauses`, `subtitle_narration_text`) and `synthesize_clause_audio` (l.265-304) feed `text_de` directly to the TTS backend. There is no separate spoken form and no pronunciation normalizer. A caption "Δθ" would therefore also be sent to the TTS verbatim. A split is needed (see section 3).
2. **Citations** are rendered by `_din_ref(text)` (one copy per video file, top-right corner, `LABEL_FONT_SIZE - 3`, opacity 0.72, displays the argument verbatim) plus ad-hoc `Text("Willems et al. 2024", ...)` and a bibliography card in `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` Beat8 (l.3160-3179).
3. **Units** are attached to symbols via the idiom `(None, r"\;[\mathrm{W}]", COLOR)` inside `math_panel` tuples and via `math_label(r"X\;[\mathrm{unit}]")` / `_axes(..., r"X\;[\mathrm{unit}]")` for labels. This is the bracket-unit form that F00 rejects.

---

## 1. Bracketed units used as labels (F00, F10, F22)

Scene names are from the class declarations. Form column: `tuple` = `(None, r"\;[\mathrm{..}]")` term in a formula panel; `label` = `math_label`/`Text`; `axis` = `_axes` label; `table` = header or list literal.

Proposed fix, applied per row: remove the `\;[unit]` suffix from the symbol. Where the unit is needed, use one of the approved forms: `Q in W` (labels, axes), `[Q] = W` (headers, legends), or `Q = 120 W` (values). Equations keep the unit on the result only (see section 2).

### 1.1 `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (scenes Beat1 l.563-895, Beat6 l.2308-2670, Beat7 l.2681-3086)

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 630 | Beat1_Leistung | label | `_m(r"P\;[\mathrm{W}]")` | `P in W` |
| 650 | Beat1_Leistung | axis | `_axes(O, 5.1, 2.95, r"t\;[\mathrm{h}]", r"P\;[\mathrm{kW}]")` | axis `t in h`, `P in kW` |
| 761 | Beat1_Leistung | axis | `_m(r"t\;[\mathrm{s}]")` | `t in s` |
| 762 | Beat1_Leistung | axis | `_m(r"P\;[\mathrm{W}]")` | `P in W` |
| 845 | Beat1_Leistung | axis | `_axes(YO, 10.7, 3.0, "t", r"P\;[\mathrm{kW}]")` | `P in kW` |
| 2467 | Beat6_ThermischeMasse | axis | `_axes(CO, 7.15, 3.05, r"t\;[\mathrm{h}]", r"\vartheta\;[\mathrm{°C}]")` | `t in h`, `ϑ in °C` |
| 2742 | Beat7_SensibelLatent | axis | `_axes(QO, 7.4, 3.15, r"Q\;[\mathrm{kJ}]", r"T\;[\mathrm{°C}]")` | `Q in kJ`, `T in °C` |
| 2828 | Beat7_SensibelLatent | axis | `_axes(AO, 6.2, 3.25, r"\vartheta\;[\mathrm{°C}]", r"x\;[\mathrm{g/kg}]")` | `ϑ in °C`, `x in g/kg` |

### 1.2 `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` (Beat1..Beat9)

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 979 | Beat5_Zusammenfassung | tuple | `(None, r"\;[\mathrm{W}]", P_PRIMARY)` | drop; unit on result |
| 1270 | Beat7_Waermedurchlasswiderstand | tuple | `(None, r"\;[\mathrm{m^{2}K/W}]")` | drop; unit on result |
| 1463 | Beat8_UWert | tuple | `(None, r"\;[\mathrm{m^{2}K/W}]")` | drop; unit on result |
| 1468 | Beat8_UWert | tuple | `(None, r"\;[\mathrm{W/(m^{2}K)}]")` | drop; unit on result |
| 1648 | Beat9_WaermestromFormel | tuple | `(None, r"\;[\mathrm{W}]")` | drop; unit on result |

### 1.3 `🔥️heating/🎬️clip/🧮️calculation/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 487 | Scene2 | tr-string | `H = \Phi / \Delta\theta\;[\mathrm{W/K}]` (EN) | `H = Φ / Δθ` ... `H in W/K` in prose |
| 489 | Scene2 | tr-string | same, DE | same |
| 529 | Scene2 | label | `\theta_i - \theta_e\;[\mathrm{K}]` | `Δθ in K` |
| 843 | Scene4 | label | `t\;[\mathrm{h}]` | `t in h` |
| 845 | Scene4 | label | `P\;[\mathrm{kW}]` | `P in kW` |

### 1.4 `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 574 | Beat2_BestrahlungUndFlaeche | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 713 | Beat3_GWert | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 1066 | Beat5_Verschattung | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 1118 | Beat8_Hauptgleichung | list | `unit_src = [r"[\mathrm{W/m^{2}}]", r"[\mathrm{m^{2}}]", "[–]", "[–]", "[–]"]` | table header form `[I_S] = W/m²` etc. (F00) |
| 1139 | Beat8_Hauptgleichung | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 1214 | Beat8_Hauptgleichung | tuple | `(None, r"\;[\mathrm{J}]")` | drop |
| 1219 | Beat8_Hauptgleichung | math_text | `m\;\;\text{mass of the component}\;[\mathrm{kg}]` (EN) | `m = mass of the component, kg` form (F00 "[m] = kg") |
| 1220 | Beat8_Hauptgleichung | math_text | `c ... \;[\mathrm{J/(kg\,K)}]` (EN/DE) | `c in J/(kg·K)` |
| 1222 | Beat8_Hauptgleichung | math_text | `\Delta T ... \;[\mathrm{K}]` (EN/DE) | `ΔT = temperature rise, K` |

### 1.5 `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 168 | docstring of panel helper | docstring | ``Φ_p + Φ_e + … [W]`` | docstring only; fix text if kept |
| 175 | helper `parts.append` | tuple | `(None, r"\;[\mathrm{W}]", PASTEL_WHITE)` | drop |
| 753 | Beat5_SummeUndDichte | tuple | `(None, r"\;[\mathrm{W/m^{2}}]")` | drop |

### 1.6 `🔥️heating/🎬️clip/🧱️conduction/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 601 | Beat2_RWert | tuple | `(None, r"\;[\mathrm{m^{2}\,K/W}]")` | drop |
| 697 | Beat3_UWertUndGradient | tuple | `(None, r"\;[\mathrm{W/(m^{2}\,K)}]")` | drop |
| 770 | Beat3_UWertUndGradient | tuple | `(None, r"\;[\mathrm{W/K}]")` | drop |
| 824 | Beat3_UWertUndGradient | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 1076 | Beat5_Gebaeudehuelle | tuple | `(None, r"\;[\mathrm{W}]")` | drop |
| 1088 | Beat5_Gebaeudehuelle | label | `A_{i}\;[\mathrm{m^{2}}]` | `A_i in m²` |
| 1090 | Beat5_Gebaeudehuelle | label | `\Delta\theta_{i}\;[\mathrm{K}]` | `Δθ_i in K` |
| 1091 | Beat5_Gebaeudehuelle | label | `\Phi_{i}\;[\mathrm{W}]` | `Φ_i in W` |
| 1093 | Beat5_Gebaeudehuelle | label | `[\mathrm{W/(m^{2}K)}]` (standalone) | `[U] = W/(m²K)` |
| 1272 | Beat6_Waermebruecken | label | `l\;[\mathrm{m}]` | `l in m` |
| 1273 | Beat6_Waermebruecken | label | `U \cdot l \cdot h\;[\mathrm{W/K}],\; h = 3\,\mathrm{m}` | `U·l·h in W/K` |
| 1418 | Beat6_Waermebruecken | tuple | `(None, r"\;[\mathrm{W/K}]")` | drop |
| 1459 | Beat6_Waermebruecken | tuple | `(None, r"\;[\mathrm{W/K}]")` | drop |

### 1.7 `🔥️heating/🎬️clip/🌬️convection/🐍️.py`

No bracket-unit form found. Units there are written as `\mathrm{Wh/(m^{3}K)}` in `unit` slots (l.226-228); those are table headers without brackets and are fine under F00.

### 1.8 `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 869 | Beat3_Querlueftung | tuple | `(None, r"\;[\mathrm{m^{2}}]", P_PRIMARY)` | drop |
| 1084 | Beat4_Auftrieb | tuple | `(None, r"\;[\mathrm{Pa}]", P_PRIMARY)` | drop |

### 1.9 `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 378 | Beat1_Luftwaerme | tuple | `("v", r"\dot{V}\,[\mathrm{m^{3}/h}]", AIR)` | `V̇ in m³/h` |
| 379 | Beat1_Luftwaerme | tuple | `c_{\mathrm{Luft}}\,[\mathrm{Wh/(m^{3}\,K)}]` | `c_Luft in Wh/(m³·K)` |
| 382 | Beat1_Luftwaerme | tuple | `q_{v,\mathrm{R}}\,[\mathrm{m^{3}/s}]` | `q_v,R in m³/s` |
| 383 | Beat1_Luftwaerme | tuple | `\rho \cdot c_{p}\,[\mathrm{J/(m^{3}\,K)}]` | `ρ·c_p in J/(m³·K)` |
| 508 | Beat2_SensibleWaerme | tuple | `(None, r"\;[\mathrm{W}]", P_WHITE)` | drop |
| 666 | Beat3_LatenteLast | tuple | `(None, r"\;[\mathrm{W}]", P_WHITE)` | drop |
| 768 | Beat4_WinterUndSommer | tuple | `(None, r"\;[\mathrm{W}]", P_WHITE)` | drop |

### 1.10 `🌬️ventilation/🎬️clip/📦️volume/🐍️.py`

| line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|
| 344 | helper function (module level) | label | `n\;[\mathrm{h^{-1}}]` | `n in 1/h` |
| 817 | Beat2_Luftwechselrate | tuple | `("vd", r"\dot{V}"), (None, r"\;[\mathrm{m^{3}/h}]", None)` | drop |
| 942 | Beat3_Volumenstrom | label | `n\;[\mathrm{h^{-1}}]` | `n in 1/h` |
| 943 | Beat3_Volumenstrom | label | `V\;[\mathrm{m^{3}}]` | `V in m³` |

### 1.11 `🌬️ventilation/🎬️clip/❓️need/🐍️.py`, `🤝️handoff/🐍️.py`, `⚙️mechanical/🐍️.py`

No bracket-unit form found in these three files.

### 1.12 `❄️cooling/🎬️clip/*/🐍️.py`

| file | line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|---|
| `☀️solar-radiation` | 328 | Beat1_SolarIrradiance | tuple | `(None, r"\;[\mathrm{W/m^{2}}]", P_PRIMARY)` | drop |
| `☀️solar-radiation` | 442 | Beat5_SolarCoolingLoad | tuple | `(None, r"\;[\mathrm{W}]", P_PRIMARY)` | drop |
| `📐️system-sizing` | 294 | Beat2_VolumeFlowEquation | tuple | `(None, r"\;[\mathrm{W}]", P_PRIMARY)` | drop |
| `📐️system-sizing` | 312 | Beat2_VolumeFlowEquation | label | `q_{v,R}\;[\mathrm{m^{3}/s}]` | `q_v,R in m³/s` |
| `📐️system-sizing` | 431 | Beat3_IsolateAirflow | tuple | `(None, r"\;[\mathrm{W}]", P_PRIMARY)` | drop |
| `📐️system-sizing` | 441 | Beat3_IsolateAirflow | tuple | `(None, r"\;[\mathrm{m^{3}/s}]", P_PRIMARY)` | drop |
| `💧️transmission-humidity` | 292 | Beat1_TransmissionOpaque | tuple | `(None, r"\;[\mathrm{W}]", P_WHITE)` | drop |
| `💧️transmission-humidity` | 489 | Beat3_LueftungslastImSommer | tuple | `(None, r"\;[\mathrm{W}]", P_WHITE)` | drop |
| `👥️internal-gains` | 437 | Beat2_HumanFactor | tuple | `\dot{q}_{p}` then `\;[\mathrm{W}]` | drop |
| `👥️internal-gains` | 627 | Beat3_DevicesLighting | tuple | `f_{N}` then `\;[\mathrm{W}]` | drop |
| `👥️internal-gains` | 721 | Beat3_DevicesLighting | tuple | `f_{g}` then `\;[\mathrm{W}]` | drop |
| `👥️internal-gains` | 847 | Beat4_CumulativeLoad | tuple | `\dot{Q}_{\text{Licht}}` then `\;[\mathrm{W}]` | drop |

### 1.13 `📊️demand/🎬️clip/*/🐍️.py`

| file | line | scene | form | current snippet | proposed fix |
|---|---|---|---|---|---|
| `⚖️regulations` | 581 | Beat1_GEGundNormen | standalone label | `math_label(r"[\mathrm{kW}]")` | `[P] = kW` |
| `⚖️regulations` | 965 | Beat3_Klimadaten | axis | `\theta_e\,[\mathrm{°C}]` | `θ_e in °C` |
| `⚖️regulations` | 966 | Beat3_Klimadaten | axis (right) | `I_S\,[\mathrm{kWh/m^2}]` | `I_S in kWh/m²` |
| `⚖️regulations` | 1153 | Beat4_Gradtagszahl | axis | `\theta_e\,[\mathrm{°C}]` | `θ_e in °C` |
| `⚡️demand-power` | 144 | Beat3_Jahresdauerlinie | axis default | `_wide_chart(y_src = r"P\;[\mathrm{kW}]")` | `P in kW` |
| `⚡️demand-power` | 453 | Beat1_LeistungUndEnergie | label | `Q\;[\mathrm{kWh}]` | `Q in kWh` |
| `⚡️demand-power` | 454 | Beat1_LeistungUndEnergie | card | `P\;[\mathrm{kW}]` | `P in kW` |
| `⚡️demand-power` | 455 | Beat1_LeistungUndEnergie | card | `Q\;[\mathrm{kWh}]` | `Q in kWh` |
| `⚡️demand-power` | 823 | Beat2_Normheizlast | tr | `\text{Area} = Q_h\;[\mathrm{kWh/a}]` | `Q_h = … kWh/a` |
| `⚡️demand-power` | 916 | Beat2_Normheizlast | label | `Q_{h}\;[\mathrm{kWh}]` | `Q_h in kWh` |
| `⚡️demand-power` | 1214 | Beat4_RichtigAuslegen | label | `Q_{h}\;[\mathrm{kWh}]` | `Q_h in kWh` |
| `🧪️practice` | 1211 | Beat3_SerienAbschluss | tuple | `\;[\mathrm{kWh/(m^{2}\cdot a)}]` | drop |
| `🔁️recap` | 531 | Beat1_Bilanzgrenze | tuple | `Q_{\mathrm{Senke}}` + `\;[\mathrm{kWh/a}]` | drop |
| `🔁️recap` | 671 | Beat2_Verlustseite | tuple | `t` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 782 | Beat2_Verlustseite | tuple | `t` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 811 | Beat2_Verlustseite | tuple | `t` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 859 | Beat3_Gewinnseite | tuple | `Q_{V}` + `\;[\mathrm{kWh/a}]` | drop |
| `🔁️recap` | 1005 | Beat3_Gewinnseite | tuple | `F_{S}` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 1066 | Beat3_Gewinnseite | tuple | `t` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 1160 | Beat4_WinterUndSommer | tuple | `Q_{\mathrm{Quelle}}` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 1311 | Beat4_WinterUndSommer | tuple | `Q_{\mathrm{Quelle}}` + `\;[\mathrm{kWh}]` | drop |
| `🔁️recap` | 1317 | Beat4_WinterUndSommer | tuple | `Q_{\mathrm{Senke}}` + `\;[\mathrm{kWh}]` | drop |

`🏭️primary-energy`, `♨️system-losses`, `🏷️certificate`: no bracket-unit form found.

### 1.14 Authoring (in scope)

`🏗️authoring/🎨️theme/🐍️.py`, `🏗️authoring/🎨️visuals/🐍️.py`, `🏗️authoring/📏️running-example/🐍️.py`: no bracket-unit form found. `🔣️.json` quiz files: no bracket-unit form found (the quiz uses `W/m²`, `kWh/(m²·a)` in prose).

---

## 2. Equations with numbers and missing units (F10, F22)

Every numeric substitution line found by a regex over `\cdot`, `\times`, `=` with digits, and decimal-comma numbers. "Factors" are the numbers before `=`/`\approx`. Units column: `each` = every factor has a unit, `none` = no factor has a unit, `partial` = some do.

### 2.1 On-screen formulas (`math_readout` / `_readout` / `top_read` / `math_panel` tuples)

| file | line | scene | line content (abridged) | factor units | result unit | fix |
|---|---|---|---|---|---|---|
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 341-345 | Beat1_Luftwaerme | `Q = {RHO_AIR} · {CP_AIR} · {Δθ} = {…} J` then `= {…} Wh` (divided by 3600) | none | J (then Wh) | `ρ = 1.2 kg/m³ · c_p = 1005 J/(kg·K) · Δθ = 20 K = … J` |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 543-545 | Beat1_Innenvolumen | `V_e = {A_e} · {depth} = {…} m³` | none | m³ | `A_e = … m² · depth = … m` |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 580-582 | Beat1_Innenvolumen | `d_i = {depth} − 2 · {wall} = {…} m` | none | m | `… m − 2 · … m = … m` |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 616-618 | Beat1_Innenvolumen | `V = {net area} · {d_i} = {…} m³` | none | m³ | `… m² · … m = … m³` |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 969-970 | Beat3_Volumenstrom | `{V_NET} · {N_RATE} = {V_DOT} m³/h` | none (`N_RATE` has no `1/h`) | m³/h | `… m³ · … 1/h = … m³/h` |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 1084-1085 | Beat3_Volumenstrom | `{PERSONS} · {PER_PERSON} m³/h = {…} m³/h` | partial (`PERSONS` none) | m³/h | `… Personen · … m³/h = … m³/h` |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 1353-1355 | Beat6_Waermebruecken | `U · 2d = {…} W/(m·K)` | none | W/(m·K) | `U = … W/(m²·K) · 2d = … m` |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 2429-2430 | Beat6_ThermischeMasse | `{n} m² · 0.133 kWh = {…} kWh` | each | kWh | ok |
| `📊️demand/🎬️clip/🧪️practice/🐍️.py` | 1225-1227 | Beat3_SerienAbschluss | `Q_E = {Q_h} · {e_p} = {…}`; `Q_P = {Q_P} · {f_p} = {…}` | none (dimensionless expected) | none | state factors dimensionless, or show `kWh` on both sides |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 1383-1385 | AnlagenVerluste | `Q_h,d = {m} m · {W/m} W/m · {h} h = {…} kWh/a` | each | kWh/a | ok |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 1386 | AnlagenVerluste | `Q_h,s = {1.8} kWh/d · 200 d = {…} kWh/a` | each | kWh/a | ok |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 1387 | ReviewingHeatLosses | `Q_h,g = {Q} · (1/{0.95} − 1) ≈ {…} kWh/a` | partial (Q unit missing) | kWh/a | `Q_before_G = … kWh/a · (1/0.95 − 1)` |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 253 | Beat2 | `25−18 = 7 K` | none | K | `25 °C − 18 °C = 7 K` |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 300-308 | Beat2 | `ρ_a = 1,2 kg/m³`, `c_p,a = 1,0 kJ/(kg·K)`, `Δθ = 7 K` | each | n/a | ok |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 590-591 | Beat2_StossOderKipp | `V = 4 m · {D} m · {H} m = {…}` | each | m³ | ok |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 754 | Beat2_StossOderKipp | `Δθ = 6 K` | each | K | ok |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 2372 | Beat6_ThermischeMasse | `m = ρ · V = 2 400 kg/m³ · 0.1 m³ = 240 kg` | each | kg | ok (note decimal point vs comma in the DE variant: `0{,}1`) |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 2370 | Beat6_ThermischeMasse | `V = 100 cm · 100 cm · 10 cm = 0.1 m³` | each | m³ | ok |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 2378-2380 | Beat6_ThermischeMasse | `Q = 240 kg · 1 000 J/(kg·K) · {Δθ} K` | each | J | ok |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 3007 | Beat7 | `ϑ_i = 20 °C, φ = 50 %` | each | n/a | ok |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 3041, 3046 | Beat7 | `x = 11.7 g/kg`, `x = 9.5 g/kg` | each | g/kg | ok |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 312 | Beat1_Luftwaerme | `m = ρ · 1 m³ = {…} kg` | partial (ρ no unit) | kg | `ρ = 1.2 kg/m³ · 1 m³ = …` |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 656, 674, 676 | Beat3_LatenteLast | `Δx = … g/kg`, `ρ · Δx = … g/m³` | ρ none | g/m³ | `ρ = 1.2 kg/m³` |
| `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | 539-544 | Beat2_Waermerueckgewinnung | `η = …` | dimensionless | none | ok |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 342-345 (same as above) | | | | | |
| `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` | 437-847 | Beat3/Beat4 | tuple terms `q̇_p`, `f_N`, `f_g`, `Q̇_Licht` each followed by `\;[\mathrm{W}]` | each (unit only on the term) | W | drop term unit, add unit to sum result |

### 2.2 Subtitles and narration with unit-less factors (display and spoken)

| file | line | German subtitle as displayed and spoken | factors | fix |
|---|---|---|---|---|
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 550 | "Beim Stoßlüften wird nur die Luft nachgeheizt: 0,34 · 50 · 20 = 340 Wh." | none | "0,34 Wh/(m³·K) · 50 m³ · 20 K = 340 Wh" |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 562 | "Für dieselben 50 m³ Frischluft: 340 Wh beim Stoßlüften" | ok | |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 440 | "Im Winter: 220 mal 0,34 mal 20 — rund 1500 Watt" | none | "220 m³/h · 0,34 Wh/(m³·K) · 20 K ≈ 1500 W" (unit units in the spoken form) |
| `🔥️heating/🎬️clip/🌬️convection/🐍️.py` | 153 | "220 Kubikmeter pro Stunde, 0,34 Wh pro Kubikmeter und Kelvin und im Winter 20 Kelvin — rund 1500 Watt." | units in words | ok (keep) |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 209 | "Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R." | symbols only | see section 3 |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 278 | docstring: "c_Luft = ρ·c_p/3600 = 1,2 · 1005 / 3600 ≈ 0,34 Wh/(m³·K)" | `1,2` and `1005` no units | fix docstring |

### 2.3 Factor lists in code that also become displayed text

Not separately listed; they are the `label_number(...)` arguments of rows in 2.1.

---

## 3. Spelled-out Greek and symbol names in displayed text

### 3.1 How the text pipeline works (answer to the question)

- Narration is authored as class attribute `NARRATION = [(section, text_en, text_de), …]` per Beat, e.g. `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:925` (Beat5).
- `🏗️authoring/🎨️visuals/🐍️.py`: `tr(en, de)` (l.242) picks the label by `SEMIO_TEACHING_LANGUAGE`. `subtitle_text(narration, key)` (l.2035) returns the caption for the active language. `narration_text(...)` (l.2022) returns the English text and is only used for the legacy English VO.
- `🏗️authoring/🔊️speech/🐍️.py`: `subtitle_clauses` (l.88) and `subtitle_narration_text` (l.93) take `text_de`. `synthesize_clause_audio` (l.265) sends `subtitle_clauses(narration)` to the backend. So the German subtitle is displayed and spoken from the same string.
- Conclusion: **no displayed-vs-spoken split exists.** Writing "Δθ" in a German subtitle would also be sent to the TTS model as "Δθ", with unknown pronunciation. Writing "Delta-Theta" (current) is pronounced literally and shown wrongly (F11). A split requires a second field per clause (for example `(section, text_en, text_de, speech_de)`) or a deterministic normalizer applied only in `🔊️speech` before `backend.synthesize` (map `Δθ→Delta Theta`, `Φ→Phi`, `Q̇→Q Punkt`, `η→eta`, `λ→Lambda`, `ρ→Rho`), while the caption shows the Greek letter. No pronunciation dictionary exists in the repo.

### 3.2 Narration tuples containing spelled-out symbols (displayed and spoken)

German string is the subtitle and the TTS input. EN string is the English variant. Spelled forms are the ones to fix (F11).

| file | line | spelled form in German | German string (abridged) | EN string spells too? |
|---|---|---|---|---|
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 216-217 | "Q-Punkt-T", "Delta-Theta-äquivalent" | "Die opake Transmissionslast Q-Punkt-T ist U mal A mal Delta-Theta-äquivalent — in Watt." | yes ("Delta-theta eq") |
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 225-226 | "Delta-Theta-eq", "Luft-Delta-Theta" | "Delta-Theta-eq ist die äquivalente Temperaturdifferenz nach Sol-Air … nicht das Luft-Delta-Theta." | yes |
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 374 | "Q-Punkt-L" | "Zusammen ergibt sich Q-Punkt-L von rund 1260 W" | no |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 209 | "Q Punkt V", "Rho a", "c p a", "Delta Theta", "q v R" | "Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R." | yes (EN "Q-dot V") |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 212 | "Rho a", "c p a" | "Rho a und c p a sind die Stoffwerte der Luft…" | no |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 215 | "Delta Theta" | "Delta Theta ist der Temperaturhub zwischen Raumluft und gekühlter Zuluft" | yes |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 366 | "Q Punkt V", "Q Punkt S t r", "Rho a", "c p a", "Delta Theta" | "Weil Q Punkt V gleich Q Punkt S t r ist, stellen wir um …" | yes |
| `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | 406 | "Q-Punkt-S,tr" | "Das ist Q-Punkt-S,tr — die solare Kühllast durch die Verglasung." | no |
| `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` | 264, 551, 563, 767, 773, 776 | "Q-Punkt Personen", "q-Punkt p", "Q-Punkt Geräte", "P el", "f N", "Q-Punkt Licht", "Q-Punkt intern", "Q-Punkt i" | e.g. 773 "Q-Punkt i gleich Q-Punkt Personen plus Q-Punkt Geräte plus Q-Punkt Licht" | no |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 620 | "Q-Punkt-k" | "Diesen Wärmestrom durch den Feststoff nennen wir Q-Punkt-k." | no |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 730 | "Q-Punkt-c" | "Wärme, die bewegte Luft trägt, nennen wir Q-Punkt-c." | no |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 843 | "Q-Punkt-r" | "Diesen Wärmestrom nennen wir Q-Punkt-r." | no |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 933-935 | "Q-Punkt-k", "Q-Punkt-c", "Q-Punkt-r" | "In der Wand wirkt nur die Leitung: Q-Punkt-k." (and -c, -r) | yes ("Q dot k") |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 940-941 | "R-si", "R-se", "Summe d durch Lambda", "U-Wert" | "DIN EN ISO 6946 fasst beide Oberflächen zu R-si und R-se zusammen — mit Summe d durch Lambda ergibt das den U-Wert." | yes (EN "Sigma d over lambda") |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 1167 | "Lambda" | "… die Leitfähigkeit Lambda." | yes |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 1172-1173 | "Lambda" | "Lambda ist das Material selbst. Mauerwerk gegen Dämmung senkt Lambda rund dreißigmal" | yes |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 1548 | "Delta theta" | EN only: "Delta theta is the driver…" (DE pair at 1549) | yes |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 548 | "Leitfähigkeit Lambda" | "R ist Dicke d geteilt durch Leitfähigkeit Lambda — Einheit …" | yes |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 553-554 | "Lambda" | "Lambda ist die Wärmeleitfähigkeit in Watt pro Meter und Kelvin, nach DIN 4108 …" | yes |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 666 | "Lambda" | "Die Dicke steckt schon in U, über R gleich d durch Lambda —" | no |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 672 | "Delta-Theta" | "Durch die Wand fällt die Temperatur von innen nach außen — diese Lücke ist Delta-Theta." | yes |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 849 | "Lambda" | "Jede Schicht addiert ihren Widerstand d durch Lambda." | no |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 1008 | "Delta-Theta-i" | "Der Transmissionswärmeverlust ist die Summe aus U-i mal A-i mal Delta-Theta-i" | no |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 1188-1192 | "Psi" | "Mit Außenmaßen zählt das Eckquadrat doppelt … Psi wird negativ" / "Psi wird positiv" | yes |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 1201 | "Psi" | "Genau gerechnet addiert man jedes Psi mal Länge" | no |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 1204 | "Delta-U-WB" | "Pauschal kommt Delta-U-WB auf die ganze Hülle" | no |
| `🔥️heating/🎬️clip/🌬️convection/🐍️.py` | 149-150 | "Phi V", "delta theta", "c Luft" | "Phi-V gleich V mal n mal c-Luft mal Delta-Theta." | yes |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 329-336 | "Phi T", "Phi V", "Delta theta", "Phi Verlust", "Phi-T", "Phi-V" | "… als Phi-T — Summe aus U mal A mal Delta-Theta …" | yes |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 421-425 | "Phi-Verlust", "Phi-T", "Phi-V", "Phi geteilt durch Delta-Theta" | "Die Verlustleistung ist Phi-Verlust gleich Phi-T plus Phi-V" | yes |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 645-649 | "Phi-sol", "Q-sol", "Phi-int", "Q-int" | "… ist Phi-sol … wird daraus Q-sol …" | yes |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 802-815 | "Eta-h" (several) | "Eta-h ist die grüne Fläche geteilt durch alle Gewinne" | yes |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 812, 1054, 1057 | "Q-nutz", "Q-h", "Q-Verlust", "Eta-h", "Q-Gewinn" | "Heizwärmebedarf Q-h ist Q-Verlust minus Eta-h mal Q-Gewinn" | yes |
| `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py` | 360-361 | "Phi p" | "We call the person heat flux Phi p — unit watt." / "Personenwärmestrom Phi-p" | yes |
| `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py` | 431-432 | "Phi e" | "That equipment heat flux is Phi e" | yes |
| `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py` | 535-536 | "Phi l", "Phi p", "Phi e" | "Lighting heat is Phi l. Together: Phi p plus Phi e plus Phi l." | yes |
| `🔥️heating/🎬️clip/👥️internal-gains/🐍️.py` | 639-640 | "Phi int" | "Phi int equals Phi p plus Phi e plus Phi l" | yes |
| `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | 1085-1086 | "Phi solar", "F-f", "F-sh", "g" | "Put it together: Phi solar equals G times A times F-f times g times F-sh" | yes |
| `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | 1095 | "Eta-h" | "… den Ausnutzungsgrad Eta-h zeigt das Finale." | yes |
| `🔥️heating/🎬️clip/☀️solar-gains/🐍️.py` | 1098 | "Delta-T" | "… Temperaturanstieg Delta-T — in Joule." | yes |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 281, 284 | "c-Luft", "c-Luft", "ρ mal c-p" | "… die volumenbezogene Wärmekapazität der Luft, c-Luft" | yes |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 428, 431, 437, 443 | "Delta-Theta", "c-Luft", "Delta-Theta minus 6" | "Treiber ist die Temperaturdifferenz Delta-Theta" | yes |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 572, 575 | "Delta-x", "Delta-x" | "… Delta-x beträgt 4,5 g/kg." | yes |
| `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` | 222, 225, 227-228, 234, 240, 390 | "V-Punkt", "c-Luft", "Delta-Theta", "Delta-x", "Eta gleich 0,8", "Phi-V" | "Zusammen ergeben sie den Volumenstrom V-Punkt" | yes |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | 606 | "Rho" (EN only) | "Rho times c, the heat capacity of air…" | EN only |
| `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | 688 | "Eta h" | "… der Ausnutzungsgrad Eta h sagt, wie viel." | yes |
| `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | 912 | "Delta Theta" | "Sie ist das Delta Theta in jedem Transmissionsterm." | yes |

Physics fundamentals (`🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py`) contains no spelled-out Greek names in narration (only `\Delta` in math strings).

### 3.3 Spelled-out symbols on screen outside narration

No on-screen label or `Text(...)` contains a spelled-out Greek name; on-screen math uses LaTeX (`\Delta\theta`, `\Phi`, `\eta`). The only exceptions are the subtitle strings above and the `Text("Willems et al. 2024")` lines (section 4).

---

## 4. Literature and standard references

### 4.1 How references are rendered

- `_din_ref(text)` — one copy per video file (l. defined in: `🌬️ventilation/…/🪟️natural` l.97, `…/❓️need` l.103, `…/🤝️handoff` l.99, `…/🌡️heat-flow` l.91, `…/⚙️mechanical` l.111, `…/📦️volume` l.98, `🔥️heating/…/🧲️fundamentals` l.434, `…/🌬️convection` l.77, `…/🧱️conduction` l.145, `…/☀️solar-gains` l.129, `…/👥️internal-gains` l.199, `❄️cooling/…/📐️system-sizing` l.74, `…/☀️solar-radiation` l.80, `…/💧️transmission-humidity` l.79, `…/⚖️heating-comparison` l.88, `…/🌤️summer-strategy` l.86, `…/👥️internal-gains` l.79, `📊️demand/…/🧪️practice` l.133, `…/🔁️recap` l.131, `…/⚡️demand-power` l.93, `…/♨️system-losses` l.121, `…/🏭️primary-energy` l.122, `…/🏷️certificate` l.92, `…/⚖️regulations` l.154). Its body (`ref = Text(text, font_size=LABEL_FONT_SIZE - 3, color=P_PRIMARY)`, opacity 0.72, corner UR, buff 0.30) renders the argument verbatim.
- Literature cards: `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3160-3179` (Beat8_Zusammenfassung): two books with DOIs, shown as cards.
- Inline: `Text("Willems et al. 2024", ...)` and `_m(... "(Bohne 2022)")`.
- Quiz JSON: prose citations and a URL in `🧲️physics/❓️quiz/🔣️.json:3441-3502` (SDES).
- Docstrings: `🌬️ventilation/…/❓️need/🐍️.py:216` (`https://de.wikipedia.org/wiki/Pettenkofer-Zahl`).

### 4.2 Inventory and flags

Flag legend: **E** = no edition/year, **C** = no chapter/page/section/keyword that exists in the source, **U** = URL instead of a findable citation, **?** = cannot verify without web access.

| file | line | scene | displayed string | claim it backs | flags |
|---|---|---|---|---|---|
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 1902 | Beat4_Waermepumpe | `\mathrm{SPF} \approx 3 \quad \text{(Bohne 2022)}` (DE: `\mathrm{JAZ}`) | seasonal performance factor about 3 for air-source heat pumps | C (no page or table), ? (F01: new edition exists) |
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 2114 | Beat5_Strahlung | `Willems et al. 2024` | g-value / radiation rule | C (no chapter, page or keyword), ? |
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 2243 | Beat5_Strahlung | `Willems et al. 2024` | radiation/solar | C, ? |
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 2577 | Beat6_ThermischeMasse | `Willems et al. 2024` | thermal mass | C, ? |
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 3161 | Beat8_Zusammenfassung | card "Willems WM, Schild K, Stricker D (2024) Formeln und Tabellen Bauphysik", 8th edition, Wiesbaden, DOI 10.1007/978-3-658-44974-2 | general resource for radiation, thermal mass, moisture | keyword list given; no chapter; ? |
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 3164-3166 | Beat8_Zusammenfassung | card "Bohne D (2022) Gebäudetechnik und technischer Ausbau von Gebäuden", 12th edition, Wiesbaden, DOI 10.1007/978-3-658-37488-4 | heat pump, ventilation, building services | E-risk (F01: new edition), C, ? |
| `🔥️heating/…/🧲️fundamentals/🐍️.py` | 934-941 | Beat5_Zusammenfassung | `DIN EN ISO 6946` | R_si, R_se and U-value | no year, no clause |
| `🔥️heating/…/🧲️fundamentals/🐍️.py` | 1409 region | | `DIN EN ISO 6946` (many calls, lines 628, 738, 851, 998, 1069, 1184, 1409) | U-value of a component | E, C |
| `🔥️heating/…/🧲️fundamentals/🐍️.py` | 477 | | `2. Hauptsatz` (`_din_ref(tr(...))`) | second law, not a standard | no standard, not a citation |
| `🔥️heating/…/🧲️fundamentals/🐍️.py` | 1563 | | `DIN EN 12831-1` | heat load | E, C |
| `🔥️heating/…/🌬️convection/🐍️.py` | 171 | Beat1_LueftungImWinter | `DIN EN 12831-1` | ventilation loss Φ_V | E, C |
| `🔥️heating/…/🌬️convection/🐍️.py` | 82 (docstring) | | "chain of DIN EN 12831-1 (Φ_V = V · n · c_Luft · Δθ)" | | E |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 383 | Beat2_RWert | `DIN 4108-2` | envelope insulation | E, C |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 566, 684, 864 | | `DIN EN ISO 6946` | U-value | E, C |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 1026 | Beat5_Gebaeudehuelle | `DIN EN ISO 13789` | envelope coefficient | E, C |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 1216 | Beat6_Waermebruecken | `DIN 4108 Bbl. 2 · DIN V 18599-2` | thermal bridges, Δ U_WB | E, C ("Bbl. 2" abbreviation) |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 1204 (narration) | | "nach DIN 4108 Beiblatt 2" | 0,10 / 0,05 values | E |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 553-554 (narration) | | "nach DIN 4108: 0,035" | lambda 0.035 for insulation | E, C |
| `🔥️heating/…/☀️solar-gains/🐍️.py` | 381, 503, 748, 1110 | | `DIN V 18599-2` | solar gain chain | E, C |
| `🔥️heating/…/☀️solar-gains/🐍️.py` | 627 | | `DIN EN 410 · DIN V 18599-2` | g-value | E, C |
| `🔥️heating/…/☀️solar-gains/🐍️.py` | 883 | | `DIN V 18599-2 · DIN 4108-2` | sun-shading/solar | E, C |
| `🔥️heating/…/👥️internal-gains/🐍️.py` | 368, 439, 543 | | `DIN V 18599-10` | internal gains | E, C |
| `🌬️ventilation/…/❓️need/🐍️.py` | 216 | Beat1_Lueftungsbedarf (docstring) | `https://de.wikipedia.org/wiki/Pettenkofer-Zahl` | Pettenkofer value | U (Wikipedia, not a citable source) |
| `🌬️ventilation/…/❓️need/🐍️.py` | 227-228, 289 | Beat1_Lueftungsbedarf | "At about 1000 ppm the air counts as used up: the Pettenkofer value." / on screen `Pettenkofer limit`, `1000 ppm` | 1000 ppm limit (F04) | no source; no year; no chapter (F04 requests a findable reference) |
| `🌬️ventilation/…/❓️need/🐍️.py` | 252, 443, 610 | | `DIN EN 16798-1 · DIN 1946-6`, `DIN EN 12831-1`, `DIN EN 12831-1 · DIN 1946-6` | air change / ventilation | E, C |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 338, 582 | Beat1 / Beat2 | `DIN 1946-6` | ventilation | E, C |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 802, 1002, 1442 | | `DIN EN 16798-7` | natural ventilation | E, C |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 1231 | | `DIN 4108-2` | insulation | E, C |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 547 (narration) | Beat2_StossOderKipp | "beim gekippten erst etwa 8 %." | tilted window exchanges about 8 % of air in 5 min (F12) | **no source at all** (F12) |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 279 | | `DIN EN 16798-3` | ventilation system | E, C |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 458 | | `DIN EN 308` | heat recovery test | E, C |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 732 | | `DIN 1946-6` | ventilation | E, C |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 937 | | `VDI 2087` | duct design | E, C |
| `🌬️ventilation/…/🤝️handoff/🐍️.py` | 255, 413 | | `DIN EN 12831-1 · VDI 2078` | heating/cooling handoff | E, C |
| `🌬️ventilation/…/🌡️heat-flow/🐍️.py` | 296, 452, 596, 753 | | `DIN EN 12831-1`, `DIN EN 12831-1 · VDI 2078`, `VDI 2078` | heat flow | E, C |
| `🌬️ventilation/…/📦️volume/🐍️.py` | 488, 734, 928 | | `DIN V 18599-1 · DIN EN 12831-1`, `DIN EN 12831-1 · DIN 1946-4`, `DIN EN 12831-1 · DIN EN 16798-1` | volume | E, C |
| `❄️cooling/…/📐️system-sizing/🐍️.py` | 225, 398 | | `VDI 2078` | cooling load | E, C |
| `❄️cooling/…/💧️transmission-humidity/🐍️.py` | 234, 385 | | `VDI 2078` | | E, C |
| `❄️cooling/…/☀️solar-radiation/🐍️.py` | 200, 417 | | `VDI 2078` | | E, C |
| `❄️cooling/…/🌤️summer-strategy/🐍️.py` | 173 | | `DIN 4108-2` | summer heat protection | E, C |
| `❄️cooling/…/🌤️summer-strategy/🐍️.py` | 306 | | `VDI 2078` | | E, C |
| `❄️cooling/…/🌤️summer-strategy/🐍️.py` | 503 | | `DIN EN 16798-1` | | E, C |
| `❄️cooling/…/⚖️heating-comparison/🐍️.py` | 275 | | `DIN V 18599-2` | | E, C |
| `❄️cooling/…/⚖️heating-comparison/🐍️.py` | 343 | | `DIN 4108-2` | | E, C |
| `❄️cooling/…/⚖️heating-comparison/🐍️.py` | 413 | | `VDI 2078` | | E, C |
| `❄️cooling/…/👥️internal-gains/🐍️.py` | 199, 281, 579, 786, 897 | | `VDI 2078` | internal cooling loads | E, C |
| `📊️demand/…/🧪️practice/🐍️.py` | 313 | | `DIN V 18599-10:2018-09` | | has year, C |
| `📊️demand/…/🧪️practice/🐍️.py` | 706 | | `GEG § 60c · VOB/C DIN 18380` | | C, E |
| `📊️demand/…/🧪️practice/🐍️.py` | 1063 | | `GEG · DIN V 18599:2018-09` | | E (GEG without edition/year) |
| `📊️demand/…/🔁️recap/🐍️.py` | 436, 624, 944 | | `DIN V 18599-1`, `DIN V 18599-2` (×2) | | E, C |
| `📊️demand/…/🔁️recap/🐍️.py` | 1225 | | `DIN V 18599-2 · VDI 2078` | | E, C |
| `📊️demand/…/⚡️demand-power/🐍️.py` | 433 | | `Grundlagen · DIN V 18599-2` | | E, C |
| `📊️demand/…/⚡️demand-power/🐍️.py` | 630 | | `DIN EN 12831-1 · DIN/TS 12831-1` | | E, C |
| `📊️demand/…/⚡️demand-power/🐍️.py` | 890 | | `DIN EN 12831-1 · DIN V 18599-2` | | E, C |
| `📊️demand/…/⚡️demand-power/🐍️.py` | 1086 | | `DIN EN 12831-1 · DIN/TS 12831-1 · VDI 4645` | | E, C |
| `📊️demand/…/⚖️regulations/🐍️.py` | 588 | | `GEG § 20 ff.` | | E (no year), C |
| `📊️demand/…/⚖️regulations/🐍️.py` | 721 | | `DIN V 18599-2` | | E, C |
| `📊️demand/…/⚖️regulations/🐍️.py` | 955 | | `DIN V 18599-10 · DWD TRY` | | E, C, DWD TRY without year |
| `📊️demand/…/⚖️regulations/🐍️.py` | 1142 | | `VDI 3807 · DIN 4108-6` | | E, C |
| `📊️demand/…/♨️system-losses/🐍️.py` | 570 | | `DIN V 18599-5:2018-09 · -6` | | year on first only |
| `📊️demand/…/♨️system-losses/🐍️.py` | 756 | | `DIN V 18599-5 · VDI 4650-1` | | E, C |
| `📊️demand/…/♨️system-losses/🐍️.py` | 984 | | `DIN V 18599-5:2018-09` | | has year, C |
| `📊️demand/…/🏭️primary-energy/🐍️.py` | 410 | | `DIN V 18599-1:2018-09 · GEG § 20` | | GEG without year |
| `📊️demand/…/🏭️primary-energy/🐍️.py` | 616 | | `GEG Annex 4` / `GEG Anlage 4` | | E, C |
| `📊️demand/…/🏭️primary-energy/🐍️.py` | 934 | | `GEG § 20 · DIN V 18599:2018-09` | | E |
| `📊️demand/…/🏭️primary-energy/🐍️.py` | 1160 | | `GEG §§ 15–17 · § 71 · Anlage 1` | | E, C |
| `📊️demand/…/🏷️certificate/🐍️.py` | 340 | | `GEG § 79 ff. · § 80` | | E, C |
| `📊️demand/…/🏷️certificate/🐍️.py` | 537 | | `GEG § 86 · Anlage 10` | | E, C |
| `📊️demand/…/🏷️certificate/🐍️.py` | 757 | | `GEG § 84 · § 87` | | E, C |
| `🧲️physics/❓️quiz/🔣️.json` | 3441-3502 | quiz | `SDES: https://www.statistiques.developpement-durable.gouv.fr/bilan-energetique-de-la-france-pour-2023` | France 2023 national energy | U, E (no edition, no table) |
| `❄️cooling/❓️quiz/🔣️.json` | 101-102, 117-118, 124-125, 167-168, 230-231 | quiz | "GEG 2024", "DIN 4108-2", "GEG (German Buildings Energy Act)", "DIN 1946-4" | summer protection, medical equipment | E, C |
| `🔥️heating/❓️quiz/🔣️.json` | several | quiz | "GEG", "DIN" references | | E, C (not enumerated; same pattern) |

Note on `_din_ref` at `📊️demand/🎬️clip/⚖️regulations/🐍️.py:934` (`_source_card`): the card function is a separate source display. Its contents were not expanded in this pass.

### 4.3 Findings for F01 / F03 / F12

- **F01 (Bohne 2022)**: the only uses are `1902` (inline) and the card at `3164-3166`. Both say "12th edition" (card). The card's title and DOI are as given. Edition in the inline cite is missing. Also no page/chapter. A new edition in 2026 (per reviewer) means the year and edition must be checked and updated in both places and in any "Bohne" mention elsewhere (`🔥️heating/🧮️calculation/` has none).
- **F03 (Willems et al. 2024)**: three inline uses (`2114`, `2243`, `2577`) with no chapter, page, keyword. The card at `3161` gives 8th edition and DOI but not a chapter. Proposed inline form: `Willems, Schild, Stricker (2024), Formeln und Tabellen Bauphysik, 8. Aufl., Kap. <Strahlung|Speicherung>, S. <n>` (page to be confirmed in the book).
- **F12 (8 %)**: `🌬️ventilation/…/🪟️natural/🐍️.py:546-547` narration only; no citation at all. Needed: a source (for example a measured air-exchange table) with the exact claim "gekipptes Fenster: rund 8 % des Luftvolumens in 5 Minuten" or a rewording that the source supports.
- **F04 (Pettenkofer)**: no source. The 1000 ppm limit is shown on screen (`❓️need` l.289) and in narration (l.227-228). The docstring points to Wikipedia (l.216), which is not a citable reference.

---

## 5. Symbol inventory (heat flow and factors)

Counts from ripgrep over `E/` (`*.py`). "Files" = number of files with at least one hit. Representative lines given. Inconsistencies flagged with ⚠.

### 5.1 Heat-flow symbol Φ (`\Phi`, `\varPhi`, `φ`, spelled "Phi")

Total 99 occurrences in 16 files.

| file | count | representative lines |
|---|---|---|
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 6 | |
| `🔥️heating/…/☀️solar-gains/🐍️.py` | 12 | l.3 `Φ_solar = G · A · F_f · g · F_sh`; l.1081 "Master equation Φ_solar" |
| `🔥️heating/…/🌬️convection/🐍️.py` | 12 | l.149-150 `Φ_V = V · n · c_Luft · Δθ` |
| `🔥️heating/…/👥️internal-gains/🐍️.py` | 16 | l.360 "Phi p", l.431 "Phi e", l.535 "Phi l", l.639 "Phi int" |
| `🔥️heating/…/🧮️calculation/🐍️.py` | 26 | l.329 "Phi T", l.332 "Phi V", l.421 "Phi Verlust", l.645 "Phi sol", l.682 `\Phi_{sol}` |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 3 | l.1091 `\Phi_{i}\;[\mathrm{W}]` |
| `🌬️ventilation/…/🌡️heat-flow/🐍️.py` | 1 | |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 1 | l.1091 `\Phi_{\mathrm{V}}` |
| `🌬️ventilation/…/📦️volume/🐍️.py` | 1 | |
| `🌬️ventilation/…/❓️need/🐍️.py` | 2 | l.621 `\Phi_{\mathrm{V}},\;\dot{Q}_{\mathrm{L}}` |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 1 | |
| `🌬️ventilation/…/🤝️handoff/🐍️.py` | 5 | l.390 "Phi-V" |
| `❄️cooling/…/☀️solar-radiation/🐍️.py` | 1 | |
| `📊️demand/…/🔁️recap/🐍️.py` | 2 | |
| `📊️demand/…/⚡️demand-power/🐍️.py` | 8 | |
| `🏗️authoring/🎨️visuals/🐍️.py` | 2 | |

⚠ Heating and ventilation use Φ for heat flow (`Φ_V`, `Φ_solar`, `Φ_T`, `Φ_p`, `Φ_e`, `Φ_l`, `Φ_int`). Cooling and the ventilation heat-flow chain use `\dot{Q}` (see 5.2). F06 asks Q' (Q-Punkt) for both. Counts above show the split.

### 5.2 `\dot{Q}` / Q-Punkt

Total 92 occurrences in 15 files.

| file | count | representative lines |
|---|---|---|
| `🧲️physics/…/🧲️fundamentals/🐍️.py` | 2 | |
| `❄️cooling/…/⚖️heating-comparison/🐍️.py` | 1 | |
| `❄️cooling/…/☀️solar-radiation/🐍️.py` | 3 | l.406 "Q-Punkt-S,tr" |
| `❄️cooling/…/📐️system-sizing/🐍️.py` | 11 | l.209 "Q Punkt V", l.366 "Q Punkt S t r" |
| `🌬️ventilation/…/🪟️natural/🐍️.py` | 1 | |
| `🌬️ventilation/…/⚙️mechanical/🐍️.py` | 9 | l.691 `\dot{Q}_{\mathrm{V,sens}}\cdot(1-\eta)` |
| `🌬️ventilation/…/🌡️heat-flow/🐍️.py` | 7 | l.281 "c-Luft" (and `\dot{Q}` in formulas) |
| `🔥️heating/…/🧲️fundamentals/🐍️.py` | 13 | l.620 "Q-Punkt-k", l.730 "Q-Punkt-c", l.843 "Q-Punkt-r", l.933-935 |
| `❄️cooling/…/👥️internal-gains/🐍️.py` | 24 | l.264 "Q-Punkt Personen", l.773 "Q-Punkt i" |
| `❄️cooling/…/💧️transmission-humidity/🐍️.py` | 10 | l.216 "Q-Punkt-T", l.374 "Q-Punkt-L" |
| `🔥️heating/…/🌬️convection/🐍️.py` | 1 | |
| `🌬️ventilation/…/❓️need/🐍️.py` | 3 | l.621 `\dot{Q}_{\mathrm{L}}` |
| `🔥️heating/…/🧱️conduction/🐍️.py` | 2 | |
| `🌬️ventilation/…/🤝️handoff/🐍️.py` | 4 | "V-Punkt" l.222 |
| `🏗️authoring/🎨️visuals/🐍️.py` | 1 | |

Note: the heating solar-gains and internal-gains chapters use Φ in the formula but "Q-Punkt" in narration (`🔥️heating/…/👥️internal-gains` l.360 "Phi p" vs l.264 of cooling "Q-Punkt Personen"). Same physical quantity, two notations, across videos.

### 5.3 Volume flow `\dot{V}` and index V

`\dot{V}` total 17 occurrences in 5 files: `🌬️ventilation/…/🌡️heat-flow` (4), `…/📦️volume` (3), `…/❓️need` (2), `…/🤝️handoff` (6, incl. "V-Punkt" l.222), `📊️demand/…/🧪️practice` (2). Index `V` (ventilation) occurs in the `Φ_V` chain (heating, ventilation) and in `q_{v,R}` (cooling `📐️system-sizing` l.312, 382, 441 area). ⚠ Index V is in use for ventilation in heat-flow (`Φ_V`) and for the room-air flow `q_{v,R}` in cooling, which F06 calls out as "V belegt".

### 5.4 Shading and solar factors

| symbol | where | count / lines | notes |
|---|---|---|---|
| `F_{sh}` | `🔥️heating/…/☀️solar-gains` | 9 regex hits (l.3, 103, 133, 854, 1058, 1066, 1081, 1117, 1139), plus narration `F-sh` at l.1085-1086 | Beat5 Verschattung factor; F28 says it must be F_C per DIN 4108-6 |
| `F_{sh}` | `❄️cooling/…/☀️solar-radiation` | 1 (l.439) as `("fv", r"F_{sh}")` | ⚠ used in cooling with a different tag name `fv` |
| `F_C` | none | 0 | ⚠ F28 target symbol is absent from the whole energy scope |
| `F_{\mathrm{f}}` / `F_f` | `🔥️heating/…/☀️solar-gains` (l.574, 1085), `…/🧮️calculation` (682) | ~8 | window frame factor |
| `F_F` | `❄️cooling/…/📐️system-sizing` | 1 (`A_GLAZING, F_F, F_V, G_TOT, I_S_MAX = 30.0, 0.49, 0.15, 0.60, 800.0`, l.97) and l.510 | ⚠ same letter F for glazing-frame factor 0.49 here vs F_f in heating |
| `F_V` | `❄️cooling/…/📐️system-sizing` | l.97-98 | ⚠ "F_V = 0.15" conflicts with ventilation index V |
| `F_S` | `📊️demand/…/🔁️recap` | 6 regex hits (l.117 `F_S = 0.7`, l.119, 126, 986, 1005, 1038) | ⚠ shading factor named F_S here, F_sh in heating solar-gains |
| `F_{N}`, `f_{g}` | `❄️cooling/…/👥️internal-gains` | l.627, 721 | usage factors of devices and lights |
| `g` (g-value) | `🔥️heating/…/☀️solar-gains` (l.103, 713), `❄️cooling/…/📐️system-sizing` (G_TOT = 0.60, l.97) | | ⚠ cooling uses `G_TOT`, heating uses `g`. Symbol `g_\perp`: 0 occurrences in the scope. |
| `\Delta x` (Δx) | `🌬️ventilation/…/🌡️heat-flow` (l.572, 575, 656, 665, 674, 676, 812), `…/🤝️handoff` (l.204, 234, 329) | 10 | moisture difference; spelled "Delta-x" in narration (l.572, 575, 234) |
| `η` | `🌬️ventilation/…/⚙️mechanical` (l.539-559, 678, 691, 788-821), `…/🔥️heating/…/🌬️convection` (l.314-325), `❄️cooling/…/💧️transmission-humidity` (l.509) | `η_WRG` | heat recovery |
| `η_h` | `📊️demand/…/🔁️recap` (1109-1317), `⚖️regulations` (736, 795, 880), `🔥️heating/…/🧮️calculation` (939-1170) | | ⚠ demand uses `η_h`, `η_{h}` and `η_k`; heating calc uses `\eta_{\mathrm{h}}` |
| `Q_S`, `Q_I` | `📊️demand/…/🔁️recap` (451-452, 1001, 1063) | | ⚠ solar and internal gains as `Q_S`, `Q_I` here; heating uses `Q_{sol}`, `Q_{int}` (calc l.986-1171) |
| `Q_{\mathrm{sol}}`, `Q_{\mathrm{int}}` | `🔥️heating/…/🧮️calculation` (986, 987, 995-997, 1171) | | see above |
| `Q_S` as storage | `🔥️heating/…/🧮️calculation` (164-165) | | ⚠ `_Q_S` is storage energy in heating; `Q_S` is solar gain in demand recap |
| `H_T`, `H_V` | `🔥️heating/…/🧱️conduction` (l.133, 1417, 1458), `🔥️heating/…/🧮️calculation` (478-596), `📊️demand/…/⚡️demand-power` (78-79, 627, 779-813), `🏗️authoring/📏️running-example` (47-48) | | consistent |
| `g_\perp` | none | 0 | |

### 5.5 Other symbols checked

- Density `ρ`: `RHO_AIR = 1.2 kg/m³` from `🏗️authoring/📏️running-example/🔣️.json` (`airDensity 1.2`). F08 asks 1.27. Single source; not changed here.
- Specific heat: `c_p = 1005 J/(kg·K)`, `c_Luft = 0.34 Wh/(m³·K)` (rounded from 1.2 · 1005 / 3600 = 0.335). The rounding is shown as "0,34" and "≈ 0,34" (natural l.278).

---

## 6. Roadmap ("Fahrplan") position per video

Source: manifest `E/🎬️clip/🔣️.json`, ordered. Only energy videos are listed. The roadmap scene is `Beat3_Fahrplan` (class at `🌬️ventilation/🎬️clip/❓️need/🐍️.py:581`, docstring: "Roadmap of the video as five cards").

| video id | ordered scenes (chapter → beats) | roadmap position |
|---|---|---|
| `physical_fundamentals` | PhysicalFundamentals_FullSection → Beat1_Leistung, Beat2_EnergieImAlltag, Beat3_Energieerhaltung, Beat4_Waermepumpe, Beat5_Strahlung, Beat6_ThermischeMasse, Beat7_SensibelLatent, Beat8_Zusammenfassung | **none** |
| `ventilation` | Ventilation_01_WarumLueften → Beat1_Lueftungsbedarf, Beat2_Lueftungsweg, **Beat3_Fahrplan**; Ventilation_02_Luftmenge → Beat1..3; Ventilation_03_WaermestromLuft → Beat1..4; Ventilation_04_NatuerlicheLueftung → Beat1..6; Ventilation_05_MechanischeLueftung → Beat1..4; Ventilation_06_Uebergabe → Beat1..2 | **third beat of the first chapter** (after two content beats). Not at the start (F06). |
| `heating` | Heating_01_Introduction → Beat1_DreiWegeDerWaerme … Beat9_WaermestromFormel; Heating_02_Conduction → Beat1..6; Heating_03_Convection → Beat1; Heating_04_InternalGains → Beat1..5; Heating_05_Solar → Beat1..6, Beat8; Heating_06_FinalCalculation → ReviewingHeatLosses, Scene2, ReviewingHeatGains, Scene4, UltimateEnergyBalance, AnlagenVerluste | **none** |
| `cooling` | Cooling_01_HeatingVsCooling → Beat1..3; Cooling_02_InternalGains → Beat1..5; Cooling_03_TransmissionHumidity → Beat1 (cooling), Beat6_ThermischeMasse (physics script, clause ranges "day…lag" and "summer…summer"), Beat3 (cooling); Cooling_04_SolarRadiation → Beat1, Beat2, Beat5, Beat3, Beat5; Cooling_05_Systemauslegung → Beat2, Beat3; Cooling_06_Sommerstrategie → Beat1..3 | **none** |
| `energy_balance` | EnergyBalance_01_Energiebilanz → Beat1..4; _02_RegelwerkKlima → Beat1..4; _03_BedarfVsLeistung → Beat1..4; _04_NutzZuEndenergie → AnlagenVerluste, Beat2..4; _05_Primaerenergie → Beat1..4; _06_Energieausweis → Beat1..3; _07_TheorieVsPraxis → Beat1..3 | **none** |

Segments may point at scripts of other videos (for example Cooling_03 and Cooling_04 reuse physics `Beat6_ThermischeMasse` and heating `solar-gains` scenes, some with clause ranges). The table above lists the scene order as the manifest assembles it. The roadmap check depends only on the manifest's scene names; `Heating_01_Introduction` is the only intro-named scene, and its beats (`Beat1_DreiWegeDerWaerme` …) contain no roadmap card.

Outside this scope (for completeness): BIM, Grasshopper, Excel, optimization entries in the manifest, none with a roadmap.

Observation for F06: four of five energy videos have no roadmap at all; the ventilation one has it after two beats. Proposed: one roadmap scene per video as first chapter (or `Beat0`) reusing `Beat3_Fahrplan`'s five-card layout with the video's own chapters.

Note on the manifest: the energy-balance id uses the `📊️demand` folder. The `🔥️heating` and `❄️cooling` folders are the same scope as the manifest's ids (`heating`, `cooling`).

---

## Summary of counts

| check | result |
|---|---|
| bracket-unit label/tuple/axis occurrences (section 1) | ~100 rows in 25 files |
| numeric substitution lines with missing factor units (section 2) | 10 on-screen rows with missing factor units, 6 subtitle rows |
| spelled-out symbols in displayed text (section 3) | ~110 narration rows across 15 files; **no split displayed/spoken exists** |
| citation/standard references (section 4) | ~80 `_din_ref` calls, 3 inline Willems, 1 Bohne inline, 2 book cards, 1 Wikipedia URL, 1 SDES URL, 1 uncited 8 % claim |
| symbol inconsistencies (section 5) | Φ vs Q-punkt split; F_sh vs F_S vs F_F/F_f vs F_V; F_C absent; g vs G_TOT; η_h spelled variants; Q_S/Q_I vs Q_sol/Q_int |
| roadmap (section 6) | ventilation only, third beat; four videos none |

No source file was edited. Only this report was written.
