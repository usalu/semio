# 📝️ sonnet-course: Course Consumer Report (staffage top, front, side)

Test command (cross-platform, resolves the repo root with `git rev-parse --show-toplevel`):

```
bash ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/🛠️scripts/course/tests.sh" [module ...]
```

Without arguments it runs the guard (`🎨️visuals.🧪️tests.🐍️staffage_only`) and the staffage Python API tests (`🎨️visuals.🧍️staffage.🧪️tests.🐍️`) through `uv run ... python -m unittest`, with `SEMIO_TEACHING_OUTPUT_ROOT` under `T/🗑️generated/course`. Scene tests need `SEMIO_TEACHING_LANGUAGE=en` (or `de`) in the environment.

## Projection findings

- Of 31 clip scripts that draw casts, only two are plan scenes: `🏗️bim/📊️information` (Villa Savoye floor plan, the 1929 car) and `📊️excel/📋️program` (capsule plan, one occupant). Both draw `plan` casts. Every other clip is a vertical drawing (building section, elevation or diagram).
- No cast was in the wrong view for its scene. The rows with `front` (radiators, tilt window, extrema cooler) all stand on the back wall of a room section or on a facade and are seen frontally, which is correct there: people and furniture are seen from the side in the section plane, wall-mounted devices frontally.
- The 88 rows without `viewpoint` were all implicit `elevation`; they now say so explicitly. 94 of 94 rows carry `viewpoint`.
- Mixed viewpoints in one scene (front radiator beside elevation people): physics fundamentals, heating fundamentals, heating calculation, heating conduction, demand practice, recap, certificate, primary-energy, optimization extrema, optimization experiments. Decision per scene: wall devices frontal, people and furniture side-on, which is how a room section reads. `📉️extrema` icon row (radiator, cooler, lamp) and `🧪️experiments` icon row (occupant beside window) are schematic icon rows; all casts in the row now share one scene metre.
- Mirrors: five manifest rows mirrored an elevation (never a plan or front). They are now named `*-mirrored` so the flag is visible at the call site. Four call sites pass `mirror=True` or a computed flag. Each is allowlisted with a physical reason in `🧪️tests/🔣️staffage_only.json` (`cast_rules`), asymmetric assets (split AC, radiator valve side, blind cut-off) included. Per D6 a mirrored elevation is a mirror image (near side stays the right side); acceptable here because no held object or handed part decides the teaching content.
- Plan yaws: bim car (`_car`, model heading) and excel occupant (-90 degrees, faces the core side with back to the window wall). Both documented in the guard allowlist `plan_yaws` and the docstrings.
- BIM car: non-uniform `stretch` removed. The car scales uniformly until it fits inside the model's assumed 4.6 m by 1.75 m, so it is 4.32 m by 1.75 m on paper (real proportions held, conservative against the clearance rectangle). The parameter study scales the same car uniformly.
- Extrema cooler: sized by `staffage_height` like its row neighbours.
- Crate casts: literal full ids, so `casts_in` keys them.
- `scale: 20` of the two natural-ventilation radiators removed (default 1:50, the finest authored band): the clip fits the cast by height, so only line weight changes.

## Changes

- Manifest: `viewpoint` explicit on all 94 rows, five rows renamed `*-mirrored`, two `scale: 20` removed.
- Python API (`🧍️staffage/🐍️.py`): no elevation default (`cast_key` and `_index` raise on a missing or unknown viewpoint); `ensure_casts` compares the rendered index viewpoint with the manifest row; `casts_in(sources)` maps every cast a source names to its `cast_key`; `_prune` survives vanished, loose and empty folders per folder; a `staffage-pattern` paint reads as no fill (the base fill precedes it).
- Clip key (`🎬️clip/⚙️runtime/🐍️.py`, `segment_state`): now contains `casts` (id to `cast_key`), so a catalog, exporter, schema or viewpoint change re-renders the clip.
- Visuals (`🎨️visuals/🐍️.py`): `ROOM_HEIGHT_M = 2.5` and `room_metre(room_height)`; 13 sites in 11 clips replaced `/ 2.5` by it.
- Guard (`🧪️tests/🐍️staffage_only.py` plus `🔣️staffage_only.json`): scene projection declared per clip (`plan` or `vertical`, every cast-drawing clip declared, plan scene takes plan casts only, vertical scene takes elevation or front); every row names its viewpoint; mirror rows end in `-mirrored` with an allowlisted reason; call-site mirrors allowlisted; no stretch of a cast; rotation of a cast only as a documented yaw; no f-string or concatenated cast ids.
- Tests (`🧍️staffage/🧪️tests`): viewpoint required in entries and indexes; `casts_in` changes with the catalog; prune robustness; pattern paint.

