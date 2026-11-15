# Units consistency fragment (agent `consistency-units`)

Scripts: `consistency/units/{scan,display,frac,thin,bare,bare_text,balance,quiz_fix}.py`, `run-tests.sh`. Tool output: `🗑️generated/consistency-out/units/`.
ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`; scene paths are relative to it.

## 1. Compound units in display math -> `\frac{\mathrm{A}}{\mathrm{B}}` (121 unit occurrences, 66 lines, `frac.py`)

Rule applied: every `\mathrm{A/B}` outside `$…$` (display MathTex/label/readout strings, f-string `{{ }}` variants included) becomes `\frac{\mathrm{A}}{\mathrm{B}}` (parentheses of the denominator dropped: `W/(m\,K)` -> `\frac{\mathrm{W}}{\mathrm{m\,K}}`). Re-greped: the audit rows 1-61 had drifted; the generator found all current ones (also `kg/m^3`, `J/(kg\,K)`, `g/kg`, `kJ/kg`, `s/h`, `min/h`, `l/s`, `g/h`, `Kd/a`, `h/a`, `kWh/(m^2\,a)` that the audit missed).

| file | lines (before -> after, all `\mathrm{X/Y}` -> `\frac{\mathrm{X}}{\mathrm{Y}}`) |
|---|---|
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` | 886 (kWh/a, en+de), 2895 (g/kg), 2999 (g/h x2), 3063, 3068 (g/kg, en+de) |
| `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` | 1395-1403 (kWh/a, W/m, kWh/d), 1463 |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` | 632 (W/(m K)), 1328 (W/K x2), 1340, 1359, 1469, 1113 (header `in W/(m^2 K)`, by hand) |
| `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py` | 1350 (W/(m K)) |
| `📊️demand/🎬️clip/♨️system-losses/🐍️.py` | 1020, 1022, 1117 (kWh/(m^2 a), also constant `unit_tex`) |
| `📊️demand/🎬️clip/⚖️regulations/🐍️.py` | 1207, 1236, 1238 (Kd/a) |
| `📊️demand/🎬️clip/⚡️demand-power/🐍️.py` | 522, 988, 1019, 1225 (kWh/a), 1021 (h/a) |
| `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` | 113 (`UNIT` constant kWh/(m^2 a)), 1331 (W/(m^2 K) x2) |
| `📊️demand/🎬️clip/🏷️certificate/🐍️.py` | 82 (`UNIT` constant) |
| `📊️demand/🎬️clip/🧪️practice/🐍️.py` | 104 (`UNIT_INLINE` constant) |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | 811 (W/(m^2 K)) |
| `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py` | 990, 992, 1025 (m^3/s, m/s, x2 each) |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py` | 360, 364, 373, 374, 384 (m^3/h) |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` | 317, 319, 321, 431 (kg/m^3), 340, 431 (J/(kg K)), 371, 431 (s/h), 373, 375, 554, 543, 723, 667, 669, 682, 713, 831, 853 |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py` | 797, 884 (min/h), 960, 969, 1003 (m^3/h, s/h, m^3/s), 1004 (l/s), 1024-1058, 1081-1086, 816 (header, by hand) |
| `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` | 208-215, 321, 512 |
| `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py` | 1111, 1113, 1140, 1149 |
| `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` | 370 (W/m^2) |
| `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py` | 434 (W/K) |
| `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py` | 304, 308 (kg/m^3, kJ/(kg K)), 526, 528, 542, 573 (m^3/s, m/s), 316, 445 (headers `q_v,R in m^3/s`, by hand) |

Restructured by hand: `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1140` `\sqrt{2 \cdot Δp\,Pa\,/\,ρ\,kg/m^3}` -> `\sqrt{\frac{2 \cdot Δp\,\mathrm{Pa}}{ρ\,\frac{\mathrm{kg}}{\mathrm{m^3}}}}`.

Kept inline on purpose (judgement):
- chart axis names (`θ in °C`, `x in g/kg` `🧲️physics…fundamentals:2850`, `Q in kWh/Monat` `📊️demand…regulations:728`, `I_s in kWh/m^2` `…regulations:964`, `Irradiance in W/m^2` `❄️cooling…solar-radiation:237`): axis labels, as the brief allows.
- `$…$` in narration (caption layer), the lexicon/tex fixtures and the chain fixture `narration_values` (pinned inline narration strings).
- `\mathrm{t20/15}` is an index of `G_t20/15`, not a unit.

## 2. Thin space / upright (`thin.py`, 70 places, all non-narration TeX)

`number\;\mathrm{unit}` / `number\;\frac{…\mathrm{…}}` / `number\;^{\circ}\mathrm{C}` -> `\,` in `📊️demand/🎬️clip/{⚖️regulations,⚡️demand-power,🔁️recap,🧪️practice}/🐍️.py` (regulations 1158, 1183, 1236, 1238, 1254-1256; demand-power 461-1225 value rows; recap 717-1352 value rows, 875/876 rho note; practice 435). Narration `$…$` untouched (prose). Greps afterwards: digit glued to `\mathrm`/`\text` = 0; `number\;unit` in display = 0. All units are `\mathrm`.

## 3. Numeric factors without units (2c)

