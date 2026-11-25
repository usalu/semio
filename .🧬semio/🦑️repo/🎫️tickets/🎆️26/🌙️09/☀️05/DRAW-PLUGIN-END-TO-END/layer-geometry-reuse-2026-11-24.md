# Layer geometry reuse in the mounted scene (design, 2026-11-24)

Read-only analysis. No source file was edited and nothing was compiled. "Proven" means read in code; "inferred" means not shown by a read or a measurement.

Paths: S = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene`; D = `S/..` (the schema crate root, `🦀️.rs` holds `DrawingSnapshot`).

## 1. Where the whole scene is rebuilt (proven)

- `S/🧵️mounted/🦀️.rs` `reconcile` builds a fresh `State` per `SceneIdentity` (instance, base, generation, canonical revision) and spawns `semio.draw.mounted-vector`. `GeometryJob::advance` first drives `DocumentVectorJob` (preparation, tracing, text, boolean resolution), then creates `ScenePaintJob::new(plan, CANVAS_FLATNESS, paint_limits())` and drives it node by node. Nothing from the previous `State.visual` is read.
- The expensive stage is `ScenePaintJob` (`S/🎨️paint/📋️prepare/🦀️.rs`), phases per node: `bounds` (one iteration per segment), `pathInput`, `path` (`PaintedPathPrepareJob`: flatten, fill, stroke outline, union tree), `pathCleanup`/`admit`. The measured 1,200 steps for one ellipse are this stage (geometry-cost doc).
- The paint stage is a pure function of the plan node. `ScenePaintJob::step` reads only: `node.visible`, `node.transform`, and for `Path`/`Glyphs` content `segments`, `fill.is_some()`, `fill_rule`, `stroke.{width,cap,join,dash}`; `Image` reads `width`, `height`, `transform`; `Group` and invisible nodes give `Empty`. It also reads `flatness` and the limits. Boolean and Trace nodes are refused there ("Unresolved algorithm in painted scene"), so by the time paint runs, boolean results, trace results, ancestor transforms (`node.transform` is the accumulated world matrix, `Frame.matrix`) and glyph outlines are already folded into the plan node. Ids, colors, opacity, blend mode and group metadata do not influence the painted geometry.

## 2. The cheapest sound key (proven for the key, inferred for the cost)

Key = the paint-relevant fields of the plan node listed above, compared by value (`PartialEq`; `PathSegment`, `FillRule`, `StrokeCap/Join` derive it, `StrokeStyle` and its `PagedList` dash derive it as well), plus bit-equal `flatness`.

- It subsumes every input the task lists: ancestors' transforms (folded into `transform`), boolean operand edits and trace source (folded into resolved `segments`), viewport-independent parameters (stroke, fill rule, flatness). It needs no digest, no pointer identity and no change to `DrawingSnapshot`.
- The snapshot does share structure (`layers: PagedList<DrawingLayerNode>`, segments `PagedList<PathSegment>`, dash `PagedList<f64>` with shared pages), but the plan copies segments into `Vec<PathSegment>` one funded step per segment, so snapshot-level pointer identity would be a second, weaker key (does not see transforms or boolean inputs). Not needed.
- Comparison cost is O(segments of the matched node), plain CPU, bounded by `max_segments` 65,536 for the whole scene. Comparing is orders of magnitude cheaper than flatten plus union (3.7 us per work unit, 13,000+ units per ellipse). NaN never compares equal, so it is recomputed (conservative). Sign of zero: `==` treats -0.0 and 0.0 as equal; the matrices are normalised in `matrix()`, segment coordinates are not; use `to_bits` equality if bit-identity is wanted.
- Candidate lookup: match by `node.id` through the previous plan (reorder keeps ids); the id is only a lookup hint, the value comparison is the proof. A miss or a mismatch means recompute.

Not reusable by this key: plan preparation (`DocumentVectorJob`, about 90 steps for the emblem) and the boolean and trace resolution inside it are recomputed for every revision. A boolean layer therefore still pays its union in the plan stage when any layer changes (its painted output is reused, its resolved segments are not). Reusing those needs the same idea one stage earlier (key = operand plan nodes) and is a separate step.

## 3. Reuse algorithm that keeps the ownership system intact

Constraint found: every allocation of the paint job goes through `RetainedWorkLedger` (`reserve_vec`, `vec_slot`, `vec_push`, `copy_bytes`) and is retired by `RetireOwned`. A clone of an old `PaintedPathRegions` (`Vec<Vec<[f64;2]>>`) would have to be re-admitted point by point (the existing `begin`/`append` path of `PaintedPathPrepareJob`, about 1 iteration per point, 1,024 per ellipse) and the clone source must be borrowable. An unfunded `.clone()` would be released later through retirement demands for bytes that were never retained, which is exactly what the funding laws forbid.

Therefore: move, do not clone, and move at publication.

1. When the producer finishes, `GeometryJob::advance` has the new plan. It borrows (shared, `try_borrow`) the instance's current visual shell (`Instance.visual`, a different `RefCell` than its own) and, for each visible `Path`/`Glyphs` node of the new plan, looks for an old node of equal key (section 2) in `old.visual.plan`.
2. `ScenePaintJob` receives for matched nodes a "reused" marker. For such a node it still runs the `nodes` phase checks (segment limit, transform validity) but skips `bounds`, `pathInput` and `path`, and pushes through the normal `admit` a placeholder `PreparedGeometry` carrying the old `geometry_bounds`, `bounds` and a `PaintedPathRegions` with empty vectors and the old `points`/`contours`, so `counts`, the point/contour limits and `PreparedScene.counts` are exactly the full-recompute values. The pair (new index, old index) is recorded in a funded vector (same pattern as `retired_paths`: `reserve_vec`, `vec_slot`, `vec_push`; one `RetireOwned` leaf type via `artifact_retire_leaf!`), one iteration per reused node.
3. At publication, in the `progress.done` branch right after `transfer(authority)`, before `drop(shell)`: `try_borrow_mut` the old shell (on failure return `Running(None)` and retry next step, as the code already does for a busy shell), verify `old.job_id` equals the recorded one, validate all index pairs, then `std::mem::swap` `new.geometry[k]` with `old.geometry[j]` for each pair. Swap is allocation-free and cannot fail after validation, so no funding event is needed; the old scene keeps the placeholders, which retire as empty vectors through the unchanged `PreparedSceneCloseJob`.
4. Why this is sound with the existing ownership rules:
   - The old picture stays intact and queryable (`with_visual`, `with_query`) until the instant the new scene is published, because nothing is taken before publication.
   - Only the current candidate can publish (`current` check at the top of `advance`), and `i.visual` only changes in that same branch, so the old visual recorded at map time is the one swapped at publication. A mismatch (should be unreachable) fails the job with a typed error instead of publishing placeholders.
   - Cancellation: a cancelled job never reaches the swap, so the old scene is untouched; `transfer(false)` retires the new partial scene whose placeholders are empty. `begin_close` cancels every candidate before it retires the visual.
   - Retirement ladders (`State::close_step`, `PreparedSceneCloseJob`, `ScenePaintRetirement`) see the same owner types; only the contents differ.
   - Progress: the work counter counts fewer units for reused nodes (that is the point); output equals a full recompute. One deviation: a full recompute charges `limits.max_work` per node and could fail with "Painted scene work limit exceeded" where a reuse would not. Acceptable, it makes the scene more permissive only in a failure case.
   - Image, Group and invisible nodes are recomputed (cheap).

## 4. Law (not written, see obstacles)

`#[test]` in `S/🎨️paint/🧪️tests` (or `S/🧵️mounted/🧪️tests`): for each fixture pair (A, B) in {add layer, edit layer, move layer, reorder, group transform, boolean operand edit, undo (B equal to an earlier A)}: scene_full(B) must equal scene_reuse(A -> B) with `PartialEq` over `PreparedScene.geometry`, `counts`, `flatness`; additionally assert that the number of paint work units of scene_reuse is below the full count when at least one node is unchanged, and equal when none is. Language-agnostic variant: the same fixtures as JSON in `S/🎨️paint/🧪️tests/🧫️fixtures` with the expected per-node reused/recomputed flags. `PreparedGeometry` is not `PartialEq` today (`PaintedPathRegions` is); a derive is needed.

