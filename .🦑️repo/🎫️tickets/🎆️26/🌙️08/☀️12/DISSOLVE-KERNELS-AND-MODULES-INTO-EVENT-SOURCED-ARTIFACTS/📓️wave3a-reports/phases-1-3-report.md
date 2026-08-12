# W3a-0 implementation report — Phases 1–3

Boundary: `🧰️framework/🔨️modules/🧊️3d/**` (crate `semio-framework-3d`). Everything else read-only.
Spec: `📓️wave3a-design/brep-dissolution-design.md`. This agent executes **Phases 1–3 only**;
Phases 4–6 are cross-session-gated and not this agent's.

## Baseline, re-measured before any edit

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-3d --lib
test result: ok. 407 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.15s
```
Matches the coordinator's recorded baseline exactly. `semio-framework-3d` has no dependency on
`semio-framework-plugin`, confirmed (unaffected by that crate's state).

---

## Phase 1 — OpRecorder plumbing (DONE)

**What changed.** ~14 top-level constructive `pub fn`s discarded their `OpRecorder` at the end of
the call (`let mut rec = OpRecorder::new();` built locally, never returned) — `Correction 2` in the
design doc. Every one of them, plus every private helper on their call path, now takes
`rec: &mut OpRecorder` as its last parameter instead, mirroring `crate::brep::euler`'s own
convention exactly. The caller now owns the recorder and can call `rec.into_delta()` after the call
to see everything the operation touched — this is Phase 1's entire deliverable, and it now escapes
every function boundary in the crate, not just euler's own.

Full signature-change list, by file:

- **`🧱️primitives/🦀️component.rs`**: `make_box`, `make_sphere`, `make_cylinder`, `make_cone`,
  `make_torus`, `solid_from_triangle_soup`, `make_convex_hull`, `make_polyline_wire`,
  `make_rectangle_wire`, `make_regular_polygon_wire`, `make_planar_face_from_points`,
  `make_planar_face_from_wire` — 12 functions (2 more than the design doc's literal list:
  `make_rectangle_wire`/`make_regular_polygon_wire`/`make_planar_face_from_points` transitively call
  `make_polyline_wire`/`make_planar_face_from_wire`, so they needed the parameter too or the
  provenance would still be swallowed one call frame up).
- **`🔀️boolean/🦀️component.rs`**: `boolean_solid`, `compound_cut`, `section_solid_by_plane`,
  `split_solid_by_plane` (public), plus private helpers `aabb_fast_path`, `mesh_boolean`,
  `solid_from_outer_faces`, `clone_solid_shells`.
- **`➡️sweep/🦀️component.rs`**: `extrude_face`, `revolve_face`, `loft_profiles`, `sweep_along_path`,
  `pipe`, `helical_sweep` (public), plus private helpers `solid_from_prism`,
  `try_extrude_circle_cylinder`, `solid_from_lofted_sections`.
- **`🎨️blend/🦀️component.rs`**: `fillet_edges`, `fillet_variable`, `chamfer_edges` (public — these
  did not construct a recorder themselves before; they call into primitives, so they needed a `rec`
  param sourced from *their* caller so the compound op's delta isn't split across throwaway
  recorders), plus private helper `solid_from_blend_samples`.
- **`↔️offset/🦀️component.rs`**: `offset_face`, `thicken_face`, `offset_solid`, `shell_solid`
  (public, one more than the design doc's list — `shell_solid_with_open_faces` calls it and needed
  its delta too), `shell_solid_with_open_faces`, `draft_angle` (public), plus private helpers
  `thicken_face_hull`, `shell_copy_solid`, `solid_with_void_shell`, `make_box_from_aabb`.
- **`🧵️sew/🦀️component.rs`**: `sew_faces` (public), private helper `get_or_create_vertex` already
  took `rec`, unchanged.
- **`🩹️heal/🦀️component.rs`**: `defeature` (threads `rec` to `sew_faces`); `heal_solid` and
  `convert_to_nurbs` gained `rec` too, and now actually **use** it — both bypassed euler entirely
  before (`heal_solid` repositions `body.vertices` directly; `convert_to_nurbs` swaps
  `body.surfaces`/`body.edges[..].curve` directly), so neither ever recorded what it touched. They
  now call `rec.record_modified(label)` for every vertex merged / edge or face whose geometry pool
  entry changed — a real fix, not just plumbing, matching the "surface the provenance" intent of
  this phase rather than papering over the pre-existing gap.

**Not touched, on purpose.** `📄️step/🦀️component.rs::read_step` and
`📦️mesh-io/🦀️component.rs::import_stl_to_body`/`import_obj_to_body`/etc. — the design doc's §5
classifies whole-file import as an `ArtifactStore::reset` target (not a mutation) and flags
per-solid import as genuinely ambiguous (create-vs-reset, depends on plugin UI gesture, out of this
wave's visibility). Neither was in the design doc's Phase 1 file list. One exception:
`import_triangle_mesh_to_body` (`📦️mesh-io/🦀️component.rs:207-245`) already built its own local
`rec` and calls `make_planar_face_from_points` — since that primitive's signature changed, this one
real (non-test) call site needed `, &mut rec` appended (`:237`); no new design decision, purely
mechanical.

**`🖋️imprint/🦀️component.rs`'s Correction-3 crack** (direct `body.coedges.remove`/`body.loops.remove`
bypassing euler, flagged by the design doc as a pre-existing issue needing a full read to fix
correctly) — left untouched, exactly as the design doc recommended ("flagged for the phase-1 agent
to either route through a new euler-level `kill_loop`/`kill_coedge` pair or explicitly re-document
the exception; not designed here"). Not designed there, not fixed here either — still flagged only.

**External call-site fallout, all mechanical.** Every changed function's callers needed the extra
argument:
- `🧰️kernel/🦀️component.rs`'s ~30 SyncApi wrapper methods each gained a local
  `let mut rec = OpRecorder::new();` and pass `&mut rec` — exactly the "throwaway recorder per call
  site for now" the design doc specifies, since these wrappers' own deletion is a separate,
  partially-blocked Phase 3 concern (below). `vertex_sync` already had this shape; unchanged.
- Test-only call sites in `🔮️oracle`, `🏷️classify`, `📦️mesh-io`, `🖋️imprint`, `📄️step`,
  `🧩️tessellate` each got a local `rec` too. Two files (`🔮️oracle`, `🖋️imprint`) have a pre-existing
  structural oddity — a bare `#[test] fn` living **outside** any `#[cfg(test)] mod tests` block
  (`🔮️oracle/🦀️component.rs:317`, `🖋️imprint/🦀️component.rs:462`) — flagged here, not restructured;
  it predates this wave and both still compile and run correctly either way.

