# 📓️ block-puzzle — native test debt, the block3d pane defect, puzzle5d framing, puzzle describe cost

Topic `block-puzzle` (fleet v4). Crates: `semio-s-artifact-block-{2d,3d,5d}`, `semio-s-plugin-block`,
`semio-s-artifact-puzzle-{2d,3d,5d}`, `semio-s-plugin-puzzle`.
Scratch + logs: `🗑️generated/block-puzzle/` (`run1.txt` … `run7.txt`, `STATUS.md`, probes, canvas crops).
Command used for every run (one cargo, all crates batched, private `CARGO_TARGET_DIR`, shared build dir):

```
CARGO_TARGET_DIR=$T/🗑️generated/block-puzzle/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools \
cargo test -p <8 crates> --lib --tests --no-fail-fast --features <crate>/component-app-assembly,… -- --test-threads=4
```

## 1. Per crate: first run → latest measured run

| crate | first run (`run1.txt`, 15:52) | latest measured (`run5.txt`, 21:42) |
|---|---|---|
| semio-s-artifact-block-2d | 261 pass / **1 fail** | **262 / 0 — GREEN** |
| semio-s-artifact-block-3d | 333 / **19** | 350 / **2** (both fixed after run5, see §2.4; verifying in `run7.txt`) |
| semio-s-artifact-block-5d | **366 / 0 — GREEN** | **366 / 0 — GREEN** |
| semio-s-plugin-block | 7 / **1** (`descriptor_is_fresh`) | 7 / **1** — stale descriptor, needs a wasm `describe` (mutex-owned, not mine to run) |
| semio-s-artifact-puzzle-2d | 845 / **34** | 851 / **28** (4 more root causes fixed after run5) |
| semio-s-artifact-puzzle-3d | 723 / **21** | 734 / **10** (1 more fixed after run5; 5 of the 10 are timing laws) |
| semio-s-artifact-puzzle-5d | **SIGKILLed** (10-min watchdog; hang) | **SIGKILLed** again (30-min watchdog) — root cause found and fixed after run5, see §2.6 |
| semio-s-plugin-puzzle | **7 / 0 — GREEN** | **7 / 0 — GREEN** |

`run6.txt` died on a shared-build-dir race (clang: "no such file or directory" for ~20 rlibs another cargo
pruned mid-link — the repo cache was pruned around 22:15, disk 36 GB → 70 GB free), not on anything in this
tree; `run7.txt` is the full cold re-run of all eight crates with every fix below.

## 2. Root causes fixed (grouped)

### 2.1 App-level actions are no longer cloned onto every window kind (framework change — 14 test sites)
`semio_framework::window_kind_actions(app, window)` now resolves "the window's own roster + every app-level
action no window claims" at READ time; `AppDefinition::actions` holds the app roster (peer ticket
26/09/18 slice DS1: the clone made a package descriptor grow as `apps × window kinds × actions`, 21 rows stored
675 times, 31.9 % of every descriptor's bytes). Every law that read `WindowKindDefinition::actions` directly
saw only the framework-injected interaction verbs, so `patchNodeKind`, `patchObjectKind`, `addObjectKind`,
`setCamera`, `worldRelocate`, `exportFixture`, `setActiveTool`, `setActiveUtility`, the file menu … all "vanished".
Updated to the current contract (and only that): block2d ×1, block3d ×1, puzzle2d ×3 (+1 helper),
puzzle3d ×16 (+2 helpers — `dispatchable_actions` for "what can this window dispatch", `action_declarations`
for "exactly one manifest declaration", which is now app-roster ∪ window rosters).

### 2.2 block3d editor test harness (buckets 2, 3, 12) — 15 of its 19 failures
The harness dispatched but never settled, never closed and bound a literal instance id:
- no self-closing fixture → 9 × "artifact store reached Drop without its exact terminal-empty shallow-shell witness";
- no `settle_registered_typed_operation` after `dispatch_typed` → every count/identity assertion read the BOOT
  document back (`AddVortex` "did nothing", `setActiveExample` "did nothing", undo "did not revert");