## 5. Obstacles that stopped the implementation

1. Not compilable here, by instruction, and the change is not a small one: new field and phase in `ScenePaintJob` (a `RetireOwned` + `send_only` struct), a new funded vector with a new leaf type, a mutable-borrow splice in `mounted`, `PartialEq` on `PreparedGeometry`/`PreparedPaint`. The ledger helpers (`reserve_vec`, `vec_slot`, `vec_push`, `work.enter/exit/note_source`, `semantic_step_pending`, `settlement_pending`) enforce accounting laws I can read but not execute; getting `settlement_pending`/`semantic_step_pending` wrong for the new phase would hang the job or trip the work-law tests in `S/🎨️paint/📋️prepare/🧪️tests/🎟️work`. A blind edit in the shared tree would break the other agents' compile for all machines.
2. Open design choice that needs the compiler: the marker for "reused" nodes. Cheapest allocation-free marker is the funded pair vector (above); a new `PreparedPaint::Reused` variant would break every exhaustive match in render, pick, export and viewer crates (not searched).
3. The old plan must be readable while the new job runs: a `Rc<RefCell<Option<State>>>` borrow across two shells inside `GeometryJob::advance`. Local to `mounted`, but the `ScenePaintJob` constructor needs a typed, schema-first hook (`reuse` input) so the Rust/TS implementations stay aligned (`S/🎨️paint/🧬️schema/🔣️.json`, `🟦️.ts`).
4. Plan-stage boolean and trace costs remain (section 2).