| item | result |
|---|---|
| ventilation quiz 898/899 | `28 × 25 m³/h` -> en `28 persons × 25.2 m³/(h·person) + 60 m² × 2.52 m³/(h·m²) ≈ 860 m³/h`, de `28 Pers. × 25,2 m³/(h·Pers.) + 60 m² × 2,52 m³/(h·m²) ≈ 860 m³/h` (exact EN 16798-1 cat. II values: 7 l/s = 25.2 m³/h, 0.7 l/(s·m²) = 2.52 m³/(h·m²); 705.6 + 151.2 = 856.8 ≈ 860, 4.8 1/h unchanged). Sports hall 816/817: `× 30` -> `× 30 athletes` / `× 30 Personen` |
| `integration.json:401` | already corrected by another agent (`30 m² · 0.70 · 0.15 · 0.60 · 800 W/m² ≈ 1510 W`, mirrors `❄️cooling…system-sizing:387/388`, test green); the `previous` entry intentionally keeps the old 0.49 text (it must still wrap to 3+ lines) |
| concepts 187 | `1,2 kg/m³ · 1 005 J/(kg·K) / 3 600 J/Wh ≈ 0,34 Wh/(m³·K)` |
| concepts 188 | `440 m³ · 0,5 h⁻¹ · 0,34 Wh/(m³·K) · 20 K ≈ 1 500 W` (220 m³/h · 0,34 · 20 = 1 496 W) |
| concepts 189 | `1,2 kg/m³ · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s ≈ 810 W` (recomputed 807 W) |
| concepts 190 | `1 500 W → 300 W, 450 W → 90 W` |
| concepts 308 | same as 188 |
| concepts 101 | unchanged: `g` is dimensionless |
| physics quiz 2270/2271 | `1,496 TWh × 0.27 / 8,760 h ≈ 0.046 TW = 46 GW` (de `1.496 TWh × 0,27 / 8.760 h ≈ 0,046 TW = 46 GW`) |
| `📊️demand/🎬️clip/README.md:43/44` | `Q_E`, `Q_P` now carry `kWh/(m²·a)` |
| scenes | re-grep (`bare.py`: every `\cdot` chain with a number beside it) found no further substitution lacking a unit; remaining bare factors are dimensionless (η, g, F_*, ratio, counts) |

## 4. Bare bracketed units (concepts, `📖️concepts/📝️.md`)

Unit text only, form `X, in unit` / `U in W/(m²·K)`: 175 (`h⁻¹`), 186 (`m³/h`), 226 (`m²·K/W`), 227, 228, 238, 266, 267, 275, 276, 282 (table header), 456. Audit rows 238/267/275/276/456 all done, plus 175/186/226/227/228/266/282 which the audit had not listed.

## 5. Re-grep proof

- inline compound `\mathrm{A/B}` outside `$…$` in scene/README/quiz/concepts: 0 except the 4 kept axis-label lines, the index `t20/15`, and test fixtures (above) -> display math = 0.
- `\mathrm{W}/\mathrm{K}`, `\,/\,\mathrm{…}` variants: 0.
- bracket units (`[W]`, `[kWh]`, `\left[`, `\,[\mathrm`, `[W/(m²·K)]`, `[m²·K/W]`, `[h⁻¹]`, `[m³/h]`, …) in concepts/README/quiz/scenes: 0 (the cooling README line 46 only describes the rule).
- Narration guard: no offender in the five energy videos; 16 offenders remain in OTHER families (`🏛️architecture/🎯️optimization/…/📉️extrema` θ ×14 and `📊️excel/…/⚡️energy` Φ ×2), not touched.
- Brace balance of all `\frac` strings (`balance.py`): ok.

## 6. Tests

Ran green: cooling (internal-gains, system-sizing, clip tests), ventilation (volume, architecture, numbers, transitions), heating conduction, physics fundamentals, demand (regulations sun, primary-energy chain+scale, recap balance, practice, practice speech/composition, tests chain+guard), visuals (labels, captions, scenes, readout, `🐍️.py`), narration tex, running-example. One pre-existing unrelated failure: `🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py` `test_physics_scene_constructor_bounds` (radiator height 0.6812 vs 0.7384, a radiator geometry case; no unit string involved).

## 7. Needs visual check (fractions are taller than inline units)

- `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1019,1021` (kWh/a and h/a fractions inside the `t_VL = Q_h/Q̇_H` panel: fraction in fraction numerator) and `:522,988,1225` (labels), `:779-817` (H_T, H_V rows, were `\;` now `\,`)
- `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:431` (nested: `kg/m³ · J/(kg K)` over `s/h`), `:371`, `:340`, `:667-713` (readouts), `:317-321` (m = ρ·V rows)
- `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1003` (`V̇/3600 s/h = … m³/s` nested), `:884`, `:816` (header `V̇ in m³/h`), `:1024-1086` (tight table rows)
- `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:1025` (`0,061 m³/s / 2,5 m/s` fraction with fraction units, numerator and denominator)
- `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1140` (new `\sqrt{\frac{…}{…\frac{kg}{m³}}}`), `:1149`, `:1133`
- `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:214` (`4,5 / 11,2 g/kg` pair), `:208-215` (card values)
- `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1395-1403` (readout column, fraction units in each row)
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:632` (`R = d/λ` fraction whose denominator holds a fraction unit), `:1113` (header), `:1328-1359` (ψ fractions with W/K numerators)
- `📊️demand/🎬️clip/🏷️certificate/🐍️.py` (`UNIT` constant used in the class-scale labels, lines 558-839) and `📊️demand/🎬️clip/🏭️primary-energy/🐍️.py` (113, 793, 963, 1104-1258) and `🧪️practice/🐍️.py:1247`, `♨️system-losses/🐍️.py:1117-1178`: the unit constant is now a fraction wherever `Q in UNIT` / `Q = 140 UNIT` is shown
- `📊️demand/🎬️clip/⚖️regulations/🐍️.py:1207,1236-1256` (Kd/a, kKh/a fractions)
- `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:316,445` (headers), `:304,308`, `:526-573`
- `🔁️recap:811` (`du_unit` under the ΔU meter)