- `window_measures` read with an empty `ViewModel` → 0 measures (they are collected per LIVE window instance);
- `undo`/`redo` went through `handle_action` without `settle_framework_reserved_admission` (admission only).
Ported block5d's recipe verbatim (`Block3dAppFixture` + Deref/DerefMut/Drop close ladder,
`bind_instance_id(meta("local").instance_id)`, settling `dispatch`, `settle_history_verb`).

### 2.3 block3d `drive_preview_operation` spun to its deadline (bucket 6)
The hand-rolled turn loop drained result pages, effects, events and UI scopes but never
`take_typed_operation_completion` — the terminal witness sits in its OWN outbox, which
`has_pending_typed_operations` counts, so every operation spun 30 s and reported "did not finish". Replaced by
the framework's own `settle_registered_typed_operation`, with the lane counts read off its receipt.

### 2.4 block3d viewer: two real production gaps
- **No document-store disposer.** `Block3dViewer` never overrode `build_document_store_owners`, so every close
  ladder of a mounted `ViewerApp<Block3dViewer>` failed with `interactive-job.close-owned-disposer-missing`
  and the store reached `Drop` without its witness. It now installs the same
  `♻️retirement::document_store_owners()` the editor installs (same document, same ownership).
- **Registryless harness.** The viewer laws used `artifact_app_laws::new_app` (no registry, no bound instance)
  → `interactive-job.missing-factory`. They now run on `VcsArtifactApp<ViewerApp<Block3dViewer>>` under the
  app's own manifest with a closing fixture; the "sole command never mutates" law now asserts what the live
  registry really does (a viewer declares no command, so the typed channel REFUSES it — and the document comes
  through the refusal untouched).

### 2.5 block3d `rename-vortex-kind` assertion contradicted its own committed fixture
`vortex_kinds_of_parts` reassembles a vortex kind from the BLOCK-OWNED overflow row, so a rename writes
`name` there — exactly what the committed after-snapshot says. The extra assertion demanded a byte-identical
overflow row (with a value that was never in the before-fixture either). It now pins the NARROWNESS instead:
only `name` moves, every other field of that row and every sibling row are byte-identical.

### 2.6 puzzle5d: every app construction parsed 3 MB of DSL and built a 4×4 diff matrix
`Puzzle5dPlayApp::initial_snapshot()` pre-warmed `NAKAGIN_EXAMPLE_DOCUMENT` (168 355 B of DSL),
`CAPSULE_DREAM_EXAMPLE_DOCUMENT` (3 035 200 B of DSL → ~3.5 MB JSON → 2 880 typed parts) and
`PUZZLE5D_EXAMPLE_OPERATIONS` — the matrix of PAIRWISE semantic diffs between ALL FOUR example documents
(sixteen diffs, several between a 2 880-part and a 180-part document) — before returning the boot document.
Every mount paid all of it: 22 puzzle5d tests logged "has been running for over 60 seconds" and the whole test
binary was SIGKILLed by the watchdog twice. Removed; nothing is lost (each is a `LazyLock`, and
`puzzle5d_operations_from_document_change` already computes the one pair it needs on a miss).
`Puzzle3dPlayApp::initial_snapshot()` had the same shape (`NAKAGIN_EXAMPLE_FIXTURE`, 128 755 B) — also removed.

