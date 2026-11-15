# Fragment: symbols and German wording (agent consistency-symbols)

ENERGY = `🎓️teaching/🏛️architecture/⚡️energy`. All edits are in place (no copies); scripts and probes live in `consistency/symbols/` (`apply_I.py`, `apply_I2.py`, `apply_G.py`, `apply_concepts.py`, `scan.py`, `proof.py`, `speak.py`).

## A. Symbols

### A1. Internal gains: lower-case `i`, no `\mathrm{I}`
Rendering decision: the loads keep the family style of heating (already shipped as `\dot{Q}_{\mathrm{i}}`, `\dot{Q}_{\mathrm{i,p}}`): upright lower-case i inside `\mathrm{}`, so cooling, heating and ventilation draw the identical glyph and sit beside the upright `T V S C` indices. Parts: `i,p` persons, `i,e` equipment, `i,l` lighting, `i,lat` latent internal. README text uses the plain form `Q̇_i`.

| file | change |
|---|---|
| `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` 403, 404, 460 | `\dot{Q}_{\mathrm{I}}` -> `\dot{Q}_{\mathrm{i}}` |
| `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py` 265-266, 437, 517, 555-556, 567-568, 629, 723, 771-786, 822-824, 849-857, 901-902 | `\mathrm{I}`, `\mathrm{I,p}`, `\mathrm{I,e}`, `\mathrm{I,l}` -> `\mathrm{i}`, `\mathrm{i,p}`, `\mathrm{i,e}`, `\mathrm{i,l}` (also the f-string `{{ }}` forms) |
| `❄️cooling/🎬️clip/🗺️roadmap/🐍️.py` 38, 39, 57 | `\dot{Q}_{\mathrm{I}}` -> `\dot{Q}_{\mathrm{i}}` |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py` 286 | `\dot{Q}_{\mathrm{I,lat}}` -> `\dot{Q}_{\mathrm{i,lat}}` |
| `❄️cooling/🎬️clip/README.md` 42, 46 | `Q̇_I` -> `Q̇_i`; `\dot{Q}_\mathrm{I}` -> `\dot{Q}_i`; `I,p / I,e / I,l` -> `i,p / i,e / i,l` plus `i,lat` |
| `🔥️heating/🎬️clip/README.md` 47 | `Q̇_I` -> `Q̇_i` |
| `🔥️heating/🎬️clip/🗺️roadmap/🐍️.py` 44, 45, 60 | `\mathrm{I}` -> `\mathrm{i}` |
| `📊️demand/🎬️clip/🔁️recap/🐍️.py` | already `Q_i` / `Q_{i}` when this pass started (another agent), verified |
| concepts `📖️concepts/📝️.md` ~456-459 | `Q̇_Pers`, `Q̇_Geräte`, `Q̇_Licht` -> `Q̇_i,p`, `Q̇_i,e`, `Q̇_i,l` |

Not touched on purpose: Python identifiers (`THETA_I`, `Q_I_SPEC`), IWU type names `EFH_I` in quiz answers (a building type, not a symbol), irradiance `I_s`.
Open style point (not decided here): heating and cooling write all load/energy indices upright (`\mathrm{}`), the demand scenes write `Q_{h}`, `Q_{i}`, `\dot{Q}_H`, `\dot{Q}_C` italic; both read correctly, the glyph differs between families.

### A2. Speech lexicon
`🏗️authoring/🔊️speech/🗣️lexicon/🔣️.json`: new whole-symbol entry `G_{t,20/15}` -> "G t 20 15" (otherwise spoken "G t 20 durch 15", i.e. "20 divided by 15").
`🔊️speech/🧪️tests/🗣️lexicon/🔣️.cases.json` new cases: `\dot{Q}_{\mathrm{i}}`, `\dot{Q}_{\mathrm{i,p}}`, `\dot{Q}_{\mathrm{i,e}}`, `\dot{Q}_{\mathrm{i,l}}`, `\dot{Q}_{\mathrm{i,lat}}` ("Q-Punkt i p", "Q dot i latent", ...), `\Phi_{\mathrm{c,si}}` ("Phi c s i"), `G_{t,20/15}`, `e_{E}` ("e E"). Everything else (`\dot{Q}_{\mathrm{H}}`, `\dot{Q}_{\mathrm{C}}`, `F_{Gt}`, `e_{ce}`, `\Sigma\,\Phi`, `I_{s,max}`, `\Delta\theta_{eq}`) was already spoken by the generic composition (probe: `speak.py` + `speak-in.txt`, all fourteen probes speak in de and en).

### A3. Anlagenaufwandszahl `e_E`
Quantity check: `e_E = e_ce · e_d · e_s · e_g = Q_E / Q_h` (final energy per useful energy, DIN V 4701-10 chain; gas boiler 1,40, heat pump 0,31). `e_P` in `📊️demand/🎬️clip/🏭️primary-energy` is a different quantity, `Q_P / Q_h = e_E · f_P` (1,54 / 0,56), so it stays `e_P` but is no longer called "the plant expenditure figure": primary-energy `🐍️.py` 941-942 now "primary-energy expenditure figure / primärenergetische Aufwandszahl `e_P = e_E · f_P = Q_P/Q_h`". `e_g` is the generator stage factor (one of the four stage factors `e_ce e_d e_s e_g`), not the chain figure: kept.
Unified to `e_{E}`: `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 44, 45, 64 (`e_{\mathrm{E}}` -> `e_{E}`; the English roadmap line now says "plant expenditure figure"); comment in `♨️system-losses/🐍️.py` 108 (`e_p` -> `e_E`); test fixture key `e_p` -> `e_E` in `📊️demand/🎬️clip/🧪️tests/🔗️chain/🔣️.json` and `🐍️.py` 40-41. No `e_ges`/`e_tot`/`e_{\mathrm{E}}` variants remain.

