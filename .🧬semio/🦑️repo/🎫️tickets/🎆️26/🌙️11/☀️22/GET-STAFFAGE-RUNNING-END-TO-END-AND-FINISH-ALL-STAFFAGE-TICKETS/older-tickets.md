# 🗄️ The Four Older Open Staffage Tickets: State on 2026-11-23

Finisher report for the umbrella ticket. `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26`, `U` = this folder. Everything below marked "today" was measured between 03:55 and 05:35 on 2026-11-23 with the pinned bun, one process at a time (one exception, stated in section 7); scripts are in `U/older/`, raw output was deleted. Nothing was run through `repo run` except the one bounded measurement of section 5.4. The repo MCP server (`semio`) did not connect in this session; no ticket was opened, closed or reopened.

## 1. Verdicts

| # | Ticket | Verdict |
|---|---|---|
| 1 | `🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE` | **Can close.** Every point of its description holds on today's catalog or was taken further by a later ticket. |
| 2 | `🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE` | **Can close.** Its deliverables hold for all 375 assets. Its guard `🧊️three-views` is red for 28 assets today; each of them was redrawn or added on 2026-11-22 under a ticket that is still open and carries it (section 3.3). |
| 3 | `🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE` | **Cannot close**, because (a) two reference cases of its own regression suite `🔭️multiview-coverage` are red and need a decision of the owner of the structural view and a redraw of `walking-commuter` (both in `U/requests.md`), and (b) no coverage matrix without unexplained failures is on record: the last recorded full matrix (2026-11-02) had 8,068 failing cells of 37,233, a sample of three small assets today has 79 failing cells of 777 with three named codes, and a full run over today's 375 assets takes many hours in one process (section 4.3). |
| 4 | `🌙️11/☀️10/ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE` | **Can close.** The four gaps of the reopening were closed on 2026-11-11/12 with evidence in the ticket; DWG is re-measured green today, the silhouette and model guards of today are stricter than the ones of this ticket and are the acceptance of the open 3D ticket of 2026-11-14 (`U/perfect-3d.md`), the Python environment and `repo run` could not be re-measured to the end today for reasons outside the ticket (sections 5.2, 5.4). |

## 2. Ticket 1: Architectural Staffage Archive

No acceptance list exists in the ticket besides its description (`📌️important/📝️.md` is empty); the points are the clauses of the description. Its last own audit (`completion-audit-2026-10-30.md`) was green for 107 assets.

| Point of the description | State today | Evidence |
|---|---|---|
| 48+ handcrafted assets in six categories | holds | `U/older/measure-views.py`: 375 manifest entries = 375 asset folders, all `completion: "authored"`; humans 75, vegetation 67, objects 58, components 58, furniture 39, appliances 39, machinery 16, animals 14, vehicles 9 |
| Typed versioned schema, SI dimensions, drawing scales | holds | `S/🧬️schema/🔣️.json` `semio.staffage/v2`, presets 1:500, 1:200, 1:100, 1:50; `🧬️schema-twin` 3 of 3, `🧊️volume-schema` 115 of 115, `🧭️representation-resolution` 8 of 8 today. `🧬️schema-conformance` 42 of 43: the one red case names `mobility-scooter-user` and `elderly-person-knitting` (`provenance.notes` is a string), people of today's wave, not mine |
| Seven synchronized views | holds | `VIEWS` has seven modes; `🎨️seven-modes` 6 of 6, `🎨️view-synchrony` 15 of 15 today |
| Rigs, poses, clips | holds | 2,097 poses (1,465 named in the manifest) and 930 clips; every asset has at least one clip |
| Asset-specific thermal and structural models | holds for presence; physics not re-run by me | every asset carries `thermal` and `structure` (0 missing); manifest analytics: 375 thermal and structural, 240 with transport. `🌡️catalog-physics` is the coordinator's run |
| Website with viewer, compare, playground | superseded by `🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS` and `🌙️11/☀️20/OPTIMIZE-STAFFAGE-AND-ARCHITECTURE-COURSE-FOR-PHONES` (both closed) | today's `U/browser-check.md` (another agent, 03:55): overview with 375 assets, detail, 3D stage, thermal view, collections and compare load without console errors; the playground was not looked at |
| Genuine SVG, PDF, DXF, DWG, video exports | SVG, PDF, DXF, DWG hold today; video not re-measured | `📑️multiview-sheet` 27 of 27 (SVG, PDF, DXF and DWG of the sheet, read back by lxml, PyMuPDF, ezdxf, ezdwg), `🖊️dwg-reopen` 5 of 5, `🧊️axonometric-exports` 19 of 19. MP4/WebM (`🎞️frame-timing`, `🗜️zip-reopen`) were not run today; last evidence is the ticket's audit of 2026-10-30 |
| Catalog check | 361 of 375 `✓` | the coordinator's check CLI run of today (`U/🗑️generated/check-<category>.txt`, 02:38 to 04:28), read by me, not re-run: `✗` for courier-with-hand-truck, elderly-with-cane, jogger, kneeling-gardener, painter-at-easel, parent-with-stroller, person-drinking-coffee, person-signing, person-with-white-cane, wheelchair-user, worker-carrying-box, cafe-chair, gas-boiler, charcoal-grill |