### 2.7 puzzle2d harness: `resolve_ready` and a one-page-per-turn settle
- Every app call went through `semio_framework::io::resolve_ready`, whose contract is "an artifact-IO body must
  complete without a real suspension" — it PANICS on the first `Pending`, which is what the four clipboard laws
  hit. That contract belongs to the synchronous `IoEntry`/compose thunks, not to a harness driving a live app;
  replaced with a harness-local `block_on` (noop-waker spin, the same one block3d's window-transient law uses).
- The settle loop took ONE result page and ONE completion per turn. `has_pending_typed_operations` does not
  count a presented page waiting for its ACK, so the turn that leaves the terminal completion queued can be the
  turn the loop exits on — and that completion carries the HISTORY PATCH, which is what `committed_edits`
  counts. That is the "addNode must commit exactly one document edit: 0 vs 1" family. Now drains both in inner
  loops, exactly as the framework's own `settle_registered_typed_operation` does and for the reason its comment
  states.

### 2.8 puzzle2d `engagementSubmit` was declared a View action (production defect)
Its own publication contract declares the Artifact lane and its handler emits document operations, so kind
discipline refused every submitted engagement line at dispatch ("View-kind command 'engagementSubmit' must not
emit operations") — the command line could not move, connect or place anything. Now `ActionKind::Mutation`,
exactly as `📐️cad` and `🏭️process` declare the identical verb.

## 3. The `block3d` pane showed no geometry (visual audit class 1) — fixed

**Diagnosis (live, read-only):** the pane boots ready with the curated
`hexagonal-cut-concrete-forest-left` example selected and the GLB IS fetched — probe
`🗑️generated/block-puzzle/probe-scene.mjs` recorded `GET /mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb
→ 200 model/gltf-binary` (86 112 B). So neither the mesh catalogue, the transport url
(`meshAssetTransportUrl`) nor the loader is at fault.
The defect is FRAMING: block3d's world window published **no `fit` lane at all**, so the only framing was the
document's authored `camera3d` (`position=@4,-4,3 target=@0,0,0 zoom=1`, authored around the 0.36 m rim ring of
vortices), while the representation GLB is an ~11 m × 4.7 m slab whose own origin is at its corner (measured
from the file: x 0…10.8, y 2.735…3.0, z −4.68…0, and the R3F host rotates glTF y-up into the scene's z-up).
The slab therefore sits ~60° off the view axis — entirely outside the frustum — which is exactly the crop in
`🗑️generated/audit-visual/canvas-crops/block3d-0.png`: camera gizmo, eleven faint vortex markers, no geometry.

**Fix (production code):** `block3d_world_fit_revision(&Block3dSnapshot)` + `BLOCK3D_FIT_PADDING` at the crate
root, and `scene.fit_json = world3d_fit_json(revision, padding, None)` in BOTH the editor's and the viewer's
world window — the same one-shot `WorldAutoFit` lane puzzle3d publishes. The revision is a document IDENTITY
(object-kind id + each representation's id and mesh url), so a document swap refits and a vortex drag does not.
Bounds stay unpublished (`None`): this crate knows mesh URLs, never mesh extents — the host measures what it
actually loaded (`Box3.setFromObject`), which also covers the vortex markers.

**Tests (all three green in `run4.txt`/`run5.txt`):**
`editor::…::world::component::tests::world_scene_stages_a_one_shot_fit_for_the_booted_example`,
`…::fit_revision_tracks_document_identity_not_vortex_edits`,
`viewer::…::world::tests::render_stages_a_one_shot_fit_for_the_booted_example`.

**Browser verification is NOT done yet** and is the one open item on this defect: the fix is Rust guest code,
so the `block2d` lane must be re-activated before `:6033` can show it. Requested per the brief:
`touch "$T/🗑️generated/activate.request/block2d"`. After the coordinator's activation, re-run
`node $T/🗑️generated/block-puzzle/probe-scene.mjs http://127.0.0.1:6033 <outdir> block3d` and compare
`block3d-canvas-0.png` with the pre-fix crop.

## 4. `puzzle5d` pane — NOT a camera-framing problem

Live probe (same script, `puzzle5d-probe.json`, `puzzle5d-shell.png`): the pane boots ready, the navbar picker
shows "Capsule Dream", the 3D viewport is EMPTY (grid + gizmo only) and the board shows a small off-centre ring
of ~12 parts. **Not one part mesh is requested** — no `/mesh/🧊️base.glb`, no `/mesh/🧊️bridge.glb`; the only GLB
on the wire is the block3d forest mesh. A scene that publishes no meshes cannot be a framing problem: the
renderer has nothing to frame. Two facts point at the same place: the curated `capsule-dream` document has
2 880 parts (the board shows ~12), and this crate's own
`set_active_example_switches_the_document_and_never_faults_on_capacity` FAILS natively — i.e. the example
switch does not land the document it names, which is why the pane's 3D lane is empty while the picker says
"Capsule Dream".
New native law added so this can never be invisible again:
`editor::…::windows::🧊️3d::…::every_world_instance_names_a_mesh_this_scene_publishes` — every instance's
`meshId` must be a row of the SAME scene's `meshesJson`, and a document whose parts carry `/mesh/…` urls must
publish at least one real GLB row. Its verdict lands with `run7.txt` (in puzzle5d's first non-killed run).

## 5. `@semio-tech/puzzle-plugin:describe` burning 3.05e9 fuel / 1 800 s

**PZ1 owns this and its fix is already on disk — I added nothing to it.** Verified in the tree (not
re-derived): `puzzle5d_part_kind_options()` (`🖐️5d/…/✏️editor/🦀️.rs`) now unions `concrete-forest` +
`nakagin` ONLY, carries PZ1's two-reason docstring (capsule-dream's own `kindCatalogs.parts` is 0 rows so the
fallback inferred 2 880 UUID "kinds"; and it was the remaining describe cliff), and every puzzle example
document source is `ExampleSource::deferred(...)` — `into_example_definition` leaves a deferred body's
`artifact_json` empty, so example BODIES no longer travel or get built on the describe path. The re-describe
that would confirm it is PZ1's (mutex-owned); the 17:24 log I was pointed at predates their fix landing.

What I can add from this side, measured on the tree rather than re-derived:

- **Residual describe-path fixture decodes.** Two call sites still decode whole example documents inside
  `create_*_app()`, i.e. inside bundle assembly: `puzzle3d_object_kind_options()` forces the puzzle3d
  `concrete-forest` + `nakagin` fixtures (128 755 B of DSL), and `puzzle5d_part_kind_options()` forces the
  puzzle5d pair (168 355 B). Together ~300 KB of DSL parsed in the owned interpreter — two orders below the
  3 MB door PZ1 closed, so probably affordable, but they are the ONLY remaining describe-path document work in
  this package and therefore the next thing to measure if the re-describe still does not finish.
- **If it needs closing**, the shape that keeps the descriptor identical: ship a small committed `(id, label)`
  kind-row table beside the examples, generated by a maintainer-`#[ignore]`d printer test (the pattern this
  crate already uses for `export_capsule_dream_document_json_fixture`), `include_str!` it in both
  `*_kind_options()`, and add a normal test that recomputes the rows from the example documents and asserts
  equality — the parse then happens in a native test, never on the describe path. A bounded-count law
  (`options.len() <= *_KIND_OPTIONS_MAX`, already a constant in both files) pins the descriptor-time size.
- **Not the describe path, but found while tracing it** (§2.6): `Puzzle5dPlayApp::initial_snapshot` forced the
  3 MB capsule-dream document AND the 4×4 example-diff matrix on every app construction, and
  `Puzzle3dPlayApp::initial_snapshot` forced the 3d nakagin fixture. That is bundle-INSTANCE cost (mount), not
  descriptor cost — describe builds definitions and never mounts an app — but it is the same "example fixtures
  decoded eagerly" family and it is what made puzzle5d's native suite un-runnable. Removed.

## 6. Left failing, with the exact reason and next step

- **`semio-s-plugin-block::descriptor_is_fresh`** — `🛂️.descriptor.semio` is stale (the LocalizedLabel sweep
  rewrote block's labels; a peer regenerated block's `🔣️.json`/descriptor at ~20:20 but the test still failed
  at 18:41). Next step: a `describe` for `@semio-tech/block-plugin` through the peer's wasm mutex, which this
  topic is not allowed to run.
- **puzzle3d timing laws (5)** — `brush_suggestions_run_step…` (128 419 µs vs 2 000), `penetration_of_flush_…`
  (30.5 ms vs 8 ms), `every_maintenance_unit…` (13 908 µs), `fill_run_job_step_and_overlay_append…` (2.08 ms vs
  2 ms), `one_mutation_publishes_in_a_bounded…`. Measured with load average ~50 and 8–14 peer rustc running;
  the 2.08 ms/13.9 ms ones are load jitter, but 128 419 µs against a 2 000 µs ceiling is 64× over and is a real
  defect. Next step: re-measure on a quiet machine, then profile `brush_suggestions` step granularity.
- **puzzle2d (28 in run5)** — four more root causes landed after that run (§2.7, §2.8); the residue is the
  fill/brush engine family (`IconPaintCache … terminal-empty`, `field cursor job terminated before checkpoint`,
  `zero-count fill published a nonterminal outcome`, `Puzzle2d fill placement must reach exact terminal-empty
  before Drop`, `exact fill session callback exceeded its clock authority`) plus `cohort_hostile_static_law…`,
  `hover_id_reaches_the_board_scene…` (index out of bounds) and `context_menu_grouped_disclosure…`. These are
  per-engine ownership/close-ladder laws, each needing its own owner walk; none share a cause with the buckets
  fixed here. Next step: take them one engine at a time (brush fill session owners first — three of them share
  the fill session).
- **block3d `set_active_example_loads_capsule_fixture` / `export_media_catalog_out_…`** — fixed by pinning the
  CURRENT contract, and the contract itself is a gap worth a ticket: block3d/block5d have no mutation that can
  change an object kind's `id`, so loading another example carries the kind's name and catalogue but leaves the
  document's identity at the one it booted with — and that stale id is what `puzzle3d_catalog_fragment` exports
  over the `catalog:out` seam.

## 7. Files changed (absolute)

```
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/🧪️tests/🧪️renames-door-to-portal/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🧪️tests/🔬️unit/🦀️.rs
```

No framework file was modified. No test was deleted, ignored or weakened; no fixture was hand-edited.

---

# 📅️ 2026-09-22 (session 6 — successor after the coordinator restart)

## 8.1 What the predecessor left (verified, not assumed)

The handover said the predecessor "died mid-edit while adding the missing document-store owners to
the block3d viewer". **It is not half-applied.** Inspected with `git diff`/`git status` on
`✏️s/🔌️plugins/{🧱️block,🧩️puzzle}` before touching anything:

- `👁️viewer/🦀️.rs` lines 57–65 carry `fn build_document_store_owners() -> … Some(♻️retirement::document_store_owners())`
  with its docstring; committed in `ef2210a418` (real committer date 2026-09-21 21:41).
- puzzle5d's and puzzle3d's `initial_snapshot()` pre-warm removals (§2.6) are in the tree, with the
  `🐛️` docstrings.
- puzzle2d's `engagementSubmit` is declared `ActionKind::Mutation` (§2.8) with its `// refused every
  submitted line at dispatch …` comment.
- The ONLY uncommitted changes under those two plugins were a PEER's: ticket
  `26/07/13/PUZZLE-3D-SUGGESTION-COLLISION-FILL-PARITY` is rewriting puzzle3d
  `⏳️precompute/{📐️geometry,🪣️fill}`, `🎭️modes/✏️edit/🛠️tools/🪣️fill` (+ a new unit-test file and the
  `🎞️fill-run.json` fixture). Left untouched.

So there was nothing to complete; the session went straight to new work.

## 8.2 The machine, and why there are no new test numbers

Six attempts to run the batched 8-crate suite this session:

| run | outcome |
|---|---|
| `run8.txt` | 01:31 → 02:12. Compiled 68 units, then **stopped dead for 30 min**. |
| `run9.txt` | 02:13 → 02:23, `exit=143` (SIGTERM from outside this topic). |
| `run10.txt` | 02:24 → 03:35. **6 compile units in 71 minutes.** Killed by me to free its locks. |
| `check1.txt` | 03:11 → 03:13, **`exit=0`** — see §8.3. |
| `run11.txt` | 03:14 → 03:37, reached `semio-framework-job` then stalled. Killed on the new mutex rule. |
| `run12.txt` | 03:38, requeued through `📜️native-test-mutex.sh`; **8th in the queue** behind raster, cad-content, design, knowledge-children, media, stdio-b, knowledge. Never got the lock inside this session. |

**Machine note (02:12, for the coordinator).** The shared build dir was in a textbook N-way
`fine-grain-locking` flock cycle: **14 cargo processes, ZERO rustc**, load average down to 6, each
cargo holding 100–800 `⚡️cache/cargo/build/debug/build/*/.lock` files and waiting on another's.
Four of the fourteen were `ppid=1` orphans (10640, 73803, 74100, 93150) left by dead agent turns —
those never finish and re-form the ring; they are not mine to kill. I killed **only my own** cargo
(74756) and its wrapper; within 45 s six rustc came up fleet-wide, which confirms that releasing one
participant's 439 unit locks breaks the cycle. It re-formed twice afterwards. The `📜️native-test-mutex.sh`
rule the coordinator introduced at ~03:25 is the right answer to exactly this; it just means a topic
this far down the queue gets no numbers in its own session.

`⚡️cache/cargo/build` is **341 GB** (debug alone 223 GB) against **64 GB** free, so a fully private
`CARGO_BUILD_BUILD_DIR` (the only way to escape the shared-dir lock) is not affordable — checked
before rejecting it.

## 8.3 What IS verified this session

**Compile/type-check of everything changed here:** `check1.txt`,
`cargo check -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-5d --all-targets --features …component-app-assembly`,
**`exit=0` in 1 m 43 s**, with 3 + 27 warnings emitted (proof the units really were type-checked, not
cache-skipped). No warning points at any line this session added; all of them are pre-existing
`unused_qualifications` noise in framework and puzzle3d files.

**Live pane content on `:6033`** (coordinator's activation of 03:04, headless Playwright via
`🗑️generated/block-puzzle/probe-scene.mjs`, one page per variant, `--use-angle=metal`):

- **`block3d` — FIXED, confirmed visually.** `block3d-canvas-0.png` now shows the hexagonal-cut
  concrete-forest slab with its six columns, centred in the viewport, over the ground grid; the GLB
  is fetched once (200, `model/gltf-binary`). The audit's "canvas shows only the camera gizmo" is
  gone. This closes `📓️audit-visual.md` symptom class 1 for `block3d`; the fix was the predecessor's
  `block3d_world_fit_revision` + `fit` lane, and this is its first end-to-end confirmation.
- **`puzzle5d` — framing fixed, CONTENT defect remains and is now precisely characterised.**
  `puzzle5d-canvas-0.png` / `puzzle5d-shell.png`: the 3D window is no longer "gizmo + grid only" —
  it frames and draws real geometry, which is what the new `fit` lane (§8.4) does. But what it draws
  is **the concrete-forest slab**, and the only mesh the page fetches is
  `/mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb` — a string that appears **zero**
  times in `📚️examples/🌙️capsule-dream/…/🗣️.dsl.semio` and only in puzzle5d's *concrete-forest*
  example. Meanwhile the picker reads **"Capsule Dream"**, whose 2 880 parts declare **34 distinct**
  `/mesh/🧊️*.glb` urls (12 × `🧊️base.glb`, 204 × `🧊️capsule-with-balcony_slash.glb`, …), not one of
  which is requested. So the pane is showing a DIFFERENT document from the one its picker names.
  Next step for whoever takes this: `set_active_example` to `capsule-dream` is the known
  capacity-bounded switch (`PUZZLE_COMMAND_WORK_ITEMS`, 2 880 parts + 2 865 fasteners — the crate's
  own `set_active_example_switches_the_document_and_never_faults_on_capacity` law is red), so the
  likely shape is: the switch faults or half-applies, the config records the new example id and the
  document keeps the old one. Verify by dispatching the switch natively and diffing
  `projection_of(&app)` against `capsule_dream_example_document()`.

## 8.4 Production fix landed this session

**puzzle5d's world window published no `fit` lane** — the third instance of the same family
(puzzle3d closed it on 26/09/02, block3d on 26/09/21, puzzle5d now).
`🖐️5d/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/🦀️.rs` gains `PUZZLE5D_FIT_PADDING = 1.25` and
`world_fit_revision(&Puzzle5dDocument)` — hashing schema · domain · label · `kindCatalogs` ·
`kindCompatibility`, i.e. document IDENTITY, never part poses, so an example swap refits and a part
drag does not yank the camera — and `render` now sets
`scene.fit_json = Some(world3d_fit_json(revision, PUZZLE5D_FIT_PADDING, None))`. Bounds stay
unpublished exactly as puzzle3d and block3d leave them: a part is a mesh URL here, so the crate
knows what it asked for and only the host that loaded it can measure it.

Two new laws in `🪟️windows/🧊️3d/🧪️tests/🔬️unit/🦀️.rs`:
`the_world_scene_stages_a_one_shot_fit_for_its_document` (enabled, carries this document's revision,
carries no `boundsMin`) and `the_fit_revision_tracks_document_identity_not_part_edits` (moving a part
keeps the revision; a new label or a new kind catalog changes it).

## 8.5 Test-harness fix landed this session

**puzzle2d's two icon-painting board-host laws released an admitting `BoardHost` raw** (stale-test
bucket 2). `board_host_fixture_drop_preview_json_paints_while_select_utility_active` and
`board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod` both failed inside the
framework with *"IconPaintCache with admitted resources must reach terminal-empty through close_step
before release"* (`♾️infinite/🎲️board/🔌️ports/➡️directed/🦀️.rs:972`). The guard permits a drop only
when the cache `never_admitted` anything — which is why the dozens of sibling `BoardHost` laws pass
and only these two, the ones that set an `iconKind`, do not. The production owner
(`📺️renderer/…/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`) closes a host through
`BoardHostRetirement::close_step`; the test harness had no way to. Added `close_board_host(BoardHost)`
to the one shared board-scene harness (`⚙️engine/🎲️board-host/🧪️tests/🔬️unit/🦀️.rs`, `mod context`),
which pumps that ladder to terminal-empty, and called it from those two laws.

## 8.6 Diagnosis done but NOT fixed (for the next session, with the evidence)

- **`puzzle2d::precompute::fill::…::fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict`
  is a STALE test, not an engine defect.** `Puzzle2dFillRunJob::decide` deliberately spends no fuel
  since 2026-09-17 (its docstring: this run's unit IS a placement, and charging a unit per decided
  candidate made `Step` burn its single paused-grant unit on a `host-collision` rejection —
  "atPause=29 after=29 waitedMs=30131"). Only `accept` calls `context.consume_fuel(1)`. The law still
  asserts "one unit of fuel ⇒ at most one *candidate verdict* per tick" and reports
  `[81, 107, 142, …]`. Update it to the current contract — at most one *placement* (`Success`) per
  tick, `placements` ticks carrying one — keeping what it really proves (single-stepping decides
  exactly what a free run decides).
- **`fill_run_job_places_only_inside_visible_target_regions` — the VECTOR no longer bites.** The
  fixture states the region as `halfSpan: 120` around the seed (`targetRegion.document` =
  nakagin-capsule-tower, `keepNodes: 1`). That seed is node `01890804-…` at radius 20 with ONE handle
  at angle −1.5708 (straight down); candidates land at `PUZZLE2D_DEFAULT_SUGGESTION_OFFSET = 80`
  beyond it and carry a 48·scale half-extent, so every candidate reaches ≈131 from the centre against
  a 120 half-span and `fill_regions_admit` rejects all of them — hence "a region-constrained run must
  still place something, or this vector proves nothing". `fill_regions_admit` and
  `fill_visible_region_bounds` are correct (puzzle3d rejects the same way, at the same point in the
  run, via `world_volumes_contain_aabb`), so this is a vector that outlived the geometry it was
  authored against, and it must be re-derived by a printer, never hand-edited.
- **`context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last` counts a row the
  framework does not.** puzzle2d emits 9 INTERACTIVE rows (7 leaves + 1 group + 1 destructive), so
  `organize_context_menu` takes the ≤9 path and appends `separator-organized-8` + the destructive row
  AFTER its budget check → 10 rows on screen, and the law counts "leaves+groups+separator". Either
  the framework's `CONTEXT_MENU_ROW_BUDGET` check must include the separator it is about to add
  (framework-wide, affects every plugin — not this topic's call), or puzzle2d must stop emitting
  copy/cut/paste as three top-level leaves and pre-group them as one `menu.group.clipboard` row,
  which is the plugin-local fix and the better menu.
- **`puzzle3d::…::file_menu_import_row_opens_the_file_picker` prints `file menu keeps Export: []`** —
  `dispatchable_actions(&definition)` yields NO action with `category == Some("file")` even though
  `exportFixture`/`openImportFixture` are declared app-level with `.category("file")` at
  `✏️editor/🦀️.rs:8479`. `semio_framework::window_kind_actions` looks correct by inspection, so the
  category is being dropped between `ActionDefinition::bounded_catalog(..).category("file")` and the
  built `AppDefinition` — worth one `dbg!` on `definition.actions` before suspecting the test. NOTE
  this one is potentially a **user-visible production regression** (an empty File menu), not test
  debt.

## 8.7 State of the eight crates

No test numbers were produced this session — see §8.2. The last measured numbers remain `run5.txt`
(2026-09-21 21:42), already tabulated in §1, and every fix listed in §2 plus §8.4/§8.5 is still
**unverified by a run**. `run12.txt` is queued through the fleet's native-test mutex with the exact
batched command; whoever picks this up should re-launch it (a queued ticket does not outlive the
agent that made it) and read the numbers from there.
