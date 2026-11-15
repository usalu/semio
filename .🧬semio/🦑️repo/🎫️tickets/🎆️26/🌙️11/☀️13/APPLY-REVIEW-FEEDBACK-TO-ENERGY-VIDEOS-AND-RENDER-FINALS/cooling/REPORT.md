# cooling report (cooling video, all six chapters), recorded by the coordinator

Verified with direct manim renders (preview, German); frames in `🗑️generated/cooling-out/frames`.
Pipeline render with captions/speech not run. Narration guard clean for cooling. The three clip
test modules pass separately; combined in one run an import side effect (`set_vo_language("de")`)
makes English captions fail the two-line check.

- Units: no bare `[W]`; units on every number; fraction bars.
- Symbols: narration `$…$` (Δθ, Δθ_eq, Q̇_T, Q̇_V, Q̇_S, Q̇_I, Q̇_C, q_{v,R}, ρ_a, c_{p,a}).
  `F_sh`/`F_V` → `F_C`, `F_f` → `F_F`, `g` → `g_⊥` at the glazing. `Q̇_L` → `Q̇_V`, `Q̇_K` → `Q̇_C`.
  Internal gains `Q̇_{I,p}`, `Q̇_{I,e}`, `Q̇_{I,l}`.
- Sum: `Q̇_C = Q̇_T + Q̇_V + Q̇_S + Q̇_I` shown in the first chapter; ventilation
  `Q̇_V = Q̇_{V,sens} + Q̇_{V,lat}` ≈ 1940 W; internal 1200 W + 420 W + 400 W = 2020 W.
  "Kühlung" → "Kühltechnik" where the system is meant.
- Visuals: supply low / extract high; sun, rays, blinds visible; proportional thermometers;
  solar Beat 1 links sunlight to the electromagnetic wave.
- References (WebSearch): DIN 4108-2:2013-02 Abschn. 8 Tab. 7 (F_C); DIN V 18599-2:2016-10
  "Interne Wärmequellen"; DIN EN 16798-1:2022-03 Anh. B.2; VDI 6022 Blatt 1 replaces VDI 2078
  on the sorption scene; GEG §14 and DIN 1946-4:2018-09 in the quiz.

## Numbers changed
Sizing example now derives from the running example and the solar scene: glazing factor 0,49
contradicted heating's 0,70 → solar load 1510 W (was 1060 W), temperature lift 8 K (was 7 K),
airflow 0,157 m³/s (was 0,126), duct area 0,0629 m² (was 0,0505). Test cases JSON updated.

## Not checked / open
- Frames for Beat1_OfficeRoom, Beat5_Mitigation, summer-strategy Beat 1 and 3, Beat1_WinterGains.
- VDI 2078:2015-06 locators are keywords (clause numbers paywalled).
- `Q̇_{I,l}` hard to read. Coordinator note: heating uses lower-case `Q̇_i` (DIN V 4108-6 `Q_i`) –
  unify in the closing consistency pass.
- Heating-owned scenes reused in Cooling_04 used `F_sh` until heat-b changed them (done since).

## Files (`🎓️teaching/🏛️architecture/⚡️energy/❄️cooling/`)
Six `🎬️clip/<chapter>/🐍️.py`, `📐️system-sizing/🧪️tests/🔣️.cases.json`, `🎬️clip/README.md`,
`❓️quiz/🔣️.json`.