Open: nothing that this ticket owns. Changed by me for this ticket: nothing.

Proposed close summary: "The archive of 2026-10-29/30 (107 assets, schema v2, seven views, thermal, transport and structural models, website, exports) is the base of everything since; measured on 2026-11-23 the catalog has 375 authored assets in nine categories, every one with poses, clips, thermal and structural models, and the schema, seven-mode, view-synchrony, sheet and DWG suites are green. Website, exports and catalog validity are carried by the later tickets (CDN readiness, phones, 3D models, collections of 2026-11-22). See `U/older-tickets.md`."

## 3. Ticket 2: Top, Front and Side Views for Every Staffage

### 3.1 Acceptance points

| Point (ticket description, `brief.md`) | State today | Evidence |
|---|---|---|
| Plan and front geometry mandatory in the schema, `presentation.viewpoints` removed | holds | `Presentation` has only `defaultPose`, `defaultClip`, `defaultScale` and `additionalProperties: false`; no asset carries `viewpoints` (`measure-views.py`) |
| Handcrafted front and plan for every asset at every authored scale | holds | 1,416 authored representations in 375 assets, each with non-empty `elevation`, `front` and `plan`; 84 unavailable ones, each with a cause; 152 symbols, each with three views; 0 gaps |
| Language-agnostic orthographic-consistency guard with a third-party oracle | exists; 747 of 775 today | `S/🧪️tests/🧊️three-views` (Gherkin, vectors, shapely oracle): vectors, rules, oracle and the default-pose consistency of all 375 assets pass; the pose and clip sweep passes for 347 assets and fails for 28 (section 3.3) |
| Every consumer offers the three views (depiction, print suitability, copy check, exporter, casts, React app, site) | holds for what I measured | today: `🧍️front-projection` 9 of 9, `🧍️plan-projection` 12 of 13 (the red case is the commuter pin, section 3.3), `🪞️representation-copied` 8 of 8, `✂️section-views` 26 of 26 (after the pin update of section 3.2), `📐️dimension-clearance` 2 of 2, `🔤️caption-fit` 3 of 3, `📑️multiview-sheet` 27 of 27, `🧊️lateral-spread` 27 of 27, `🧊️axial-twist` 33 of 33, `🧊️depth-order` 5 of 5, `🧊️solid-placements` 3 of 3. Casts and the React suite are the coordinator's runs; the browser check of today lists six viewpoints on the detail page |
| Suite through `repo run @teaching/staffage:test` | superseded by ticket 4 (`repo-run.md`, exit 0 on 2026-11-12) | not reachable in 300 s today (section 5.4) |

### 3.2 What I changed

