# Wave PEEL2 — brep dissolution, batch 1 ("queries") landed

## Scope executed vs scope deferred

Landed **batch 1 (queries)** from the brief in full: `📏️measure`, `🔮️oracle`, `✂️int-cc`,
`✂️int-cs`, `✂️int-ss`, `🌳️bvh` + `🧊️3d/🗺️spatial` — moved, repointed, verified, source deleted,
mounts removed, all in one change. **Batch 2 (topology SCC)** and **batch 3 (foundations)** were
NOT attempted — see "Honest remainders" below for why and what the next wave needs. This is a
deliberate scope cut to avoid the predecessor's failure mode (dying mid-verification); one clean,
fully-verified batch beats three attempted-and-unproven ones.

## The dependency graph I actually measured (not assumed)

Before moving anything I grepped every `brep::X` cross-reference inside the remaining 25
`📐️brep` subdirs (see command below) to find what the "queries" batch touches and who touches it
back:

```
grep -ohE "brep::[a-z_]+" <dir>/*.rs   # what each queries-batch dir itself imports
grep -lE "brep::(measure|oracle|int_cc|int_cs|int_ss|bvh)\b" <every remaining dir>  # who imports the batch back
```

Findings:
- `measure` → `arena, curve, curve_ops, error, surface, surface_ops, topo, vec, mat, tolerance` (all foundations/topology, none moving this wave — legal forward edge, same pattern the euler/imprint compute dir already established).
- `oracle` → **only used by stdio's `classification` test module**, and only doc-references `measure` (no real code dependency). Zero internal framework-3d consumers.
- `int_cc/int_cs/int_ss` → `bezier, bspline, curve, error, mat, surface, surface_ops, vec` (foundations only).
- `bvh` → `arena, curve, engine, error, euler, history, mat, surface, tolerance, topo, vec` **and** `crate::spatial` (framework-3d's own generic BVH, 🧊️3d root-level, not under `📐️brep`).
- **Zero** of the 19 *remaining* brep subdirs (topology SCC + foundations) reference `measure/oracle/int_cc/int_cs/int_ss/bvh` back — confirmed by grepping all 19 for those six names before touching anything. This is what made the queries batch safe to extract as a unit without waiting for topology/foundations to move first.
- External (non-framework-3d) consumers of these six modules: **zero outside `stdio`'s own `✳️brep` facet** (`🔺️diff/{🎨️blend,↔️offset,🔀️boolean,➡️sweep}`, `⚙️engine`, `💡️inferences/🏷classification` — six files, all already same-crate).
- `🧊️3d/🗺️spatial`'s only consumer repo-wide was `bvh` (confirmed by grepping `crate::spatial`/`semio_framework_3d::spatial` across the whole tree — the only other `crate::spatial` hits are an unrelated same-named module inside the `remodel` plugin's own crate).

## What moved, and where

| Source (deleted) | LOC | Destination | Shape |
|---|---|---|---|
| `📐️brep/📏️measure` | 973 | `✳️brep/🧬️schema/💡️inferences/📏mass-properties/🦀️component.rs` | flat (public API unchanged: `solid_volume`, `solid_bounding_box`, `classify_point_on_solid`, …) |
| `📐️brep/🔮️oracle` | 329 | same file, nested `pub mod oracle { … }` | oracle is test-only ground truth (own doc comment says so); nesting keeps it out of the real mass-properties public surface while staying reachable at `…::mass_properties::oracle::{Sdf, ClosedFormMass}` |
| `📐️brep/✂️int-cc` | 565 | `✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️component.rs`, `pub mod curve_curve { … }` | **must** be namespaced — `newton_refine`/`intersect_general` are private helper names that collide with `int-cs`'s own |
| `📐️brep/✂️int-cs` | 494 | same file, `pub mod curve_surface { … }` | see above |
| `📐️brep/✂️int-ss` | 469 | same file, `pub mod surface_surface { … }` | no collision, namespaced for symmetry; top-level `pub use` re-exports `{intersect_curve_curve, CurveCurveHit}` / `{intersect_curve_surface, CurveSurfaceHit}` / `{intersect_surface_surface, IntCurve}` |
| `📐️brep/🌳️bvh` | 271 | `✳️brep/🧬️schema/💡️inferences/🌳bounding-volume/🦀️component.rs`, flat (public API: `build_face_bvh`, `build_edge_bvh`, `FaceBvh`, `EdgeBvh`, `BvhIndex`) | no collision with spatial, kept flat since this is the file's real public surface |
| `🧊️3d/🗺️spatial` | 247 | same file, nested `pub mod spatial { … }` | generic `Bvh<T>`; bvh's own `use crate::spatial::Bvh;` became `use spatial::Bvh;` (same-file sibling now) |

