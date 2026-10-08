# 🤝️ Handoff to the inference tree: what the solids, plan and quantities must do with the new authored parameters (r7, from `z-depth`)

Audience: `z-graph` or a later agent. `z-depth` changed the AUTHORED model (snapshot, diff, patches, leaves, fixtures, IO) and the
framework geometry; it did NOT change the inference tree except the mechanical edits listed in section 0. Nothing below is implemented
yet; until it is, the solids and the plan ignore the new fields and every committed inference fixture keeps its old expected values.
Paths: `S` = `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any`, `I` = `S/🧬️schema/💡️inferences`, `E` = `I/🧊️element-solids`.
API of the skeleton and roof surfaces: `r7-api-skeleton.md`.

## 0. Edits already made inside the inference tree (mechanical, compile-keeping)

* `I/🪟️opening-frames/🦀️.rs` `resolve_size`: `sill: opening.sill_override.unwrap_or(kind_sill)` (F27 fixed at the source) and the two
  doc lines. All other readers (`quantities`, `plan-linework`, `walls`, `fillers`, IFC export) go through the frame or through this
  function; their python oracles (`S/🧪️tests/{🪟️infer-bim-1-opening-frames,🧮️infer-bim-1-quantities,🗺️infer-bim-1-plan-and-diagnostics,🔳️infer-bim-1-wall-solids}/🐍️.py`)
  now read `opening.get("sill_override", type_sill_or_0)`.
* Struct literals in unit tests of `quantities`, `spaces`, `element-solids`, `stair-runs` (`Opening`, `Stair`, `Railing`, `Storey`
  gained fields): `sill_override: None`, `cut_height: None`, and the standard construction fields (`crate::STANDARD_*`).
* Committed inference fixtures were migrated by hand (`r7-z-depth-migrate.py`): `Opening.sill` (ADDED to the type sill) became
  `sill_override` (REPLACES it): old `0` -> absent, old `x` on a window -> `type_sill + x`, on a door or void -> `x`. Resolved heights
  in every fixture are unchanged. Stairs got `landing_depth = width`, railings the standard sections.

## 1. Openings (F27, done) — what is left

Semantics now: `sill_override: Option<f64>` replaces the sill of the type for windows (`None` -> `WindowType.sill`) and is the sill
of doors and voids (`None` -> 0). Frames, cuts, quantities and plan already follow. IFC export writes `frame.sill`; IFC import
produces `Some(sill)` only when the imported absolute sill differs from the type sill. Remaining: nothing, except the F06 hole
validity rule (unchanged).

## 2. Stairs: stringer, nosing, tread thickness, risers, landing depth

New authored fields of `Stair` (all required, validated in `create-stair`/`set-stair`): `stringer: StairStringer{kind: None|Closed|Open|Mono,
width, depth}` (width, depth > 0 unless kind is None; metres), `nosing >= 0` and `< min_tread`, `tread_thickness > 0`, `riser: Open|Closed`,
`landing_depth > 0`. Standard values of a new stair: stringer None 0.05 x 0.25, nosing 0, tread 0.04, Closed, landing = width.

* `I/🪜️stair-runs`: landings take `depth = stair.landing_depth` (today `width`), both in `layout()` (L-turn `StairLanding.depth`, U-turn)
  and the U-turn centre (`aside(... , (width + gap) / 2)` stays; the landing rectangle is `landing_depth` along the arriving
  flight x `width` / `2 width + gap` across). The run lengths of L/U stairs change accordingly (the second flight starts after the landing):
  `second_start = along(centre, turned, width / 2)` becomes `along(foot_of_landing + landing_depth, ...)` — pick one consistent rule and
  update the oracle `🪜️infer-bim-1-stair-runs` and its fixtures (they currently encode depth = width; the migrated stairs have
  `landing_depth == width`, so NOTHING changes for them, the new freedom only shows with other values).
