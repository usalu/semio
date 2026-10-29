# Content Authoring Guide (all content workers)

S = `/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage`. Types: `S/🧬️schema/🟦️.ts` (normative JSON: `S/🧬️schema/🔣️.json`).
Template: `S/🧫️fixtures/🚶️reference-walker/🔣️.json` (a minimal, valid, but deliberately simple asset — catalog assets must be far richer).

## Files

- One `CategoryDocument` per category: `S/🗂️catalog/<emoji><category>/🔣️.json` with `"$schema": "../../🧬️schema/🔣️.json#/$defs/CategoryDocument"`,
  `"version": "semio.staffage/v1"`, `"category"`, `"assets": [...]`. Only the category's worker edits it.
- Do NOT edit `S/🗂️catalog/🔣️.json` (manifest); the coordinator syncs it from the assets. Use the manifest's asset ids; you may rename
  pose/clip ids if better, and report them.
- Helper/generator scripts are NOT allowed to produce the final content: everything is handcrafted JSON. You may use scratch scripts in
  the ticket folder `🧰️scripts/` to compute coordinates (e.g. mirroring, unit conversion), but you author and review the result.

## Tools

- Full check (validation + evaluation at every pose and 9 instants per clip + thermal + structure + all 7 views):
  `repo tools exec bun -- S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file S/🗂️catalog/<cat>/🔣️.json [--asset id]` → must print ✓ for every asset.
- Visual preview: `S/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts` (see `T/depiction.md` for usage once written; `--sheet` renders all poses ×
  7 views). Write PNGs to `T/🗑️generated/<your-category>/` and LOOK at them with the Read tool. Iterate until silhouettes are convincing at
  1:50 and still legible at 1:200.

## Quality bar

- Side elevation (`elevation`, x right, y up, metres, ground y = 0). Recognisable silhouettes: proper proportions (humans ≈ 7.5 heads;
  children ≈ 6 heads), clothing outlines, hair, shoes, carried objects; trees with authored trunk → branch hierarchy (≥ 12 bones), crown
  masses as layered foliage paths; animals with correct anatomy and gait (digitigrade dog/cat, unguligrade horse/deer/cow, bird);
  vehicles with wheels as rotating bones, doors/booms/masts as joints; components with structural members drawn to section size.
- Use `path` shapes with cubic curves (`C`) for organic outlines; capsules/ellipses only where appropriate. Line hierarchy: `body`
  for the silhouette, `detail` for seams, faces, panel joints, window mullions, `texture` (lod ≥ 2) for bark, fabric folds, foliage
  texture, tread. `glazing` for glass. Give `lod` 0 to the silhouette, 1–3 to progressively finer details. `tone` 0..1 for solid views.
- Every shape carrying physics gets `region` (thermal) and `material`.
- Rig per category: humans/animals anatomical (pelvis root → spine → chest → neck → head; shoulders → upper arm → forearm → hand;
  hips → thigh → shank → foot), joint limits realistic (knee 0…150° flexion one-way, elbow 0…150°), both sides; vegetation botanical
  (trunk → limbs → branches → foliage masses, revolute with small limits for sway); machines mechanical (chassis root, wheels revolute
  unlimited, booms revolute with limits, masts prismatic); components structural (root fixed; members; moving parts only where real:
  sliding door prismatic, tilt sash revolute). Never invent anatomical joints for static objects.
- Masses: humans by segment fractions (de Leva 1996 / Winter 2009, tag `literature`); animals scaled estimates (`authored-estimate`);
  objects/vehicles from typical specs; components from section × density (`literature` for densities). Sum ≈ `dimensions.massKg`.
- Contacts/supports: feet/paws/hooves/wheels with footprints and friction; components with pin/roller/fixed supports as built.
- Poses ≥ 3, meaningful (different activities / configurations / load states), joint limits respected. Clips ≥ 2 with phases, events
  (contact-on/off, inhale/exhale, ignition, load-on…), contact spans that always keep the body supported unless a flight phase is
  physically real (running), `rootMotion: "plant"` for locomotion with `advanceM` = distance per cycle, tracks for activity/emitters/loads.
- Thermal: regions per exposed skin / clothing / hair / fur / foliage / bark / engine / exhaust / glazing / surfaces with areas that sum
  to a plausible total surface (humans ≈ DuBois area), `metabolicShare`s summing ≈ 1 for bodies; emitters: breath at the mouth/nostrils
  with `periodS` ≈ 4 s (adult rest), duty ~0.4…1.0 exhale fraction, faster when active; exhaust at the tailpipe with engine temps;
  tree transpiration emitters as `carries: "vapour"`, `kind: "transpiration"`. Every parameter has `source` + `reference` id resolving
  to `provenance.references`. Poses/keys bind `thermal` (activityMet: rest 1.0, walking 2.0–2.6, running 6–8; operating levels; region
  overrides such as covered areas when seated; emitter strengths).
- Loads: carried objects as `carried` loads at the hand anchor (and/or as bone masses), wind on umbrellas/parasols/trees (`wind`),
  imposed/live loads on components per EN 1991-1-1 (balcony 4 kN/m², stairs 3 kN/m², barrier line load 1 kN/m) — tag `literature`.
- Titles/descriptions in natural English and German. `completion: "authored"`.
- Distinctness: assets in a category must differ in geometry, rig, function and physical data; no recolours or renamed duplicates.

## Engine facts (from the core worker, binding)

- Use ease `"smooth"` on clip keys of continuous motion: it is C1 (Catmull-Rom through neighbouring keys, loop-aware), so
  accelerations stay finite for the inertial structural model. Use `linear` only for constant-velocity segments, `step` for switches.
- Emitters and operating levels a pose does not bind default to 0 — bind `thermal.emitters` (and `operating`) explicitly in EVERY pose.
  `activityMet` defaults to `thermal.baseMet ?? 1`.
- Region `metabolicShare`s must sum to 1 (validator code `metabolic-share`); every region must be bound by ≥ 1 shape (`region-unbound`).
- A contact span crossing the loop seam must be written as two spans `[a, duration]` and `[0, b]`; double-support spans overlap.
- Locomotion `advanceM` must equal the plant-solved advance per cycle (validator `clip-advance`, tolerance 2 mm).
- Contact anchors at the ball of the foot / tyre patch centre, not the ankle; give every contact a footprint (length × width) — the
  solver lets a footprint carry a centre-of-pressure moment up to N·length/2.
- `structure.dynamics`: use `"quasi-static"` by default (honestly labelled). Use `"inertial"` only for clips whose keys are dense and
  smooth enough that the check stays ✓ (locomotion at real speed with ≥ 8 smooth keys per cycle); never fake it.

- Mark in-between gait/motion key poses with `"key": true` (schema field): they stay evaluable for clips but are not offered as
  named poses. Every asset needs ≥ 3 NAMED poses (key absent) that are meaningful configurations a user would pick.

- Airborne phases (running flight, jumps, take-off) carry `"ballistic": true` on the clip phase; unsupported instants elsewhere fail.
- Continuous rotation (wheels, cranks, rotors) loops: keep the closing key at t = duration equal to the first key + 360°·k on
  revolute joints without limits (engine update in progress — use it for vehicles' drive loops).

## Done means

The check prints ✓ for every asset; preview sheets reviewed for every asset (all 7 views at 1:50 and 1:200); write a short
`T/content-<category>.md` with per-asset notes (dimensions, pose/clip ids, physics sources, known simplifications).
