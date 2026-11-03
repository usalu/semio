# M9 report: static-audit findings (Lists A, B, C, D)

V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, E = `🎓️teaching/🏛️architecture/⚡️energy`. All new casts are in `V/🧍️staffage/🔣️.json` (`theme: dark`, thermal, animated through `staffage_motion` where a clip exists). Radiator casts were not touched (A11).

## List A: hand-built devices

| # | Done |
|---|---|
| A1, A6, A8 boilers | `_boiler` in `♨️system-losses`, `🏭️primary-energy`, `🧪️practice` is now a cast: `demand-{system-losses,primary-energy,practice}-boiler` (`gas-boiler`, clip `heating-cycle`, thermal, `office-neutral`, `BOILER_RATE` 60). Flame helper of system-losses deleted (`_flame` stays in primary-energy, it is used for the fuel source icon). |
| A2 `_fan` (system-losses) | Deleted with the old heat pump (the heat pump is a cast now). |
| A3, A7 heat pumps | `demand-system-losses-heat-pump`, `demand-primary-energy-heat-pump` (`heat-pump-outdoor-unit`, clip `heating-sweep`, thermal, `winter-overcast`). The fan spin (`Rotate` of the old blades) became the clip: `rate` is 1 while the pump runs, 0 otherwise (`pump.rate` set in `live`, `p["fan"].rate` set in `ambient_streams`). Unused `heat_pump` entry of the system-losses plant dict removed. |
| A4 tank | `_tank` is `hot-water-cylinder` (`demand-system-losses-tank`, clip `charge-up`, thermal). The old fill level is now the clip time: the `level` tracker (kept, same API) seeks `level * 900 s`, so 1.0 = charged and 0.72 = partly cooled. `fill` is an empty group. |
| A5 pump | `_pump` in primary-energy: `demand-primary-energy-pump` (`circulation-pump`, `run-high`, loops). |
| A9 `⚙️mechanical` `_fan` | `ventilation-mechanical-duct-fan` (`duct-fan`, clip `spin`, thermal), fitted into the old fan diameter. `_spin` now sets `rate = 1` and returns `[Wait(rt)]` (the pendulum scene indexes `[0]`); the rotation sign of the reversing pendulum fan is not drawn any more (the air streams carry the reversal). |
| A10 `❓️need` | Fan glyph on the "mechanisch" card replaced by `ventilation-need-mvhr` (`mvhr-unit`, pose `normal`, thermal), because that card is the heat-recovery unit (eta WRG). |
| A11 `📦️volume` | Passive-house mini room: unit box plus fan replaced by `ventilation-volume-mvhr` (`mvhr-unit`). |
| A12 tealight | `heating-conduction-tealight` (`tealight`, clip `flicker`, thermal, loops); `_emitters` use the cast top. `_devices` kept as the row builder. |
| A13 split unit | The "Lüftungsgerät mit Kühlregister" in `⚖️heating-comparison` is `cooling-heating-comparison-cooler` (`split-ac-indoor-unit`, clip `start-cooling`, thermal, `summer-heatwave-calm`), fitted into the old 0.9 x 1.5 box. The fan `Rotate` and the coil recolouring are gone; the coil beat starts the clip (`COOLER_RATE` 15). Flow paths still use the unit's top and bottom. |
| A14 shower | `physics-fundamentals-shower` (`shower-head`, `turn-on-and-run`, thermal, rate 40). |

## List B: watt badges

- `V/🐍️.py`: `_WATT_DEVICES`/`_WATT_CASTS` gained `person` ("stehende Person", `shared-watt-person`, standing-adult `idle-breathing`) and `seated` ("sitzende Person", `shared-watt-seated`, seated-office-worker `typing`), both thermal and animated; `sun` ("volle Sonne", 1000 W/m², unit text via `_WATT_UNITS`) draws `icon_glyph("sun")` (`_WATT_ICONS`).
- Call sites: B1 `compare="sun"` (solar-radiation), B2 and B3 `compare="person"` (heating-comparison, cooling internal-gains), B4 `compare="seated"` (heating internal-gains).
- `fitted_staffage(..., rate=, loop=)` plays clips; badges that have clips (`_WATT_RATES`) are animated.

## List C: thermal view

