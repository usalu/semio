# Consistency audit: energy videos after reviewer feedback

Read-only audit of `🎓️teaching/🏛️architecture/⚡️energy/` on 2026-11-13. No source file, git state or working tree was changed. Scratch and sub-audits live in `audit2/`.

## Summary

- 1. Symbols: 40 uppercase internal-gains lines (`_I`, `_{\mathrm{I}}`, `Q̇_{I,lat}`); 21 forbidden-symbol lines; 10 indexed `Φ` groups; 3 value contradictions or conflicts (g-value 0,50 vs 0,60; design basis 0 °C / Δθ 20 K vs −12 °C / Δθ 32 K) and 2 notation variants (`G_t20/15` vs `G_{t,20/15}`; `F_Gt` in two forms).
- 2. Units: 5 bare brackets (all in concepts); 61 inline compound units in display math; numeric factors without units in concepts, two physics quiz lines and one ventilation quiz line; thin-space pass partial.
- 3. Unicode and spelled symbols: 66 bare Unicode lines in README, concepts and quiz text; 3 spelled-out lines in a fixture; the rest are intended guard or lexicon tests.
- 4. References: 47 distinct; 2 verified, 12 edition-only, 17 unverified, 16 wrong. DIN V 4108-6 (withdrawn, ~51 uses), DIN V 18599-2 (withdrawn), DIN EN ISO 13789 edition, DIN EN 410, DIN EN 14825, DIN EN ISO 13786, VDI 6022, VDI 3807 editions wrong; GModG renaming confirmed.
- 5. Parentheses: no code or font remap; Anta draws `(` as a rounded arc and `[` as a square bracket. The bracket look in the frame is not confirmed at the resolution checked. Test gap: no parentheses in font tests.
- 6. German: ~18 `Kühlung` system uses to change to `Kühltechnik`; `Pettenkoferwert` (1); one animation-describing clause (`lights up`); no `verblasst`/`erscheint` in narration.
- 7. Tests: 70 test-folder modules, 65 registered, 5 not tests; no unregistered test module.

Scripts used: `audit2/g.py` with regex files `audit2/p-*.txt`; `audit2/s3.py`, `audit2/s3md.py`, `audit2/s7.py`, `audit2/paren_glyph.py`, `audit2/build_report.py`.

## 1. Cross-video symbol inconsistencies

Decisions applied: internal gains lower case `i` (`Q̇_i`, `Q̇_{i,p}`, `Q̇_{i,e}`, `Q̇_{i,l}`); `e_E` for the Anlagenaufwandszahl; `G_{t,20/15}` for degree days; cooling demand `Q_c`, cooling load `Q̇_C`; heating load `Q̇_H`; no `Φ_V`, `Q̇_L`, `Q̇_K`, `F_sh`, `F_V`, `F_f`, `R_ges`, `ϑ`, `ΔT`.

### 1a. Uppercase internal-gains symbols (40 lines, lower case `i` required)