### A4. Degree days `G_{t,20/15}` and `F_{Gt}`
`📊️demand/🎬️clip/⚖️regulations/🐍️.py` 97, 144, 1107-1108, 1131-1132, 1321; `📊️demand/🎬️clip/🔁️recap/🧪️tests/🧮️balance/🐍️.py` 73; `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 38, 39, 62 (`G_{\mathrm{t}}` -> `G_{t,20/15}`); `🔥️heating/🎬️clip/🧮️calculation/🐍️.py` 196, 424, 425, 430, 431, 487, 489, 566, 597, 639, 640 (`G_{\mathrm{t20/15}}`, `G_{\mathrm{t}}`, `Gt20/15` -> `G_{t,20/15}`; the plain-text source label reads `G_t,20/15`); `F_{\mathrm{Gt}}` -> `F_{Gt}` in the same file (430, 431, 636); `📊️demand/🎬️clip/README.md` 28, 61, 90, 101; concepts 400, 401.
Needs visual check: heating calculation 566 (`gt_def`), 597 (formula bar with the longer `G_{t,20/15}`), 636 (`f_gt`), 487/489 (formula strip).

### A5. Loads
`\dot{Q}_{\mathrm{C}}` cooling load, `\dot{Q}_{\mathrm{H}}` heating load, `Q_c` cooling demand: no `Q̇_K`, `Q_K`, `Q̇_L` anywhere (grep proof). Demand scenes write the same symbols italic (`\dot{Q}_H`); see style point in A1.

### A6. Forbidden symbols removed from shown text, README, concepts, docstrings
- `📖️concepts/📝️.md`: `ΔT`/`ΔT_eq` -> `Δθ`/`Δθ_eq` (65, 67, 84, 95, 492, 498, 505, 612; table row 612 now just `Δθ`); `ϑ` -> `θ` (66); `Φ_V` -> `Q̇_V`, `Φ_T` -> `Q̇_T` (197, 276, 302, 308, 309, 313); `Q̇_L` -> `Q̇_V` (198, 499); `R_ges` -> `R_T` (236, 237, 274); `F_V` -> `F_C` and `I_S,max` -> `I_s,max`, `I_reduziert` -> `I_s,red` (527-538; the `F_C` row now reads "Sonnenschutzfaktor"); mechanism fluxes `Q̇_k/Q̇_c/Q̇_r` -> `Φ_k/Φ_c/Φ_r` (222-224), surface chain `Φ_c,si + Φ_r,si = Φ_k = Φ_c,se + Φ_r,se` (234), `Φ = U · A · Δθ` (238, 290, 216). Unicode tables were not rewritten into `$…$` (the concepts file is a plain reference, not shown on screen); only the lines carrying forbidden or changed symbols were fixed.
- `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py` docstring lines 1, 5, 410: `Φ_V` -> `Q̇_V` and Kühlung -> Kühltechnik; same in its `🔊️speech/🐍️.py` docstring.
- `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py` 179 region comment `I_S,max` -> `I_s,max`; `🏗️authoring/🎨️visuals/🧪️tests/🔣️scenes.json` 104 layout fixture `I_{S,max} = 800` -> `I_{s,max} = 800`.
- Kept deliberately: `ϑ`/`\vartheta` in the font tests (`🔤️fonts/🧪️tests`), the TeX map `📝️narration/🧮️tex/🔣️.json`; `F_sh` named as "never F_sh" in `❄️cooling/🎬️clip/README.md` and as ISO 52016-1 notation in `📚️references/🔣️.json:401`.

### A7. Indexed Φ
Allowed set after this pass: `\Phi_{\mathrm{k}}`, `\Phi_{\mathrm{c}}`, `\Phi_{\mathrm{r}}` (defined in heating fundamentals 624/734/911) and the surface split `\Phi_{\mathrm{c,si}}`, `\Phi_{\mathrm{r,si}}`, `\Phi_{\mathrm{c,se}}`, `\Phi_{\mathrm{r,se}}` (fundamentals 1055-1057, was `c,i r,i c,e r,e`), plus `\Phi_{HL}` once.
- `🔥️heating/🎬️clip/🧱️conduction/🐍️.py` 1024-1025 narration: `\Phi_i = U_i A_i Δθ_i` -> `\Phi = U · A · Δθ` per part, "their sum `\Sigma\,\Phi` is the transmission load `\dot{Q}_{\mathrm{T}}`" (en + de); 1091 `\Sigma\,\Phi_{i}` -> `\Sigma\,\Phi`; 1109 table header `\Phi_{i}` -> `\Phi`.
- `\Phi_{HL}` (demand video): only the regulations scene (chapter 2, `⚖️regulations/🐍️.py` 549-550, "dort heißt sie Φ_HL" / "which calls it Φ_HL") keeps it. `⚡️demand-power/🐍️.py` 587-588 narration now says "eine eigene Norm, DIN EN 12831-1, und eigene Regeln" and the `Symbol:` fact row (650) shows only `\dot{Q}_{H}`. README mentions of "Norm: Φ_HL" stay (documentation).
- Internal-gain parts are never `Φ_l`-style (grep proof).
Needs visual check: fundamentals 1055-1057 (longer `c,si` subscripts in the surface-chain formula), conduction 1091 and 1109 (shorter sum, header).

## B. German wording
- Kühlung -> Kühltechnik (system meant): `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py` 777 (narration), 860 (chip `→ Kühltechnik:`); `📦️volume/🐍️.py` 912 (narration), 1025 (card `Kühltechnik · Sommer`), 1093 (chip); `🗺️roadmap/🐍️.py` 51 (narration), 66 (chip); `🤝️handoff` docstrings; `🌬️ventilation/🎬️clip/README.md` 100; `📊️demand/🎬️clip/🗺️roadmap/🐍️.py` 61 (chip `← Kühltechnik: Q_c`); `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` 3134; quiz `🌬️ventilation/❓️quiz/🔣️.json` 647, 651 and `❄️cooling/❓️quiz/🔣️.json` 859; `🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json` 91 ("Auslegung der Kühltechnik"); concepts 18, 157, 168, 176, 202, 204. Already changed by others before this pass: `integration.json` 213 and `🪟️natural` 1551, `README.md` 10 and 121, `handoff` `TITLE_DE`.
- Kept: video title "Kühlung" (concepts headings, `Cooling_*` titles, roadmap handover names in `🗺️roadmap/🧪️tests`), "freie Kühlung", "sorptionsgestützte Kühlung", process uses (physics 2743, 3058 "Kühlung entfeuchtet", concepts 481), and the reviewer's own sentence in `🌬️ventilation/🎬️clip/❓️need/🐍️.py:237` ("…bedarf es immer der Lüftung neben Heizung und Kühlung", F05 quotes it verbatim; the integration fixture mirrors the English/Kühltechnik handover instead, not this line).
- Pettenkoferwert -> "1 000 ppm Grenzwert nach Pettenkofer" (concepts 163); grep: 0 remaining.
- "each facade lights up in turn": the shipped narration was already physics wording ("Facade power peaks east in the morning, south at noon, west in the afternoon"); the phrase survives only in the `previous` block of `integration.json:349`, which is the long-caption regression fixture (`previous` texts must still wrap to 3+ lines, `🐍️integration.py:191`), so it is left as historical input. Narration grep for `leuchtet`, `erscheint`, `verblasst`, `blendet`, `lights up`: 0 hits in clauses (docstrings only).

## C. Verification

Final grep proof (`python consistency/symbols/proof.py`, whole ENERGY tree, py/md/json/ts):

| token | remaining |
|---|---|
| `\mathrm{I}` index, `Q_I`, `Q̇_I`, `I,p/e/l` | 0 |
| `Φ_V`, `Φ_T` | 0 |
| `Q̇_L`, `Q̇_K` | 0 |
| `F_V`, `F_f` | 0 |
| `F_sh` | 2, both deliberate negative mentions (README "never F_sh", references note) |
| `R_ges` | 0 |
| `ΔT` / `\Delta T` | 0 |
| `ϑ` / `\vartheta` | 12, all font tests and the TeX letter map (deliberate fixtures) |
| `G_t20/15`, `G_{\mathrm{t…}}`, bare `G_t` | 0 |
| `e_{\mathrm{E}}`, `F_{\mathrm{Gt}}` | 0 |
| `e_P` | 8, all the primary-energy expenditure figure and the norm symbol list (README), a different quantity |
| `Pettenkoferwert`, `I_S,max`, `Q̇_Licht/Pers/Geräte` | 0 |

Roadmap chips (`🗺️roadmap` ventilation 66 `→ Kühlung:`, demand 61 `← Kühlung:`) stay "Kühlung": the roadmap contract test requires the chip to name the partner VIDEO ("Kühlung"); I had changed them and reverted. Narration at ventilation roadmap 51 stays "die Kühltechnik".

Tests (private output root, `SEMIO_TEACHING_PROFILE=preview`, language env set where a test needs it; `run-tests.sh` runs each module alone):
- Narration guard on the five energy videos: clean (907 clauses). Only 2 offenders, both in `🏗️excel/🎬️clip/⚡️energy` (Φ in `load`), not ours.
- Pass: speech lexicon (12, incl. the new cases), cooling (internal-gains, system-sizing, tests), ventilation (volume, architecture, numbers, transitions), visuals (captions, scenes, sections, section_consumers, icons, labels, readout, transforms, semio-style needs the language env only), presentation hierarchy + chapters + pointer, running-example, narration (guard, tex, caption-layout), roadmap (11), demand (regulations sun, primary-energy chain/scale, recap balance, chain, guard), heating conduction, physics fundamentals.
- Not green and not caused by this pass: `visuals.🧪️tests.🐍️integration` `test_physics_scene_constructor_bounds` (radiator height 0.681 vs 0.738, staffage geometry); `visuals.🧪️tests.🐍️staffage_only` timed out at 900 s in the combined run.
- Fixture `integration.json` `previous` blocks (long-caption regression inputs, e.g. "lights up", "Kühlung") are intentionally unchanged.

## D. Needs visual check
- heating calculation 566, 597, 636, 487/489 (longer `G_{t,20/15}` / `F_{Gt}`)
- heating fundamentals 1055-1057 (`c,si` subscripts)
- heating conduction 1091, 1109 (`\Sigma\,\Phi`, header `\Phi`)
- demand-power ~650 (Symbol row shows only `\dot{Q}_{H}`, three fact rows still stacked), demand roadmap symbols chip `G_{t,20/15}`
- ventilation heat-flow 860, volume 1025 and 1093 (longer `Kühltechnik` chips/cards)
- cooling internal-gains 849-857 (`\mathrm{i,p}` formula rows), unchanged width expected