- C1 `physics-fundamentals-clerk`, C3 `demand-certificate-buyer`: thermal pose, `office-neutral` (no standing-adult clip for pointing, arms-crossed).
- C2 `demand-practice-talker`, C4 `demand-certificate-tenant`: thermal `idle-breathing` clip (animated). `_talker` lost its colour argument; the tenant uses `staffage_motion`.
- C5 `shared-watt-toaster` (clip `toast-cycle`, rate 60), C6 `shared-watt-heater` (clip `warm-up`, rate 180): thermal, animated, `office-neutral`.
- Judgment: `shared-watt-laptop`, `-monitor`, `-bulb`, `-vacuum` stay dark-line (they show electrical power, not heat flow).

## List D: dead code

- Deleted `radiator()`, `respiration_parts()`, the `radiator_height/width` entries of `V/🔣️.json`, manifest casts `standing`, `walking`, `radiator`, `seated-reader`, `shared-radiator`. Icon test updated (no radiator; new test for person and sun badges).
- Guard `V/🧪️tests/🐍️staffage_only`: forbids `respiration_parts` and has `test_manifest_has_no_orphan_casts` (`forbidden_casts` in `🔣️staffage_only.json`).
- D7 kept (icons only). D8: contract note added (`fitted_staffage` and `pendant_lamp` live in `V/🐍️.py`).
- Removed in clip code: `_fan` (system-losses, need, volume, mechanical rewritten as cast), old boiler/tank/pump/heat-pump/tealight/shower constructions.

## Verification

- Tests (all OK): V `🐍️`, `icons`, `staffage_only`, `integration`, `scenes`, `sections`, `section_consumers`, `transforms`, staffage package (84 tests); cooling clip tests, cooling internal-gains, volume, regulations sun (15 of 16 OK).
- One failure, not from M9: `ventilation/🎬️clip/🧪️architecture` `test_closeup_radiator_scales_and_clears_floor` (radiator height 0.68 vs 0.568), caused by A11's concurrent radiator front-viewpoint change.
- Rendered one scene at a time (quality l, 15 fps) and looked at frames: heating-comparison Beat3 (cooler), system-losses Beat2 (boiler, tank), primary-energy Beat1 and Beat3 (boiler, pump, heat pump; heat pump box raised to 0.8 x 0.8 after the first look showed it too small), practice Beat3 (boiler), certificate Beat3 (buyer, tenant), conduction Beat4 (tealight), mechanical Beat1 (duct fans), need Beat3 (mvhr card), volume Beat2 (passive mvhr), fundamentals Beat2 (clerk absent in sampled frames; toaster badge in cooling internal-gains Beat2; shower), cooling internal-gains Beat2 (person, toaster badges), heating internal-gains Beat2 (seated badge), solar-radiation Beat1 (sun badge).
- Not individually viewed: mechanical Beat3 (fan2/fan3 pendulum), heating-comparison Beat1/2 person badge, heating fundamentals Beat9 heater badge (rendered, sheets not inspected), practice Beat2 has no changed staffage.

## Observations

- The shower asset is a thin line figure (rail, hose loop, head); at course scale (unit 1.1) it reads small. The thermal view barely shows warmth at the end of the clip.
- The split-ac thermal view stays orange during `start-cooling` in `summer-heatwave-calm`; the cooling effect is subtle.
- Concurrent exporter changes by other agents trigger full cast re-exports; one render failed once on the atomic rename of a shared cache folder (certificate) and passed on retry.
- Python writes to the emoji-path source files fail intermittently with `Errno 22`; a retry loop (see `m9_*.py`) works.

Files: `V/🐍️.py`, `V/🔣️.json`, `V/🧍️staffage/🔣️.json`, `V/🧪️tests/{🐍️icons.py,🐍️staffage_only.py,🔣️staffage_only.json}`; clips: `📊️demand/{♨️system-losses,🏭️primary-energy,🧪️practice,🏷️certificate}`, `🌬️ventilation/{⚙️mechanical,❓️need,📦️volume}`, `🔥️heating/{🧱️conduction,👥️internal-gains}`, `❄️cooling/{⚖️heating-comparison,👥️internal-gains,☀️solar-radiation}`, `🧲️physics/🧲️fundamentals`; ticket `contract.md`; scripts `m9_*.py|sh|txt` in the ticket (inputs, kept).
