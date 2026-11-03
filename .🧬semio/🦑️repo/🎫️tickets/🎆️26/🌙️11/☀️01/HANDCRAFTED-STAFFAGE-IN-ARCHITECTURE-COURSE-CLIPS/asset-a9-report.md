# Asset agent A9 report: heating plant

Three handcrafted appliances in `S/🗂️catalog/🔌️appliances/` (S = `🎓️teaching/🏛️architecture/🧍️staffage`), manifest rows appended after `server-rack`. Generators (JSON emitters, shapes by hand): `T/🧰️a9/` (`pump.mjs`, `boiler.mjs`, `cylinder.mjs`, `geo.mjs`, `build.mjs`, `cast.sh`, `sheet.sh`). Scales 1:50, 1:100, 1:200 authored individually, 1:500 unavailable (reason, limiting feature, alternatives 200/100), elevation only, en/de.

| Asset | Size (default pose) | Poses | Clips | Heat |
|---|---|---|---|---|
| `gas-boiler` | 0.44 x 0.72 x 0.35 m, 30 kg, wall bracket | `off`, `heating`, `hot-water` | `heating-cycle`, `hot-water-priority` | burner key 0 / 0.7 (14 kW) / 1 (20 kW); room losses at 1: casing 55, flue collar 10, stubs 12, burner 3, glass 2, display 3 W (about 85 W); `flue` emitter 0.0072 m3/s, +38 K, 0.07 kg/kg vapour |
| `circulation-pump` | 0.18 x 0.15 x 0.13 m, 2.8 kg, hangs on the pipe stubs (pin + roller) | `off`, `low`, `high` | `run-high` (loop), `run-low` (loop), `start-and-throttle` | power key 0 / 0.111 (5 W) / 1 (45 W); room gain 30 W (motor 18, casing 10, box 2); plume emitter |
| `hot-water-cylinder` | 0.6 x 1.6 x 0.6 m, 275 kg filled (about 200 L), two feet | `cold`, `half-charged`, `charged` | `charge-up`, `draw-off` | four jacket quarters = four regions with their own keys (heat 20/17/14/10 W) + cap 6 W + stubs 9 W = 76 W standby; half-charged levels 1/0.9/0.15/0; plume emitter |

Design notes
- Boiler: front elevation; a sight glass is a real hole in the casing path (even-odd fill), the burner tube and two flame bones (prismatic, retract behind the casing when off; inner flame at heating, outer flame added at full output) sit behind it. Mode knob is a revolute bone (off / heating / hot water ticks). The 20 kW fuel input goes to water and flue gas, not the room; only surface losses are heat gains (documented in notes and assumptions).
- Pump: front view along the motor shaft; rotor indicator (pointer, tail, hub) is an unlimited revolute bone behind a window; loop clips close with 360 degrees (slow motion, stated in the text). The pointer carries no region on purpose: a same-region child shape inside its parent loses its outlines (joined rule, same trap A7 found).
- Cylinder: stratification shows in the thermal view through the quarter regions; the charged state is uniformly orange in the palette (differences of 20 vs 10 W stay within one colour band), half-charged clearly splits warm top / cold bottom. Provenance: EN 12897, EU 812/2013, EN 16297-1, EU 622/2012, EN 15502-1, Turns, Incropera, ISO 10456; dimensions and splits tagged `authored-estimate`.

Verification
- `check --asset` and `--category appliances`: 12/12 pass, all three suitable under office-laser at 1:50/100/200 (min stroke 0.13-0.18 mm).
- Casts (`theme: dark`; dark-line, dark-solid, thermal at 1:50/100/200, all poses; clip frames of the pump, cylinder and boiler) rasterised with PyMuPDF and looked at. First boiler version had an empty upper casing: door line, logo bar and LED added. Cylinder 1:100/1:200 first failed `extent-width` (stub widths), fixed with 0.03 stubs and two thickened side stubs at 1:200.
- Not run: vitest staffage suites and the Python clip tests (dependencies broken on this machine, see `q1-tests-report.md`); no clip call sites were migrated (assets only).

Cast examples (T/🧰️a9/casts/)
- `b3.png`: boiler hot-water at 1:50, 1:100, 1:200 dark-line, 1:100 dark-solid, thermal 1:200 and heating 1:100.
- `b2.png`: boiler thermal off / heating / hot-water at 1:50.
- `c1.png`: cylinder 1:50/100/200 dark-line, dark-solid, thermal cold / half-charged / charged.
- `s50.png`: pump high at 1:50 dark-line, dark-solid, thermal.
- `m.png`: clip frames (pump rotor, pump ramp, cylinder charge, boiler priority).
Cast ids for call sites: `{asset: "gas-boiler", pose: "hot-water" | clip: "hot-water-priority", view: "thermal", environment: "office-neutral", theme: "dark"}` (same for the other two).
Generated folder `T/🗑️generated/a9` deleted.