Estimate: about 250 lines of Rust in four files (`paint/📋️prepare/🦀️.rs`, `paint/🦀️.rs` for the placeholder, `mounted/🦀️.rs`, schema json and the TS twin) plus the law and fixtures; one to two compile-and-fix rounds of the `semio-s-artifact-draw-drawing-scene-paint` and `semio-s-artifact-draw-drawing-scene-mounted` crates plus the paint work-law tests.

Expected gain once built: editing or adding one layer costs the plan stage (about 90 steps for the emblem) plus that layer's paint; the other layers cost one iteration each. For the measured case (ellipse 1,200 steps) a following rectangle or polygon no longer re-pays the ellipse.

## 6. The boot job that fails with `drawing.geometry.stale-factory`

Proven:
- `drawing.geometry.stale-factory` is produced in exactly one place, `geometry_admit` in `S/🧵️mounted/🦀️.rs`: at admission the shell of the job's slot must hold a `State` with the same `job_id` and identity, otherwise admission is refused. (The in-step check `state.job_id!=self.job||!identity.matches` returns the different string `drawing.geometry.factory-stale`.)
- In `unblock4.txt` (and unblock2/3/5) job 3659174697238529 (counter 1) starts at 4044 ms, job ...530 (counter 2) starts at 4724 ms and finishes `done 93` at 5412 ms, then ...529 ends `failed 1316` at 6068 ms. So the first job is superseded by the second revision: `reconcile` marks the first candidate `retiring`, sets `state.cancelled` and emits `Effect::CancelJob`. A second `reconcile` is therefore the local trigger; the first job is the boot revision.
- The code never cancels a candidate that is also the visual, and a cancelled `advance` returns `drawing.geometry.cancelled`, not stale-factory. So the stale-factory text means a (re)admission of job 529 after its shell was cleared by the maintenance ladder (`GeometryOwner::close_step` sets the shell to `None` once `State::empty()`), i.e. the host admitted or restored the job after the cleanup.

Inferred (not shown): whether the host re-admits a job per crossing or only restores it from `checkpoint()`; why 1,316 steps (about 1.5 ms each, so mostly zero-work `Running(None)` returns, probably from the busy-shell or `actual==0` early exits) happen between supersession (about 4724) and failure (6068). The spinning of a cancelled job is host cadence (cancel delivery), not guest code, and the host/framework side is out of the allowed scope.

Why no local edit: skipping the first spawn would need the guest to know the boot revision is about to be replaced, which it cannot. Deferring `reconcile`'s spawn until the revision is stable would change the first-picture latency for every document load. The local part that can safely be improved is making the superseded job end at once: in `GeometryJob::advance` the check order already ends it with `cancelled`; the host must stop stepping it.

To settle it with one run (next compile): add `[DEBUG] stale-factory shell=<none|job|identity> job=<job> state=<id>` before the `return Err(invalid("drawing.geometry.stale-factory"))` in `geometry_admit`, and `[DEBUG] cancelled job=<job>` in the `!current||state.cancelled` branch of `advance`; if the second line prints right after supersession and the first never prints, the failure text is from another source and the 1,316 steps are host cancel latency (framework owner); if the first prints, the host re-admits cancelled jobs and `GeometryOwner::close_step` must keep the shell until the host job is terminal.

## 7. Compile request

Nothing to compile now. When the implementation of section 3 is approved: crates `semio-s-artifact-draw-drawing-scene-paint` and `semio-s-artifact-draw-drawing-scene-mounted` (their tests: the paint work laws, the new reuse law, the mounted unit tests).
