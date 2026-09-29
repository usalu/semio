# CD1 — CAD Gaps (session 14c)

Slice CD1, 2026-09-29 09:0x (new slice, spawned after TS1). Scope: root-cause and fix the CAD vitest reds TS1 left
unowned (semio kernel `brep_invoke` transforms, AEC typology catalog, remaining renderer/geometry reds). Guest freeze (rule 27):
guest changes are PREPARED T6 sets registered in `📓️t6-queue.md`; host-only/test-only lands per rules 20/22. Backups, captures,
scratch ONLY under `.🧬semio/🌐hub/s14-cd1-*` (rule 26). Rules: `📓️session-14-preamble.md` (+ 13/12).

Status legend: **measured** = ran here, command + capture named; **unverified** = read from source only; **written, not run**.

## Session 15

| # | item | state | evidence |
|---|---|---|---|
| R | reconcile predecessor (rule 30) | **done 19:4x**: rows 14/15 landed T6R1 16:12 (both scripts `--dry-run` → "already applied"); owner laws = LW1 orphan block `t6-e-cd1-1` (native lane, queued behind LW1 b/c/d + 5 slices; capture `.🧬semio/🌐hub/s14-lw1-logs/t6-e-cd1-1.txt`), no half-applied hunks; overlay `s14-cd1-overlay` + build-dir `s14-cd1-build` (3.8 GB) kept for the next proof | `📓️landing.md` T6R1 rows |
| 1 | cad `brep:out` export failure (G12 coverage p33-3/p33-4: `media payload error on port brep:out: brep export failed`) | root cause found (below); set **A `step-exchange`** written in the overlay (bases `s14-cd1-work/s15/base/`), **written, not run** — proof `s15a` queued | OCCT design probe measured (log 19:5x) |
| 2 | wall `From2PointsAndHeight` typology model | set **B `construction`** written in the overlay (bases `base2/`, after A): schema, 21 assets, vectors fixture (independent Python planner), TS+Rust planners, both TS kernels, Rust commit + extent-span shapes, laws; cad TS `tsc` 0 + suites `tsc` 0 (measured, `s15/tsc-*-pre.txt`); Rust/vitest **written, not run** — in proof `s15a` | design §Item 2 |
| 3 | mass-properties accuracy (1.3e-4 origin-dependent) | cause (read): planar faces with curved edges (cylinder/cone caps) were polygonised in UV → area short, and the cap's `(1/3)∫P·n dA` scales with its offset from the origin; set **C `planar-exact`** written (Green's theorem per edge with 16-point Gauss–Legendre, dead polygon helpers removed, placement law at 1e-9) — **written, not run**, in proof `s15a` | `base3/` |
| 4 | flow-core wasm rebuild + cad semio vitest | waits WINDOW 5 | — |

### Item 1 — root cause (read + data, 19:5x; measurement queued with the set proof)

`brep:out` (`✏️editor/🦀️.rs` `export_media`) collects EVERY pane's shapes (`collect_modelspace_solids` → `pane_world_solids`) and
exports STEP through `export_solids_as` → `kernel.export_step` → `solid_id()` per handle. The forest document's panes are not all
solids (`📚️examples/🖼️assets/🎮️play/🔣️.json`): Shape 1 solid, Building 11 solids, **Energy 1 surface** (`energy.energy.baseplate`
→ planar face), **Structure 10 curves + 1 surface** (walls/columns as wires). `pane_world_solids` resolves a surface primitive to a
FACE handle and a curve primitive to a WIRE handle, so `export_step_sync` answers `… is not a solid`, `.ok()?` swallows it, and the
editor reports the opaque `brep export failed`. The Building-pane law (`current_pane_exports_its_real_solids`) is green because that
pane is solids only. Compounding defects on the same path: the kernel's own STEP writer (a KNOWN DUPLICATE of stdio's
`SemioBrepToStep`) writes `({items},)` (invalid Part-21) and `SI_UNIT(.MILLI.,.METRE.)` for metre documents, drops NURBS weights;
cad compensates with a string "repair" shim and a STEP→semio→STEP double round trip; `SaveInPlay` silently falls back to a spatial
DSL file when the STEP export fails.

### Item 2 — construction design (20:2x, refines the session-14 proposal after reading `✏️s/🔌️plugins/📐️cad/AGENTS.md`)