## Open

- Magic sizing not yet derived from the scene metre: physics fundamentals `unit=1.2` (1759) and `_ROOM_METRE` users, heating conduction `/ 1.75` (1034), experiments `unit=.5`, internal-gains `unit=0.8/0.5` icons and several others in the r2 audit list. The storey constant exists; these sites still need a scene-metre derivation each and a visual check.
- Cache regeneration of the 94 casts and pruning waits for the coordinator signal; no scene was rendered by this slot yet.
- The clip test module `🎬️clip.🧪️tests.🐍️` reports `SyntaxError: authored intro` errors in other owners' tests (not touched here).

## Cast inventory

| cast | asset | view | viewpoint | mirror | scale | scenes (chapter/clip) | scene projection |
|---|---|---|---|---|---|---|---|
| cooling-internal-gains-sleeper | sleeping-person | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-seated-worker | seated-office-worker | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-standing | standing-adult | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-walker | walking-commuter | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-jogger | jogger | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-laptop | laptop | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-laptop-closed | laptop | thermal | elevation |  | 50 | library or unused | - |
| cooling-internal-gains-server-rack | server-rack | thermal | elevation |  | 50 | library or unused | - |
| shared-lamp-summer | pendant-lamp | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, ❄️cooling/👥️internal-gains | vertical |
| cooling-internal-gains-lamp-dimmed | pendant-lamp | thermal | elevation |  | 50 | library or unused | - |
| shared-desk | height-adjustable-desk | dark-line | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, ❄️cooling/👥️internal-gains, 📊️demand/🔁️recap, 🔥️heating/👥️internal-gains, 🔥️heating/🧮️calculation | vertical |
| cooling-internal-gains-office-chair | office-swivel-chair | dark-line | elevation |  | 50 | library or unused | - |
| shared-radiator-front-winter | panel-radiator | thermal | front |  | 50 | 📊️demand/🏭️primary-energy, 📊️demand/🏷️certificate, 📊️demand/🔁️recap, 📊️demand/🧪️practice, 🔥️heating/🧲️fundamentals, 🧲️physics/🧲️fundamentals, 🎯️optimization/📉️extrema | vertical |
| shared-person-winter | standing-adult | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, 📊️demand/♨️system-losses, 📊️demand/⚖️regulations, 📊️demand/⚡️demand-power, 📊️demand/🏭️primary-energy, 📊️demand/🏷️certificate, 📊️demand/🔁️recap, 📊️demand/🧪️practice, 🔥️heating/👥️internal-gains, 🔥️heating/🧮️calculation, 🔥️heating/🧱️conduction, 🔥️heating/🧲️fundamentals, 🧲️physics/🧲️fundamentals | vertical |
| heating-internal-gains-office-worker | seated-office-worker | thermal | elevation |  | 50 | 🔥️heating/👥️internal-gains | vertical |
| shared-laptop-winter | laptop | thermal | elevation |  | 50 | 📊️demand/🔁️recap, 🔥️heating/👥️internal-gains | vertical |
| heating-internal-gains-tower-server | tower-server | thermal | elevation |  | 50 | 🔥️heating/👥️internal-gains | vertical |
| heating-internal-gains-pendant-lamp-warm-up | pendant-lamp | thermal | elevation |  | 50 | 🔥️heating/👥️internal-gains | vertical |
| shared-lamp-winter | pendant-lamp | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, 📊️demand/⚖️regulations, 📊️demand/⚡️demand-power, 📊️demand/🏷️certificate, 📊️demand/🔁️recap, 📊️demand/🧪️practice, 🔥️heating/👥️internal-gains, 🔥️heating/🧮️calculation, 🧲️physics/🧲️fundamentals | vertical |
| shared-person-relaxed-line | standing-adult | dark-line | elevation |  | 50 | 🏗️bim/🔌️exchange, 🏗️bim/🔗️consistency | vertical |
| bim-consistency-decider | standing-adult | dark-line | elevation |  | 50 | 🏗️bim/🔗️consistency | vertical |
| excel-program-occupant | standing-adult | dark-line | plan |  | 50 | 📊️excel/📋️program | plan |
| bim-information-car | vintage-car-1929 | dark-line | plan |  | 50 | 🏗️bim/📊️information | plan |
| optimization-room-cooler-mirrored | split-ac-indoor-unit | thermal | elevation | yes | 50 | 🎯️optimization/🎨️visuals | vertical |
| shared-lamp-neutral | pendant-lamp | thermal | elevation |  | 50 | 🎯️optimization/⚖️index, 🎯️optimization/🎨️visuals, 🎯️optimization/📉️extrema | vertical |
| shared-lamp-off-neutral | pendant-lamp | thermal | elevation |  | 50 | 🎯️optimization/🎨️visuals | vertical |
| optimization-room-occupant | standing-adult | thermal | elevation |  | 50 | 🎯️optimization/🎨️visuals, 🎯️optimization/🧪️experiments | vertical |
| optimization-extrema-cooler | split-ac-indoor-unit | thermal | front |  | 50 | 🎯️optimization/📉️extrema | vertical |
| optimization-experiments-occupant | standing-adult | thermal | elevation |  | 50 | 🎯️optimization/🧪️experiments | vertical |
| optimization-experiments-window | tilt-window | thermal | front |  | 50 | 🎯️optimization/🧪️experiments | vertical |
| shared-person-pathlines | standing-adult | thermal | elevation |  | 50 | 🌬️ventilation/⚙️mechanical, 🌬️ventilation/❓️need, 🌬️ventilation/📦️volume, 🌬️ventilation/🪟️natural | vertical |
| ventilation-natural-radiator-on-mirrored | panel-radiator | thermal | elevation | yes | 50 | 🌬️ventilation/🧪️architecture, 🌬️ventilation/🪟️natural | vertical |
| ventilation-natural-radiator-off-mirrored | panel-radiator | thermal | elevation | yes | 50 | 🌬️ventilation/🪟️natural | vertical |
| ventilation-heat-flow-heater | electric-fan-heater | thermal | elevation |  | 50 | 🌬️ventilation/🌡️heat-flow | vertical |
| shared-office-worker-pathlines | seated-office-worker | thermal | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-cook | person-cooking | thermal | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-operating-table | operating-table | thermal | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-hood-mirrored | kitchen-extractor-hood | thermal | elevation | yes | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-blackboard | chalkboard | dark-line | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-crates-low | storage-crate-stack | dark-line | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-crates-half | storage-crate-stack | dark-line | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| ventilation-volume-crates-high | storage-crate-stack | dark-line | elevation |  | 50 | 🌬️ventilation/📦️volume | vertical |
| demand-certificate-sleeper | sleeping-person | thermal | elevation |  | 50 | 📊️demand/🏷️certificate | vertical |
| physics-fundamentals-vacuum | vacuum-cleaner | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| shared-radiator-winter | panel-radiator | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 📊️demand/⚖️regulations, 📊️demand/⚡️demand-power, 🧲️physics/🧲️fundamentals, 🎯️optimization/🎨️visuals | vertical |
| demand-primary-energy-tree | english-oak | dark-line | elevation |  | 50 | 📊️demand/🏭️primary-energy | vertical |
| shared-person-neutral | standing-adult | thermal | elevation |  | 50 | 📊️demand/🏷️certificate, 📊️demand/🧪️practice | vertical |
| shared-lamp-off-winter | pendant-lamp | thermal | elevation |  | 50 | 📊️demand/🧪️practice | vertical |
| demand-certificate-buyer | standing-adult | thermal | elevation |  | 50 | 📊️demand/🏷️certificate | vertical |
| physics-fundamentals-clerk | standing-adult | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| shared-blind-lowered-mirrored | external-venetian-blind | thermal | elevation | yes | 50 | 🔥️heating/☀️solar-gains, 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-hob | gas-hob | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-pot | cooking-pot | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-kettle | electric-kettle | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-compressor | scroll-compressor | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-evaporator | finned-heat-exchanger | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-condenser | finned-heat-exchanger | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| physics-fundamentals-valve | expansion-valve | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| cooling-summer-strategy-wheel | sorption-wheel | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| cooling-summer-strategy-exchanger | plate-heat-exchanger | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| cooling-summer-strategy-humidifier | evaporative-humidifier | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| cooling-summer-strategy-collector | solar-thermal-collector | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| cooling-summer-strategy-heater | finned-heat-exchanger | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| shared-person-fanning | standing-adult | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, ❄️cooling/🌤️summer-strategy | vertical |
| cooling-heating-comparison-table | dining-table | dark-line | elevation |  | 50 | ❄️cooling/⚖️heating-comparison | vertical |
| shared-toaster-winter | toaster | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison, 🔥️heating/🧱️conduction | vertical |
| cooling-heating-comparison-toaster-summer | toaster | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison | vertical |
| cooling-heating-comparison-laptop | laptop | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison | vertical |
| cooling-heating-comparison-laptop-summer | laptop | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison | vertical |
| cooling-summer-strategy-blind | external-venetian-blind | thermal | elevation |  | 50 | ❄️cooling/🌤️summer-strategy | vertical |
| shared-watt-laptop | laptop | thermal | elevation |  | 50 | library or unused | - |
| shared-watt-toaster | toaster | thermal | elevation |  | 50 | library or unused | - |
| shared-watt-seated | seated-office-worker | thermal | elevation |  | 50 | library or unused | - |
| ventilation-mechanical-duct-fan | duct-fan | thermal | elevation |  | 50 | 🌬️ventilation/⚙️mechanical | vertical |
| ventilation-mechanical-mvhr | mvhr-unit | thermal | elevation |  | 50 | 🌬️ventilation/⚙️mechanical | vertical |
| ventilation-mechanical-pendulum-fan | decentralised-hrv-fan | thermal | elevation |  | 50 | 🌬️ventilation/⚙️mechanical | vertical |
| shared-mvhr-normal | mvhr-unit | thermal | elevation |  | 50 | 🌬️ventilation/❓️need, 🌬️ventilation/📦️volume | vertical |
| heating-conduction-tealight | tealight | thermal | elevation |  | 50 | 🔥️heating/🧱️conduction | vertical |
| physics-fundamentals-shower | shower-head | thermal | elevation |  | 50 | 🧲️physics/🧲️fundamentals | vertical |
| shared-boiler-heating-cycle | gas-boiler | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 📊️demand/🏭️primary-energy, 📊️demand/🧪️practice, 🔥️heating/🧮️calculation | vertical |
| shared-tank-charge-up | hot-water-cylinder | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 🔥️heating/🧮️calculation | vertical |
| shared-pump-run-high | circulation-pump | thermal | elevation |  | 50 | 📊️demand/🏭️primary-energy, 📊️demand/🧪️practice | vertical |
| demand-practice-valve | thermostatic-radiator-valve | thermal | elevation |  | 50 | 📊️demand/🧪️practice | vertical |
| shared-gas-meter-run | gas-meter | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 📊️demand/🏭️primary-energy, 📊️demand/🧪️practice | vertical |
| shared-electricity-meter-run | electricity-meter | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 📊️demand/🏭️primary-energy | vertical |
| cooling-system-sizing-diffuser | ceiling-diffuser | thermal | elevation |  | 50 | ❄️cooling/📐️system-sizing | vertical |
| demand-primary-energy-pumpjack | oil-pumpjack | thermal | elevation |  | 100 | 📊️demand/🏭️primary-energy | vertical |
| heating-calculation-thermostat | room-thermostat | thermal | elevation |  | 50 | 🔥️heating/🧮️calculation | vertical |
| shared-heat-pump-heating-sweep | heat-pump-outdoor-unit | thermal | elevation |  | 50 | 📊️demand/♨️system-losses, 📊️demand/🏭️primary-energy | vertical |
| cooling-heating-comparison-cooler | split-ac-indoor-unit | thermal | elevation |  | 50 | ❄️cooling/⚖️heating-comparison | vertical |
| shared-watt-heater | electric-fan-heater | thermal | elevation |  | 50 | library or unused | - |
| shared-radiator-thermostat-front | panel-radiator | thermal | front |  | 50 | 🔥️heating/🧮️calculation, 🔥️heating/🧱️conduction | vertical |
| shared-radiator-thermostat | panel-radiator | thermal | elevation |  | 50 | 🔥️heating/🧮️calculation | vertical |
| heating-calculation-laptop | laptop | thermal | elevation |  | 50 | 🔥️heating/🧮️calculation | vertical |
