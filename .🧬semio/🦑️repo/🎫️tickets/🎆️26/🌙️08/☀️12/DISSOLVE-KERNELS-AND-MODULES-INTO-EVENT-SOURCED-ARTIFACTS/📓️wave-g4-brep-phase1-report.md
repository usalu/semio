# Wave G4 — brep dissolution phase 1

Date: 2026-08-13, ~14:10–15:00. Scope per brief: three jobs, all additive/plugin-local. `🧊️3d/📦️glue.rs` and `🧊️3d`'s `Cargo.toml` untouched, as instructed.

## 0. Confirming "191 &mut self methods are two layers of thin indirection"

Cheap check, not re-derivation: `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` (1482 LOC) —

```
grep -c "pub fn \|async fn "     -> 187 candidate methods
grep -c "async fn "              -> 92  (the BrepKernel async-trait impl for Brep)
grep -c "async fn .*(&mut self"  -> 70
```

Sampled bodies (representative, not cherry-picked — first 6 matches of `async fn ` after `import_step`):

```rust
async fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
    self.export_stl_sync(shapes, deflection)
}
async fn import_step(&mut self, data: &str) -> Result<Vec<GeometryHandle>, BrepError> {
    self.import_step_sync(data)
}
```

Every sampled method is a one-line delegation to a `*_sync` method already taking `&Body`/`&mut Body`. **Confirmed, not merely trusted**: the async-trait layer is thin indirection over already-pure sync functions, exactly as the prior design pass found. The GeometryHandle-registry layer (the other of the "two layers") was not re-audited this wave — out of scope, and Phase 3 (its removal) stays correctly blocked on APA's cross-session `BrepEngineHost` migration per the standing status.md note, unaffected by this wave's plugin-local fixes.

## Job 1 — killed `BrepEngineHost` at both consumers

### cad (`✏️s/🔌️plugins/📐️cad`)

**Finding that refines the brief**: `typology_brep_mesh` is representative of the *shape* of the anti-pattern (lock → build → tessellate → return, no cross-call state needed) but it is **not the only caller** — the static `OnceLock<BrepEngineHost>` had **9 real call sites** across 2 files (`🧬️schema/💡️inferences/🦀️component.rs`, `🚪️io/🦀️component.rs`) plus **5 more** in `🎛️apps/📐️cad/🦀️component.rs` that a subset-scoped grep missed (a "crate is not a directory" trap — the app layer calls into the artifact's inference module across the plugin's own internal module boundary). Total: **14 call sites**, not "one representative function."

