# r2-rx: staffage audit of the non-energy series (excel, grasshopper, morph, nakagin, fit, typology, parametric, pets, course)

Paths relative to `A = 🎓️teaching/🏛️architecture/`. Method: read every helper of every clip script outside `⚡️energy`, `🎯️optimization` and `🏗️bim` (defs listed per file, then keyword sweeps for person, occupant, tree, car, furniture, lamp, radiator, silhouette, standin, figure, glyph, icon), then read the hits and rendered stills where unsure.

## Series covered

| Series | Clip code | Verdict |
|---|---|---|
| `📊️excel` (`📋️program`, `📍️references`, `🦴️anatomy`, `⚡️energy`, `🎨️visuals`, `🧱️model`) | `🎬️clip/*/🐍️.py` | one cast figure (program occupant), fixed below; rest is UI chrome, data diagrams and the Nakagin drawing |
| `🦗️grasshopper` (`🌳️trees`, `💊️capsule`, `🗼️tower`, `🦴️anatomy`, `🫗️casting`, `🎨️visuals/🦗️canvas`, `🧱️model`) | `🎬️clip/*/🐍️.py` | no figure; clean |
| `🔀️morph` | `🐍️.py` | generic morph helpers; clean |
| `🧲️fit` | `🐍️.py` | `building_fit` layout helper; clean |
| `🏢️nakagin` | `🐍️.py`, `🎨️visuals/🐍️.py` | building model geometry; kept (see N1) |
| `🏙️typology`, `📐️parametric` | LaTeX assignments (`📝️assignment`), PNG schemas, `icons.json` (capsule icons only) | no clips, no figures |
| `🐾️pets` | species JSON + `🟦️.ts` | their own product (twenty companions), not course clips |
| `📚️course` | site package | no drawings |
| `🧍️staffage` | the archive itself | out of scope |

## Verified claims of the earlier audits (by reading code, not by trust)

| Name | Where | What it really is |
|---|---|---|
| `_standin` | `🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py:516` | `_morph.silhouette(cap)` plus the window ring of a Nakagin capsule: the morph target of a casting token. Not a figure. |
| `figures` / `_figures` | `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py:40` | fills numeric values into narration text. Not a drawing. |
| `trees`, `_diagram`, `_mini` | `🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py:298, 733` | Grasshopper data trees drawn as a branching graph (paths, branches, items). A data-structure diagram; the staffage guard allowlists it as such. |
| `silhouette` | `🔀️morph/🐍️.py:92`, used in excel `📍️references`, `📋️program`, `🦴️anatomy` and grasshopper `🗼️tower`, `🫗️casting` | convex-hull outline of any mobject group as a morph source. Generic geometry, no person. |
| `_car` | no hit anywhere in this scope | the car cast lives in bim, not here. |
| program fitting rectangles (`bed`, `bath`, `storage`, `desk`, `sofa`) | `📊️excel/🎬️clip/📋️program/🐍️.py` `plan()`/`layout()`, `🦴️anatomy/🐍️.py:154-191` | width x length rectangles computed from the workbook rows, hatched and dimensioned: the data being summed in Excel, not entourage. Kept (same ruling as m7). |
| `padlock` | `📍️references/🐍️.py` `_padlock` | `$` lock UI icon of Excel absolute references. Kept. |
| GH ribbon / capsule legend icons | `🦗️canvas/🐍️.py`, `💊️capsule/🐍️.py:530` | Grasshopper UI widgets and abstract legend marks. Kept. |
| sun disc in GH anatomy | `🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py:288` | an orange tint circle on the window (Ladybug plugin teaser), diagram glyph. Kept. |

## Findings

### N1 Nakagin building model furnishing (kept, documented)

`🏢️nakagin/🐍️.py:1388-1445` `_podium_room_parts` (café tables and chairs, reception desk, bench, office desks, chairs, filing), `2440-2590` `_reference_interior`/`_study_interior` (floor bed, drawers, tub, toilet, TV, folding desk, sofa) and `🎨️visuals/🐍️.py:1675` (flat ground tile) build 3D boxes and cylinders inside the dimensioned, sourced building model. I rendered `podium_axon` (scratch probe `🧰️rx/rx_podium_probe.py`): the café furniture shows through the podium glass at a few pixels, exactly as a model part. Reasons to keep it:

- It is the building's own reconstructed program with `basis`/`layout_basis` metadata and semantic part names, consumed by occlusion, cutaway, grouping and dimension tests under `🏢️nakagin/🧪️tests`.
- The pipeline is a projected 3D mesh with painter sorting. A staffage cast is a 2D SVG and cannot take part in that ordering.
- Exterior assemblies of the 140 capsules omit all furnishings (`detail="exterior"`).