| # | file:line | token(s) and proposed replacement |
|---|---|---|
| 1 | `❄️cooling/🎬️clip/README.md:42` | `Q̇_I` → `Q̇_i` |
| 2 | `❄️cooling/🎬️clip/README.md:46` | `\dot{Q}_\mathrm{I}` → `\dot{Q}_i` |
| 3 | `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:405` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 4 | `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:406` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 5 | `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py:462` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 6 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:267` | `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}` |
| 7 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:268` | `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}` |
| 8 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:439` | `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}` |
| 9 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:557` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}` |
| 10 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:558` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}` |
| 11 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:569` | `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}` |
| 12 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:570` | `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}` |
| 13 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:631` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}` |
| 14 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:725` | `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}` |
| 15 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:773` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 16 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:774` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 17 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:779` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}`; `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}`; `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}`; `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 18 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:780` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}`; `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}`; `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}`; `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 19 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:782` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 20 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:783` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 21 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:785` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 22 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:786` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 23 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:851` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 24 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:852` | `\dot{Q}_{\mathrm{I,p}}` → `\dot{Q}_{i,p}` |
| 25 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:853` | `\dot{Q}_{\mathrm{I,e}}` → `\dot{Q}_{i,e}` |
| 26 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:854` | `\dot{Q}_{\mathrm{I,l}}` → `\dot{Q}_{i,l}` |
| 27 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:859` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 28 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:903` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 29 | `❄️cooling/🎬️clip/👥️internal-gains/🐍️.py:904` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 30 | `❄️cooling/🎬️clip/🗺️roadmap/🐍️.py:40` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 31 | `❄️cooling/🎬️clip/🗺️roadmap/🐍️.py:41` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 32 | `❄️cooling/🎬️clip/🗺️roadmap/🐍️.py:59` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 33 | `📊️demand/🎬️clip/🔁️recap/🐍️.py:143` | `Q_I` → `Q_i` |
| 34 | `📊️demand/🎬️clip/🔁️recap/🐍️.py:144` | `Q_I` → `Q_i` |
| 35 | `📊️demand/🎬️clip/🔁️recap/🐍️.py:1130` | `Q_I` → `Q_i` |
| 36 | `🔥️heating/🎬️clip/README.md:47` | `Q̇_I` → `Q̇_i` |
| 37 | `🔥️heating/🎬️clip/🗺️roadmap/🐍️.py:46` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 38 | `🔥️heating/🎬️clip/🗺️roadmap/🐍️.py:47` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 39 | `🔥️heating/🎬️clip/🗺️roadmap/🐍️.py:62` | `\dot{Q}_{\mathrm{I}}` → `\dot{Q}_{i}` |
| 40 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:288` | `\dot{Q}_{\mathrm{I,lat}}` → `\dot{Q}_{i,lat}` |

Notes: `i` is reserved for internal gains in the energy videos. Heating scenes already use lower-case `Q̇_i` (heat-b). The `Q̇_{I,lat}` variant sits in ventilation `❓️need/🐍️.py:288`. Python identifiers such as `THETA_I` are not symbols and are excluded.

### 1b. Forbidden symbols in shown or documented text (21 lines)

Scope: README, concepts, quiz JSON and shown strings in scenes. Test fixtures, the speech lexicon and the font test that deliberately contain `ϑ` or `\vartheta` are excluded, as are Python identifiers such as `PHI_V_W`, which are not shown.

| # | file:line | forbidden token → replacement |
|---|---|---|
| 1 | `📖️concepts/📝️.md:65` | `ΔT` → `Δθ` |
| 2 | `📖️concepts/📝️.md:66` | `ϑ` → `θ` |
| 3 | `📖️concepts/📝️.md:67` | `ΔT` → `Δθ` |
| 4 | `📖️concepts/📝️.md:84` | `ΔT` → `Δθ` |
| 5 | `📖️concepts/📝️.md:95` | `ΔT` → `Δθ` |
| 6 | `📖️concepts/📝️.md:197` | `Φ_V` → `Q̇_V` |
| 7 | `📖️concepts/📝️.md:198` | `Q̇_L` → `Q̇_V` |
| 8 | `📖️concepts/📝️.md:236` | `R_ges` → `R_T` |
| 9 | `📖️concepts/📝️.md:237` | `R_ges` → `R_T` |
| 10 | `📖️concepts/📝️.md:274` | `R_ges` → `R_T` |
| 11 | `📖️concepts/📝️.md:302` | `Φ_V` → `Q̇_V` |
| 12 | `📖️concepts/📝️.md:308` | `Φ_V` → `Q̇_V` |
| 13 | `📖️concepts/📝️.md:309` | `Φ_V` → `Q̇_V` |
| 14 | `📖️concepts/📝️.md:313` | `Φ_V` → `Q̇_V`; `Φ_T` → `Q̇_T` |
| 15 | `📖️concepts/📝️.md:458` | `Q̇_L` → `Q̇_V`; `Q̇_Licht` → `Q̇_{i,l}` |
| 16 | `📖️concepts/📝️.md:459` | `Q̇_L` → `Q̇_V`; `Q̇_Licht` → `Q̇_{i,l}`; `Q̇_Pers` → `Q̇_{i,p}`; `Q̇_Geräte` → `Q̇_{i,e}` |
| 17 | `📖️concepts/📝️.md:499` | `Q̇_L` → `Q̇_V` |
| 18 | `📖️concepts/📝️.md:529` | `F_V` → `F_C`; `I_S,max` → `I_{s,max}`; `I_reduziert` → `I_{s,red}` |
| 19 | `📖️concepts/📝️.md:531` | `F_V` → `F_C`; `I_S,max` → `I_{s,max}` |
| 20 | `📖️concepts/📝️.md:538` | `F_V` → `F_C` |
| 21 | `📖️concepts/📝️.md:612` | `ΔT` → `Δθ` |

Also: `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:5` (docstring) still says `Φ_V`; change to `Q̇_V`. The concepts file `📖️concepts/📝️.md` contains about 20 of these and needs a full pass, not only the listed lines.

### 1c. Indexed `Φ` uses (10 groups, from 21 hits)

| # | file:line | token | definition / role | proposed |
|---|---|---|---|---|
| 1 | `📊️demand/🎬️clip/⚖️regulations/🐍️.py:551-552` | `\Phi_{HL}` | Quoted DIN EN 12831-1 name for the heating load. Defined here (the norm's symbol, not a building index). Keep only as a norm quote, or drop: convention puts the load at `Q̇_H`. | keep as norm quote with `Q̇_H` on the same line |
| 2 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:589-590, 652` | `\Phi_{\mathrm{HL}}` | Same norm quote, plus a `Symbol:` fact row (line 652) that pairs `Q̇_H` with `Φ_HL`. | keep the quote; drop the `Φ_HL` fact row or label it 'in der Norm' |
| 3 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1026-1027, 1093, 1111` | `\Phi_i`, `\Sigma\,\Phi_{i}` | Per-element heat flow with a building index `i` (sum over building elements). Convention: `Φ` without building index. | `\Phi` alone; write the sum as `\Sigma\,\Phi` |
| 4 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:626-627, 1014, 1058` | `\Phi_{\mathrm{k}}` | Mechanism index (conduction). Defined at line 626. Not a building index, so acceptable; keep upright. | keep `\Phi_{\mathrm{k}}`; owner decision |
| 5 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:736-737, 1015, 1059` | `\Phi_{\mathrm{c}}` | Mechanism index (convection). Defined at line 736. Acceptable as mechanism index. | keep `\Phi_{\mathrm{c}}` |
| 6 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:913-914, 1016, 1060` | `\Phi_{\mathrm{r}}` | Mechanism index (radiation). Defined at line 913. Acceptable as mechanism index. | keep `\Phi_{\mathrm{r}}` |
| 7 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1057-1059` | `\Phi_{\mathrm{c,i}}`, `\Phi_{\mathrm{r,i}}`, `\Phi_{\mathrm{c,e}}`, `\Phi_{\mathrm{r,e}}` | Index `i` (inside) collides with the internal-gains index `i`; `e` is outside. | `\Phi_{\mathrm{c,si}}`, `\Phi_{\mathrm{r,si}}`, `\Phi_{\mathrm{c,se}}`, `\Phi_{\mathrm{r,se}}` (matches `R_si`, `R_se`) |
| 8 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:375, 1747 and 1739` | `\Phi_{\mathrm{...}}` (typeset helper, `Φ = U·A·Δθ`) | Generic `Φ` with a sub-index chosen by the helper; the derived line itself is correct. | keep; no building index |
| 9 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1093, 1442-1484 area` | `\Sigma\,U_{i}A_{i}`, `\Sigma\,\psi_{k}\,l_{k}` | Sums over building elements / thermal bridges with lower-case index: a building index on the component `U`, not on `Φ`. Convention is silent on `U_i`; keep but state the sum once. | keep; judge |
| 10 | `📖️concepts/📝️.md:197, 276, 302, 308-313` | `Φ_V`, `Φ_T` | Ventilation and transmission heat flows written as `Φ` with a building index; forbidden by convention (`Q̇_V`, `Q̇_T`). | `Q̇_V`, `Q̇_T` |

### 1d. Same quantity, different values across files

| quantity | value per file (file:line) | verdict and proposed single value |
|---|---|---|
| g-value g⊥ (glazing, normal incidence) | physics `🧲️fundamentals/🐍️.py:2124` `g_{\perp} = 0.50`; heating `☀️solar-gains/🐍️.py:115` comment and `G_VALUE` = 0,60 (`:115` `g_⊥ = 0,60`); cooling `☀️solar-radiation/🐍️.py:102` uses `_heating_solar.G_VALUE` (0,60); demand `🔁️recap/🐍️.py:1079` `G_VALUE`; test `🏗️authoring/🎬️clip/🧪️tests/🔣️.cases.json:486` `0.6` | CONTRADICTION. Heating `G_VALUE = SHARE_T + SHARE_IN` = 0,50 + 0,10 = 0,60 (`☀️solar-gains/🐍️.py:124`). The physics 0,50 equals only the direct-transmission share `SHARE_T`, labelled as the total g⊥. Proposed: physics line 2124 shows `g_⊥ = 0{,}60` (total), or relabels 0,50 as direct transmission τ with the 0,10 secondary share named. |
| F_C (sun-protection, blind) | heating `☀️solar-gains/🐍️.py:116` comment `F_C = 0,15` (and `FC_WINTER = 1.0` at `:131` for the unshaded case); cooling `system-sizing/🐍️.py:103` takes `F_C` from the solar factors; test `shading: 0.15` at `solar-radiation/🐍️.py:102` | Consistent value 0,15 for the blind; `FC_WINTER = 1.0` means no blind in winter: label it. Proposed: name the case in the scene text. |
| F_S (roof overhang), F_F (glazed fraction), F_W (incidence) | heating `☀️solar-gains/🐍️.py:121-130`: `FF_GLASS = 0.70`, `FW_INCIDENCE = 0.90`, `FS_WINTER = 0.80`; cooling test `glass_share: 0.7` (`🔣️.cases.json:485`) | Consistent (0,70 / 0,90 / 0,80). No change. |
| H_T and H_V (W/K) | running example `🏗️authoring/📏️running-example/🐍️.py:52-53` derived: H_T = 2300 W / 20 K = 115 W/K, H_V = 1500 W / 20 K = 75 W/K; demand `⚡️demand-power/🐍️.py:610, 613` `H_T = 115 W/K`, `H_V = 75 W/K` | Consistent. Proposed: keep, but state the reference Δθ (20 K, 0 °C) on the same slide. |
| design outdoor temperature and Δθ (heating) | running example `🏗️authoring/📏️running-example/🔣️.json` `winter.outdoorDesignC = 0` (Δθ = 20 K); demand peak `⚡️demand-power/🐍️.py:88` `DESIGN_DT = THETA_I - THETA_E` (= 32 K, θ_e = −12 °C, `🐍️.py:89` `PEAK_KW`); physics quiz `🧲️physics/❓️quiz/🔣️.json:1557` `−12 °C design temperature` (other building) | CONTRADICTION in the design basis: 0 °C (winter ventilation and transmission) vs −12 °C (design peak). Proposed: name both cases explicitly, `θ_e = 0 °C` for the running example and `θ_e = −12 °C` (DIN EN 12831-1, Hannover) for the design load, on the demand slide. |
| ρ (air density) and c_p | running example `🏗️authoring/📏️running-example/🔣️.json` 1.2 kg/m³, 1005 J/(kg·K); heat-flow `🌡️heat-flow/🐍️.py:107` 1.2 / 1005.0; natural `🪟️natural/🐍️.py:281` 1.2 / 1005.0; cooling `💧️transmission-humidity/🐍️.py:101` 1.2 / 0.34 / 2450; README `🌬️ventilation/🎬️clip/README.md:29` ρ ≈ 1,2 at 20 °C, 1,27 at 5 °C | Consistent (1,2 kg/m³ at 20 °C; 1,27 kg/m³ at 5 °C). Ok under F08. The concept file `📖️concepts/📝️.md:187` derives 0,34 from `1,2 · 1 005 / 3 600` with no units (see section 2). |
| c_Luft = 0,34 Wh/(m³·K) | consistent across ventilation, heating, cooling, demand, concepts (≈ 0,34 Wh/(m³·K)); test case `🏗️authoring/📝️narration/🧪️tests/🧮️tex/🔣️.cases.json:16` | Consistent. |
| latent ventilation loss Q̇_V,lat (winter, summer) | running example: 770 W (Δx = 4,3 g/kg, winter); 810 W (Δx = 4,5 g/kg, summer); vent-b reports and `🌬️ventilation/🎬️clip/README.md:50, 53` agree; concept `📖️concepts/📝️.md:189` 810 W | Consistent. |
| ventilation sensible loss (winter, summer) | 1500 W (Δθ = 20 K), 450 W (Δθ = 6 K), 300 W and 90 W with η = 0,8; cooling `💧️transmission-humidity/🐍️.py:367` 450 W; heating `🧱️convection/🐍️.py:144` 1500 W | Consistent. |
| degree days G_t20/15 | 3 500 Kd/a: demand `🔁️recap/🧪️tests/🧮️balance/🐍️.py:73`, heating `🧮️calculation/🐍️.py:427` (θ_i = 20 °C, θ_HG = 15 °C); notation differs: `G_t20/15` in README `📊️demand/🎬️clip/README.md:28`, concepts `📖️concepts/📝️.md:400`; `G_{t,20/15}` in demand `⚖️regulations/🐍️.py:1121`; `G_{\mathrm{t20/15}}` in heating `🧮️calculation/🐍️.py:568` | Value consistent. Notation inconsistent: propose `G_{t,20/15}` everywhere (convention). |
| F_Gt (heating-period factor) | demand `⚖️regulations/🐍️.py:1133-1134` `F_{Gt} = 0,024 · G_t`; concepts `📖️concepts/📝️.md:401` `F_Gt = 24 h/d · G_t` | Same quantity, two forms (0,024 kKh/(Kd·…) vs 24 h/d). Consistent after unit conversion. Proposed: one form, `F_{Gt} = 0{,}024\,\mathrm{kKh/(Kd)} \cdot G_t`. |
| solar load, heating (winter) vs cooling (summer) | heating winter `☀️solar-gains/🐍️.py:120` `G_WINTER = 400 W/m²`, 232 W after the blind and overhang (heat-b); cooling summer `solar-radiation/🐍️.py:102` `irradiance: 800.0`; cooling sizing 1510 W (system-sizing, report) | Different cases (winter 400 W/m² vs summer 800 W/m²); no contradiction. Label the case on each slide. |
| U-value wall (running example) | running example JSON `wall` 0,24 W/(m²·K); heat-a: `U = 0,24 W/(m²K)` shown, computed layer result 0,242 | Consistent (rounded). Proposed: show 0,24 and say 'gerundet'. |

Not exhaustively checked: U-values in every quiz line and every `H_T` derivation outside the running example; the cooling sizing numbers were only checked against the report.

## 2. Remaining unit violations

### 2a. Bare bracketed units (`[W]`, `[kWh]`, `[\mathrm…]`, `\left[`)

| file:line | text | proposed |
|---|---|---|
| `📖️concepts/📝️.md:238` | Wärmestrom durch Bauteil [W] | Wärmestrom durch Bauteil, in W |
| `📖️concepts/📝️.md:267` | [W/(m²·K)] | in W/(m²·K) |
| `📖️concepts/📝️.md:275` | [W/K] | in W/K |
| `📖️concepts/📝️.md:276` | Transmissionswärmestrom der Hülle [W] | Transmissionswärmestrom der Hülle, in W |
| `📖️concepts/📝️.md:456` | Personenlast [W] | Personenlast, in W |

No `\left[` or `\,[\mathrm` remains in shown scene code (grep of 364 source files). Quiz and README are clean for bracket units.

### 2b. Compound units inline in display math (`\mathrm{m^{3}/h}` etc. must become `\frac`)

Rule: `\mathrm{A/B}` in a formula → `\frac{\mathrm{A}}{\mathrm{B}}`. Running text, captions (`$…$` in narration) and quiz unit fields may keep `W/(m²·K)`.

| # | file:line | inline unit in display | proposed |
|---|---|---|---|
| 1 | `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:372` | `\mathrm{W/m^{2}}` → `\frac{\mathrm{W}}{\mathrm{m^{2}}}` |
| 2 | `❄️cooling/🎬️clip/💧️transmission-humidity/🐍️.py:436` | `\mathrm{W/K}` → `\frac{\mathrm{W}}{\mathrm{K}}` |
| 3 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:318` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 4 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:528` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 5 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:530` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 6 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:544` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 7 | `❄️cooling/🎬️clip/📐️system-sizing/🐍️.py:575` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}`; `\mathrm{m/s}` → `\frac{\mathrm{m}}{\mathrm{s}}` |
| 8 | `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:993` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 9 | `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:995` | `\mathrm{m/s}` → `\frac{\mathrm{m}}{\mathrm{s}}` |
| 10 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:362` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 11 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:366` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 12 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:375` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 13 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:376` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 14 | `🌬️ventilation/🎬️clip/❓️need/🐍️.py:386` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 15 | `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:377` | `\mathrm{Wh/(m^{3}K)}` → `\frac{\mathrm{Wh}}{\mathrm{m^{3}K}}` |
| 16 | `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:545` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 17 | `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:556` | `\mathrm{Wh/(m^{3}K)}` → `\frac{\mathrm{Wh}}{\mathrm{m^{3}K}}` |
| 18 | `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:725` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 19 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:799` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 20 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:962` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 21 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:971` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 22 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1005` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}`; `\mathrm{s/h}` → `\frac{\mathrm{s}}{\mathrm{h}}`; `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 23 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1026` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 24 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1027` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 25 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1038` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 26 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1039` | `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}` |
| 27 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1057` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 28 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1060` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 29 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1083` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 30 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1086` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 31 | `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1088` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 32 | `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:210` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 33 | `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:212` | `\mathrm{Wh/(m^{3}K)}` → `\frac{\mathrm{Wh}}{\mathrm{m^{3}K}}` |
| 34 | `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:323` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 35 | `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1124` | `\mathrm{m/s}` → `\frac{\mathrm{m}}{\mathrm{s}}` |
| 36 | `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1126` | `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 37 | `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1153` | `\mathrm{kg/m^{3}}` → `\frac{\mathrm{kg}}{\mathrm{m^{3}}}`; `\mathrm{m/s}` → `\frac{\mathrm{m}}{\mathrm{s}}` |
| 38 | `🌬️ventilation/🎬️clip/🪟️natural/🐍️.py:1162` | `\mathrm{m/s}` → `\frac{\mathrm{m}}{\mathrm{s}}`; `\mathrm{m^{3}/s}` → `\frac{\mathrm{m^{3}}}{\mathrm{s}}`; `\mathrm{m^{3}/h}` → `\frac{\mathrm{m^{3}}}{\mathrm{h}}` |
| 39 | `📊️demand/🎬️clip/♨️system-losses/🐍️.py:1022` | `\mathrm{kWh/(m^{2}\,a)}` → `\frac{\mathrm{kWh}}{\mathrm{m^{2}\,a}}` |
| 40 | `📊️demand/🎬️clip/♨️system-losses/🐍️.py:1024` | `\mathrm{kWh/(m^{2}\,a)}` → `\frac{\mathrm{kWh}}{\mathrm{m^{2}\,a}}` |
| 41 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:524` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 42 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:990` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 43 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1021` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 44 | `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1227` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 45 | `📊️demand/🎬️clip/🔁️recap/🐍️.py:813` | `\mathrm{W/(m^{2}K)}` → `\frac{\mathrm{W}}{\mathrm{m^{2}K}}` |
| 46 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1397` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 47 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1398` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 48 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1399` | `\mathrm{W/m}` → `\frac{\mathrm{W}}{\mathrm{m}}` |
| 49 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1400` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 50 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1401` | `\mathrm{kWh/d}` → `\frac{\mathrm{kWh}}{\mathrm{d}}` |
| 51 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1402` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 52 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1403` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 53 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1404` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 54 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1405` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 55 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1465` | `\mathrm{kWh/a}` → `\frac{\mathrm{kWh}}{\mathrm{a}}` |
| 56 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:634` | `\mathrm{W/(m\,K)}` → `\frac{\mathrm{W}}{\mathrm{m\,K}}` |
| 57 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1115` | `\mathrm{W/(m^{2}K)}` → `\frac{\mathrm{W}}{\mathrm{m^{2}K}}` |
| 58 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1330` | `\mathrm{W/K}` → `\frac{\mathrm{W}}{\mathrm{K}}` |
| 59 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1342` | `\mathrm{W/K}` → `\frac{\mathrm{W}}{\mathrm{K}}` |
| 60 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1361` | `\mathrm{W/K}` → `\frac{\mathrm{W}}{\mathrm{K}}` |
| 61 | `🔥️heating/🎬️clip/🧲️fundamentals/🐍️.py:1352` | `\mathrm{W/(m\,K)}` → `\frac{\mathrm{W}}{\mathrm{m\,K}}` |

61 display formulas. The owner's convention demands `\frac` in display math; each row is one formula line and may hold more than one unit.

### 2c. Numeric substitutions where a factor lacks a unit

Dimensionless factors (η, F_C, 0,70, 1,04 …) are acceptable. Listed are factors that carry a physical quantity without a unit, or a result without unit.

| file:line | text | status |
|---|---|---|
| `🌬️ventilation/❓️quiz/🔣️.json:898` | "en": "About 4.8 1/h: EN 16798-1 category II gives 28 × 25 m³/h + 60 m² × 2.5 m³/(h·m²) ≈ 860 m³/h for 180 m³… | `28 × 25 m³/h` (28 persons unit missing); proposed `28 Pers. × 25 m³/(h·Pers.)` |
| `🌬️ventilation/❓️quiz/🔣️.json:899` | "de": "Rund 4,8 1/h: EN 16798-1 Kategorie II ergibt 28 × 25 m³/h + 60 m² × 2,5 m³/(h·m²) ≈ 860 m³/h für 180 m… | same as 898 (German) |
| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:401` | "en": "Part 4: 30 m² of glazing × 0.49 × 0.15 × 0.60 × 800 W/m² ≈ 1060 W.", | `30 m² × 0.49 × 0.15 × 0.60 × 800 W/m²` (glazing factor 0,49 is an old value, see solar section, now 0,70); fix test fixture |
| `📖️concepts/📝️.md:101` | \| **Q̇_S = g · A · I** \| 0,5 · 2,0 m² · 500 W/m² = 500 W \| | 0,5 · 2,0 m² · 500 W/m² = 500 W: dimensionless g, ok; add unit on g if shown |
| `📖️concepts/📝️.md:187` | \| **c_Luft = ρ · c_p / 3 600** \| 1,2 · 1 005 / 3 600 ≈ 0,34 Wh/(m³·K) \| | missing units on 1,2 (kg/m³) and 1 005 (J/(kg·K)); proposed `1{,}2\,\mathrm{kg/m^3} \cdot 1005\,\mathrm{J/(kg\,K)} / 3600\,\mathrm{J/Wh} ≈ 0{,}34\,\mathrm{Wh/(m^3K)}` |
| `📖️concepts/📝️.md:188` | \| **Q̇_V,sens = V̇ · c_Luft · Δθ** \| sensible Lüftungsleistung; Winter 440 · 0,5 · 0,34 · 20 ≈ 1 500 W \| | 440 · 0,5 · 0,34 · 20 without units; proposed `440 m³ · 0,5 h⁻¹ · 0,34 Wh/(m³·K) · 20 K ≈ 1 500 W` |
| `📖️concepts/📝️.md:189` | \| **Q̇_V,lat = ρ · r · Δx · q_v,R** \| latente Last; 1,2 · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s ≈ 810 W \| | 1,2 · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s without units; proposed with each unit |
| `📖️concepts/📝️.md:308` | \| **Φ_V = V · n · c_Luft · Δθ** \| Lüftungswärmeverlust ohne WRG; 440 · 0,5 · 0,34 · 20 ≈ 1 500 W \| | 440 · 0,5 · 0,34 · 20 without units; same fix as line 188 |
| `🧲️physics/❓️quiz/🔣️.json:2270` | "en": "About 46 GW: electricity accounts for approximately 27% of France’s 1,496 TWh of final energy in 2023 … | 1,496 × 0.27 / 8,760 without TWh or h units; proposed `1{,}496\,\mathrm{TWh} \cdot 0{,}27 / 8760\,\mathrm{h} ≈ 46\,\mathrm{GW}` |
| `🧲️physics/❓️quiz/🔣️.json:2271` | "de": "Rund 46 GW: Strom macht näherungsweise 27 % von Frankreichs 1.496 TWh Endenergie 2023 aus (SDES). 1.49… | same as 2270 (German) |

Other numeric chains (concepts 101, 114-115 scales) were judged acceptable or are pure exponent notation.

### 2d. Thin space, upright and number/unit spacing

A complete pass is not done. The scan of 517 candidate lines (`s2-thin.txt`) mixes running text (`20 °C` with a space, which is correct in prose) and formulas. Clear TeX cases: `$…\mathrm{…}$` directly after a digit without `\,` (`1500\mathrm{W}`) must get `\,`. Units in on-screen labels should be upright (`\mathrm`), not italic, e.g. the axis label in `❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py:239` uses `\mathrm{W/m^{2}}` inside `tr(...)`, which is acceptable as a label.

## 3. Remaining spelled-out symbols and bare Unicode math letters

Narration (`(key, en, de)` clauses), labels, README and quiz: symbols must sit inside `$…$`, with real Unicode only as the symbol itself inside math.

### 3a. Bare Unicode Greek or Φ in README, concepts and quiz text (outside `$…$`)

| # | file:line | text (excerpt) | proposed |
|---|---|---|---|
| 1 | `🌬️ventilation/🎬️clip/README.md:28` | \| `V`, `n`, `V̇ = V·n` (= `q_v,R`) \| Kapitel 2 \| `Q̇_V = V̇·c_Luft·Δθ` (Φ as the generic heat flow, Q̇ as the load at t… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 2 | `🌬️ventilation/🎬️clip/README.md:29` | \| `c_Luft = ρ·c_p / 3600 ≈ 0,34 Wh/(m³·K)` (ρ ≈ 1,2 kg/m³ at 20 °C, 1,27 kg/m³ at 5 °C; the norm uses the fixed 0,34) \|… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 3 | `🌬️ventilation/🎬️clip/README.md:49` | Winter: Δθ = 20 K → Q̇_V,sens ≈ 1500 W   → mit WRG (η = 0,8) ≈ 300 W | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 4 | `🌬️ventilation/🎬️clip/README.md:50` | Δx = 4,3 g/kg → Q̇_V,lat ≈ 770 W (Plattentauscher gewinnt nichts zurück; | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 5 | `🌬️ventilation/🎬️clip/README.md:52` | Sommer: Δθ =  6 K → Q̇_V,sens ≈  450 W   → mit WRG (η = 0,8) ≈  90 W | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 6 | `🌬️ventilation/🎬️clip/README.md:53` | Δx = 4,5 g/kg → Q̇_V,lat ≈ 810 W (Plattentauscher gewinnt nichts zurück) | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 7 | `📖️concepts/📝️.md:62` | \| 3 \| Energie bleibt erhalten — jedes Watt wird Wärme \| E_ein = E_aus + ΔE_Speicher · Lampe 5 % Licht + 95 % Wärme → 10… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 8 | `📖️concepts/📝️.md:65` | \| 6 \| Thermische Masse — speichern und zeitversetzt abgeben \| Q = m · c · ΔT an 1 m² Betondecke · 24-h-Verlauf, Phasenv… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 9 | `📖️concepts/📝️.md:66` | \| 7 \| Sensible und latente Wärme \| T-Q-Diagramm Wasser · Sättigungskurve x_s(ϑ) · Taupunkt · Mensch 70 W sensibel + 30 … | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 10 | `📖️concepts/📝️.md:67` | \| 8 \| Zusammenfassung — was wir gelernt haben \| Sieben Zeilen, eine je Baustein: P = E / t · 1 kW · 1 h = 1 kWh · E_ein… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 11 | `📖️concepts/📝️.md:84` | \| **Thermische Masse** \| Wärmespeicher eines Bauteils, Q = m · c · ΔT \| 1 m² Beton, 10 cm, 2 K → 480 kJ ≈ 0,13 kWh \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 12 | `📖️concepts/📝️.md:95` | \| **Q = m · c · ΔT** \| sensible Wärme (Dusche, Betondecke, Wasser) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 13 | `📖️concepts/📝️.md:98` | \| **E_ein = E_aus + ΔE_Speicher** \| 1. Hauptsatz für einen Raum \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 14 | `📖️concepts/📝️.md:102` | \| **m = ρ · A · d** \| 2 400 kg/m³ · 1 m² · 0,10 m = 240 kg \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 15 | `📖️concepts/📝️.md:104` | \| **A₁ · v₁ = A₂ · v₂**, **p + ½ ρ v² = konstant** \| Kontinuität und Bernoulli \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 16 | `📖️concepts/📝️.md:166` | \| 4 · Natürliche Lüftung \| Fensterregeln · Stoß oder Kipp · Querlüftung · Auftrieb · Nachtlüftung · Grenzen \| Fenster a… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 17 | `📖️concepts/📝️.md:167` | \| 5 · Mechanische Lüftung und WRG \| Grundtypen · Wärmerückgewinnung · Anlagenvergleich · Kanalquerschnitt \| η_WRG = (θ_… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 18 | `📖️concepts/📝️.md:177` | \| **c_Luft** \| volumenbezogene Wärmekapazität, ρ · c_p / 3 600 \| ≈ 0,34 Wh/(m³·K) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 19 | `📖️concepts/📝️.md:178` | \| **Δθ** \| Temperaturdifferenz innen − außen, mit Vorzeichen \| Winter 20 K · Sommer −6 K (Gewinn) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 20 | `📖️concepts/📝️.md:179` | \| **Δx** \| Differenz der absoluten Feuchte \| 4,5 g/kg (15,0 gegen 10,5 g/kg) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 21 | `📖️concepts/📝️.md:180` | \| **η_WRG** \| Temperaturänderungsgrad der Wärmerückgewinnung \| 0,8 \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 22 | `📖️concepts/📝️.md:187` | \| **c_Luft = ρ · c_p / 3 600** \| 1,2 · 1 005 / 3 600 ≈ 0,34 Wh/(m³·K) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 23 | `📖️concepts/📝️.md:188` | \| **Q̇_V,sens = V̇ · c_Luft · Δθ** \| sensible Lüftungsleistung; Winter 440 · 0,5 · 0,34 · 20 ≈ 1 500 W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 24 | `📖️concepts/📝️.md:189` | \| **Q̇_V,lat = ρ · r · Δx · q_v,R** \| latente Last; 1,2 · 2 450 kJ/kg · 4,5 g/kg · 0,061 m³/s ≈ 810 W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 25 | `📖️concepts/📝️.md:190` | \| **Q̇_V,sens · (1 − η_WRG)** \| mit Wärmerückgewinnung; 1 500 → 300 W, 450 → 90 W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 26 | `📖️concepts/📝️.md:197` | \| **Heizung** \| Φ_V = Q̇_V,sens als Lüftungsterm der Heizlast (1 500 W, mit WRG 300 W); Stoß- statt Kipplüftung \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 27 | `📖️concepts/📝️.md:205` | - Ein **offenes Fenster** gewinnt nichts zurück (η = 0); erst die Zu-/Abluftanlage kann η_WRG nutzen. | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 28 | `📖️concepts/📝️.md:225` | \| **Δθ** \| Temperaturdifferenz — einziger Antrieb des Wärmestroms (warm → kalt) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 29 | `📖️concepts/📝️.md:228` | \| **λ (Lambda)** \| Wärmeleitfähigkeit des Materials [W/(m·K)] \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 30 | `📖️concepts/📝️.md:235` | \| **R = d / λ** \| Widerstand einer Schicht \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 31 | `📖️concepts/📝️.md:236` | \| **R_ges = R_si + Σ(d/λ) + R_se** \| Gesamtwiderstand inkl. Oberflächenwiderständen \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 32 | `📖️concepts/📝️.md:238` | \| **Q̇ = U · A · Δθ** \| Wärmestrom durch Bauteil [W] \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 33 | `📖️concepts/📝️.md:245` | - **Nur U ist eine Entwurfsentscheidung** — Dämmung ist der Hebel (A und Δθ sind gegeben). | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 34 | `📖️concepts/📝️.md:263` | \| **U-Wert je 1 m²** \| Watt durch genau 1 m² Bauteil bei 1 K — die Dicke steckt schon über R = d/λ im U-Wert \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 35 | `📖️concepts/📝️.md:267` | \| **ΔU_WB** \| pauschaler Wärmebrückenzuschlag auf die ganze Hüllfläche [W/(m²·K)] \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 36 | `📖️concepts/📝️.md:273` | \| **R = d / λ** \| pro Schicht \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 37 | `📖️concepts/📝️.md:276` | \| **Φ_T = Σ (U_i · A_i · Δθ_i)** \| Transmissionswärmestrom der Hülle [W] \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 38 | `📖️concepts/📝️.md:277` | \| **H_T = Σ U_i·A_i + Σ Ψ_k·l_k** \| mit Wärmebrücken (detailliert) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 39 | `📖️concepts/📝️.md:278` | \| **H_T = Σ U_i·A_i + ΔU_WB · A** \| mit Wärmebrücken (pauschal, ΔU_WB = 0,10 bzw. 0,05 nach DIN 4108 Beiblatt 2) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 40 | `📖️concepts/📝️.md:280` | ### Wattvergleich (25 m² Wand, Δθ = 20 K) | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 41 | `📖️concepts/📝️.md:302` | Die Lüftungswärmeverluste aus dem Lüftungsvideo (`2_ventilation/`) mit einer kurzen Erinnerung in die Heizlast einsetze… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 42 | `📖️concepts/📝️.md:308` | \| **Φ_V = V · n · c_Luft · Δθ** \| Lüftungswärmeverlust ohne WRG; 440 · 0,5 · 0,34 · 20 ≈ 1 500 W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 43 | `📖️concepts/📝️.md:309` | \| **Φ_V = V · n · (1 − η_WRG) · c_Luft · Δθ** \| mit Wärmerückgewinnung η_WRG = 0,8: ≈ 300 W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 44 | `📖️concepts/📝️.md:313` | - Φ_T ≈ 2 300 W plus Φ_V ≈ 1 500 W ergibt 3 800 W; mit WRG bleiben 2 600 W. | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 45 | `📖️concepts/📝️.md:314` | - Die Bausteine V, n, c_Luft, Δθ und η_WRG stammen aus dem Lüftungsvideo (Kapitel 2, 3 und 5). | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 46 | `📖️concepts/📝️.md:397` | \| **Q̇_T = Σ U · A · Δθ** \| Transmissionsverluste in W (Wärmestrom Φ als Last am Gebäude) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 47 | `📖️concepts/📝️.md:398` | \| **Q̇_V = n · V · ρ c_p · Δθ** \| Lüftungsverluste in W \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 48 | `📖️concepts/📝️.md:400` | \| **G_t20/15 = Σ (θ_i − θ_e)** \| Gradtagzahl in Kd/a: nur Tage mit θ_e < 15 °C (Heizgrenze), Raumtemperatur θ_i = 20 °C… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 49 | `📖️concepts/📝️.md:403` | \| **Q_h = Q_l − η · Q_g, Q_g = Q_S + Q_i** \| **Heizwärmebedarf** (DIN V 4108-6) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 50 | `📖️concepts/📝️.md:410` | \| **η** \| Ausnutzungsgrad der Gewinne — Anteil der Gewinnfläche unter der Verlustkurve; schwere Bauweise speichert den … | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 51 | `📖️concepts/📝️.md:492` | Sommerliche Transmissionslast (mit ΔT_eq für solare Aufheizung, Zeitverzögerung) und — als kurze Erinnerung, ohne Herle… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 52 | `📖️concepts/📝️.md:498` | \| **Q̇_T = U · A · ΔT_eq** \| Transmissionskühllast (opake Bauteile, Sommer) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 53 | `📖️concepts/📝️.md:505` | \| **ΔT_eq** \| Äquivalente Temperaturdifferenz (solare Oberflächenaufheizung) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 54 | `📖️concepts/📝️.md:510` | - Luftmenge, c_Luft, Δx und Rückgewinnung werden im Lüftungsvideo (Kapitel 2, 3, 5) erklärt; hier steht die Last neben … | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 55 | `📖️concepts/📝️.md:556` | \| **Q̇_V = ρ_a · c_p,a · Δθ · q_v,R** \| Volumenstrom-Kühlleistung \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 56 | `📖️concepts/📝️.md:557` | \| **q_v,R = Q̇_S,tr / (ρ_a · c_p,a · Δθ)** \| Erforderlicher Volumenstrom aus solarer Last (≈ 0,126 m³/s bei Q̇_S,tr ≈ 1… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 57 | `📖️concepts/📝️.md:595` | - Ein Passivhaus braucht Zu-/Abluft mit WRG; ein offenes Fenster hat η = 0 — die sommerliche Nachtlüftung bleibt als Er… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 58 | `📖️concepts/📝️.md:605` | \| Leistung \| P, Q̇, Φ \| W, kW \| Fluss (momentan) \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 59 | `📖️concepts/📝️.md:608` | \| Wärmeleitfähigkeit \| λ \| W/(m·K) \| Material \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 60 | `📖️concepts/📝️.md:612` | \| Temperaturdifferenz \| Δθ, ΔT \| K \| Treiber \| | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 61 | `🔥️heating/❓️quiz/🔣️.json:357` | "en": "0.28 W/(m²·K): e.g. masonry with about 12 cm of insulation (λ = 0.035 W/(m·K)). Source: GEG 2024, Annex 1.", | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 62 | `🔥️heating/❓️quiz/🔣️.json:358` | "de": "0,28 W/(m²·K): z. B. Mauerwerk mit rund 12 cm Dämmung (λ = 0,035 W/(m·K)). Quelle: GEG 2024, Anlage 1." | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 63 | `🔥️heating/❓️quiz/🔣️.json:379` | "en": "0.20 W/(m²·K): reference value for roofs and top-floor ceilings, roughly 17 cm of insulation (λ = 0.035 W/(m·K))… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 64 | `🔥️heating/❓️quiz/🔣️.json:380` | "de": "0,20 W/(m²·K): Referenzwert für Dächer und oberste Geschossdecken, rund 17 cm Dämmung (λ = 0,035 W/(m·K)). Quell… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 65 | `🔥️heating/❓️quiz/🔣️.json:423` | "en": "0.10 W/(m²·K): about 35 cm of insulation (λ = 0.035 W/(m·K), R ≈ 10 m²·K/W), typical for passive-house roofs. So… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |
| 66 | `🔥️heating/❓️quiz/🔣️.json:424` | "de": "0,10 W/(m²·K): rund 35 cm Dämmung (λ = 0,035 W/(m·K), R ≈ 10 m²·K/W), typisch für Passivhausdächer. Quelle: Pass… | wrap every symbol as `$…$`, e.g. `$\Delta\theta$`, `$\dot{Q}_V$`, `$\lambda$` |

66 lines. The concepts file accounts for most of them and is written as Unicode tables, so it needs a full rewrite into `$…$` form, not line edits.

### 3b. Spelled-out symbol names in narration or fixtures that are spoken or shown

| file:line | text | proposed |
|---|---|---|
| `🎨️visuals/🧪️tests/🔣️integration.json:369` (de) | "Q Punkt V ist gleich Rho a mal c p a mal Delta Theta mal q v R." | "$\dot{Q}_V$ ist gleich $\rho \cdot c_p \cdot \Delta\theta \cdot q_{v,R}$." (fixture mirrors shipped narration: fix) |
| `🎨️visuals/🧪️tests/🔣️integration.json:380` (de) | "Weil Q Punkt V gleich Q Punkt S t r ist, …" | "Weil $\dot{Q}_V = \dot{Q}_{S,tr}$ ist, …" |
| `🎨️visuals/🧪️tests/🔣️integration.json:382` (en) | "Because Q-dot V equals Q-dot S,tr, …" | "Because $\dot{Q}_V = \dot{Q}_{S,tr}$, …" |
| `📊️demand/🎬️clip/🧪️tests/🔗️chain/🔣️.json:8` | forbidden list (`"Delta Theta"`, `"Delta-Theta"`) | keep: guard fixture |
| `🏗️authoring/📝️narration/🧪️tests/🛡️guard/🐍️.py:98-99` | "Die Temperaturdifferenz Delta-Theta hier" | keep: guard must reject it |
| `🏗️authoring/🔊️speech/🧪️tests/🗣️lexicon/🔣️.cases.json:56, 64` and `🏗️authoring/🔊️speech/🗣️lexicon/🔣️.json:166-171` | "Delta-Theta", "Q-Punkt V" as speech input and the lexicon's spoken names | keep: these are the speech layer's input and output mapping, not shown text |

The scan `s3-spelled.txt` (159 hits) matched mostly TeX `\Delta` and `\Phi` commands (word-boundary false positives), not words. No spelled-out name was found in on-screen `Text(`, `body_text(` or `tr(` labels outside `$…$`.

### 3c. Bare Unicode letters in code strings of scenes

The scan `s3-uni.txt` (43 hits) lists Greek letters in quoted strings outside `$…$`. Entries are docstrings (not shown), speech lexicon keys, the TeX-to-speech map `🏗️authoring/📝️narration/🧮️tex/🔣️.json`, and the quiz and README lines already listed in 3a. Not classified one by one: the docstrings do not reach the screen.

## 4. On-screen references and citation strings

The reference audit ran as a sub-task and is embedded here in full: `audit2/sec4-references.md` holds the occurrence index (d1-d47 with every file:line). Summary: 47 distinct references; VERIFIED 2, EDITION-ONLY 12, UNVERIFIED 17, WRONG 16.

### 4a. Reference table (sub-audit)

Distinct references: 47. Verdicts: VERIFIED 2, EDITION-ONLY 12, UNVERIFIED 17, WRONG 16.

| # | Displayed string | Occurrences (n, full list in d) | Edition/year | Locator | Report claims verified | Web result | Source URL | Verdict | Proposed replacement |
|---|---|---|---|---|---|---|---|---|---|
| 1 | DIN 4108-2:2013-02 | 9 (d1) | Y (2013-02) | N | cooling: WebSearch 'Abschn. 8, Tab. 7' of 2013-02 (claimed verified) | 2013-02 superseded: DIN lists DIN 4108-2:2026-05 as current/published (2026-05 TOC preview shows section 8) | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **WRONG** | DIN 4108-2:2026-05, Abschn. 8 (Sommerlicher Wärmeschutz) |
| 2 | DIN 4108-2 (no edition) | 11 (d2) | N | N | demand, cooling, heating (claimed, no edition) | current edition DIN 4108-2:2026-05 | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **WRONG** | DIN 4108-2:2026-05 |
| 3 | DIN 4108-2:2026-05 | 2 (d3) | Y | partial (Abschn. 8 in TOC preview; Tab. 7 not verified) | demand: 'final vs pre-published unconfirmed' (not verified by demand) | edition confirmed as published/current; Tab. 7 F_C values not read | https://www.dinmedia.de/en/standard/din-4108-2/401017482 | **EDITION-ONLY** | DIN 4108-2:2026-05, Abschn. 8 |
| 4 | DIN V 4108-6:2003-06 (and bare 'DIN V 4108-6') | 51 (d4) | Y (2003-06) / N | claimed 3.2, 5.5, 6.1-6.5, Anh. A, B.4, D.2, D.5 (not checkable: standard withdrawn) | heat-b (TOC only), demand ('Verified locators' 3.2, 5.5, 6.1-6.5, Anh. A, D.2, D.5) | WITHDRAWN (DIN listing); DIN recommends DIN V 18599 parts and DIN/TS 18599-12:2021-04; a corrigendum is also withdrawn; DIN 4108-6:2024 not found | https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447 | **WRONG** | DIN/TS 18599-12:2021-04 (successor per DIN; clause numbers must be re-checked, do not keep 3.2/5.5/6.x) |
| 5 | DIN V 18599-2 (2016-10 / 2018-09 / no part edition) | 5 (d5) | Y (both 2016-10 and 2018-09 used) | claimed 5.5.3 (demand, 2018-09) | demand (2018-09 5.5.3 'verified'), cooling (2016-10 'Interne Wärmequellen') | DIN listing marks DIN V 18599-2 as WITHDRAWN; successor for part 2 not found (2025-10 TS parts exist for -4 and -10) | no DIN URL retrieved for part 2 | **WRONG** | no verified replacement; cite the DIN/TS 18599-2 part of the 2025-10 series only after a DIN check |
| 6 | DIN V 18599-1:2018-09 | 7 (d6) | Y | N | none (not claimed by reports) | not checked individually; DIN/TS 18599-1:2025-10 is named in demand files | - | **UNVERIFIED** | check status at DIN; if superseded use DIN/TS 18599-1:2025-10 |
| 7 | DIN V 18599 / DIN V 18599:2018-09 (no part) | 19 (d7) | mixed (bare or 2018-09) | N | demand (energy balance text) | series superseded in part by DIN/TS 18599 2025-10 (confirmed for parts -4, -10) | - | **UNVERIFIED** | name the part and edition explicitly, e.g. DIN V 18599-1:2018-09 or its 2025-10 successor |
| 8 | DIN V 18599-5:2018-09 | 7 (d8) | Y | N | demand, heating (not claimed verified) | not verified; cited as a recommended successor of withdrawn DIN V 4701-10 | https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | **UNVERIFIED** | verify status at DIN before use |
| 9 | DIN V 18599-10:2018-09 (internal heat sources) | 9 (d9) | Y (2018-09) | N | heating/demand (not claimed verified) | replaced by DIN/TS 18599-10:2025-10 (DIN listing of both editions) | https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970 | **WRONG** | DIN/TS 18599-10:2025-10 (Vornorm, replaces DIN V 18599-10:2018-09) |
| 10 | DIN/TS 18599 parts 2025-10 | 4 (d10) | Y (2025-10) | N | demand (2025-10 notes) | DIN/TS 18599-4:2025-10 and -10:2025-10 confirmed; -1 and -2 mapping not confirmed | https://www.dinmedia.de/en/pre-standard/din-ts-18599-10/389788970 | **UNVERIFIED** | keep only parts confirmed at DIN |
| 11 | DIN EN 12831-1 (no edition) | 34 (d11) | N | claimed Abschn. 6 (demand) | demand: 'Verified locators: DIN EN 12831-1:2017-09 Abschn. 6' (only with edition) | edition DIN EN 12831-1:2017-09 current; national annex DIN/TS 12831-1:2020-04 (secondary source) | https://www.dinmedia.de/de/norm/din-en-12831-1/261292587 | **WRONG** | DIN EN 12831-1:2017-09 |
| 12 | DIN EN 12831-1:2017-09 | 7 (d12) | Y | claimed Abschn. 6 (not verified) | demand (Abschn. 6 claimed verified) | edition confirmed (DIN); Abschn. 6 not read | https://www.dinmedia.de/de/norm/din-en-12831-1/261292587 | **EDITION-ONLY** | DIN EN 12831-1:2017-09 |
| 13 | DIN/TS 12831-1:2020-04 (national annex) | 2 (d13) | Y (2020-04) | N | demand (named) | existence via Haustechnikdialog snippet; DIN page not opened | https://www.haustechnikdialog.de/SHKwissen/3109/DIN-TS-12831-1 | **EDITION-ONLY** | DIN/TS 12831-1:2020-04 (national annex to DIN EN 12831-1:2017-09) |
| 14 | DIN EN ISO 13789:2017-10 | 2 (d14) | Y (2017-10 claimed) | N | conduction file (not verified by heat-b) | no 2017-10 edition found; DIN lists DIN EN ISO 13789:2018-04 (German version of ISO 13789:2017, replaces 2008-04) | https://www.dinmedia.de/en/standard/en-iso-13789/277780736 | **WRONG** | DIN EN ISO 13789:2018-04 |
| 15 | DIN EN ISO 6946:2018-03 (and bare) | 16 (d15) | Y (2018-03) | claimed Tab. 7 (secondary sources only; not verified) | heat-a (edition and Tab. 7 'confirmed via secondary sources only') | 2018-03 current, Berichtigung 1:2023-04; Tab. 7 not verified | https://www.dinmedia.de/de/norm/din-en-iso-6946/255315365 | **EDITION-ONLY** | DIN EN ISO 6946:2018-03 (drop 'Tab. 7' unless checked) |
| 16 | DIN 4108 Beiblatt 2:2019-06 (and bare) | 4 (d16) | Y (2019-06) | N | heat-a (edition 'confirmed via secondary sources') | 2019-06 current; Berichtigung 1:2025-02 exists (CD-ROM replacement) | https://www.dinmedia.de/en/technical-rule/din-4108-beiblatt-2/305990077 | **EDITION-ONLY** | DIN 4108 Beiblatt 2:2019-06 (Berichtigung 1:2025-02 beachten) |
| 17 | DIN EN 16798-1:2022-03 (and bare) | 6 (d17) | Y (2022-03) | claimed Anh. B.2 / Tab. B.2 (cooling); Tab. B.9 not cited (vent-a) | cooling (Anh. B.2 'verified') | 2022-03 current (replaces 2021-04); Anhang B is informative criteria; table numbers not read | https://www.dinmedia.de/de/norm/din-en-16798-1/349622591 | **EDITION-ONLY** | DIN EN 16798-1:2022-03, Anh. B (drop 'Tab. B.2' unless checked) |
| 18 | DIN EN 16798-3 (no edition) | 2 (d18) | N | N | cooling, ventilation (no edition) | DIN EN 16798-3:2017-11 withdrawn; successor DIN EN 16798-3:2026-10 per DIN timeline (some old pages still show 2017-11 as current) | https://www.dinmedia.de/en/standard/din-en-16798-3/390958579 | **WRONG** | DIN EN 16798-3:2026-10 (check clause numbers) |
| 19 | DIN EN 16798-7:2017-11 | 3 (d19) | Y (2017-11) | N (procedure 1/2 exists; formulas not read) | ventilation natural (2017-11 'verified'; C_d = 0,6 not from a clause) | current; replaces DIN EN 15242:2007-09 | https://www.dinmedia.de/en/standard/din-en-16798-7/258797864 | **EDITION-ONLY** | DIN EN 16798-7:2017-11 (C_d = 0,6 is an assumption, cite as such) |
| 20 | DIN EN 410 (no edition) | 6 (d20) | N | N | physics, concepts | current edition DIN EN 410:2011-04 | https://www.dinmedia.de/de/standard/din-en-410/138424295 | **WRONG** | DIN EN 410:2011-04 |
| 21 | DIN EN 14825 (no edition) | 5 (d21) | N | N (SCOP keyword) | physics, concepts | current DIN EN 14825:2023-10 (German version of EN 14825:2022); A1 draft only | https://www.dinmedia.de/de/norm/din-en-14825/343036621 | **WRONG** | DIN EN 14825:2023-10 (SCOP) |
| 22 | DIN EN ISO 13786 (no edition) | 4 (d22) | N | N | physics | current DIN EN ISO 13786:2018-04 (replaces 2008-04) | https://www.dinmedia.de/de/norm/din-en-iso-13786/255912584 | **WRONG** | DIN EN ISO 13786:2018-04 |
| 23 | DIN EN 14511 | 2 (d23) | N | N | concepts (not verified) | not checked | - | **UNVERIFIED** | check edition or drop |
| 24 | DIN EN 308 | 1 (d24) | N | N | ventilation mechanical (not verified) | not checked | - | **UNVERIFIED** | check edition or drop |
| 25 | VDI 4650 Blatt 1:2024-02 (and bare 'VDI 4650') | 10 (d25) | Y (2024-02) / N (bare) | Y (JAZ term in title) | physics (2024-02 verified, Jahresarbeitszahl) | VDI 4650 Blatt 1:2024-02 is the current edition; title 'Berechnung der Jahresarbeitszahl von Wärmepumpenanlagen' | https://vdi.de/en/home/vdi-standards/details/vdi-4650-blatt-1-calculation-of-the-seasonal-coefficient-of-performance-of-heat-pumps-electric-heat-pumps-for-space-heating-and-domestic-hot-water | **VERIFIED** | VDI 4650 Blatt 1:2024-02 (unify bare variants) |
| 26 | VDI 2078:2015-06 (and bare) | 30 (d26) | Y (2015-06) / N (bare) | partial (keywords; 'Kühllast' in title; 'innere Wärmequellen' not verified) | cooling/heating (2015-06 'verified', keyword locators only) | 2015-06 current; no successor or 2027 revision found (the README claim of a 2027 revision is UNVERIFIED) | https://vdi.de/en/home/vdi-standards/details/vdi-2078-calculation-of-thermal-loads-and-room-temperatures-design-cooling-load-and-annual-simulation | **EDITION-ONLY** | VDI 2078:2015-06 (remove 'revision announced for 2027' unless verified) |
| 27 | VDI 4645:2023-04 | 2 (d27) | Y (2023-04) | N | demand (2023-04 'verified') | 2023-04 issued; a draft VDI 4645:2026-03 was seen; current status not confirmed | https://www.dinmedia.de/de/technische-regel-entwurf/vdi-4645/399378368 | **UNVERIFIED** | check status on vdi.de before citing |
| 28 | VDI 6022 Blatt 1 (no edition) | 1 (d28) | N | N | cooling (no edition) | current VDI 6022 Blatt 1:2018-01 (Berichtigung 2023-04) | https://vdi.de/richtlinien/details/vdi-6022-blatt-1-ventilation-and-indoor-air-quality-hygiene-requirements-for-ventilation-and-air-conditioning-systems-and-units-vdi-ventilation-code-of-practice-1 | **WRONG** | VDI 6022 Blatt 1:2018-01 |
| 29 | VDI 3807 Blatt 1 (no edition) | 4 (d29) | N | N (G20/15 not verified) | heating (G20/15 'secondary sources only', VDI 3807 not read) | current VDI 3807 Blatt 1:2013-06 (replaces 2007-03) | https://vdi.de/en/home/vdi-standards/details/vdi-3807-blatt-1-verbrauchskennwerte-fuer-gebaeude-grundlagen | **WRONG** | VDI 3807 Blatt 1:2013-06 (G20/15 locator still to be checked) |
| 30 | VDI 2052 | 2 (d30) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 31 | VDI 2087 | 1 (d31) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 32 | DIN 1946-4:2018-09 (and bare) | 9 (d32) | Y (2018-09) / N (bare) | N (values 60 m³/(h·m²), 20 1/h not verified) | cooling (2018-09 'verified') | 2018-09 current (title 'Raumlufttechnische Anlagen in Gebäuden und Räumen des Gesundheitswesens'); amendment A1:2025-11 exists | https://6sku3uaup9f9.din.de/en/standard/din-1946-4/294438653 | **EDITION-ONLY** | DIN 1946-4:2018-09 (check A1:2025-11) |
| 33 | DIN 1946-6 (no edition) | 15 (d33) | N | N | ventilation (many, no edition); vent-a claims Pettenkofer link | edition not confirmed; 2009-05 was the prior edition, a 2017 draft was announced; 2019-12 claim not confirmed | - | **UNVERIFIED** | confirm current edition at Beuth before adding it (e.g. DIN 1946-6:YYYY-MM) |
| 34 | DIN 1946-7 | 2 (d34) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 35 | DIN 18032-1 | 2 (d35) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 36 | DIN 18380:2019-09 | 1 (d36) | Y (2019-09) | N | none | not checked | - | **UNVERIFIED** | check edition |
| 37 | DIN 51603-1 | 4 (d37) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 38 | DIN V 4701-10:2003-08 | 3 (d38) | Y (2003-08) | N | demand (heat-related text, not verified) | WITHDRAWN (DIN listing); DIN recommends DIN V 18599-5/-6/-8:2018-09 (not a formal replacement) | https://dinmedia.de/en/pre-standard/din-v-4701-10/64443135 | **WRONG** | drop, or cite DIN V 18599-5:2018-09 / -6 / -8 as the recommended successors |
| 39 | GEG / GModG (law) | 85 (d39) | Y (GEG 2024 used) | N (paragraph numbers not verified) | demand (GEG §15/§20 'verified' on gesetze-im-internet.de; the GModG rename was flagged for checking) | GModG promulgated 28.07.2026 (BGBl. I Nr. 226) and in force 29.07.2026, renaming the GEG (see section c) | https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284 | **WRONG** | Gebäudemodernisierungsgesetz (GModG, vormals GEG) - paragraph numbers to be checked against the new text |
| 40 | Bohne, Fachbuch Gebäudetechnik, 13. Aufl. 2026, ISBN 978-3-658-51570-6 | 1 (d40) | Y (13. Aufl. 2026) | N | physics (13th ed. 2026, ISBN 978-3-658-51570-6 'verified' via baunetzwissen) | ISBN 978-3-658-51570-6 belongs to Dirk Bohne / Marius Bohne, Springer Vieweg 2026, 13th ed.; the on-screen title must match this book's title | https://www.baunetzwissen.de/gebaeudetechnik/tipps/buecher/technischer-ausbau-von-gebaeuden-4347729 | **EDITION-ONLY** | Bohne, D.; Bohne, M.: Fachbuch Gebäudetechnik, 13. Aufl., Springer Vieweg 2026, ISBN 978-3-658-51570-6 |
| 41 | Willems / Schild / Stricker, 8. Aufl. 2024, ISBN 978-3-658-44973-5 | 1 (d41) | Y (8. Aufl. 2024 claimed) | N | physics (ISBN 978-3-658-44973-5 'verified') | 8. Aufl. 2024 of 'Praxisbeispiele Bauphysik' exists with ISBN 978-3-658-44971-1 (9783658449711); the printed ISBN 978-3-658-44973-5 was not confirmed | https://www.jpc.de/jpcng/books/detail/-/art/wolfgang-m-willems-praxisbeispiele-bauphysik/hnum/11834057 | **WRONG** | Willems, W. M.; Schild, K.; Stricker, D.: Praxisbeispiele Bauphysik, 8. Aufl., Springer 2024, ISBN 978-3-658-44971-1 (ISBN to confirm at Springer) |
| 42 | Maas 1995 | 4 (d42) | Y (1995, Diss. Univ. Gesamthochschule Kassel) | Y (Kap. 8.2 'Luftwechsel', S. 74; values 0,56-1,38 1/h at 58,5 m³ room) | vent-b (ch. 8, 0,56-1,38 1/h, pages ~74-75 estimated) | dissertation 'Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung', eingereicht 27.06.1995; Kap. 8 'Luftwechsel und Lüftungswärmeverluste in der Heizzeit' S. 73-81; S. 74 gives 0,56 bis 1,38 1/h for a permanently tilted window (Heizgrenztemperatur 15 °C) | https://www.uni-kassel.de/fb6/bpy/de/forschung/abgeschlprojekte/pdfs/maas_diss.pdf | **VERIFIED** | Maas, A. (1995): Experimentelle Quantifizierung des Luftwechsels bei Fensterlüftung, Diss. Univ. Gesamthochschule Kassel, Kap. 8.2, S. 74 |
| 43 | Pettenkofer 1858 (Ueber den Luftwechsel in Wohngebäuden) | 3 (d43) | Y (1858) | N | vent-a (1858 cited; no locator) | title page: M. v. Pettenkofer, Ueber den Luftwechsel in Wohngebäuden, München, Cotta, 1858 (one bibliography says Brunswick; title page says Munich) | https://lessing.ub.uni-duesseldorf.de/download/pdf/2385693.pdf | **EDITION-ONLY** | M. v. Pettenkofer: Ueber den Luftwechsel in Wohngebäuden, München: Cotta, 1858 |
| 44 | Bundesgesundheitsbl. 51 (2008) 1358-1369 (Ad-hoc-AG Innenraumrichtwerte) | 3 (d44) | Y (2008, vol. 51) | N (article title confirmed; 1000 ppm value in the text not read) | vent-a (confirmed by web search) | 'Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft', DOI 10.1007/s00103-008-0707-2, published 29.11.2008; pages 1358-1369 | https://link.springer.com/content/pdf/10.1007/s00103-008-0707-2.pdf | **EDITION-ONLY** | Ad-hoc-AG Innenraumrichtwerte: Gesundheitliche Bewertung von Kohlendioxid in der Innenraumluft, Bundesgesundheitsbl. 51 (2008) 1358-1369, DOI 10.1007/s00103-008-0707-2 |
| 45 | IWU Deutsche Wohngebäudetypologie 2015 | 62 (d45) | Y (2015 claimed) | N | demand (no verification recorded) | not checked in this audit | - | **UNVERIFIED** | check the edition and cite the table/typology code |
| 46 | IEST-RP-CC012 | 4 (d46) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |
| 47 | ISO 14644-4 | 2 (d47) | N | N | none | not checked | - | **UNVERIFIED** | check edition or drop |

### 4b. Conflicting editions across files

- DIN 4108-2: 2013-02 in cooling (quiz, README, solar, heating-comparison, summer-strategy), heating conduction and ventilation natural; 2026-05 in demand (README, regulations). The 2013-02 edition is superseded.
- DIN V 18599-2: 2016-10 (cooling heating-comparison, README) vs 2018-09 (demand regulations, recap). Both are withdrawn per DIN; part 2 successor not confirmed.
- DIN V 18599-10: 2018-09 in heating/demand; DIN/TS 18599-10:2025-10 replaces it (confirmed).
- DIN EN 12831-1: bare (about 20 files) vs DIN EN 12831-1:2017-09 (demand) vs DIN/TS 12831-1:2020-04 (national annex, named in demand).
- DIN EN ISO 13789: 2017-10 in conduction vs 2018-04 current.
- DIN EN 16798-3: bare (cooling, ventilation) vs withdrawn 2017-11 vs 2026-10 successor.
- DIN 4108 Beiblatt 2: bare in heating vs 2019-06 in concepts/conduction.
- VDI 4650 Blatt 1: bare 'VDI 4650' and '-1:2024-02' variants vs 'Blatt 1:2024-02'.
- DIN V 4108-6:2003-06 (withdrawn) vs bare in demand/heating; the 'F_C' / 'Annex B.4' citation depends on it.
- GEG 2024 (cooling quiz, demand README) vs GModG 2026 (demand chips, GEG renamed on 29.07.2026): name in use is mixed.
- Willems: printed ISBN 978-3-658-44973-5 vs found 978-3-658-44971-1 (same 8th ed. 2024 'Praxisbeispiele Bauphysik').
- Bohne: ISBN 978-3-658-51570-6 is verified for the 13th ed. 2026 (Dirk and Marius Bohne, Springer Vieweg); the card title must match.

### 4c. GEG / GModG status

- Verified: the Gebäudemodernisierungsgesetz (GModG) was promulgated in the BGBl. on 28.07.2026 (BGBl. I Nr. 226) and the Gebäudeenergiegesetz is renamed effective 29.07.2026. Sources: https://cci-dialog.de/wp-content/uploads/2026/08/Bundesgesetz-zur-Ueberfuehrung-des-GEG-ins-GModG.pdf ; https://www.bundesregierung.de/breg-de/aktuelles/neues-gebaeudemodernisierungsgesetz-2430284 ; https://www.baunetzwissen.de/drucken/gebaeudemodernisierungsgesetz-gmodg-3559433
- Conflicting vote dates in secondary sources (Bundestag 9, 10 or 14 July 2026); the promulgation date 28.07.2026 is consistent across sources. Do not cite the vote dates.
- Not verified: the paragraph numbers (§14, §15, §20, §85, Anlage 4, Anlage 10) under the new text; the draft amendment that moves the DIN 4108-2 reference to 2026-05 (not confirmed in force).
- Required change: where the files say 'GEG 2024' as the current law, write 'Gebäudemodernisierungsgesetz (GModG, vormals GEG, Stand 2026-07-29)'.


### 4d. My spot checks (independent of the sub-audit)

- DIN V 4108-6:2003-06 is withdrawn. Withdrawal dated 2021-07 (baunormenlexikon); DIN Media names DIN/TS 18599-12:2021-04 as replacement; sources disagree whether it was replaced directly. Cited ~51 times in heating and demand: needs a rewrite to DIN/TS 18599-12 or a note 'zurückgezogen'. https://www.baunormenlexikon.de/norm/din-v-4108-6/bc30ac26-55c3-44a2-ac9e-782a9e61931f ; https://www.dinmedia.de/en/pre-standard/din-v-4108-6/63939447
- DIN 4108-2:2026-05 is the current edition and replaces 2013-02 (DIN Media). But the GModG § 11 still cites the 2013-02 edition according to two secondary sources, so 'DIN 4108-2:2013-02' is correct only as the legal reference. https://www.dinmedia.de/en/standard/din-4108-2/401017482 ; https://www.legat-living.de/de/ueber-uns/magazin/magazin-leser/sommerlicher-waermeschutz-was-die-din-4108-2-verlangt-und-was-sie-nicht-regelt.html
- GModG: promulgated 28.07.2026, in force 29.07.2026 (sub-audit, confirmed by sources listed in section (c)). Files still write 'GEG 2024' as the current law.

## 5. Parentheses drawn as brackets

### 5a. Verdict

Not settled. The font setup does not remap `(` to `[`: no `replace`, `translate` or `maketrans` on parentheses exists in `energy/`, and the fallback in `🏗️authoring/🔤️fonts/🐍️.py:138-161` only substitutes characters the face lacks. My own glyph render of the body face shows that `(` and `[` are visibly different in this face: `(` is a rounded arc, `[` a square bracket. Evidence: `audit2/paren-glyph-check.png` (rendered from `🧰️framework/.../🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf`, which is `FONT_SANS` = Anta from `🎨️theme/🐍️.py:13`).

The sub-audit's claim that a rendered frame shows `[2024]` for `(2024)` (`🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png`, `audit2/crop-refs-willems.png`) is not confirmed at the resolution inspected: the 2560 px contact sheet is too small to separate the glyphs reliably. The source at `🧲️fundamentals/🐍️.py:3185, 3188` uses real parentheses. So the observed bracket look is probably a rendering effect of the small rounded face, not a substitution. Next step: re-render `Beat8` at full resolution and crop the citation line.

Test gap confirmed: `🏗️authoring/🔤️fonts/🧪️tests/🔣️.cases.json` has no `(` or `)` in `required_symbols` or `rendered_glyphs`, so the outline test never checks them. Fix: add `()[]` to both lists.

### 5b. Parenthesis strings in on-screen text (sub-audit count, per folder)

| Folder | Count | Call paths | Examples (file:line, string) |
|---|---|---|---|
| 🧲️physics | 15 | `tr` 12; citation/source literals 3 | `🧲️fundamentals/🐍️.py:1968` "Luft-WP (JAZ)"; `:735` "Shower (instantaneous heater)"; `:3185` "Willems WM, Schild K, Stricker D (2024)"; `:3188` "Bohne D, Bohne M (2026)"; `:3191` "VDI 4650 Blatt 1 (2024-02) · DIN EN 14825 · DIN EN 410" |
| 🌬️ventilation | 15 | `tr` 10; `legend` 4; 1 unclear | `⚙️mechanical/🐍️.py:821` "Zuluft (16 °C)"; `:834` "(Speichern)"; `:801` "(0 °C)"; `:835` "(" and ")"; `🪟️natural/🐍️.py:996` "Treppenhaus oder Schacht (Kamin) …" (unclear: narration or caption) |
| 🔥️heating | 33 | `tr` 24; 9 unclear | `👥️internal-gains/🐍️.py:744` "Spezifische Lastdichte (DIN V 18599-10)"; `🧮️calculation/🐍️.py:397` "Gesamtverlustleistung (DIN V 4108-6)"; `🧱️conduction/🐍️.py:126` "Gedämmt (12 cm)", "Ungedämmt (Ziegel 1960)" |
| ❄️cooling | 7 | `Text` 2 (`" ("`, `")"` in one label); `tr` 2; 3 unclear | `👥️internal-gains/🐍️.py:491` `Text(" (")` and `Text(")")`; `🌤️summer-strategy/🐍️.py:305` "Sorptionsgestützte Kühlung (DEC)"; `:165`, `:487`, `:493` narration-style tuples (unclear) |
| 📊️demand | 30 | `tr` 22; 8 unclear | `🏭️primary-energy/🐍️.py:426` "GEG seit 29.07.2026: Gebäudemodernisierungsgesetz (GModG)"; `:627` "GEG (now GModG) § 22 (1), …"; `🏷️certificate/🐍️.py:330` "… certificate (§ 80)." (unclear) |
| 🏗️authoring (visuals) | 2 | f-string `Text` labels | `🎨️visuals/🐍️.py:334` and `:338` `f"≈ 1× {name} ({unit_text(…)})"` |
| **Total** | **102** | | Of which about 14 are `Text`/`tr`/`legend` literals with a confirmed on-screen call; the rest are unclear narration/caption tuples and need a per-scene check |

Frames checked: `🗑️generated/physics-out/fr/Beat8_Zusammenfassung.png` (citation block), `audit2/crop-refs-willems.png`, `audit2/paren-glyph-check.png` (my render).

## 6. German wording

### 6a. `Kühlung` where the technical system is meant (should be `Kühltechnik`)

| file:line | text | proposed |
|---|---|---|
| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:213` | Basis für Heizung und Kühlung | Basis für Heizung und Kühltechnik |
| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:216` | Basis für Heizung und Kühlung | Basis für Heizung und Kühltechnik |
| `🌬️ventilation/🎬️clip/❓️need/🐍️.py:239` | …neben Heizung und Kühlung. | …neben Heizung und Kühltechnik. |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:779` | die Kühlung beide Sommeranteile als Lüftungslast | die Kühltechnik beide Sommeranteile als Lüftungslast |
| `🌬️ventilation/🎬️clip/🌡️heat-flow/🐍️.py:862` | → Kühlung: (chip) | → Kühltechnik: (chip) |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:914` | die Kühlung $q_{v,\mathrm{R}}$ | die Kühltechnik $q_{v,\mathrm{R}}$ |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1027` | Kühlung · Sommer (card) | Kühltechnik · Sommer |
| `🌬️ventilation/🎬️clip/📦️volume/🐍️.py:1095` | → Kühlung: (chip) | → Kühltechnik: |
| `🌬️ventilation/🎬️clip/🗺️roadmap/🐍️.py:53` | die Kühlung den Sommerwert | die Kühltechnik den Sommerwert |
| `🌬️ventilation/🎬️clip/🗺️roadmap/🐍️.py:68` | → Kühlung: (chip) | → Kühltechnik: |
| `🌬️ventilation/🎬️clip/🤝️handoff/🐍️.py:1 (docstring)` | Übergabe an Heizung und Kühlung | Übergabe an Heizung und Kühltechnik (align with scene title) |
| `📊️demand/🎬️clip/🗺️roadmap/🐍️.py:63` | ← Cooling: Q_c → Kühlung: | ← Kühltechnik: Q_c |
| `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:3136` | Heizung und Kühlung | Heizung und Kühltechnik |
| `❄️cooling/❓️quiz/🔣️.json:859` | durchgehende Kühlung (server room) | durchgehende Kühltechnik (judge) |
| `🌬️ventilation/❓️quiz/🔣️.json:647` | aktive Kühlung | aktive Kühltechnik |
| `🌬️ventilation/❓️quiz/🔣️.json:651` | Abgestimmte Lüftung und Kühlung | Abgestimmte Lüftung und Kühltechnik |
| `🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json:91` | Auslegung der Kühlung | Auslegung der Kühltechnik (judge: video title) |
| `📖️concepts/📝️.md:18` | Übergabe an Heizung und Kühlung | Übergabe an Heizung und Kühltechnik |
| `📖️concepts/📝️.md:25-30` | Kühlung Teil 1…6 (video title) | keep as video name 'Kühlung', judge |

Process sense, kept as 'Kühlung': `🧲️physics/…:2745` (Kühlung entfeuchtet), `❄️cooling/…/🌤️summer-strategy/🐍️.py:211, 275, 305, 502` (freie Kühlung, sorptionsgestützte Kühlung), `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:437`.

### 6b. Other wording

| file:line | text | status | proposed |
|---|---|---|---|
| `📖️concepts/📝️.md:163` | "1 000 ppm Pettenkoferwert" | F04 violation | "1 000 ppm, Grenzwert nach Pettenkofer" |
| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:349` (en) | "each facade lights up in turn" | animation-describing narration | "each facade receives the sun in turn and draws its own power curve" |
| `🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json:203` | "the energy … simply disappears through the opening" | conceptual, not animation: keep, judge | "… leaves through the opening" |
| `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:1222` | "Real bridges appear where material pierces the insulation" | conceptual, keep | — |
| `📊️demand/🎬️clip/⚡️demand-power/🐍️.py:1060` | "a large reserve appears" | conceptual, keep | — |
| `📊️demand/🎬️clip/🏷️certificate/🐍️.py:750` | "its key entries have to appear in commercial property ads" | conceptual, keep | — |
| all energy narration | `verblasst`, `erscheint`, `leuchtet auf`, `blendet`, `fade`(in clauses) | 0 hits in narration | — (F07 closed) |

Other `fade` hits are code docstrings and transition recipe names (`"recipe": "fade"`), not narration.

## 7. Test registration in `🏗️authoring/📜️script.ts`

Test modules found: 70 `.py` files under `🧪️*` folders in the energy tree, of which 65 are registered in the `test` list. The 5 unregistered files define no tests (no `unittest`, `TestCase` or `def test_`), so they are scene or helper modules, not test modules. **Result: no unregistered test module.**

| file | defines tests? | status |
|---|---|---|
| `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖼️posters/🧪️tests/🧰️synthetic/🐍️.py` | no | not a test module (scene or helper); do not register |
| `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🧩️chapters/🐍️process.py` | no | not a test module (scene or helper); do not register |
| `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🏗️composition/🐍️.py` | no | not a test module (scene or helper); do not register |
| `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py` | no | not a test module (scene or helper); do not register |
| `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🔊️speech/🐍️.py` | no | not a test module (scene or helper); do not register |

Registry entries with no matching file: none under the energy tree. The excel entry `📊️excel/…/⚡️energy/🧪️tests/📐️result` is outside the energy tree and not compared.