All three destination files already existed as pre-mounted stub placeholders (per wave3a/G4's
job 3) — no new `#[path]` lines needed in stdio's `📦️glue.rs` for the *destination*; `mod intersect`,
`mod mass_properties`, `mod bounding_volume` were already declared there pointing at these exact
files.

Every `crate::brep::` reference inside the moved files became `semio_framework_3d::brep::`
(mechanical, sed-verified — `grep -c "crate::brep::"` on every transformed copy came back 0 before
assembly). `crate::spatial::` became `spatial::` (now a same-file sibling module, not a
cross-crate forward edge). Doc-comment intra-links were fixed alongside the real `use` statements,
not just the code (one stale `[\`semio_framework_3d::brep::measure\`]` self-reference inside the
moved oracle text, corrected to describe the actual sibling relationship).

## Consumer repoint (6 files, all inside stdio, all mechanical)

`🔺️diff/🎨️blend`, `🔺️diff/↔️offset`, `🔺️diff/🔀️boolean`, `🔺️diff/➡️sweep`, `⚙️engine`,
`💡️inferences/🏷classification` — every `use semio_framework_3d::brep::{measure,oracle,int_cc,int_cs,int_ss,bvh}::…`
repointed to the new in-crate paths (e.g.
`crate::artifacts::semio::standards::v1::subsets::brep::schema::inferences::mass_properties::{…}`),
matching the import style already used elsewhere in these same files for the previously-landed ops
batch. Verified zero residual `semio_framework_3d::brep::(measure|oracle|bvh|int_cc|int_cs|int_ss)`
references repo-wide after the edit (one grep, clean).

## Framework-3d glue.rs surgery

Removed 4 `#[path]` mount blocks from `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs` in
the same change as the source deletion: `oracle`, `measure`+`int_cc`+`int_cs`+`int_ss` (adjacent
block), `bvh`, and the whole `//#region 🔖️Spatial` block (`pub mod spatial`). Checked all seven
deleted directories for non-`.rs` files first (the `🔺️mesh` DWG-codec near-miss from an earlier
wave) — none found, `rm -rf` was safe.

`📐️brep` now has **18 subdirs / 7,726 LOC** (was 25 / 10,827). `✳️brep` (stdio) is now 15,786 LOC.

## Test arithmetic — exact, both directions checked

```
semio-framework-3d --lib:  273 passed / 0 failed  →  233 passed / 0 failed   (−40, 0 failed both sides)
semio-s-plugin-stdio --lib: … → 3201 passed / 5 failed / 4 ignored
```

The **−40 on framework-3d is exactly accounted for**: I counted `#[test]` per moved source file
before deletion — `measure 7, oracle 11, int_cc 4, int_cs 5, int_ss 4, bvh 4, spatial 5 = 40`,
matching `273 − 233 = 40` precisely. Zero tests lost, zero silently dropped.

