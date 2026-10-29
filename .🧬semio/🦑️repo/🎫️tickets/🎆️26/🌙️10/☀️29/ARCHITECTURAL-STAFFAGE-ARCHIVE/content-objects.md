# 🪑️ Objects content: per-asset notes

Category file: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/🔣️.json` (`category: objects`, 8 assets, all passing `🩺️check`).
Verification: `repo tools exec bun -- …/🩺️check/🟦️.ts --file … ` prints ✓ for all eight; 1:100 light-line previews of every pose in `🗑️generated/objects/`.

Manifest sync needed by the coordinator (not edited here): all eight assets have `completion: "authored"` while `🗂️catalog/🔣️.json` lists `planned`; `height-adjustable-desk` is 1.18 m (manifest 1.2), `office-swivel-chair` width is 0.60 m (manifest 0.65, within 10 %), `playground-swing` is 2.35 × 2.63 m (manifest 2.3 × 2.6, within tolerance). All pose and clip ids equal the manifest; no renames.

Conventions: masses = nominal section or volume × density (`literature`, Wood Handbook / EN 1993-1-1 / EN 1999-1-1 / Ashby 2011); occupant masses, equipment, gust drag and uplift, castor and plume values are `authored-estimate`. No physical measurement was invented. Legends and text are `{en, de}`.

## 1. park-bench (Parkbank)
- Size: 1.80 × 0.85 m, mass 60.9 kg (end frames 2 × 12.6 kg steel; seat slats 20.4 kg oak; backrest slats 15.3 kg oak).
- Rig: `structural`, root `ground` (fixed), fixed members `end-left`, `end-right`, `seat`, `backrest`. No moving parts (static object).
- Poses: `unloaded`, `occupied-one` (occupant-1 at x 0.9 m, 735.5 N), `occupied-two` (occupant-1 at 0.5 m, occupant-2 at 1.3 m). Load states, not motion.
- Clips: `occupant-sit-down` (2 s; load-on events; `load:occupant-2` ramp; both feet in contact), `sun-exposure-cycle` (60 s loop; exposedFraction tracks on the three passive regions; illustrative scenario).
- Contacts: `contact-left`, `contact-right`; footprint 0.22 × 0.05 m, μ 0.6.
- Thermal: `seat-timber`, `back-timber` (oak, passive), `steel-frame` (painted steel, passive); all bound.
- Simplifications: backrest drawn upright (no tilt); end frames as simplified steel profiles; no member sections; occupant loads are point loads.

## 2. cafe-chair (Café-Stuhl)
- Size: 0.45 × 0.88 m (depth 0.5 in dimensions), mass 4.5 kg (legs 2 × 0.7 kg, stretcher 0.2, beech seat 2.4, back 0.5).
- Rig: `structural`, fixed members; legs are pairs projected onto one elevation.
- Poses: `upright-empty`, `pulled-out` (root x −0.25 m), `tilted-on-two-legs` (root rotated 12° about the rear feet pivot (0.05, 0); root x 0.001093, y −0.010396; only `contact-rear` active).
- Clips: `pull-out-and-sit` (2 s; motion only — no occupant load exists in any pose, so sitting is not represented), `tip-back` (3 s; front contact off 0–0.6 s and on from 2.2 s).
- Thermal: `seat-beech`, `back-beech`, `steel-tube` (all passive, bound).
- Simplifications: near and far legs drawn as one projection; backrest post modelled as a single beech shape; no bending check; `supports` empty so contacts come only from pose/clip.

## 3. office-swivel-chair (Bürodrehstuhl)
- Size: 0.60 × 1.10 m (rigid castor star), mass 15.4 kg (base 5.5, castors 5 × 0.15, column 1.6, seat 4.5, backrest 3.0 — authored estimates).
- Rig: `mechanical`, root `base` at y 0.025; five arms as elevation projections of plan arms at 0/72/144/216/288° (x = 0.30, 0.093, 0.093, −0.243, −0.243 m); castors revolute unlimited; `column` prismatic 0–0.10 m (gas lift); `seat` fixed child of column; `backrest` revolute 0–20° (recline).
- Poses: `seat-low` (default), `seat-high` (column 0.10 m), `reclined` (column 0.04 m, backrest 18°).
- Clips: `swivel-rotation` (4 s; the turn about the vertical axis is NOT visible in elevation — represented by motion events only), `height-adjust` (6 s; 0 → 0.10 m → 0).
- Contacts: five castor contacts (footprint 0.015 × 0.010 m, μ 0.5), all in all poses.
- Thermal: `chair-plastic`, `chair-fabric`, `chair-steel`, `castor-rubber` (tyre kind, passive; all bound).
- Simplifications: armrests and headrest omitted; swivel not drawn; no occupant load in pose set; plan arms projected (true arm length 0.30 m).

## 4. height-adjustable-desk (Höhenverstellbarer Schreibtisch)
- Size: 1.60 × 1.18 m (default `standing-height`), mass 32.4 kg (feet 2 × 2.8 steel, columns 2 × 3.0, top 20.8 kg laminated particleboard, 650 kg/m³ literature range).
- Rig: `mechanical`, root `foot-left`; `foot-right` fixed; `column-left` and `column-right` prismatic 0–0.43 m, axis 90°; `top` fixed child of `column-left`.
- Poses (both columns set identically): `seated-height` (dy 0 → top 0.72 m), `mid-height` (dy 0.215 → 0.935 m), `standing-height` (dy 0.43 → 1.15 m). Equipment loads on the top in all three poses: `monitor` 78.5 N, `laptop` 19.6 N.
- Clips: `raise-and-lower` (8 s; sit → stand → sit with smooth ease), `load-on-surface` (4 s; `load:laptop` ramp).
- Contacts: two T-foot contacts (footprint 0.40 × 0.06 m, μ 0.5).
- Thermal: `top-board`, `steel-desk` (passive, bound).
- Simplifications: tree allows the top to be carried only by `column-left`; `column-right` follows by identical pose offsets (kinematic coupling, not a constraint). The monitor is a load without a drawn shape (its shape would breach the 1.2 m height check); the laptop has a shape but no region.

## 5. street-lamp (Straßenleuchte)
- Size: 1.20 × 6.00 m, mass 88.1 kg (base plate 14.1 kg steel, pole 70 kg authored estimate, arm 1.5 kg alu, head 2.5 kg). Plinth mass excluded (in-ground / foundation).
- Rig: `mechanical`, root `foundation` (fixed); `pole` revolute ±2° about the base (equivalent rigid sway, authored); `arm` fixed (0.95 m, aluminium) at 5.80 m height; `head` fixed luminaire.
- Poses (all bind `operating.lamp` and `emitters.lamp-plume` explicitly): `dusk-off` (0/0), `lit-night` (1/1), `lit-in-wind` (1/1, pole −1.8°, distributed wind 41 N/m on pole 0–5.88 m, 110 N on the head; sources `authored-estimate`, basis EN 1991-1-4).
- Clips: `ignition-at-dusk` (30 s; `operating:lamp` as `step` at 14 s; `emitter:lamp-plume` ramp; ignition event), `pole-sway-in-gust` (8 s; sway and wind-load tracks).
- Contact: `foundation-support` (fixed support, footprint 0.3 × 0.3 m).
- Thermal: `pole-galv`, `plinth-concrete`, `arm-alu`, `lamp-housing` (operating, `riseK` 18 K, key `lamp`), `lamp-lens` (glazing); emitter `lamp-plume` (plume, 35 °C, 0.15 m/s, 0.06 m, upward).
- Uncertainties: lamp `riseK` 18 K, plume strength, pole sway 1.8°, wind coefficients, and pole mass are authored estimates.

## 6. market-parasol (Marktschirm)
- Size: 2.60 × 2.40 m (default `open-still`), mass 66.7 kg (concrete base 57.6 kg from 0.4 × 0.3 × 0.2 m at 2400 kg/m³; mast 3.2; runner 0.4; two visible ribs 2.75 kg each including panel share — represent four ribs per side).
- Rig: `mechanical`, root `base` (fixed); `mast` revolute ±10° (tilt joint); `runner` prismatic along the mast, −0.30 to 0 m; `rib-right` (0–100°) and `rib-left` (−100–0°) revolute children of the runner.
- Poses: `open-still` (rest), `tilted-to-sun` (mast −8° towards the sun on the right; gust drag 254 N and uplift 106 N at the runner, authored estimates basis EN 1991-1-4), `furled` (runner −0.25 m, ribs ±100°).
- Clips: `wind-flap` (6 s; gust tracks; rib flutter ±4° through bone overrides), `sun-tilt` (8 s; tilt without wind via zero load tracks).
- Contact: `socket` fixed support (ground socket, footprint 0.4 × 0.3 m).
- Thermal: `canopy-fabric` (polyester, passive, 5.3 m²), `frame-alu`, `base-concrete` (passive, all bound).
- Simplifications: free-base sliding/tipping not checked (ballast data unknown); panels are rigid sectors on the ribs; only two of eight ribs drawn.

## 7. waste-bin (Abfallbehälter)
- Size: 0.51 × 1.01 m, depth 0.5 m, mass 5.7 kg (shell 4.5 kg HDPE, lid 1.2 kg).
- Rig: `mechanical`, root `body` (fixed); `base-plate` fixed (bin floor, no mass); `lid` revolute 0–110° about the rear hinge at (−0.25, 0.98) m.
- Poses: `closed-empty`, `lid-open` (lid 100°), `full-load` (distributed `waste-contents` 40 kg (980 N/m) on `base-plate` 0–0.4 m; kind `live`; authored estimate).
- Clips: `lid-open-close` (3 s), `waste-fill` (6 s; waste load ramp; load-on event).
- Contacts: two pedestal feet (footprint 0.12 × 0.03 m).
- Thermal: `bin-hdpe`, `lid-hdpe` (passive HDPE, bound).
- Simplifications: waste not modelled as a volume; lid shape is a flat plate; pedestal and shell as simple polygons.

## 8. playground-swing (Spielplatzschaukel)
- Size: 2.63 × 2.35 m, mass 54.3 kg (A-frame legs 2 × 11.7 kg, brace 5.7, beam 20, chains 2 × 1.2, seat 2.8 rubber).
- Pendulum: L = 1.9 m, T = 2π√(L/g) = 2.7656 s at g = 9.80665 m/s² (`halliday` reference). Amplitude 30° (authored).
- Rig: `mechanical`, root `ground`; `leg-left`, `leg-right`, `brace`, `beam` fixed; `chain-left`, `chain-right` revolute ±60° from the beam pivots (x ±0.15 m); `seat` revolute hitch ±60° child of `chain-left` (counter-rotated by the pose so the seat translates).
- Poses: `rest` (child 245.2 N), `pendulum-forward` (±30°, seat −30°, child plus 80 N push), `pendulum-back` (−30°, seat +30°).
- Clips: `pendulum-swing` (loop, T = 2.7656 s; 13 keys at T/12 steps with θ = 30 cos(2πt/T), `smooth` ease as coordinator asked; seat at −θ), `occupant-push` (4 s; `load:child-push` track).
- Contacts: `pin-left` (support pin) and `roller-right` (roller, normal 90°), footprint 0.1 × 0.1 m.
- `structure.dynamics: inertial` (segment accelerations from the clip).
- Thermal: `frame-galv`, `chain-steel`, `seat-rubber` (passive, bound).
- Simplifications: the brief's double pendulum is reduced to a one-DOF pendulum with a rigid occupant; the second chain is kinematically coupled by pose/clip angles, not constrained; the seat is only partly visible in some poses at 1:100 (a thin plank); the chains are drawn as capsules without links.

## Uncertainties and estimates
- Authored estimates (tag `authored-estimate` in the file): occupant masses (75 kg, 25 kg child), equipment loads, gust loads (parasol, lamp), lamp riseK and plume, castor and office masses, parasol and swing thermal areas, pole mass and sway amplitude.
- Pose/clip ids equal the manifest. No pose or clip was renamed.
- `🩺️check` validates semantics and runs the structural and thermal solvers; it does not validate the JSON Schema. JSON parses (`jq -e`).
- Preview CLI (`🖨️preview`) is still empty; previews were rendered with a scratch script (`🧰️scripts/objects-preview.ts`) that calls `depict`, and writes SVG and PNG to `🗑️generated/objects/`. Not every pose was viewed at 1:50 and 1:200; reviewed at 1:50 (bench) and 1:100 (lamp, parasol, swing, cafe chair, office chair, others only rendered).