**Verification.**
```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-3d --tests   → 0 errors
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-3d --lib
test result: ok. 408 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.78s
```
408 = 407 baseline + 1 new law test (`primitives::tests::make_box_surfaces_its_op_delta_to_the_caller`,
`🧱️primitives/🦀️component.rs`) asserting the whole box's `OpDelta` (26 generated entities: 8+12+6+1+1)
is now observable from outside `make_box` — the actual thing Phase 1 exists to prove, executed with
real output, not just a structural check.

**Benchmark delta** (`🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/benches/kernel.rs`, 9 groups):
| Group | Result |
|---|---|
| `booleans/fuse_box_box` | `320.75–325.58 µs`, criterion's own regression test: `change: [-2.8% -1.9% -1.0%] (p=0.08>0.05)` → **"No change in performance detected."** |
| `sweeps/sweep_polyline` (3 sizes) | completed clean, `861µs`–`57.9ms` scaling with segment count as expected |
| `tessellation` (6 sub-benchmarks) | completed clean, `8.7µs`–`97.2ms` |
| `features/fillet_all_edges/1`, `chamfer_all_edges/1` | completed clean, `533µs`/`124µs` |
| `primitives/box`, `primitives/sphere` | completed clean, `3.9µs`/`4.7µs` |
| `booleans/cut_box_sphere`, `sweeps/sweep_straight`, `features/fillet_all_edges/5+` | **pre-existing bench-fixture crashes, unrelated to this diff** — see below |
| `patterns` | did not complete within the session's time budget (single benchmark ran >11 minutes with zero criterion progress output); very likely pre-existing O(n·faces) chained-fuse cost in `grid_pattern`/`circular_pattern`, not measured to completion. Not claimed either way. |

**The three pre-existing bench crashes, verified by code inspection, not assumed:**
1. `booleans/cut_box_sphere` (`benches/kernel.rs:163`): `sphere_solid` returns a unit-radius sphere
   *centered at the origin* (AABB `[-1,1]³`), cut against a unit box at `[0,1]³` whose AABB is fully
   contained in the sphere's — `aabb_fast_path`'s `BooleanOp::Cut` branch deterministically hits
   `"tool contains target"` on every single invocation. Zero logic in `aabb_contains`/`boolean_solid`
   changed by Phase 1 (only the trailing `rec` parameter); this is a fixture geometry mismatch,
   not a regression.