Facts: 21 typologies carry a `…From2PointsAndHeight` construct action; their `spatial.action` documents are stubs
(`kernel.call spatial.action.capability`) that the TS runtime forwards to `kernel.executeCommandDiff(actionId)`, where both TS
kernels build the axis-aligned bounding box of the two points; the Rust twin guesses by typology NAME. No JSON Schema exists for
`spatial.typology`/`spatial.action`; TS `inferTypologyPrimitiveKinds` is a dead name heuristic (all 38 assets declare the field).
The forest's Building pane holds slab/column/beam as SOLIDS while `building.building.slab` says `surface` and `…beam` `curve`.
Model (schema-first, one planner twinned TS/Rust):
1. `spatial.typology` JSON Schema (`🧬️typology/🧬️schema/🔣️.json`): `primitiveKinds` required non-empty; `construction.from2PointsAndHeight
   = {primitive, profile, thickness?, alignment?}` — `primitive` ∈ the typology's `primitiveKinds`; `profile` says how the picks
   are read: `segment` (baseline A→B), `rectangle` (opposite footprint corners), `circle` (centre A through B), `point` (A only);
   `thickness` (m) + `alignment` (`center|left|right` of the baseline) required for a `segment` solid.
2. Planner `(construction, A, B, height)` → `prism {origin, xAxis, yAxis, extents}` | `quad {corners[4]}` | `line {start, end}` |
   `cylinder {base, radius, height}`, or a typed refusal (`construction.degenerate-input`, en/de); language-agnostic vectors
   `🧬️typology/🧫️fixtures/📐️construction-from-2-points/🔣️.json` with closed-form measures.
3. Realisations: TS semio (brep_invoke verbs), TS brepjs (OpenCascade = the oracle for the same vectors), Rust twin
   (`commit_from_2_points_and_height` → plan → `CadObject` origin/orientation/extent, a zero extent axis = a face/wire the
   kernel builds as such in `typology_local_solid`, so the viewport and `brep:out` show the declared primitive).
4. Building typologies wall/slab/roof/ceiling/foundation/door/window/stair/beam → `solid` (the forest is the ground truth).

## Log (session 15)

- 19:3x read preambles 14/15, fleet-15, t6-queue, landing, audit-s15-state §4.4; reconcile above. Native lane: LW1 chain block b
  holds (19:20), queue s19/g12/s18/u6/u6; overlay lane c13 holds, queue sh2/u6/c12. Load 29–32.
- 19:5x item 1 root cause (above).
- 19:5x OCCT design probe (`wp-cd1/step-exchange/cd1-occt-step-probe.ts --design`, captures `s14-cd1-work/s15/design-*.step`):
  OpenCascade reads a product with THREE shape_definition_representations (ABSR solid + MSSR/SBSM open shell + EBWSR/EBWM
  wire) → 1 solid / 2 shells / 7 faces / 18 edges, volume 1 m³, area 7 m², length 19 m (converted from `SI_UNIT($,.METRE.)` to its
  mm session unit) — the planned exchange structure is readable by the third party.
- 20:0x overlay `s14-cd1-overlay` re-synced to live (lb2-overlay + missing + prune; 939 + 2444 refreshed, 24 pruned); set
  `step-exchange` written in the overlay (bases `s14-cd1-work/s15/base/`). Proof chain `wp-cd1/step-exchange/cd1-step-proof.sh s15a`
  queued as ONE native-lane ticket (pid 44557, 5th: h13 holds, s18/u6/lw1/s19 ahead); log `s14-cd1-work/s15/s15a-chain.txt`.
- 20:1x–20:3x set B (construction) + set C (planar-exact) written in the same overlay; the proof script now covers all three
  (`cd1-step-proof.sh`: print golden → fixture → stdio lib → exchange again → cad laws (exports, typology, interaction, meshes)
  → cad vitest (all) → tsc). RELAY S20 sent (new fault `cad.export.failed`). Generator `wp-cd1/cd1-make-set.py`; set-1
  intermediates of the two shared files (`✏️editor/🦀️.rs`, semio suite) in `s15/s1/`.