Open choice for the owner: drop the podium furnishing from tower views (`podium_axon` with rooms off) if no clip needs it.

### E1 Excel `📋️program` occupant: overlapping and unchecked (fixed)

`📋️program/🐍️.py` `plan(... people=True)` placed occupant `k` at `(iw - 0.3 - 0.4 k, il - 0.27)`. The plan cast `excel-program-occupant` (standing-adult, plan view, rotated -90 degrees so it faces into the room) is 0.49 m wide and 0.37 m deep at true scale (`staffage_height` 0.4896, unrotated width 0.3684), so the 0.4 m pitch made the two occupants of type C overlap (seen in my first render), and nothing checked the sofa or the net-floor edge.

Fix: new pure `occupant_spots(room, fits, size, count, margin=0.02, step=0.02)` (`📋️program/🐍️.py`, above `_span`). It scans from the window-wall corner down and leftwards on a 2 cm grid and returns the first corners that stay inside the net floor, miss every fitting rectangle and miss each other; it raises `ValueError` when the floor is full. `plan()` measures the figure from a probe cast (`probe.width / scale`, `probe.height / scale`) instead of hard-coded offsets, and copies the probe to each spot.

### Look at the rendered occupant (done, scratch scene `🧰️rx/rx_plan_probe.py`, 2560x1440 still)

- Type A, B, D: one occupant in the window-wall corner at the right, inside the net floor ring, clear of all fittings.
- Type C: two occupants side by side in the corner strip beside the window, touching at their arm tips but not overlapping, clear of sofa (top y 3.40 m) and storage.
- The figure reads as a person seen from above: shoulders, head with nose pointing into the room, arms hanging, feet forward. At 0.42 scale (the four small plans) it is small but recognisable. No change to the asset was needed.
- I killed my first attempt, a full `Beat3_Program` render (24 steps, 2.4 GB after 35 minutes of tower build without a frame); the scratch scene renders only the plan, which is the part that changed.

## Tests

- New test: `📊️excel/🎬️clip/🧪️tests/🧍️occupants/🐍️.py` with cases `📊️excel/🎬️clip/🧪️cases/🧍️occupants/🔣️.json`. It builds the real plan of each capsule type without a scene, takes the drawn Manim bounds of the floor, fittings and occupants, and checks with Shapely: the occupant count equals the model's `occupants`, every occupant lies inside the net floor, intersects no fitting and no other occupant (area < 1e-6), the rotated figure measures `staffage_height` by the unrotated width within 5 mm, and `occupant_spots` raises on a full room. Result: `Ran 4 tests, OK`.
- Guard: `V/🧪️tests/🐍️staffage_only`: 18 tests, 5 failures, all in energy clips, none in this scope: `❄️cooling/📐️system-sizing::_vent_unit`, `📊️demand/🏭️primary-energy::_meter_glyph`, `📊️demand/🧪️practice::_thermo`, `📊️demand/🧪️practice::_gas_meter`, `🧲️physics/🧲️fundamentals::_thermometer` are reported as primitive-built objects without an allowlist entry (concurrent energy edits by other agents). Every clip of the scope above passes the guard, and its allowlist already holds the grasshopper `present_trees`, `_table`, `window` entries.
- Excel presentation suite (`📊️excel/🎬️clip/🧪️tests/🎭️presentation`): see the end of this file.

## Still open

- N1 decision on podium furnishing (above).
- The five energy guard failures belong to the energy agents.
- Plan-view occupants of the other types in the program clip still use the same relaxed pose; a seated or sleeping plan figure would be nicer in the bed fitting but the program clip counts users, not postures.

## Excel presentation suite result

`🎭️presentation/🐍️.py`: 19 tests OK (the whole `WorkbookToolkit` class plus seven `SceneContract` tests: registration, narration/title/steps, program hook, imports, dependency closure, custom play, observer). `test_native_lifecycle` renders a scene and ran for 25 minutes at 3.4 GB, so I killed it; it is unverified. It does not exercise `plan()`.

## Files

- Edited: `📊️excel/🎬️clip/📋️program/🐍️.py` (`occupant_spots`, `plan`).
- New: `📊️excel/🎬️clip/🧪️tests/🧍️occupants/🐍️.py`, `📊️excel/🎬️clip/🧪️cases/🧍️occupants/🔣️.json`.
- Scratch scripts: `🧰️rx/` (`rx_plan_probe.py`, `rx_podium_probe.py`, `rx_size.py`, `rx_layout.py`, `rx_still.sh`, `rx_py.sh`, `rx_crop.py`, `rx_render.sh`). Generated output deleted.