2. `sweeps/sweep_straight` (`benches/kernel.rs:114`): `straight_path` calls
   `kernel.line_curve_sync(...)`, which registers a **Curve** entity, then `sweep_sync` calls
   `self.wire_ref(path)` expecting a **Wire** — a type mismatch in the fixture, unrelated to
   `sweep_along_path`'s own logic (unchanged except the trailing param).
3. `features/fillet_all_edges/5` (and presumably `/15`) (`benches/kernel.rs:213`): `multi_box_solid`
   builds a compound via chained mesh-fallback fuses; at 5+ boxes the resulting `all_edges` set
   apparently includes a non-manifold edge (`"blend edge must be shared by two solid faces"`) — a
   property of the mesh-boolean-fallback's triangle-soup stitching at that scale, not of
   `fillet_edges`'s signature change.

None of the three touch code this wave modified beyond adding a trailing parameter; none was
"fixed" (out of Phase 1–3's mandate — bench fixtures, not dissolution code), reported instead per
the evidentiary-bar rule.

---

## Phase 2 — `impl EngineRep<BrepArenaSeed> for Body` (DONE)

**Where.** `🕸️topology/🦀️component.rs`, new `// #region 🔖️EngineRep`. Reused the frozen W1
`EngineRep<P>: Sized { fn build(snapshot: &P) -> Self; }` from
`semio_framework_os_kernel` (re-exported at the crate root via `pub use crate::os_engine::*;`) —
did **not** invent a parallel idiom. `semio-framework-3d`'s `brep` feature already depends on
`semio-framework-os-kernel` (`Cargo.toml:20-26`), and the entire `crate::brep::*` module tree
(including `topo.rs`) only compiles when that feature is on, so the dependency is always present
wherever this `impl` lives — no new Cargo edge, confirmed by a clean compile.

**One design refinement over the design doc's literal sketch, reasoned through and stated
explicitly (not silently substituted).** §3's sketch used `String` ids inline in the seed fields
(`Vec<(String, Pnt3, Tol)>` etc.). I used **`PersistentLabel` directly** instead. §2 of the same
design doc already recommends this: *"a call-scoped `HashMap<String, PersistentLabel>` built fresh
inside each diff constructor... keeps `PersistentLabel`'s own representation fully decoupled from
whatever string convention stdio's snapshot ids use."* Baking `String` into framework-3d's own seed
type would re-couple exactly what §2 says to keep decoupled — the String↔label translation is
stdio's job, on stdio's side, once the handoff lands. `BrepArenaSeed` is therefore native to
framework-3d's own id type throughout, with zero string handling.

**The seed** (`BrepArenaSeed` + `SeedVertex`/`SeedEdge`/`SeedFace`/`SeedShell`/`SeedSolid`,
`🕸️topology/🦀️component.rs`, new region): `next_label: u64` (the label high-water-mark, carried
forward per §2 — never reset to 0) plus one `Vec` per entity kind, each item keyed by its own
`PersistentLabel`. Loops (which the design doc's §2 flags as having **no** `PersistentLabel` — an
open question for SMO, not resolved by this wave) are addressed by ordinal index into
`BrepArenaSeed::loops`, matching how `SeedFace::outer`/`inners` already have to reference them.
`#[derive(... PartialEq, Serialize, Deserialize)]` throughout, reusing framework's own
`Curve3`/`Surface`/`Tol`/`Pnt3` value types (all already `Clone + PartialEq + Debug`) rather than
inventing parallel ones.

**`build()`** (`🕸️topology/🦀️component.rs`, `impl EngineRep<BrepArenaSeed> for Body`): seeds
`body.labels = LabelSource::from_next(seed.next_label)` (new constructor, `📜️history/🦀️component.rs`
— `LabelSource::next` was a private field with no way in from outside the module; added
`from_next`/`next()` as the minimal accessor pair this needed), then inserts directly into every
`Store` — vertices, then edges (resolving `v0`/`v1` labels through a local map), then loops (via
`crate::brep::euler::make_loop`, which is safe to call here specifically **because** it mints no
label — the one euler function this can use without breaking the round-trip law), then faces
(minting nothing, patching each participating loop's `.face` back-reference exactly like
`primitives::attach_face` already does), then shells, then solids. The docstring on `build()` states
explicitly why it must never call `euler::make_vertex`/`make_edge`/`add_face`/`add_shell`/
`add_solid` (they mint fresh labels — correct for a real create, wrong for restoring an existing
entity's identity) — the design doc flagged this as the one place a naive implementation would
silently break the law, so it's spelled out in the code, not just this report.

**`to_seed(&Body) -> BrepArenaSeed`** (same region): the mirror-image extraction, needed both for
the round-trip law and, per §3 Law B, by a future diff constructor reading post-op state back out.
Loop ordering is memoized by `LoopId` in first-encountered order across `body.faces.iter()`, which
is exactly the order `build()` assigns indices in — this is what makes the round-trip law hold, not
an incidental detail.

**Law A — executed, not just structural.** Four new tests in `🕸️topology/🦀️component.rs`'s existing
`mod tests` (no new test file):
- `engine_rep_build_round_trips_a_closed_box_through_to_seed` — builds a real box via the checked
  euler editors (`make_box`), extracts a seed, rebuilds, re-extracts, asserts full equality
  (8 vertices / 12 edges / 6 faces / 1 shell / 1 solid, all labels and geometry).
- `engine_rep_build_round_trips_a_loose_planar_face` — same law on a face with no shell/solid
  wrapper, to exercise the `outer`/`inners` index bookkeeping independently.
- `engine_rep_build_is_deterministic_for_identical_seeds` — the frozen W1 contract itself
  (`build(s) == build(s)` for identical `s`), executed against `to_seed` equality of two independent
  builds from the same seed.
- `engine_rep_build_preserves_the_label_high_water_mark` — asserts `next_label` actually survives
  the round trip rather than resetting to 0 (the exact collision risk §2 names).

```
CARGO_TARGET_DIR=".../🎯️target" cargo test -p semio-framework-3d --lib topo::
running 11 tests
test brep::topo::tests::engine_rep_build_round_trips_a_loose_planar_face ... ok
test brep::topo::tests::engine_rep_build_round_trips_a_closed_box_through_to_seed ... ok
test brep::topo::tests::engine_rep_build_preserves_the_label_high_water_mark ... ok
test brep::topo::tests::engine_rep_build_is_deterministic_for_identical_seeds ... ok
test brep::topo::tests::serde_round_trips_a_whole_body ... ok
[... 6 pre-existing topo tests, all ok ...]
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 401 filtered out
```
`serde_round_trips_a_whole_body` was already present before this wave (contradicting the design
doc's "not covered by any test today" note on `Body`'s serde round trip — corrected here, not
silently absorbed).

**Law B** (`SemioBrepSnapshot → Seed → SemioBrepSnapshot` identity, and diff-shortcut vs.
rebuild-from-scratch agreement) is explicitly stdio's to write once the handoff lands, per §3 — not
attempted here; there is no `SemioBrepSnapshot` code in this crate's write boundary to test against.

**Full-crate verification after Phase 2:**
```
cargo test -p semio-framework-3d --lib
test result: ok. 412 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.21s
```
412 = 408 (post-Phase-1) + 4 new law tests.

---

## Phase 3 — strip the indirection: BLOCKED in its full form, partially executed

**The design doc's own Phase 3 (`§6`) is: "delete the Registry region and the 90 SyncApi wrappers."
I did not do this, and I'm reporting why rather than forcing it, per the ticket's standing rule that
an honest partial wave beats a forced one.**

**The evidence, read directly, not inferred:**
- `🧰️kernel/🦀️component.rs`'s `🔖️BrepKernel` region (`#[async_trait(?Send)] impl BrepKernel for
  Brep`) delegates **every single one** of its ~92 async methods to a matching SyncApi method,
  1-line each (`async fn box_prim(...) { self.box_prim_sync(...) }`, verified for the whole region,
  not sampled). Diffing SyncApi's 95 `pub fn *_sync` names against the trait's async fn names by
  base name (`comm -23` on sorted name lists) found only **5** without a name-matching async
  counterpart: `export_glb`/`import_glb` (used indirectly via `export_gltf_sync`/the `Codecs`
  region's `GlbSolidExporter`/`GlbSolidImporter`), `solid_face_loops` (used directly by
  `✏️s/🔌️plugins/💠️lowpoly/…/🧵️media/🦀️component.rs:133` — a live external consumer, confirmed by
  grep), and the two now-deleted ones below.
- `BrepEngineHost` (`⚙️engine/🖥️host/🦀️component.rs`) is explicitly **OUT OF SCOPE** per this
  agent's brief: it has live external consumers — `✏️s/🔌️plugins/🏭️process/…/⚙️engine/🦀️component.rs`
  (a struct field) and `✏️s/🔌️plugins/📐️cad/…/⚙️engine/🦀️component.rs` (a process-global
  `OnceLock`) — confirmed again here by grep, unchanged from the design doc's own finding.
  `BrepEngineHost::kernel()` hands out `&Mutex<Brep>`, and every call into it — whether through the
  async `BrepKernel` trait or `Brep`'s own `pub fn *_sync` methods directly — depends on Registry
  and SyncApi staying intact.
- **Consequence, stated by the design doc itself**: *"Recommend running Phase 5 before Phase 3...
  merge Phase 3 into Phase 5's cleanup rather than doing it twice."* Phase 5 (deleting
  `BrepEngineHost`/`BrepKernel`/`GeometryHandle`) is explicitly gated on cross-session coordination
  with whoever can migrate `process3d`/`cad`, and is **not** in this agent's Phases 1–3 mandate.
  Deleting Registry/SyncApi now, before that migration, would break `BrepEngineHost`'s only public
  surface — directly violating the boundary instruction *"Delete only what is internal to
  `semio-framework-3d` and has no consumer outside it."*

**What I did instead: found and removed the two SyncApi methods that genuinely have zero callers
anywhere, verified by grep across the whole live tree (not just this crate), not assumed:**
- `retain_sync` (`🧰️kernel/🦀️component.rs`) — the async trait's own `retain` implements
  `self.live.retain(...)` **directly**, never calling `retain_sync`; grepped repo-wide for
  `retain_sync` and found zero other callers of *this* `Brep::retain_sync` (a same-named but
  unrelated `DrawingStore::retain_sync` exists in `◻2d/🗄️store/🦀️component.rs` — a different type,
  outside my boundary, correctly left untouched — the exact "same identifier, different thing" trap
  `📌️important.md` warns about, caught here rather than walked into).
- `tessellate_to_mesh_data_sync` (`🧰️kernel/🦀️component.rs`) — zero callers anywhere in the live
  tree; the only other hits repo-wide are in
  `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/OS-EXCLUSIVE-STATE-AUTHORITY/🧪brep-kernel-HEAD.rs`, a scratch
  snapshot file inside a **different, unrelated ticket's folder** — not live source, not compiled
  into any crate.

Deleting these two is genuinely "internal to `semio-framework-3d`, no consumer outside it" — unlike
the other 93, they were never reachable from `BrepEngineHost` at all.

**Verification after the two deletions:**
```
cargo test -p semio-framework-3d --lib
test result: ok. 412 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.27s
```
(412, not 413, at this point — the `LabelSource` unit test below was added after this run.)

**What Phase 3 would need to proceed further, honestly stated rather than guessed at:** the
coordinator's own Phase-4 gate (cross-session agreement on migrating `process3d`/`cad` off
`BrepEngineHost`, or an explicit greenfield exception recorded for keeping it). Until that lands,
the other 93 SyncApi/Registry methods stay exactly as they are — not because they're hard to delete,
but because deleting them breaks code outside this crate's boundary that this agent has no
authority to also migrate.

---

## Files touched

**Edited** (all within `🧰️framework/🔨️modules/🧊️3d/**`):
- `📐️brep/🧱️primitives/🦀️component.rs` — 12 signatures + 9 test call sites + 1 new law test
- `📐️brep/🔀️boolean/🦀️component.rs` — 4 public + 4 private signatures + 6 test call sites
- `📐️brep/➡️sweep/🦀️component.rs` — 6 public + 3 private signatures + 5 test call sites
- `📐️brep/🎨️blend/🦀️component.rs` — 3 public + 1 private signature + 5 test call sites, new import
- `📐️brep/↔️offset/🦀️component.rs` — 6 public + 4 private signatures + 5 test call sites
- `📐️brep/🧵️sew/🦀️component.rs` — 1 signature + 3 test call sites
- `📐️brep/🩹️heal/🦀️component.rs` — 3 signatures (2 now genuinely recording provenance for the first
  time) + 5 test call sites, new import
- `📐️brep/🧰️kernel/🦀️component.rs` — ~30 SyncApi call sites updated, 2 dead methods deleted
  (`retain_sync`, `tessellate_to_mesh_data_sync`)
- `📐️brep/📦️mesh-io/🦀️component.rs` — 1 real call site + 6 test call sites
- `📐️brep/🔮️oracle/🦀️component.rs` — 1 test call site
- `📐️brep/🏷️classify/🦀️component.rs` — 7 test call sites, new import
- `📐️brep/🖋️imprint/🦀️component.rs` — 3 test call sites
- `📐️brep/📄️step/🦀️component.rs` — 1 test call site
- `📐️brep/🧩️tessellate/🦀️component.rs` — 1 test call site
- `📐️brep/📜️history/🦀️component.rs` — `LabelSource::from_next`/`next()` added, 1 new unit test
- `📐️brep/🕸️topology/🦀️component.rs` — new `EngineRep` region (`BrepArenaSeed` + 5 `Seed*` types +
  `impl EngineRep<BrepArenaSeed> for Body` + `to_seed`), 4 new law tests

**Created** (ticket-folder scratch, per hard rule 3): this report; no other scratch files were
needed — verification ran directly against the shared `🎯️target`, no isolated harness required
since the crate itself compiled and tested cleanly throughout.

**Not touched**: `BrepEngineHost`, `BrepKernel` trait, `GeometryHandle`, `Codecs` region, `Registry`
region (except the 2 dead methods), `🖋️imprint`'s Correction-3 crack, the `Loop`/`Coedge` label
question (§2, still open — needs SMO), `📄️step/🦀️component.rs::read_step`,
`📦️mesh-io/🦀️component.rs`'s import functions' own signatures.

## Verification commands run, with real output pasted

All shown inline above, per phase. Summary of the four full-suite runs, in order:
1. Baseline: `407 passed; 0 failed`
2. Post-Phase-1: `408 passed; 0 failed`
3. Post-Phase-2: `412 passed; 0 failed`
4. Post-Phase-3 (2 dead-method deletions): `412 passed; 0 failed`
5. Final (after adding the `LabelSource` unit test): `413 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.20s`

```
CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-3d --tests
   → 0 errors (warnings only, all pre-existing or from unrelated files)
```

Repo-wide grep for every changed function name, excluding this crate and `target`/`🎯️target` dirs,
confirmed zero live external callers of any changed free-function signature — every non-`.rs`-target
hit outside the boundary is either (a) a scratch snapshot file in a different, unrelated ticket's
folder, or (b) a call through the unchanged `BrepKernel` async trait / `Brep`'s public `*_sync`
methods, whose signatures this wave never touched.

## sharedFileRequests

None. Phases 1–2 are fully self-contained inside `semio-framework-3d`. Phase 3's blocker
(`BrepEngineHost`/`process3d`/`cad`) is not a file-edit request — it's the same cross-session gate
the design doc's §4/Phase 4 already names, owned by the coordinator, not something a shared-file
patch would resolve.

## Concurrent-churn observations

None in this boundary. `git log --oneline -3` at start and end both show the same auto-commit
cadence (`🚩️495` at session start) with no unexplained gaps; `semio-framework-3d`'s only dirty files
throughout this session were the ones listed above. No peer-owned file was read as broken or
touched.

## Honest pass/fail

**Phases 1 and 2: PASS**, verified green with real, executed output at every step, including the law
tests that are each phase's actual deliverable (not just a structural gate — `make_box`'s `OpDelta`
genuinely escapes the call now; `EngineRep::build`/`to_seed` genuinely round-trip a real box built
through the checked euler editors, not a hand-waved fixture).

**Phase 3: PARTIAL, blocked by design, not by failure to try.** The bulk of it (deleting Registry +
90 SyncApi wrappers) is real work this agent could execute mechanically, but doing so today would
delete the only public surface `BrepEngineHost` exposes to `process3d` and `cad` — both outside this
crate, both explicitly out of this agent's boundary, both with live external consumers this agent
verified by grep rather than assumed. The 2 methods with zero consumers anywhere were deleted and
verified. The remaining 93 stay until the coordinator's Phase-4 cross-session gate opens — reported
as blocked, not silently skipped, per the ticket's standing rule that an honest boundary is worth
more than a forced completion.

**Benchmark evidence**: no regression detected anywhere Phase 1's `rec` parameter threading could
plausibly cause one (5 of 9 groups completed clean; `booleans`' one clean sub-benchmark showed
criterion's own "no change" verdict). Three pre-existing bench-fixture bugs were found and
attributed with code-level evidence, not fixed (out of this wave's mandate). The `patterns` group
did not complete within the session's time budget and is reported as not measured, not guessed.