I could **not** do a clean pairwise diff on the stdio side against the ticket's recorded
`3003 passed / 5 failed` baseline — that number predates several other waves that landed inside
this same ticket since it was written (the ops batch, io1, m3-series, etc. all add stdio tests
independent of this wave), so `3201` is not "3003 + 40 forward" and I'm not claiming it is. What
I *did* verify cleanly: the **5 failing tests are byte-identical by name** to
`scratch-w0-baseline-failures-sorted.txt`'s stdio section (`binary::extent`, `dwg::fixture_honesty_law`,
`dxf::bounds`, `ifc::fixture_honesty_law`, `zip::entries`) — all pre-existing, all unrelated to
brep, zero new failures introduced, zero of the 40 moved tests appear in the failure list (all 40
pass in their new home).

## Verification output (mandatory form, run once each)

```
touch 🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs
cargo check -p semio-framework-3d --all-targets   → 0 errors (warnings only, unrelated crate)
cargo test  -p semio-framework-3d --lib           → 233 passed; 0 failed

touch ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs
cargo check -p semio-s-plugin-stdio --all-targets → 0 errors (exit 0; ran long, moved to background per the no-poll rule, one check, no retries)
cargo test  -p semio-s-plugin-stdio --lib         → 3201 passed; 5 failed (pre-existing, see above); 4 ignored
```

`df -h /`: 148Gi free, 8% used — checked before trusting the long stdio compile, not a disk-space
mirage.

## `⚙️engine` / `BrepKernel` — NOT dissolved this wave; named consumers, not silently preserved

Two different things share the name "engine" in this ticket and I want to be precise about both:

1. **`📐️brep/⚙️engine`** (framework-3d, 67 LOC) — untouched this wave. It's just shared value types
   (`Vec3`, `Aabb`, `ParamDomain`, `FaceGroup`, `MeshTransfer`, `PointClassification`) that the
   still-resident foundations/topology algorithm modules return/accept directly. It has to survive
   until whatever last framework-3d brep module needs it also moves (foundations batch). Not itself
   an anti-pattern — it's plain data types, not a facade.