- 21:45 proof **s15a** done (`s14-cd1-work/s15/s15a-*.txt`): print stage **red** — `semio_brep_step_text` lived in the
  `conversion-brep`-gated io module, the ungated kernel could not import it (`E0433`); cad stage compiled (cad enables the feature):
  80/81 ✓ incl. `modelspace_brep_out_exchanges_every_pane_shape`, `export_solids_as_step_carries_solids_faces_and_wires`,
  `building_wall_follows_a_diagonal_baseline`, all construction/census laws; 1 red = battery: `energy.energy.constructBasePlate`
  (FromSurface) now reports Unsupported instead of "stuck at commit". vitest 260/270: TS construction laws ✓ (AJV schema census,
  TS planner vectors, **OpenCascade realisation of every vector**), brepjs two-point law ✓; reds = semio wire ×8 (stale flow-core
  wasm: `unknown brep_invoke method: translate`, exportStep "not a solid" — WINDOW 5), OCCT exchange law (no golden text yet),
  1 stale law (`building.building.slab` + solid expected false), renderer file `Denied ID …brepjs_single.wasm?url` (overlay-only
  vite fs root). tsc cad 0, suites 0.
- 21:5x fixes: the ONE writer `semio_brep_part21`/`semio_brep_step_text` moved into the kernel's `🟫️step` (ungated, re-exported by
  the engine; the gated serializer wraps it); exchange laws moved to the ungated step tests (census counts the written entities);
  serializer tests import their snapshot types; battery + primitiveKinds law updated; proof adds `--features conversion-brep` io
  tests. Sets generated (`wp-cd1/cd1-build-sets.py` → `cd1-make-set.py`, payload JSON beside each script): A 14+1 files, B 34+2,
  C 2 — dry runs clean (A, C on live; B on live+A scratch `s15/scratch-ab`, A+B write = overlay byte for byte). Rows **37/38/39**
  registered in `📓️t6-queue.md`. Proof **s15b** queued in the overlay lane (pid 30754).

## Session 14

| # | item | state | evidence |
|---|---|---|---|
| 0 | triage of the 10 reds in TS1 `cad-vitest-2.txt` | **done**: 3 root causes (below) | `.🧬semio/🌐hub/s14-ts1-logs/cad-vitest-2.txt`, `cad-vitest-semio-2.txt` |
| 1a | affine-transform vectors + OpenCascade oracle (landed now, test-only + host CAD TS) | **measured green**: fixture `🧊️brep/🧫️fixtures/🔁️affine-transforms/🔣️.json` (10 cases, 4 refusals), owned brepjs boundary += rotate/scale/mirror/measureVolumeProps/getBounds, semio-suite law "OpenCascade answers every vector and refuses every degenerate one" ✓; cad tsc rc 0 | `s14-cd1-work/vitest-semio-1.txt`, `tsc-cad-1.txt`, `probe/occt-probe-1.txt`; landing row |
| 1b | kernel law in the live engine unit tests (09:23) | **reverted 11:26** (coordinator: red on live, a law lands with its implementation) — byte-exact restore from my backup; U6's compiled binary re-run shows the reds are REAL kernel defects (item 1d) | `s14-cd1-work/kernel-law-u6-binary-1.txt` |
| 1c | T6 row **14** `wp-cd1/t6-brep-invoke/cd1-brep-invoke-transforms.py` (cone tessellation fix, kernel refuses degenerate transforms, 5 wire verbs, verb catalog + schema, kernel/bridge/TS laws, fixture tolerances = measured accuracy) | **registered 13:2x, dry-run clean on live; overlay-proven**: stdio-semio `--tests` all green (lib 2572 + 6 integration targets) incl. the 3 kernel laws; flow `flow_brep_invoke` 3/3 (+ mutation check: an arg-type drift turns the catalog law red); cad tsc 0; vitest catalog law + OCCT law ✓ (the TS semio vector law needs the flow-core wasm rebuild after landing) | `s14-cd1-work/proof-1-*`, `proof-2-*`, `proof-3-vitest-catalog.txt`, `tsc-cad-overlay-2.txt` |
| 1d | kernel defects found by the vectors (overlay diagnostics) | **measured**: (i) cone tessellation lost ~21 % of the volume for EVERY cone (soup 2.4598 vs π) — root cause: the apex is one UV ring vertex though the seam arrives on one `u` branch and departs on the other, so the ring closed through a diagonal; **fixed in row 14** (soup 3.1375, −0.13 % chordal like the cylinder); (ii) mass properties are numeric: volume ≤ 1.3e-4 rel (origin-dependent: translating a cone changes it by 1.3e-4), cone centre of mass off 0.004 — NOT fixed (follow-up), fixture tolerances record it; rotate/mirror/scale(−1) are exact (identical soup + volume) | `s14-cd1-work/ov-diag-2.txt`, `ov-diag-4.txt` |
| 2 | T6 row **15** `wp-cd1/t6-typology/cd1-typology-fixed-filename.py` (7 renames + Rust include_str + golden liveBindings + taxonomy/frozen-evidence sha + Rust/TS census laws) | **registered 13:2x, dry-run clean on live; overlay-proven**: cad-cad typology 37/37 incl. the census law; repo-lib projection ×2 + CAD frozen bindings + frozen-coordinate laws green; frozen registration 0 problems / sha match / 480 coordinates; census live 31 (7 misnamed) → overlay 38/38; vitest geometry suite green incl. the 3 former reds. Touches FROZEN `📚️library/🔣️taxonomy.json` | `s14-cd1-work/proof-1-cad-typology.txt`, `proof-1-library*.{txt,xml}`, `typology-proof/`, `proof-1-vitest-overlay.txt` |
| 3 | `…From2PointsAndHeight` ignores the typology (wall depth 0) | root cause found; model proposal below — pending | — |

