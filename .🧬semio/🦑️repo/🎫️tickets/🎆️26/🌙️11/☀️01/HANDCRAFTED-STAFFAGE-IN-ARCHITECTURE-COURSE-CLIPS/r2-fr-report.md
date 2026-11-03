# r2-fr: rate/loop audit of the 22 capped casts

Result: no call site needed a change. No source file was edited. `staffage_only` guard untouched.

## Method
- Probe `T/🧰️fr/probe.py` + `probe.sh`: runs a scene with `--dry_run` and wraps `_Motion.advance`, logging the cast time each motion reaches by scene end (`elapsed`) against the clip length. `sum.py` digests `🗑️generated/fr/*.probe.json`. Cast contact sheets: `casts.py` (PyMuPDF). Frames: `mont.py` (PyAV), render: `render.sh` (ql, 15 fps, one scene).
- Key fact: displayed fps = cast fps x rate. Fractional cast fps is only choppy at rate near 1. Every site below runs at rate >= 3 (or is `seek`-driven), so each shows >= 6 cast frames per scene second; no fps < 4 case exists.
- Most transients change in the first 10-20 percent of the clip (sheets: toaster pops at 155 s, heater/lamp/boiler/tank plateau within 40-130 s, vacuum parks at about 700 s), so partial coverage hides only a plateau.

## Table (scene end, cast time reached / clip)
| Cast | Call site | rate before = after | Reached | Verdict |
|---|---|---|---|---|
| cooling-heating-comparison-cooler | heating-comparison Beat3 `ahu["unit"].rate = COOLER_RATE` 45 | 45 | rendered: orange to blue exactly at "19 Grad" caption | ok, rendered |
| cooling-heating-comparison-laptop | Beat1 `LAPTOP_RATE` 24 | 24 | 83% (lid closed by 300 s) | ok |
| -laptop-summer | Beat2 24 | 24 | 88% | ok |
| -toaster-summer | Beat2 `TOASTER_RATE` 12 | 12 | 88% (pop at 155 s) | ok |
| shared-toaster-winter | Beat1 12 | 12 | 83% | ok |
| shared-toaster-winter | conduction Beat4 `TOASTER_RATE` 8 | 8 | 29% (red hot by 86 s, size comparison) | ok |
| cooling-summer-strategy-heater | Beat2 `heater.rate = 3.0` | 3 | 98% | ok |
| cooling-summer-strategy-wheel | Beat2 `WHEEL_TIMELAPSE` 30, loop | 30 | 4.8 turns, seamless loop in sheet | ok |
| heating-internal-gains-pendant-lamp-warm-up | Beat4 `LAMP_WARM_RATE` 12 | 12 | 72% (warm by 43 s) | ok |
| heating-internal-gains-server-rack | Beat3 `RACK_WARM_RATE` 30 | 30 | 100% | ok |
| physics-fundamentals-kettle | Beat1 `KETTLE_RATE` 60 | 60 | 315% | ok |
| physics-fundamentals-pot | Beat7 `POT_BOIL_RATE` 30.5 | 30.5 | 296% | ok |
| physics-fundamentals-shower | Beat1/2 `SHOWER_RATE` 40 | 40 | 147% / 476% | ok |
| physics-fundamentals-vacuum | Beat1 icon (loop=True), Beat2/6 `VACUUM_RATE` 60 | 60 | 20-31%, never wraps (loop flag inert) | ok |
| shared-boiler-heating-cycle | system-losses Beat2-4, primary-energy, calculation `BOILER_RATE` 60 | 60 | 38-203% | ok |
| shared-tank-charge-up | system-losses, calculation: rate 0 + `seek(level x 900)` | 0 | 72-100% | ok |
| shared-watt-laptop / -toaster / -heater | `watt_anchor` `_WATT_RATES` 6 / 60 / 180 | same | 20% / 23% / 229% | ok (chip icons; laptop only shows the open lid) |
| ventilation-mechanical-pendulum-fan | mechanical Beat3 `_pendulum_half` seek per 70 s phase | 0 | 100% | ok |
| heating-calculation-radiator(-mirrored), shared-radiator-thermostat-front | calculation / conduction, integer 15 fps loop | 1 (default) | loops (131% seen) | ok |

## Not probed
- The calculation clip scenes (`ReviewingHeatLosses` ... `FullFinalCalculationVideo`) were skipped after the chain proved slow; their sites (`BOILER_RATE` 60, tank `seek`, 15 fps loop radiators) are the same constants as probed elsewhere.
- Physics Beat3-5 and Beat8, mechanical Beat1/2/4 gave no watched cast.

## Notes
- First `Beat3_CoolingSystem` run failed with `defaultDepictionOptions is not defined` (a foreign mid-edit of the exporter); a later run succeeded without any change by me.
- Possible polish (not done, no fault): `shared-watt-laptop` at rate 6 never shows the closing lid; the `loop=True` on the Beat1 vacuum icon is inert.
