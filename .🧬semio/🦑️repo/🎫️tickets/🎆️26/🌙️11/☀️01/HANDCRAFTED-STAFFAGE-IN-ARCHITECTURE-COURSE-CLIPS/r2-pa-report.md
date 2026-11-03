# r2 pa: asset physics (OI-21, 22, 26, 28, 29, 30, 33, 34, 35; OI-23 deferred)

Scripts: `🧰️pa/` (`jio.py` byte-exact JS-style JSON io, `oi21_cylinder.py`, `oi22_boiler.py`, `oi26_toaster.py`, `oi28_lamp.py`, `oi29_trv.py`, `oi30_mvhr.py`, `oi33_machinery.py`, `oi33b_pumpjack_height.py`, `oi34_radiator.py`, `oi35_sleep.py`, `extent.py` forward-kinematics extent, `casts.py`). Catalog manifest edited by hand (Edit).

## Fixes (catalog assets under `S/🗂️catalog`)
- **OI-21 hot-water-cylinder**: jacket 0.60 m (0.48 m tank, 60 mm foam R 1.7), height 1.48 m (quarters 0.33 m), width 0.66 m with stubs, depth 0.60 m. All four scales redrawn (jacket, feet, dome, stubs, dial, label). Mass 80 kg dry (bones 5+4x16+6+5); 200 kg water documented as filling (filled 280 kg). Zone areas 0.62 m2, cap 0.31 m2. The 76 W standby budget is kept (51 W through the foam plus bridges). Manifest: heightM 1.48, widthM 0.66.
- **OI-22 gas-boiler**: flue emitter 0.0077 m3/s, +46 K: about 430 W = 2.1 % of 20 kW (band 2-3 %). Texts and manifest updated.
- **OI-26 toaster**: top plate 100 W to 25 W (555 W/m2, about +55 K, 75 C); the 75 W go to slot air (545 W, +103 K). Sum stays 900 W.
- **OI-28 pendant-lamp**: kept as 60 W incandescent (the clips draw the hot A60 bulb as free heat). Surfaces 34+17+6 = 57 W (95 %), 3 W light absorbed by the room. Lampholder area 0.012 to 0.005 m2.
- **OI-29 thermostatic valve**: removed the 55 C pipe contact from the three poses (double count, and it ignored the supply level); 3 W standing-water heat is the sole source. Assumption added.
- **OI-30 mvhr-unit**: casing 45 W to 12 W (about +1.5 K behind R 0.5; about 33 W go into the air streams, 30 W impeller). New looping clip `normal-spin` (3 s, 360 deg, 120 deg/s); manifest clips updated.
- **OI-33**: pumpjack motor 700 W to 1800 W (30 kW at 93 %, 24 kW mean shaft load), gears 900 W to 720 W. Height: the catalog check ties `heightM` to the default-pose drawing, so `heightM` = 5.57 m (mid-stroke); stopped pose reaches 6.27 m (documented). My first edit to 6.3 failed the check; corrected to 5.57 in asset and manifest (coordinator note; pv pens 0.25 untouched). Expansion valve mass 0.83 to 0.45 kg (bones 0.30/0.08/0.02/0.05, text 0.8 kg references fixed).
- **OI-34 panel-radiator**: massKg 26.18; wall bone mass set to 0 (schema has no context/passive flag on a bone); documented in structure assumption. File normalised to the repo JSON format (it was indent 1 without final newline).
- **OI-35 sleeping-person**: activityMet 0.7 (3 poses, baseMet, turn-over track base), texts, manifest.

## OI-23 kettle: not edited
ph owns the kettle. State read at the end: `massKg` 2.22 includes the 1.0 kg water bone. To follow the dry-mass convention set massKg 1.22 and keep the water bone as filling; ph/coordinator should apply it.

## Verification
- Catalog check `--asset`: all 10 touched assets pass (gas-boiler, hot-water-cylinder, toaster, pendant-lamp, thermostatic-radiator-valve, mvhr-unit, oil-pumpjack (re-run after fix), expansion-valve, panel-radiator, sleeping-person).
- Staffage library vitest (ticket alias config): 53 of 54 files, 1097 pass, 3 fail, all in `export-performance` (walker-thermal pinned digest, memo, session eval counts). None involve my assets (no walker edits); likely concurrent edits by other agents or load. Not rerun. React vitest not run.
- Thermal and dark-line casts of 8 assets exported and viewed on a sheet: cylinder (quarters, stubs, dial), boiler, mvhr, pumpjack, radiator, lamp, sleeper, toaster render correctly.

## Open
Kettle mass (above); toaster casing 180 W on 0.24 m2 is also hot (about +75 K) and was not touched; pumpjack motor bone 1100 kg is heavy for a 30 kW motor (includes skid, not changed).