### Root causes

1. **`brep_invoke` has no transform verbs.** `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs`
   `brep_invoke_inner` dispatches 38 methods but none of the kernel's `#region Transforms`; the TS `SemioBrepKernel`
   (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts` `primitiveHandle`) places every box/sphere/cylinder/cone with
   `translate` (+ `rotate` for tilted axes). The Rust kernel already implements them exactly (`Brep::translate_sync`/
   `rotate_sync`/`rotate_about_sync`/`scale_sync`/`mirror_sync` → `transform_shape_sync` → `🔺️diff/🔁️transform`
   topology-preserving affine deep copy, reflection flips `flipped`, tolerances scale by the max singular value). W4-A
   (ticket 26/09/03 BREP-KERNEL-DEPENDENCY-FREE-RUNTIME) wrote the bridge without a rustc check and never ran the TS suite
   (flow wasm could not be built then) — the gap was never observed. No test covers `brep_invoke_json`.
2. **7 typology assets renamed off their fixed filename.** Commit `3a6a9d6bfc` (09-05, 6 155-file path codemod) renamed
   `🗂️typologies/<X>/🔣️typology.json` → `🔣️typology-<hash6>.json` in `🌉️aec.building.structure.classic` (4),
   `📏️…fem.line`, `🗺️…fem.surface`, `🧊️…fem.solid` (1 each). The TS runtime glob
   (`…/✏️editor/⚙️engine/🏃️runtime/🟦️.ts`, `**/🗂️typologies/**/🔣️typology.json`) misses them → TS registers 31 of 38
   typologies (Rust `🧬️typology/🦀️.rs` embeds the hashed paths explicitly → 38). The repo-lib golden
   `📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` declares the category rule `🗂️typologies` =
   `nested-fixed-json`, `fixedSourceFilename: 🔣️typology.json`, count 38 and carries the 7 hashed names only as
   `liveBindings`. The canonical names fit the Windows path budget (max 217 of 239 UTF-16 units). All 5 geometry/renderer
   reds are this one cause (slab `#8B7355`/hatch, reinforced concrete wall crosshatch, `structure.*` primitiveKinds,
   `from_building` target typology `structure.structure.reinforcedconcretecolumn`). The TS law `typologies.length >= 27`
   masked it.
3. **`…From2PointsAndHeight` ignores the typology.** Both TS kernels (`🧠️semio`, `🧱️brepjs`) build an axis-aligned box
   whose footprint is the two points' bounding rectangle, so a wall drawn along an axis has depth 0 (the semio law uses
   `[0,0,0]→[4,0,0]`; the brepjs law uses `[4,3,0]` and passes for the wrong reason — a 4×3 m "wall" block). The interaction
   passes `typology` (e.g. `energy.energy.externalwall`, `primitiveKinds: ["surface"]`); the Rust twin
   (`🕹️interaction/🦀️.rs` `commit_from_2_points_and_height`) guesses by typology NAME (`wall` → span × 0.2, `windows` ×
   0.05, else footprint) and never orients the wall along the segment.

### Proposal — construction model for `…From2PointsAndHeight` (item 3)