* `E/🪜️stairs` (`steps_of`, `stair_geometry_of`, `stair_solid`): replace the monolithic prisms by parts, per flight in the flight frame
  (`u` along travel from `flight.start`, `w` across from `-width/2` to `width/2`, `z` up from `flight.base_z`), `g = flight.tread`, `h = run.riser_height`,
  `t = tread_thickness`, `n = nosing`, riser `k` (0-based) stands at `u = k g`, tread `k` (`k = 0 .. treads-1`) has its top at `z = (k + 1) h`:
  * `parts::STEP` (tread slab): box `u in [k g - n, (k + 1) g]`, `w` full width, `z in [(k+1) h - t, (k+1) h]`; volume `(g + n) * width * t`.
    Winder wedges: same slab thickness `t`, outline extended by `n` against the travel direction at the inner arc (wedge outline unchanged, nosing ignored on winders — document).
  * `parts::RISER` (new part name; only when `riser == Closed`): board `u in [k g, k g + t]`, `z in [k h, (k+1) h - t]` (height `h - t`), full width.
    The first riser (`k = 0`) rises from the flight base; the last riser of the last flight (`k = treads`) closes against the upper floor and is the arrival edge, not a part.
  * `parts::STRINGER` (new): `Closed`: two boards outside the treads, `w in [width/2, width/2 + sw]` and `[-width/2 - sw, -width/2]`; side profile = the band
    between the pitch line through the tread noses (`z = (k+1) h` at `u = k g - n`) and the same line lowered by `depth` (measured vertically), cut vertical at
    the flight start (`u = -n`) and end. `Open`: the same band's upper edge replaced by the saw tooth under the treads (treads rest on the notches), boards inside the
    tread width: `w in [width/2 - sw, width/2]` and its mirror. `Mono`: one board `w in [-sw/2, sw/2]`, band lowered by `depth`. `None`: no part. `sw = stringer.width`.
  * Landings: slab `landing_depth x width` (U-turn: `landing_depth x (2 width + gap)`), thickness `t`, top at `landing.z`; under `Closed`/`Open` stringers the landing is
    carried by the same boards (extend them along the landing outline) — optional, document the choice.
  * A stair with a single riser still has no tread and is absent. Spiral stairs: treads as wedges of thickness `t`; stringer kinds other than `None` are out of scope
    for spirals (emit a `Warning` diagnostic `stair.stringer-ignored-on-spiral`).
  * Quantities (`I/🧮️quantities`): stair volume = sum of the parts (it was the sum of prisms); add per-part counts if useful (`tread_count`, `riser_count` already inferred).
  * Plan (`I/🗺️plan-linework/🪜️stairs`): outline of treads unchanged; draw the stringer outlines as hidden lines when `kind != None`; arrow and cut line unchanged.
  * Oracles: `S/🧪️tests/📦️infer-bim-1-solids-rest/🐍️.py` computes stair prisms from the run; extend it with the formulas above (tread slab volume
    `(g + n) * width * t`, riser board `(h - t) * width * t`, stringer band area x `sw`).
* Diagnostics: `S/⚠️diagnostics` has no stair-construction rule; the authored validation already refuses impossible values, so nothing needed except the spiral note.

## 3. Railings: profile, post profile, baluster row, infill

New authored fields of `Railing`: `profile: Profile` (top rail), `post_profile: Profile`, `baluster: Option<Baluster{profile, spacing}>`, `infill: Infill`
(`None | Glass{thickness} | Panel{thickness}`). Profiles use `validity::profile_problem` (Rectangle, Circle, IShape, Custom); positive `spacing`/`thickness`.
Standard: rail Rectangle 0.06 x 0.04, post Rectangle 0.05 x 0.05, no balusters, no infill — i.e. exactly the former constants `RAIL_WIDTH`, `RAIL_DEPTH`, `POST_SIZE`.

* `E/🛤️railings` `railing_geometry`: delete the three constants. Section convention (keep): profile `width` is across the path (centred), `depth` is vertical and hangs
  below the rail top at `top_z`; a Circle has diameter = both; IShape/Custom: use `column`-style `profile_loop` (see `E/🏛️columns`) with `x` across and `y` the height, shifted so
  the section's highest point touches `top_z` (the same rule beams use). Posts: `post_profile` extruded (as columns do) at every post position, rising to the underside of the
  rail (`top_z - rail section height`); orientation = segment angle.
* Balusters (`parts::BALUSTER`, new): between posts, on every segment `n = ceil(len / spacing)` stations at `j * len / n` for `j = 1 .. n - 1` (interior of the segment, never on a post),
  each the `baluster.profile` extruded from `base_z` to the rail underside. They are NOT placed when `n <= 1`.
* Infill (`parts::INFILL`, new, material = the railing material for `Panel`, glass for `Glass`: use the model material named `Glass` category if one exists, else the railing material):
  one slab per segment between the end posts (inset by half the post width at both ends), `thickness` across the path centred on it, from `base_z + 0.05` to the rail underside.
  Glass and balusters may coexist (balusters in front of nothing: authors choose; no automatic exclusion).
* Quantities: railing `length` (exists), add `infill area` (segment length x infill height) and baluster count when present.
* Plan: rail and posts as today; infill drawn as a thin line along the path; balusters as dots (optional).
* F26 (railing defaults silent) is closed by authoring: no diagnostic needed.