2. **`✳️brep/🧬️schema/⚙️engine`** (stdio, 2,155 LOC: 1,704 facade + 451 `📦️mesh-io`) — this is the
   one the ticket brief means by "the only `⚙️engine` under any artifact tree" and the one that
   "must not survive." I did **not** dissolve it this wave; its 1,704-line `BrepKernel` trait +
   `Brep` struct (93 `_sync` methods) imports from essentially every remaining framework-3d brep
   module (I had to repoint 4 of its `use` lines just for this batch), so collapsing the whole
   facade is a much larger, separate effort than moving queries. I grepped its real consumers
   rather than assume:

   ```
   grep -rl "BrepKernel\|cad_brep_kernel\|ProcessKernelReplay" ✏️s/ (excluding the facade file itself)
   ```

   Named consumers that would need the facade dissolved and replaced with direct triad/inference
   calls before `BrepKernel` can go away:
   - `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
   - `✏️s/🔌️plugins/🏭️process/…/✳️any/🚪️io/🦀️component.rs` and `…/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (`ProcessKernelReplay`)
   - `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/⚙️engine/🕹️interaction/🦀️component.rs`, `…/🎛️apps/📐️cad/🦀️component.rs`, `…/✳️any/🚪️io/🦀️component.rs`, `…/✳️any/🚪️io/🗺️geometry-import/🦀️component.rs`, `…/✳️any/🧬️schema/💡️inferences/🦀️component.rs` (`cad_brep_kernel()` and friends — the earlier G4 report's "14 call sites" family)
   - `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs`

   That's real, current, non-trivial usage across 4 plugins — not a case of silently preserving
   ceremony. Dissolving `BrepKernel` properly needs each of these call sites rewired to the
   specific triad/`InferredField` it actually wants, one plugin at a time, verified per-plugin —
   a wave of its own, not a rider on this one.

## Honest remainders (what the next PEEL wave needs)

**Batch 2 (topology SCC — indivisible)**: `🏟️arena`(260) `🕸️topology`(668) `🔺️euler`(277)
`📜️history`(179) `🧱️primitives`(889) `✅️validate`(243) = 2,516 LOC. Not started. Consumer count
measured this session (`grep -rl "semio_framework_3d::brep::X"` across `✏️s/`): `arena` 12 files,
`topo` 12, `euler` 9, `history` 12, `primitives` 11, `validate` 4 — all inside stdio's `✳️brep`
already (same repoint mechanics as this batch), confirmed mutually recursive per the ticket brief
(primitives→validate→topology→euler→history→topology) so it has to land as one change.

**Batch 3 (foundations)**: `➡️vector`(398) `🔢️matrix`(348) `📏️tolerance`(235) `⚖️predicates`(344)
`〰️polynomial`(469) `🎢️bezier`(325) `🪢️bspline`(414) `➰️curve`(400) `✂️curve-ops`(464)
`🏄️surface`(314) `🪡️surface-ops`(214) `🚨️error`(183) `📄️step`(1,035) = 5,143 LOC. Not started.
Consumer counts measured: `vec` 13 files, `mat` 9, `tolerance` 8, `predicates` 1, `poly` 0 (!),
`bezier` 1, `bspline` 3, `curve` 10, `curve_ops` 1, `surface` 11, `surface_ops` 3, `error` 13,
`step` 1 — all inside `✳️brep`. `predicates`/`poly`/`bezier`/`curve_ops`/`surface_ops`/`step` have
very few consumers and might be quick; `error` and `vec` are imported almost everywhere and will
touch the most files.

**`📐️brep` is not yet empty or deletable** — 18 subdirs, 7,726 LOC remain (topology SCC + foundations
+ the tiny `⚙️engine`), plus `🥽️mesh` (2,769 LOC, explicitly out of scope — live MESH wave).

**`BrepKernel`/stdio's `⚙️engine` facade** — deliberately not dissolved, named consumers above.

**`📦️mesh-io`** (451 LOC, inside stdio's `⚙️engine` dir) — untouched. Per its own doc comment it's
snapshot↔snapshot dialect bridging (DWG/GLB/OBJ/STL codecs), which the ticket brief says belongs
in `🚪️io/`. Not moved this wave — it's wired into the `BrepKernel` facade's export/import methods,
so relocating it cleanly is entangled with the same facade-dissolution work as `BrepKernel` itself,
not a rider on the queries batch.

**framework-3d's own `⚙️engine` (67 LOC)** — survives until foundations lands (see above), then its
`Vec3`/`Aabb`/etc. types need a real home, most likely folded into stdio's engine or one of the
foundation compute dirs (`vector`/`predicates`) once those exist. Not decided this wave — flagging
rather than guessing.

## Files touched this wave

**Deleted** (7 dirs, all `.rs`-only, checked first):
`🧰️framework/🔨️modules/🧊️3d/📐️brep/{📏️measure,🔮️oracle,✂️int-cc,✂️int-cs,✂️int-ss,🌳️bvh}/`,
`🧰️framework/🔨️modules/🧊️3d/🗺️spatial/`

**Rewritten** (destination content, previously stub placeholders):
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/🔺️diff/✂️intersect/🦀️component.rs`
`…/💡️inferences/📏mass-properties/🦀️component.rs`
`…/💡️inferences/🌳bounding-volume/🦀️component.rs`

**Edited** (import repoint only):
`…/🔺️diff/🎨️blend/🦀️component.rs`, `…/🔺️diff/↔️offset/🦀️component.rs`,
`…/🔺️diff/🔀️boolean/🦀️component.rs`, `…/🔺️diff/➡️sweep/🦀️component.rs`,
`…/⚙️engine/🦀️component.rs`, `…/💡️inferences/🏷classification/🦀️component.rs`

**Edited** (mount removal):
`🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📦️glue.rs`

Scratch files (ticket folder, `.txt`/working copies under this session's scratchpad — not
committed, not part of the repo): none added to the ticket folder itself this wave; intermediate
transform copies lived only in the session scratchpad and are not part of the deliverable.