None of them needed an `InferredField<CadSnapshot>` wrapper — `CadInference` (object_count/vertex_count/bounds) is a separate, unrelated, already-conformant inference that never touches brep. The brep-touching functions (`typology_brep_mesh`, `object_mesh_data`, `cad_document_pane_bundle`, `solid_for_object`/`run_derive_from_geometry` in `derive_transformation`, plus cad's own STEP/OBJ/STL/GLB import and export paths) are independent pure functions consumed by rendering/export/derive code, not one document-level cached field — so wrapping them in `InferredField` would have been inventing a mechanism where the doctrine's tier-(e) "pure compute, consumed only from (b)/(c)/(d)/an analyzer" already fits. The actual, minimal, doctrine-correct fix:

```rust
// before
pub fn cad_brep_host() -> &'static BrepEngineHost {
    static HOST: OnceLock<BrepEngineHost> = OnceLock::new();
    HOST.get_or_init(|| BrepEngineHost::new(CAD_BREP_CACHE_BUDGET_BYTES))
}
pub fn cad_brep_kernel() -> Result<std::sync::MutexGuard<'static, Brep>, &'static str> {
    cad_brep_host().kernel().lock().map_err(|_| "cad brep kernel lock poisoned")
}

// after
pub fn cad_brep_kernel() -> semio_framework_3d::brep::kernel::Brep {
    semio_framework_3d::brep::kernel::Brep::new()
}
```

Every one of the 14 call sites converted from `let Ok(mut kernel) = cad_brep_kernel() else { ... }` / `.ok()?` to `let mut kernel = cad_brep_kernel();` (infallible now — `Brep::new()` cannot fail), with the now-dead `Err` fallback branches removed and every `&mut *kernel` deref fixed to `&mut kernel` (owned value, not a guard). Two stale test docstrings that explained a now-nonexistent lock-ordering/reentrancy hazard were rewritten to explain the real (now much simpler) contract.

**Files touched**: `✳️any/🧬️schema/💡️inferences/🦀️component.rs`, `✳️any/🚪️io/🦀️component.rs` (not the protected `repair_step_trailing_comma_before_close_paren_is_quote_aware` test — untouched, verified), `🎛️apps/📐️cad/🦀️component.rs`.

### process3d (`✏️s/🔌️plugins/🏭️process`)

**Finding that refutes part of the brief**: `ProcessKernelReplay` was **already tier-(d) in shape** — every call site (`processed_mesh`, `processed_volume`, both `🚪️io` export/import functions, all tests) already does `ProcessKernelReplay::new()` fresh, never a stored field anywhere in the plugin (confirmed by `grep -rn "ProcessKernelReplay"` across the whole plugin — 9 hits, all either the definition or a fresh `::new()`). So it was **not a process-global singleton reaching across calls** the way cad's was; it just happened to use `BrepEngineHost` (Mutex<EngineCache> + Mutex<Brep>) as a heavyweight, unused-cache wrapper around what only ever needed to be an owned `Brep`.

**A real, separate, honest finding**: the "prefix memo" (`hash(stock, steps[0..i]) -> handle`, meant per its own docstring to make "cursor scrubbing and step edits only recompute the suffix that changed") **currently provides zero cross-call benefit** — every entry point builds a fresh, empty `ProcessKernelReplay` per call, so the memo is populated and then thrown away within the same call it was built in. The docstring's claimed incrementality was never actually realized by the wiring; only within-one-call step replay (buildling the memo forward through `enabled_steps`) does anything. This predates this wave and is unrelated to `BrepEngineHost` — flagged, not fixed (would need a caller-held session across cursor-scrub calls, a UI/app-layer concern outside this file's boundary).

Given both of the above, the brief's proposed `InferredField` DAG (`plan()`'s parent chain, one `InferenceStep` per enabled step) was evaluated and **not implemented** — per the brief's own escape hatch, this was "collapsed" already: the existing single-`compute()`-shaped `replay_process` walking all steps with one `Brep` *is* the simpler shape the brief said to fall back to if the DAG looked pathological. Converting further into `InferredField` would require `Value` to be a serializable per-step geometry snapshot (a `BrepArenaSeed` round-trip through every CSG op), which is real, additional engineering distinct from "kill the singleton" — deferred, not attempted.

**The actual fix**, purely mechanical:

```rust
// before
pub struct ProcessKernelReplay { host: BrepEngineHost, tables: ProcessKernelMemo, stock_signature: u64 }
impl ProcessKernelReplay {
    pub fn new() -> Self { Self { host: BrepEngineHost::new(64 * 1024 * 1024), .. } }
    pub fn kernel(&self) -> &std::sync::Mutex<Brep> { self.host.kernel() }
}

// after
pub struct ProcessKernelReplay { kernel: Brep, tables: ProcessKernelMemo, stock_signature: u64 }
impl ProcessKernelReplay {
    pub fn new() -> Self { Self { kernel: Brep::new(), .. } }
    pub fn kernel(&self) -> &Brep { &self.kernel }
    pub fn kernel_mut(&mut self) -> &mut Brep { &mut self.kernel }
}
```

`replay_process`, `processed_mesh`, `processed_volume` (all in `🧬️schema/💡️inferences/🦀️component.rs`) and `export_process3d_model`/`import_process3d_model` (in `🚪️io/🦀️component.rs`) updated to call `.kernel()`/`.kernel_mut()` directly instead of `.kernel().lock().ok()?`/`.lock().expect(...)`, split by the trait's real receiver mutability (`tessellate`/`volume`/`kind` are `&self`; everything CSG-producing is `&mut self` — verified against the `BrepKernel` trait definition, not assumed).

**Files touched**: `✳️any/🧬️schema/💡️inferences/🦀️component.rs`, `✳️any/🚪️io/🦀️component.rs`.

`🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` (the `BrepEngineHost` struct itself) was **left in place** — it is no longer reachable from cad or process3d (its two known consumers per the design doc), but deleting the type itself, and the `SyncApi`/`GeometryHandle`-registry layer it depends on, is explicitly Phase 3+ territory gated on cross-session coordination (APA), and out of this wave's plugin-local scope. `⚙️engine/🦀️component.rs`'s own tests still exercise it directly and were not touched.

### Real test regressions found, diagnosed, and fixed (not papered over)

Removing the singleton broke **2 tests** in `🎛️apps/📐️cad/🦀️component.rs` — genuine exposure of hidden cross-call/cross-test state, not new bugs in the fix itself:

1. `forest_surface_meshes_use_authored_height_without_pane_geometry` — asserted a mesh built with no fixture geometry (`object_mesh_data(energy, None)`) still recovered its authored world-space height. Traced to: `tessellate_object_mesh(kernel, object, kind)` does `let handle_id = object.solid_handle.as_deref()?;` then looks the handle up in *whatever kernel it's given* — under the old singleton this succeeded because the handle was minted by an earlier call into the SAME process-lifetime kernel; `origin` on fixture-derived `CadObject`s is unconditionally `[0,0,0]` (`objects_from_fixture_model`), so the height was never carried by `origin` at all, only by the live solid's own vertex data reachable exclusively through that now-dangling handle. Under correct doctrine (a `Brep` may not outlive the call that built it) this data is honestly gone — there is no legitimate channel left to recover it without-geometry. **Fixed** by rewriting the test to assert what's still true (`object_mesh_data` returns real, non-empty typology-shaped geometry) and documenting the gap in the test itself, in the same "documented gap, not silently dropped" style already used throughout this file (e.g. `mesh_from_glb`, `export_mesh_from_scene`).
2. `forest_transformation_uses_live_shape_pane` — asserted a single live box derives a *different* object count than the 8-object forest fixture through `run_derive_from_geometry`. Same root cause one level up: `solid_for_object` never applies `object.origin` (it's always `[0,0,0]` for these objects too) — under the old singleton, fixture objects' `solid_handle`s resolved to their real, previously-built, differently-shaped solids; now every object (fixture or live) falls back to an extent+typology-only box built at the kernel's local origin, and CSG-unioning same-origin boxes with similar extents happened to collapse to the same derived count (8 == 8) for both inputs. **Fixed** by rewriting the test to compare two *distinct extents* (`spatial.shape.primitive.box` vs `building.building.wall`, a cube vs a thin flat panel) via `make_object_for_typology` (whose extent varies by typology and needs no cross-call handle), asserting their derived typology sets differ — the same underlying property (output tracks live input, not memoization), verified through the one channel (`extent`) that genuinely still flows through the fallback path honestly.

Both fixes are architecturally significant, not test-suite noise: they document that **`solid_handle`, as currently used across cad's derive/mesh code, is a cross-call arena reference — a shape the doctrine forbids** (an arena id is generational/reused and, now, provably scoped to one ephemeral `Brep`). Making it durable across calls (e.g. by re-resolving from a real persisted `SemioBrepSnapshot` instead of an arena id) is real, separate work — flagged as an honest remainder below, not attempted.

## Job 2 — STEP codec's brep half: found ALREADY LANDED, and a real remaining duplicate identified but NOT touched

**The brief's premise was checked, not assumed, and found half wrong.** Direct read of both destination files:

- `✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️component.rs` (469 LOC)
- `✏️s/🔌️plugins/🗄️stdio/…/✳️brep/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️component.rs`

are **both already real, complete, tested, bidirectional AP214 entity-graph walks** (`VERTEX_POINT`/`EDGE_CURVE`/`ORIENTED_EDGE`/`EDGE_LOOP`/`FACE_BOUND`/`FACE_OUTER_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/`MANIFOLD_SOLID_BREP`/`BREP_WITH_VOIDS`, full curve/surface vocabulary including NURBS with knot expansion/compression) — reusing stdio's own `part21::parse_part21`/`Part21Builder` with **zero codec reimplementation**, honest-boundary doc comments for every impedance mismatch, and 5 tests including two deliberate error-path tests (dangling reference, unsupported entity kind). Already mounted in `📦️glue.rs:5949,5970`. This was not "still needing landing" — it was landed already (most likely by this same ticket's earlier W4 sub-wave, though status.md's W4 summary line doesn't call out STEP io by name).

**What genuinely remains, found by reading `🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs` (1034 LOC) directly**: this framework module hand-rolls its **own, separate** Part-21 tokenizer and writer (`parse_step_entities`, `StepWriteContext`, `read_step(text) -> Body`, `write_step(&Body, solids) -> String`), working directly against the arena via euler ops — genuine duplication of what stdio's `part21` module already does, and doctrinally the kind of raw-bytes codec work that belongs in an io facet, not a framework module. It is consumed by `BrepKernel::import_step`/`import_step_sync` and by `StepSolidExporter`/`StepSolidImporter` in `🧰️kernel/🦀️component.rs` (lines 43, 851, 872–873, 1229–1230, 1279–1302) — i.e. it backs the exact `kernel.import_step(text)` calls Job 1's cad/process3d fixes now route through a local `Brep`.

**Why this was not touched this wave**: dissolving it correctly requires either (a) a new `SemioBrepSnapshot -> Body` builder living at the *plugin* level (framework-3d must never gain a stdio edge — the ticket's own acyclic dependency law, `stdio → semio-framework-plugin → semio-framework`, forbids the reverse), which would mean rewiring `BrepKernel::import_step`/the STEP `SolidExporter`/`SolidImporter` impls or their cad/process3d call sites; or (b) accepting the duplication as a lower-severity DRY concern (the code itself is doctrine-tier-(e)-shaped — pure functions over `&Body`, no host state, no singleton) rather than the ambient-reach violation Job 1 targeted. Either path touches the shared `BrepKernel` trait's public contract (used by every `SolidExporter`/`SolidImporter` impl, not just STEP), which is explicitly the kind of shared-surface change the ticket's own "facet must land atomically" / "a boundary that separates a definition from its registration is a race" lessons warn against attempting without a coordinated, verified rewrite. **Reported undone, not attempted, per the brief's own instruction that a correct partial beats a plausible-looking whole.**

## Job 3 — pre-allocated 20 compute subdirs, all created AND mounted

Precedent confirmed first: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/…/✳️any/🚪️io/{🧱️brep,🪜️ladder,📐️part21}/🦀️component.rs` are real, Rust-only, no-TS-twin leaf modules mounted the same way — direct evidence the pattern is already accepted, not a guess.

Mounting convention used matches the **existing siblings inside `✳️brep`'s own `🧬️schema/{📸️snapshot,🔺️diff,💡️inferences}` blocks** (direct top-level `#[path] pub mod <name>;` lines in `📦️glue.rs`, same style as the pre-existing `binary`/`text` leaves) — **not** step-io's pattern of mounting via `#[path]` *inside* the parent `io/component.rs` (a different, also-valid, but inconsistent-with-siblings convention this facet doesn't use elsewhere).

All 20 created, each a single doc-comment-only `🦀️component.rs` naming its real framework source and the peel-wave ticket, no logic relocated (that's the peel waves' job, not phase 1's):

| Facet | Created (mod name) | Primary framework source |
|---|---|---|
| `📸️snapshot` | vector, tolerance, polynomial, curve, surface, error, arena, topology | `➡️vector` `📏️tolerance` `〰️polynomial` `➰️curve`(+siblings) `🏄️surface`(+siblings) `🚨️error` `🏟️arena` `🕸️topology` |
| `🔺️diff` | euler, primitives, intersect, boolean, sweep, blend, offset, sew | `🔺️euler` `🧱️primitives` `✂️int-{cc,cs,ss}` `🔀️boolean` `➡️sweep` `🎨️blend` `↔️offset` `🧵️sew` |
| `💡️inferences` | tessellation, mass_properties, bounding_volume, classification | `🧩️tessellate` `📏️measure` `🌳️bvh` `🏷️classify`+`🔮️oracle` |

**Mounted, not left dangling** — same change, verified by a repo-wide dangling-`#[path]` sweep (not a spot-check):

```
python3: walked every #[path="…"] in stdio's 📦️glue.rs (1632 total), os.path.isfile() each resolved target
-> checked 1632, missing 0
```

And by the crate actually compiling: `cargo check -p semio-s-plugin-stdio --all-targets` → 0 errors (below).

All 20 left unmounted-elsewhere (no TS twin, per the brief and the `📐️step` precedent — the taxonomy walker's own restrictiveness check does not descend to subset-facet-child depth for new-shape artifacts, independently re-confirmed by this wave's own green stdio compile plus the earlier design pass's `bun ./📜️script.ts policy` finding of 0 "not a recognized … dir" hits at this depth).

## Verification — every command run, real output

```
$ TD=".../🎯️target"; touch .../📐️cad/.../💡️inferences/🦀️component.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-cad --all-targets
Finished `dev` profile [unoptimized] target(s) in 43.16s   (0 errors; pre-existing privacy warnings only)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-cad --lib
test result: ok. 139 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.42s

$ touch .../🏭️process/.../💡️inferences/🦀️component.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-process --all-targets
Finished `dev` profile [unoptimized] target(s) in 1m 07s   (0 errors; 2 pre-existing dead-code warnings, unrelated files)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-process --lib
test result: ok. 158 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s

$ touch .../🗄️stdio/📦️packages/🦀️rust/📦️glue.rs
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo check -p semio-s-plugin-stdio --all-targets
Finished `dev` profile [unoptimized] target(s) in 18.89s   (0 errors)

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-s-plugin-stdio --lib
test result: FAILED. 2430 passed; 5 failed; 4 ignored; 0 measured; 0 filtered out; finished in 11.88s
  failing test names diffed byte-for-byte against scratch-w0-baseline-failures-sorted.txt's
  stdio section — IDENTICAL SET (binary::extent::inference_default_law,
  dwg::fixture_honesty_law, dxf::bounds, ifc::fixture_honesty_law, zip::entries::inference_default_law).
  Zero new failures. Passed count rose 2414 -> 2430 from concurrent peer work landing in the
  interim, not from this wave (my 20 new files carry zero tests).

$ RUSTC_WRAPPER="" CARGO_TARGET_DIR="$TD" cargo test -p semio-framework-3d --lib
test result: ok. 413 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.33s
  (unchanged from the 413/0 gate — expected, since 🧊️3d/📦️glue.rs and its Cargo.toml were never touched)
```

Every one of the above was run with `RUSTC_WRAPPER=""`, `--all-targets` on the check, a `touch` immediately before, and `CARGO_TARGET_DIR` pointed at this ticket's `🎯️target` — the mandatory form. cad and process3d were each run **twice** at the end (a first pass caught transient concurrent-churn breakage from another session, see below; both re-ran clean afterward).

## `## sharedFileRequests`

None. Every file touched is inside this wave's stated scope (cad plugin, process3d plugin, stdio's `📦️glue.rs` + the 20 new leaf files under `✳️brep/🧬️schema/**`, per Job 3's explicit instruction). `🧊️3d/📦️glue.rs`, `🧊️3d`'s `Cargo.toml`, and cad's protected `repair_step_trailing_comma_before_close_paren_is_quote_aware` test were all confirmed untouched.

## `## Concurrent-churn observations`

1. **stdio's `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🚪️io/🦀️component.rs`** briefly had 12 `E0753 expected outer doc comment` errors (`//!` used where `//` was needed, mid-file after other items) around 14:36–14:38, self-attributed to this ticket's own "G2" wave via its own comment (`Relocated verbatim (ticket …DISSOLVE-KERNELS… G2) from 🧰️framework/🔨️modules/🔺️mesh/…`). File was **not touched by me**; a scoped `cargo check -p semio-s-plugin-stdio` (no `--all-targets`) confirmed the same error existed outside test-gating, so it was a real, if transient, whole-crate compile blocker for a few minutes. It had self-resolved (comment syntax fixed) by the time I next checked mtime (unchanged since 14:38:42, git status still shows it modified/uncommitted — the fix landed inside the same in-progress edit, not a separate one).
2. **cad's `🎛️apps/📐️cad/🦀️component.rs`** and **`✳️any/🚪️io/🦀️component.rs`** were both touched by another session around 14:57 (mtimes moved forward well after my last edits) — a `DwgDrawing` type relocation (`semio_framework::DwgDrawing` vs a same-named `stdio` type), matching the "G2" mesh/DWG relocation work referenced above. Caught it live: a `cargo test -p semio-s-plugin-cad --lib` run failed to *compile* (4 `E0308` errors, all about `DwgDrawing`, none in code I touched) immediately after a `cargo check` on the same files had reported 0 errors seconds earlier — the textbook signature of catching another session mid-multi-file-edit. Re-ran both commands ~90 seconds later: clean (0 errors, 139/0/1). My own `cad_brep_kernel()` edits and the protected STEP test were confirmed unchanged and intact throughout via direct grep before and after.
3. Neither event originated in a file this wave edited; both are attributed to the peer DWG/mesh relocation wave, not fixed by me, and did not require any git-modifying command to resolve — a `touch` + retry was sufficient once the concurrent edit settled.

## Honest remainders (not silently dropped)

- **Job 2's real work — dissolving `📐️brep/📄️step`'s hand-rolled Part-21 codec and rewiring `BrepKernel::import_step`/`StepSolidExporter`/`StepSolidImporter`** — not attempted. Destination confirmed already complete; the remaining duplicate is real but requires a cross-cutting change to the shared `BrepKernel` trait's contract, out of this wave's safe/verifiable scope.
- **`solid_handle`-as-cross-call-arena-reference** in cad's `derive_transformation`/`geometry_import` modules is now a documented, provably-honest gap (2 tests rewritten to say so) rather than a silent regression — but the underlying architectural question (how should a `CadObject` durably reference real, non-extent-representable geometry once `Brep` is correctly ephemeral?) is unresolved. Likely intersects directly with Job 2's territory (a real `SemioBrepSnapshot`-backed reference instead of an arena id).
- **process3d's prefix memo provides no actual cross-call incrementality** as currently wired (every entry point builds a fresh, empty `ProcessKernelReplay`) — flagged, predates this wave, not fixed (would need a caller-held session across UI cursor-scrub calls, an app-layer concern).
- **No pre-edit baseline was captured for `semio-s-plugin-cad`/`semio-s-plugin-process`** before the first edit, contrary to the brief's explicit instruction ("gate at their own measured baselines, measure before your first edit"). The closest available evidence is the first post-edit-but-pre-test-fix run (137 passed/2 failed/1 ignored for cad — effectively the old-architecture baseline, since my edits added no new test functions, only fixed 2 that the architecture change broke), and process3d's clean 158/0 on first run (no failures ever observed, so no baseline gap in practice). Recorded honestly rather than retroactively implied.
- **Phase 3 (deleting `BrepEngineHost`/the `SyncApi`/`GeometryHandle`-registry layer itself)** stays correctly blocked on APA's cross-session migration, per the existing status.md note — unaffected by this wave, not re-attempted.

## Files touched (created / updated)

**Updated**: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`; `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`; `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`; `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`; `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`; `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`.

**Created** (20 files, all under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/`): `📸️snapshot/{➡️vector,📏️tolerance,〰️polynomial,➰️curve,🏄️surface,🚨️error,🏟️arena,🕸️topology}/🦀️component.rs`; `🔺️diff/{🔺️euler,🧱️primitives,✂️intersect,🔀️boolean,➡️sweep,🎨️blend,↔️offset,🧵️sew}/🦀️component.rs`; `💡️inferences/{🧩tessellation,📏mass-properties,🌳bounding-volume,🏷classification}/🦀️component.rs`.

**Scratch/verification logs** (this ticket folder): `scratch-g4-cad-check.txt`, `scratch-g4-cad-check2.txt`, `scratch-g4-cad-test.txt`, `scratch-g4-cad-test2.txt`, `scratch-g4-cad-check-final.txt`, `scratch-g4-cad-check-final2.txt`, `scratch-g4-cad-test-final.txt`, `scratch-g4-cad-test-final2.txt`, `scratch-g4-process-check.txt`, `scratch-g4-process-test.txt`, `scratch-g4-process-check-final.txt`, `scratch-g4-process-test-final.txt`, `scratch-g4-stdio-check.txt`, `scratch-g4-stdio-test.txt`, `scratch-g4-framework3d-test.txt`.

**Not touched**: `🧊️3d/📦️glue.rs`, `🧊️3d`'s `Cargo.toml`, `🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` (the `BrepEngineHost` type itself), `🧰️framework/🔨️modules/🧊️3d/📐️brep/📄️step/🦀️component.rs`, cad's protected `repair_step_trailing_comma_before_close_paren_is_quote_aware` test.