- `S/🧫️fixtures/✂️section-views/🔣️.json`: the pins of the section catalog follow what the assets declare since 2026-11-20/22 (their `decisions` texts name these sections in both languages): `masonry-wall-with-lintel` plan B-B at 1.55 m (was 1.5), `retractable-bollard` and `sectional-garage-door` elevation A-A at all four scales (were 1:50 and 1:100). Traced with `git show` over the commits of 11-18 to 11-22: the three changes are in the committed asset files since `5db034ffe53` (wall, bollard) and `01324745f80` (door).
- `S/🧪️tests/✂️section-views/🟦️.ts`: the exception that allowed a front section for `retractable-bollard` is removed (the bollard has none any more); the case now asserts for every asset of the elevation catalog that no front section is declared. Stricter than before.
- Result: `✂️section-views` 26 of 26 (was 23 of 26).
- Removed `T/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated-jm-rig.txt` (tool output left in the ticket root).

### 3.3 The red list of `🧊️three-views` (28 assets, not fixed by me)

Run: whole suite once, 870 s, `Tests 28 failed | 747 passed (775)`. All 28 are sweep findings (a pose or clip instant whose three views disagree by more than max(2 cm, 3 % of the largest dimension), or an instant that does not evaluate); none is a missing view.

| Group | Assets (findings; first finding) |
|---|---|
| Someone else's by the coordinator's list (people of today's wave and the eight named assets): 11 | e-scooter-rider (1; 1:500 `kicking-off`, front y from −0.082, side −0.023), parent-with-child-on-shoulders (92), person-carrying-ladder (11), person-carrying-shopping-bags (5), person-hanging-laundry (3), person-in-rain-poncho (1; 1:200 `walk-in-rain` 0.24 s does not evaluate: polygon union), person-in-winter-coat (25), person-pushing-bicycle (22), person-with-arm-sling (21), kneeling-gardener (3; `dig-cycle`, `reach-to-trug` do not evaluate), parent-with-stroller (24; `stop-and-check-child` does not evaluate at all four scales) |
| Collections of 2026-11-22 (open tickets): 2 | reclaimed-lattice-girder (6; 1:500 `props-cleared`, `lifted`, clip `lift`: side y to 3.008 m, front y to 1.200 m), used-viola (7; 1:200 `played`, clip `bowing`: 1.201 against 1.234 m, tolerance 0.024) |
| Older assets, every file last written on 2026-11-22; 14 of them are the red list of the open `🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-…` in its `validator.md` (line 425, "asset drawing work"): 15 | child-on-kick-scooter (78), herring-gull (24), horse (3; `standing-rest`: front `mane-2` reaches 1.854 m, side 1.758 m), office-swivel-chair (8; clip `swivel-rotation`), parent-carrying-toddler (47), person-with-umbrella (7), person-with-white-cane (1; 1:500 `constant-contact-walk` 1.10 s, `boot-r` −0.026 against 0.026), pigeon (1; 1:100 `take-off` 0.43 s, toes 0.036 against 0.057, tolerance 0.020), red-fox (1; 1:200 `curled-sleeping`, `hindbody` −0.074 against −0.102), runner-with-blade-prosthesis (2; 1:50 `run-10` and `run-cycle` 0.42 s do not evaluate: union of 25 polygons), seated-office-worker (2; `settle-back` 4.20 s, `hand-l` 1.558 against 1.602), skateboarder (1; 1:500 `pop`, `deck` −0.059 against `shoe-r` 0.013), sleeping-person (5; `turn-over` 6.40 s does not evaluate at any scale), walking-commuter (8; clip `glance-at-watch`, new on the list), worker-carrying-box (15) |

Why not fixed here: each is a drawing that has to be drawn again in the view that is wrong and looked at on a sheet (the brief of ticket 2 forbids deriving one view from another, and the instants that do not evaluate are the known `unionPolygons` defect of `U/requests.md`); 14 of the 15 older assets are on the list of the 3D ticket, whose rework wrote them yesterday; I changed no tolerance and no asset. The guard itself did its job: the three-views test file is unchanged since 2026-11-12, and the closed guard ticket of 2026-11-17 recorded it green for 152 assets.

Verdict: can close. Proposed close summary: "Front and plan are mandatory in the schema and drawn by hand for every authored representation (1,416 representations of 375 assets on 2026-11-23, no gap), the viewpoint list is gone, the three-views guard with its shapely oracle runs over the whole catalog, and depiction, suitability, copy check, exports, sheets, casts and viewer offer the three views. On 2026-11-23 the guard passes the default-pose consistency of all 375 assets and the pose and clip sweep of 347; the 28 red assets were redrawn or added on 2026-11-22 and are carried by the open 3D ticket and the collection and human wave tickets (list in `U/older-tickets.md`). Section pins updated to the sections the assets declare."

Residue that stays, stated plainly: 178 scratch scripts of the ticket folder still name `C:/git/semio` (they are records of single edits of assets that have since been redrawn; I did not rewrite them, because none can be run again without changing catalog files and so none could be verified).

## 4. Ticket 3: Staffage Multiview Top Front Side Coverage

### 4.1 Acceptance points

| Point (description, `📋️brief.md`, `📦️handoff.md`) | State today | Evidence |
|---|---|---|
| One evaluated 3D state per instant, projected per view | holds | `S/🔨️modules/🔭️multiview` (`resolveViews`); `🎨️view-synchrony` 15 of 15, `🏗️structure-projection` 53 of 53, `🌐️thermal-multiview` 23 of 23, `🧊️lateral-spread` 27 of 27, `🧊️axial-twist` 33 of 33 today |
| A missing combination is a named failure, never a substitution | holds | 84 unavailable representations, each with `cause`, 152 symbols with three views (`measure-views.py`); `🔭️multiview-coverage` engine cases (universe, `cell-missing`, mutated sample figure) pass; `🧭️representation-resolution` 8 of 8 |
| All seven modes in the three views | holds except one reference case | `🎨️seven-modes` 6 of 6; `🔭️multiview-coverage` "gives every analysis mode a colour-independent cue" passes; "reads one magnitude per region and member across the three views" is red (section 4.2) |
| Consumers: sheets and exports name and draw the three views | holds | `📑️multiview-sheet` 27 of 27 after the oracle fix of section 4.2 (SVG, PDF, DXF, DWG; first and third angle); DWG sheets are verified now that the wasm writer exists (ticket 4) |
| Consumers: viewer, site, casts, course | not measured by me (coordinator's runs); superseded for the course by `🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS` (closed) and for the site by the CDN readiness ticket (closed) | `U/browser-check.md` of today |
| Regression suite | 21 of 23 | `🔭️multiview-coverage`, 55 s: two red reference cases, section 4.2 |
| Coverage matrix over the catalog | **open** | the generator `S/🧪️tests/🔭️multiview-coverage/📊️matrix.ts` exists; the last recorded full run is v2 of 2026-11-02 (37,233 cells: 28,052 ok, 1,113 unavailable, 8,068 FAIL); v3 and every later run left no record in the ticket. Sample of today: section 4.3 |
| Proof sheets, example animations | **open** | `🖨️proofs.ts` exists; `📦️handoff.md` lists them as TODO and no later record exists; not produced today |
| Handoff document | was a skeleton | I appended the state of today to `📦️handoff.md` (section "State on 2026-11-23") |
| Wrap-up W1: ticket scripts free of `C:/git/semio` | open | 162 script files of the ticket folder still name it; not rewritten (same reason as in ticket 2) |

### 4.2 The two red reference cases, and one fixed

1. "keeps planted contacts still and advances the root as one state in the three views": `expected 0.0802 to be less than or equal to 0.072` at `walk-cycle` 0.6 s of `walking-commuter`, 1:100: side x [0.242, 1.146] against plan x [0.265, 1.089] (`shoe-rear-l` 23 mm, `hand-l` 57 mm). Cause: the commuter was rewritten on 2026-11-22 16:12; its plan is 30 flat polygons now, no `followTip` capsule and no solid. The same cause makes `🧍️plan-projection` "moves the commuter's foot in plan exactly as the independent kinematics predicts" red (`TypeError`: the pinned plan capsule `shoe-heel-l` does not exist any more) and puts the commuter on the three-views list. Not fixed: it is a redraw of the plan feet and hand of the reference person; I changed no pin. Written up in `U/requests.md`.
2. "reads one magnitude per region and member across the three views for thermal, forces and moments": `standing-adult moments pelvis|bending-moment` −1.2154 in the side view against −0.0012 in the front view. Measured with `U/older/moments.ts`: `ilium-l` 10.672 / 7.001 / −0.229, `clavicle-l` 3.859 / −4.420 / −0.062 (side / front / plan). `standing-adult` was rewritten on 2026-11-22 16:58 with girdle bones beside the symmetry plane, so its moments are three-dimensional; the moments view gives each diagram path the in-plane part of its view as value, by design (`drawDiagram`). Not fixed: making the path carry the invariant total is a change of the meaning of a value that the viewer's inspector and the structure fixtures read; that is a decision of the owner of the structural view. Written up in `U/requests.md` with a proposal.
3. Fixed: `📑️multiview-sheet` "writes a forklift in the moments mode as first-angle / third-angle SVG, PDF and DXF at the physical size" (2 red: PDF paper x 73.55 against 73.24 mm, 24.46 against 24.00 mm). Measured with `U/older/sheet-outside.ts` and `sheet-clips.py`: the PDF draws the hatch dots and hatch lines of a filled shape as marks inside a clip path of that shape; PyMuPDF `get_drawings()` reports them unclipped, so marks that the clip cuts away (up to 0.44 mm outside the outline) were counted as figure. Since the rework of the forklift a hatched member lies at the edge of the figure. The oracle `S/🧪️tests/📑️multiview-sheet/🐍️.py` now reads the drawings with their clip levels (`get_drawings(extended=True)`) and counts a clipped item only inside its clip rectangles; unclipped items count as before, the tolerance (0.03 mm) is unchanged. `📑️multiview-sheet` 27 of 27.

### 4.3 Coverage matrix, sample of today

The generator runs on today's schema (`cd S/🧪️tests/🔭️multiview-coverage && bun 📊️matrix.ts --out <dir> --jobs 1 --assets <ids>`). Two bounded runs:

- Nine assets, one per category (bicycle-stand, boxwood-ball, city-car, domestic-cat, forklift, park-bench, seated-reader, split-ac-indoor-unit, tilt-window): stopped by me after seven minutes with three assets finished and the fourth running; `city-car` alone took between five and six minutes. A shard writes its cells only when it ends, so this run left no cells. At this rate a matrix of 375 assets in one process is a matter of many hours; the documented full run of the ticket used 12 to 15 parallel shards for 149 assets and 30 minutes.
- Three light assets (bicycle-stand, boxwood-ball, tilt-window), 29 s: `[done] 777 cells: 677 ok, 21 unavailable, 79 fail`. The 21 unavailable cells are the declared 1:500 of `tilt-window`. The 79 failing cells carry three named codes:
  - `derived-view-band-copy`, 42 cells of `bicycle-stand` (front and plan at 1:50, 1:100 and 1:200, all seven modes): "the front view equals the 1:50 front under identity (normalised Hausdorff 0)": the same front and plan drawing is reused in three bands (decision D34 of the ticket counts that as a failure).
  - `analysis-cue`, 20 cells of `boxwood-ball` and 11 of `tilt-window` (moments mode only, including band ends and the alternatives of the unavailable 1:500): "the moments scene has neither a hatch pattern nor a dashed stroke under default options".
  - `derived-view-in-band`, 7 cells of `boxwood-ball` at 1:500: "the front view equals the elevation under mirror-x" (a ball; the allowlist of symmetric bodies in the vectors does not name this plant of yesterday's wave).

So the matrix is not clean even on three small assets, and the codes are of the kind the ticket set out to remove. I fixed none of them: the first is a redraw of four views, the second is either a missing cue of the moments view where an asset has no moment to draw or a rule to be made exact for that case, the third a line in the vectors that the owner of the plant has to justify. Raw output deleted; the commands above reproduce it in half a minute.

Verdict: cannot close (reasons in section 1). If the coordinator closes it nevertheless with the remainder moved to the umbrella, the summary would be: "One evaluated 3D state per instant is projected into side, front and plan in all seven modes; unavailable scales carry a cause, symbols carry three views, exports and sheets name their view and projection, DWG sheets are verified. On 2026-11-23: view-synchrony, structure-projection, thermal-multiview, lateral-spread, axial-twist and multiview-sheet green, multiview-coverage 21 of 23 (two reference cases red since the people were redrawn on 2026-11-22, see `U/requests.md`). No full coverage matrix over the 375 assets and no proof sheets are on record."

## 5. Ticket 4: Engine-Neutral 3D Models for Every Staffage

The ticket was closed on 2026-11-11 and reopened for four gaps. Each has a report of 2026-11-11/12 in the ticket that ends green; the ticket was simply never closed again.

### 5.1 The four gaps

| Gap of the reopening | State in the ticket (2026-11-12) | Today |
|---|---|---|
| DWG wasm build | `dwg-wasm.md`: writer owned by the crate, `staffage_dwg.wasm` built by its own target, `dwg-reopen` 5 of 5 | **holds**: `S/🔨️modules/📤️export/🖊️dwg/staffage_dwg.wasm` exists; `🖊️dwg-reopen` 5 of 5, the DWG cases of `📑️multiview-sheet` and `🧊️axonometric-exports` (19 of 19) pass |
| Python consumer against real manim | `python-manim.md`: manim 0.22.0 with real pycairo, consumer 64 OK twice, guards 48 OK, 22 clip imports, two real renders | **not re-measurable today** (section 5.2) |
| The 15 silhouette misses | `silhouette-15.md`: 152 of 152 at the gate of the ticket (box within 8 %, IoU at least 0.80, default pose, 1:50); confirmed by the closed `🌙️11/☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET` | **superseded** by `🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-…` (open): the guard in the tree is stricter (IoU at least 0.92, largest boundary distance at most two smallest features, box within one smallest feature, every authored scale; all of it is in the commit of 2026-11-18 already), and the assets were redrawn on 2026-11-22. Section 5.3 |
| `repo run` for the 3D and staffage test targets | `repo-run.md`: five targets exit 0 (`@teaching/staffage:test` 102 files, 2,680 tests, 1,163 s) | **not reachable in 300 s today** (section 5.4) |

Other points of the summary of the ticket: volumes for every asset hold in the sense of the ticket (every one of the 375 assets has `volumes`); `🧊️volume-schema` 115 of 115, the four `🧊️axonometric-*` suites 110 of 110 today. The catalog-wide model guard is section 5.5.

### 5.2 Python consumer: what blocks

`ls ~/Library/Caches/semio/teaching-python` → `No such file or directory`: neither the native cairo prefix (`teaching-python/native`) nor a Python environment of the course exists on this host today (the paths of `python-manim-env.sh`, `…/teaching-python/env/bin/python`, are gone; the repo-local `.🧬semio/🎓️teaching/⚡️cache` that the course `📜️script.ts` uses now does not exist either). The consumer tests import manim, so they cannot start. Provisioning is the course script itself (`bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test`): it downloads the pinned micromamba and 26 conda-forge packages and builds pycairo. I did not start it: it is a download of binaries from GitHub and conda-forge and a run of the whole course suite (the ticket measured more than 30 minutes at full CPU for it), which is beyond a re-measurement on a shared host; the evidence of 2026-11-12 stands unrefuted.

### 5.3 Silhouette guard on the assets of the ticket, today

`🧊️catalog-silhouette` filtered to the 15 assets of `silhouette-15.md` and the four it names beside them, 277 s: the vectors (4 cuts, 5 member widths) and "finds every asset, excuses none" pass; 3 of the 19 assets pass (cooking-pot, retractable-bollard, solar-thermal-collector), 16 fail today's gate at their first failing check:

person-with-umbrella (1:100 side, boundary distance 0.046 m against 0.03), hedgehog (1:100 front IoU 0.905), agave (1:500 side IoU 0.912), bamboo-clump (1:100 side IoU 0.908), date-palm (1:500 side 0.252 m against 0.15), miscanthus-grass (1:100 length 2.125 m against 2.14 m, 0.2 mm over the feature), market-parasol (1:200 length 2.552 against 2.59 m), playground-swing (1:50 height 2.5135 against 2.5 m), shopping-trolley (1:100 front 0.0525 m against 0.03), cargo-bike (1:200 side 0.071 against 0.06), city-bicycle (1:200 side 0.063 against 0.06), tower-crane (1:500 side IoU 0.794), panel-radiator (1:100 side 0.0305 against 0.03), sectional-garage-door (1:100 side 0.056 against 0.03), sliding-door (1:500 side 0.157 against 0.15), spiral-stair (1:50 side 0.050 against 0.015).

These lines do not show whether an asset still meets the gate ticket 4 was reopened for (8 % box, IoU 0.80, default pose at 1:50 only): the test stops at the first failing check of the stricter gate. The whole suite over 375 assets was not run: at the measured rate (about 15 s per asset, more for the people and the new plants) it is far beyond the 1,800 s of one run; it is the coordinator's run and the acceptance of the 3D ticket of 2026-11-14.

### 5.4 `repo run`, one bounded measurement

`timeout 300 repo run @teaching/staffage:test --lang=en` from the repo root, 04:46:41 to 04:51:41: ended by the timeout (`Cancelled (SIGTERM); leases released.`), 6.4 s user time, no test started. The log: `Starting the repo graph service for this host`, `Planning 1 targets`, `Reading the project graph slice` for 222 s (last line `publishing the sealed graph to the fleet 10 of 600`), `graph: plan answered and verified`, `tasks: 1 stale, 0 up to date`, 30 s of `[capacity] run: … waiting … to grow the reservation from 512 MB to 8190 MB: not enough free memory`, then `build:source-admission` and `repository:command remains active on native-fixed-image:…/.cache/b/d (shared, 33 s)` when the timeout fired. So today the command does not hang at one place within these 300 s; it spends them on the graph service, the memory gate of the shared host and the source admission. What blocks is the repo tooling under the load of this host, not the staffage targets (see also "flow view — repo run hangs" in `U/requests.md`).

### 5.5 Catalog-wide model guard, today

`🧊️model-catalog` is no longer the guard ticket 4 delivered (one GLB per pose, one FBX per clip, 152 assets, about 8 minutes): since the commit of 2026-11-20 it demands four scale-specific volumes and one scene per scale and mode, which is the acceptance of the open 3D ticket of 2026-11-14. One run, ended by the limit of 1,800 s (`exit 124`):

- "finds every asset of the catalog, each once": passes (375).
- "ships all four digital scales for every selected asset": fails at `used-bedside-table` (`[50, 100, 200]`). Counted from the files: 27 assets have no authored 1:500 volume, all of them `used-*` assets of the collections of 2026-11-22 (used-accordion, used-bedside-table, used-brine-water-heat-pump, used-cast-iron-sectional-boiler, used-dishwasher, used-djembe-and-cajon, used-dry-rotor-inline-pump, used-electric-cooker, used-electronic-organ, used-harmonium, used-high-chair, used-insulated-copper-coils, used-microwave-oven (also no 1:200), used-moving-boxes, used-oil-filled-radiator, used-panel-radiator-pallet, used-sewing-machine, used-spring-rocker, used-steel-string-acoustic-guitar, used-towel-radiator, used-trumpet, used-tumble-dryer, used-viola, used-violin, used-wall-hung-gas-boiler, used-washbasin-vanity, used-wc-pan-cistern). The other 348 have all four.
- Per asset (70 to 277 s each): 12 of 375 reached, all people. Pass: barista, child-on-kick-scooter, construction-worker-with-shovel, courier-with-hand-truck, cyclist, e-scooter-rider, elderly-person-knitting. Fail: basketball-player (4 findings), dancer (5), doctor-with-clipboard (2), elderly-with-cane (20), hiker-with-trekking-poles (24); the limit cut the run before vitest printed the findings. All five are people of today's wave or on the coordinator's list of assets under repair. 363 assets were not reached; at this rate the suite needs many hours, or a finished batch through `SEMIO_STAFFAGE_MODELS_DIR`.

Changed by me for this ticket: nothing in `S`.

Proposed close summary: "The four gaps of the reopening are closed: the DWG writer is owned by its crate and builds `staffage_dwg.wasm` (`dwg-reopen` 5 of 5, re-measured 2026-11-23), real manim with a pinned native cairo ran the consumer tests and two renders on 2026-11-12 (`python-manim.md`), the 15 silhouette misses were removed (152 of 152 at the gate of the ticket, `silhouette-15.md`), and the five `repo run` targets completed with exit 0 on 2026-11-12 (`repo-run.md`). The silhouette and model guards in the tree are stricter today (every scale and mode) and are the acceptance of the 3D ticket of 2026-11-14, which carries them. On 2026-11-23 the course Python environment is not provisioned on the host and `repo run` does not reach the tests within 300 s under load; both are stated in `U/older-tickets.md`."

## 6. Files I changed

Repo-relative paths.

Updated in the product:
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/📑️multiview-sheet/🐍️.py` (PDF figure box from the visible, clipped drawing)
- `🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/✂️section-views/🔣️.json` (three pins)
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/✂️section-views/🟦️.ts` (bollard exception removed)

Updated in tickets:
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE/📦️handoff.md` (section appended)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/requests.md` (two sections appended)

Created (all under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`):
- `older-tickets.md`
- `older/suite.sh`, `older/measure-views.py`, `older/findings.py`, `older/extremes.ts`, `older/moments.ts`, `older/sheet-outside.ts`, `older/sheet-outside.py`, `older/sheet-clips.py`, `older/patch-sheet-oracle.py`, `older/patch-section-pins.py`

Removed:
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated-jm-rig.txt`
- my own output under `🗑️generated/older/` of the umbrella ticket

## 7. Suite results of today in one table

| Run | Scope | Result |
|---|---|---|
| `🧊️three-views` | whole suite, 375 assets, 870 s | 747 of 775; 28 assets red (section 3.3) |
| `🔭️multiview-coverage` | whole suite, 55 s | 21 of 23 (section 4.2) |
| `🧊️catalog-silhouette` | 19 assets of ticket 4 and the vectors, 277 s | 13 of 29; 16 assets red at today's gate (section 5.3) |
| `🧊️model-catalog` | whole suite, cut at 1,800 s | 14 cases ran: 8 passed, 6 failed; 12 of 375 assets reached (7 pass, 5 fail), 27 `used-*` assets without a 1:500 volume (section 5.5) |
| Quick suites of the four tickets (24 files, 232 s) | `🖊️dwg-reopen`, `🧍️front-projection`, `🧍️plan-projection`, `🧊️lateral-spread`, `🧊️axial-twist`, `✂️section-views`, `🪞️representation-copied`, `📑️multiview-sheet`, `🎨️view-synchrony`, `🎨️seven-modes`, `🌐️thermal-multiview`, `🏗️structure-projection`, `🧊️depth-order`, `🧊️solid-placements`, four `🧊️axonometric-*`, `🧊️volume-schema`, `🧬️schema-twin`, `🧬️schema-conformance`, `🧭️representation-resolution`, `📐️dimension-clearance`, `🔤️caption-fit` | 530 of 537 before my changes; after them `✂️section-views` 26 of 26 and `📑️multiview-sheet` 27 of 27, so 535 of 537: red stay `🧍️plan-projection` (commuter pin) and `🧬️schema-conformance` (two people of the wave) |
| `📊️matrix.ts` (coverage matrix) | three assets, 29 s | 777 cells: 677 ok, 21 unavailable, 79 fail (section 4.3) |
| `repo run @teaching/staffage:test` | one run, 300 s | no test reached (section 5.4) |

That one run of the quick suites used vitest's parallel workers (several processes for 232 s); every other run was one process.