**Today (measured/read).** Three implementations guess what "two points and a height" means, none reads the typology:
TS `🧠️semio` + `🧱️brepjs` kernels build the axis-aligned box spanned by the two points' bounding rectangle (a wall drawn along
an axis has depth 0 → `box depth must be positive`; a diagonal wall becomes a 4 × 3 m block); the Rust wgpu twin
(`🕹️interaction/🦀️.rs` `commit_from_2_points_and_height`) matches typology NAMES (`column` → r 0.25 cylinder, `wall` → span ×
0.2, `windows` → × 0.05, else footprint) and never orients along the segment; `inferTypologyPrimitiveKinds` is another name
heuristic. The typology assets already declare `primitiveKinds` (walls/slabs = `surface`, columns = `solid`/`curve`), which no
construction path honours. `spatial.typology` has no JSON Schema — TS `parseTypologySpec` and the Rust struct are the only
definitions (code-first).

**Model (schema-first, kernel-neutral).**
1. `spatial.typology` gets a JSON Schema (`…/✏️editor/⚙️engine/🧬️typology/🧬️schema/🔣️.json`) that both registries are held to (TS
   AJV + Rust parse laws over every asset), incl. a required `construction` block:
   `{"from2PointsAndHeight": {"profile": "segment" | "rectangle" | "point", "thickness"?: m, "alignment"?: "center" | "left" |
   "right", "section"?: {"kind": "circle", "radius"} | {"kind": "rectangle", "width", "depth"}}}`.
   `profile` says how the two picked points are read: `segment` = baseline A→B (walls, railings, windows, beams), `rectangle` =
   opposite footprint corners (slabs, base plates, hulls, roofs, foundations), `point` = A is the base centre (columns).
2. ONE pure construction planner, twinned TS/Rust over a shared vector fixture: `(construction, primitiveKinds, A, B, height)` →
   a plan in world coordinates — `quad {corners[4]}` (surface typologies: the vertical face A, B, B + h·ẑ, A + h·ẑ for `segment`;
   the horizontal rectangle for `rectangle`), `prism {origin, xAxis, yAxis, extents}` (solid typologies: the segment prism of
   `thickness`, offset by `alignment` along n = ẑ × (B − A)/|B − A|), `extrusion {section, base, height}` (`point`), `line`
   (curve typologies). Degenerate input (A = B, zero footprint area, height ≤ 0) is a typed refusal
   (`construction.degenerate-input`, en/de) — never a kernel panic string.
3. Every kernel realises a plan from primitives it already has: semio = `box`/`cylinder` + `rotate` + `translate` (the verbs set
   (1) adds) or `planarFaceFromPoints`; brepjs = the same with OCCT (the differential oracle); the Rust twin's
   `commit_from_2_points_and_height` becomes plan → primitives, `CadObject.orientation` = the plan's frame (walls finally follow
   the drawn segment).
4. Laws: planner vectors (language-agnostic fixture; TS + Rust twin answer identically), every typology asset carries a
   `construction` that fits its `primitiveKinds` (schema + census), each kernel's realisation of every vector matches the plan's
   closed form (area/volume/centroid) with OpenCascade answering the same vectors, and the interaction e2e places a diagonal
   wall whose object carries the declared primitive slot.
The semio law "energy wall … builds a box solid" then changes meaning: `energy.energy.externalwall` is `primitiveKinds:
["surface"]`, so the correct answer is a vertical face of area |AB|·h = 10.8 m², not a solid.

**Scope for T6.** Guest-linked (CAD Rust crate + 38 typology assets + TS kernels) — prepared as its own set after (1)/(2) land,
because the semio realisation uses set (1)'s `rotate`/`translate` verbs.

## Log

- 09:07 read preamble (1–27), AGENTS.md, TS1 report, t6-queue; triage from TS1 captures + source. Disk 67 GiB, load 11.
- 09:1x OCCT oracle law + fixture + brepjs boundary landed (green). 09:23 kernel law appended to live engine tests (my mistake: its implementation set is T6).
- 11:26 (after the usage cut) coordinator: law red on live → restored byte-exact; moved into the T6 set.
- 11:3x typology set written, dry-run clean. Overlay `.🧬semio/🌐hub/s14-cd1-overlay` (lb2-overlay.py + 134 ignored generated sources), private build/target `s14-cd1-build`/`s14-cd1-target`, native lane; stdio-semio lib tests build in ~3 min there.
- 12:0x–12:2x overlay diagnostics: cone mesh open along the seam/apex → pole-branch split fix → cone soup −21 % → −0.13 %.
- 13:0x–13:2x overlay proofs of both sets (above); rows 14 + 15 registered in `📓️t6-queue.md`.