## 4. Roofs: skeleton surfaces, fallbacks reported (F14)

`E/🏠️roofs` today: exact only for CONVEX straight footprints (plane envelope) and flat/shed on anything; concave, curved and invalid pitch fall back to flat silently.
New: `semio_framework_geometry::roof::{roof_surface_controlled, Roof}` (see `r7-api-skeleton.md` section 2 and 4).

1. Straight footprints (every `bulge == 0`, with or without holes — roof footprints currently have none): build the eave ring set (footprint grown by `overhang` with `loops::offset`, mitred),
   call `roof_surface_controlled(rings, &Roof::{Hip|Gable|Mansard}, control)`. The surface heights are relative to the eave plane: add `storey top + base_offset`.
2. Layers: layer `j` = `surface.shell(t_j)` translated down by the thickness of the layers above it (the shell is closed: eaves and verges get vertical edge walls). `Mansard`
   works the same (the break lines are in `surface.lines`). Gable ends are not roof material; their `RoofFace { vertical: true }` polylines give the top profile for a trimmed wall
   (optional, wall tops under a gable roof can read `surface.faces` of zone 0).
3. Ridge / hip / valley lines for the plan: `surface.lines` with `RoofLineKind` (replaces the hand-built `RoofLine` list; keep the existing plan styles: ridge and hips solid, valleys dashed, breaks dotted).
4. Shed and Flat: keep the existing code (single plane).
5. `Gable { pitch, ridge_direction }` on non-rectangular footprints: `gable_pitches` makes every edge across the ridge vertical. For a T or cross footprint this is physically wrong (a vertical edge
   next to a reflex corner makes the surface step). Rule: if any vertical edge has a reflex endpoint (`cross(d_in, d_out) < 0` at either end), fall back to `Hip` with the same pitch and report a
   `Warning`; never silently.
6. Diagnostics (F14): add rows `roof.fallback-flat.curved-footprint`, `roof.fallback-flat.degenerate-footprint`, `roof.fallback-flat.invalid-pitch`, `roof.gable-ends-adjust-to-hip` (en and de) in
   `I/⚠️diagnostics/💬️messages` and emit them from the roof solid's `fallback` field; the error mapping is in `r7-api-skeleton.md` section 4.
7. Cancellation: pass the inference `control` through (the skeleton polls once per event).
8. Oracle: extend `🧊️infer-bim-1-solids-rest` roofs with concave hip/valley cases: expected plan area and `surface_area = plan_area / cos(pitch)` for uniform pitch, ridge height `= half-width * tan(pitch)`
   for strips, and the python skeleton fixtures of the framework for the face areas.

## 5. Storeys: plan cut height

`Storey.cut_height: Option<f64>` (metres above the storey elevation, positive; `None` = the default `validity::DEFAULT_CUT_HEIGHT = 1.2`).
* `I/🗺️plan-linework/🦀️.rs`: replace `pub const CUT_HEIGHT: f64 = 1.2` by `storey.cut_height.unwrap_or(DEFAULT_CUT_HEIGHT)` where the plan resolves its cut (`cut_elevation = storey elevation + cut`).
  `PlanLinework.cut_height` keeps reporting the value used. The storey record is part of the plan's `storey_scope`, so editing `cut_height` re-infers exactly that storey's plan; add the gating test
  (`infer_field_after_diff` of a `SetStoreyCutHeight` touches `storeys/<id>/cut_height`) and the parametric test "raising the cut above a sill closes the window gap in the poche".
* The editor measure `measure_cut_height` and the plan window config should initialise from the storey value.

## 6. Beams: `top_offset` (decided, documented)

Signed metres, positive = the top of the beam lies ABOVE the storey top, negative = BELOW, zero = flush. This is what the solids (`E/➖️beams`, `top_z = storey top + top_offset`), the IFC
export/import and `📦️bodies` already do; the entity doc, the JSON schema description, the design doc and the tests now say so. Validation stays "finite" (any signed value is meaningful:
upstand beams exist). No inference change needed. The plan comment in `I/🗺️plan-linework/🏛️members` ("hangs below the storey top") is imprecise: beams are dashed when their top lies at or below the cut.

## 7. Checklist for the implementer

1. Stair parts + oracle formulas; stair-runs `landing_depth`.
2. Railing parts; remove constants; quantities.
3. Roof solids on the framework API; diagnostics rows; fixtures with concave footprints.
4. Plan cut height + gating test.
5. Re-run `bun r3-f1-gen-oracle.ts` / `r3-f1-gen-feature.ts` only if leaves change (they did not after `z-depth`).
