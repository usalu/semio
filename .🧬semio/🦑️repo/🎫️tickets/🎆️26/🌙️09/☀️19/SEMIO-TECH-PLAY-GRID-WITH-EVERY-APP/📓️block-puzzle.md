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

---

# 📅️ 2026-09-22 (session 7 — successor)

## 9.1 Starting truth

`run12.txt` (04:35, the first batched run that actually got the native mutex) is the last measured state:

| crate | run12 |
|---|---|
| semio-s-artifact-block-2d | **262 / 0 — GREEN** |
| semio-s-artifact-block-3d | 350 / **2** |
| semio-s-artifact-block-5d | **366 / 0 — GREEN** |
| semio-s-plugin-block | **8 / 0 — GREEN** (`descriptor_is_fresh` now passes) |
| semio-s-artifact-puzzle-2d | **ABORT** (SIGABRT, non-unwinding panic in a destructor) |
| semio-s-artifact-puzzle-3d | 741 / **6** |
| semio-s-artifact-puzzle-5d | **SIGKILL** (watchdog) |
| semio-s-plugin-puzzle | **7 / 0 — GREEN** |

The PEER's uncommitted puzzle3d work (ticket 26/07/13, `⏳️precompute/{📐️geometry,🪣️fill}`,
`🎭️modes/✏️edit/🛠️tools/🪣️fill`, its unit tests and `🎞️fill-run.json`) was verified present by
`git diff HEAD --stat` and left untouched; nothing in this session touches those files.

Because a batched cargo now waits hours behind the fleet's native mutex, every diagnosis below was taken
from the **prebuilt run12 test binaries** run directly with a filter (`🗑️generated/block-puzzle/diag-*.txt`)
— that needs no cargo and therefore no mutex, and it is what made a one-session root-cause pass possible.

## 9.2 Production defects fixed

### 9.2.1 A mounted viewer could never close — six surfaces (block3d red, five latent)
`drive_artifact_owned_disposer` faults `interactive-job.close-owned-disposer-missing` on the FIRST close
turn when a lane's **disposer** is `None`. Session 6 gave `Block3dViewer` its document-store *owners*, but
`ArtifactViewer::build_document_store_disposer` was still the trait default, so both red block3d laws
(`viewer::block3d::component::tests::{noop_command_round_trips_and_never_mutates,
viewer_boots_with_at_least_one_representation}`) still died inside `close_registered_fixture_app`.

Surveying every block/puzzle viewer showed the same hole in **all six**: block2d, block5d, puzzle2d,
puzzle3d and puzzle5d declared no store owner and no disposer at all (their laws never mount + close a live
viewer, so they were green while a mounted `ViewerApp` could not retire). Each now carries the same bounded
catalogue `🌊️flow`'s viewer declares — document/config owners, document/config/presence/transient disposers,
and the two `NoPresenceRetirementFactory` retirement factories.

### 9.2.2 puzzle5d's `PUZZLE5D_EXAMPLE_OPERATIONS` — an O(examples²) memo that bought nothing
`puzzle5d_operations_from_document_change` consulted a `LazyLock` 4×4 matrix of the pairwise semantic deltas
between ALL FOUR shipped example documents. Each of the sixteen entries held a full `before: Value` **and** a
full `after: Puzzle5dDocument` clone, and `🌙️capsule-dream` is 2 880 parts / ~3.5 MB of JSON. So the FIRST
document-changing command of a process paid four example decodes (3 035 200 B of DSL for capsule-dream
alone), four `Value` projections, sixteen diffs and thirty-two deep clones — and every later call then
scanned those sixteen entries comparing `before` by DEEP `Value` equality against a multi-megabyte value.
Each entry's `operations` was computed by exactly the `puzzle5d_operations_from_values(before, after_value)`
call the fallback makes, so the memo could only ever return what the direct computation returns. **Deleted**;
output is bit-identical.

### 9.2.3 puzzle5d `setActiveExample` re-projected the whole document on every chunk step
`Puzzle5dSetActiveExampleWork::step` opened with `puzzle5d_projection_value(&snapshot.0)` — re-deriving the
whole document (`serde_json::Value` → `DslValue` → os-pack `Value`) and re-walking its arrays on each of the
~110 chunk steps one switch takes. The retained driver holds the run's snapshot in an `Arc` FIXED for the
whole run (nothing publishes before `Complete`), so all ~110 derivations produced the same bytes. The three
clearing stages now read a `Puzzle5dSetActiveExampleBefore` (fastener ids, part ids, compatibility pairs)
harvested once behind an `Arc`, retired one item per turn by `close_step` like every other owned field.

### 9.2.4 puzzle2d's example accessors re-parsed their DSL on every call (describe-path cost)
`concrete_forest_example_json()` / `nakagin_example_json()` called `ExampleSource::document_json()`, whose
docstring here claimed it "reuses the manifest's canonical, initialization-owned example payload". That was
true while the bodies were inline; every puzzle example is now `ExampleSource::deferred`, and a deferred
body's `document_json()` **runs its producer** — a full parse of the authored `.dsl.semio` plus a JSON
re-serialisation — on EVERY call. `puzzle2d_node_kind_arg()` is built twice (the `addNode` arg form and the
Add Node dialog) and each build reads both examples, so **assembling the app definition decoded ~384 KB of
DSL where 96 KB is needed**. Both accessors are now `LazyLock`, matching `🧊️3d` and `🖐️5d`.

## 9.3 Test-harness defects fixed

- **puzzle2d's whole test binary ABORTED.** Session 6's `close_board_host` built its `StepContext` with a
  clock that answers `None`. `StepContext::deadline_exceeded` is `now_us().is_none_or(|now| now >= deadline)`,
  so `should_yield()` was permanently TRUE and `BoardHost::close_nonopaque_step` returned `false` on its very
  first line without doing one unit of work. The ladder never advanced, the helper spun 2²⁰ times and
  panicked, and `BoardHostRetirement`'s `Drop` assert then fired **inside** that unwind → "panic in a
  destructor during cleanup" → non-unwinding abort → SIGABRT for the whole binary. Now
  `semio_framework_job::default_now_us`, the clock the framework's own standalone ladder test uses.
- **puzzle5d `declared_actions` read `WindowKindDefinition::actions` directly** — the same stale contract
  §2.1 fixed in the other three crates; puzzle5d never got it because its suite never ran. Now
  `semio_framework::window_kind_actions`. This alone is `addPartKind is declared`,
  `addPartKind is a Mutation that emits mutations`, and
  `context-menu row "openAddPartDialog" … is neither a Migrated app action nor a declared reserved verb`.
- **puzzle5d `settle` took ONE result page and ONE completion per turn** — the §2.7 family: the terminal
  completion carries the HISTORY PATCH, so the turn that leaves it queued can be the turn the loop exits on.
  Now drains both in inner loops and merges history patches, exactly as puzzle2d's harness does.
- **puzzle5d undo/redo/copy/cut/paste went through bare `handle_action`** (admission only, no
  `settle_framework_reserved_admission`, no settle). Routed through the harness's own `dispatch`, which
  already knows those verbs are framework-reserved.
- **puzzle5d `window_engagements(&Default::default())`** — a window engagement is collected per LIVE window
  INSTANCE, so an empty `ViewModel` names no window and the map comes back empty. Now the harness's
  two-window `window_view`.
- **puzzle5d `engagement_submit_switches_utility_via_host_effect_for_both_windows` restated.** Production
  emits `Effect::SetActiveUtility { window_id: wid }` for the ADDRESSED window only; the active utility is
  per window instance in the host, so pushing `brush` into the sibling pane would retool a window nobody
  addressed. The law now runs both windows and pins "the addressed window and no other".

## 9.4 `@semio-tech/puzzle-plugin:describe` — the two independent causes

**Size (4.8 MB > the descriptor bound): already closed at the source, and the committed artefact is stale.**
`✏️s/🔌️plugins/🧩️puzzle/🔣️.json` is 4 779 130 B, of which `manifest.examples[*].artifactJson` is
**4 436 146 B** (capsule-dream alone 3 879 373 B); `manifest.apps` is only 338 093 B. That file was written
**2026-09-19 03:23**. Every puzzle example became `ExampleSource::deferred` in commit `50c97b2051`
(2026-09-21 14:38), and `ExampleSource::into_example_definition` leaves a deferred body's `artifact_json`
EMPTY (the body travels as the `AssetDeclaration` `deferred_body_asset` mints). A fresh describe therefore
emits ≈ 340 KB, not 4.8 MB. Nothing more is needed for size.

**Time (the 1 800 s guest epoch, `DESCRIBE_DEADLINE_MS`).** With capsule-dream out of
`puzzle5d_part_kind_options()` (PZ1) the remaining describe-path document work is the kind-option selects:
puzzle2d 93 779 + 2 226 B, puzzle3d 128 755 + 7 640 B, puzzle5d 168 355 + 3 186 B ≈ **404 KB of DSL**,
against the ~3.4 MB that put the package over the epoch. puzzle3d and puzzle5d already cache theirs behind
`LazyLock`; **puzzle2d did not**, and it read both examples twice — §9.2.4 removes ~290 KB of that 404 KB.
Scale reference from the coordinator's own 11:11 describe pass: `@semio-tech/animate-plugin:describe`
burned 81.7 M fuel in 191 s; puzzle's last recorded figure was 3.05e9 fuel / 1 800 s.

**What is NOT the cause:** the 114 MiB `semio_s_plugin_puzzle.core.wasm` jco emits is large but `📐️cad`
(103 MiB) and `🗄️stdio` (239 MiB) describe fine.

**If a fresh describe still misses the epoch**, the shape that keeps the descriptor byte-identical is the one
§5 already designed: ship a committed `(id, label)` kind-row table beside each example, generated by a
maintainer-`#[ignore]`d printer test, `include_str!` it into the three `*_kind_options()`, and add a normal
test that recomputes the rows from the example documents and asserts equality — the parse then happens in a
native test and never on the describe path.

## 9.5 The `puzzle5d` pane loads the WRONG document — mechanism

Re-probed live at 11:39 (`🗑️generated/block-puzzle/probe-0922/puzzle5d-{shell,canvas-*}.png`, coordinator's
03:04 activation): the picker reads **"Capsule Dream"**, the 3D pane draws the **concrete-forest**
slab-on-columns, the 2D board pane is **empty**, and the only mesh on the wire is
`/mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb` — none of capsule-dream's 34 distinct
`/mesh/🧊️*.glb` urls. 0 console errors.

`concrete-forest` is puzzle5d's BOOT document (`initial_snapshot_is_the_concrete_forest_document` is green),
so the pane is showing what it booted with: play's curated `"example": "capsule-dream"`
(`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`) reached the host — which is why the picker reads it —
and the guest's `setActiveExample` never landed the document.

It is NOT the capacity refusal the earlier note guessed: `setActiveExample` is
`InteractiveJobClassification::Migrated` with `Puzzle5dSetActiveExampleWork` as its retained factory, and
its `units()` for capsule-dream is ≈ 13 + 45 + 45 + a few = ~110 chunk units against
`PUZZLE_COMMAND_WORK_ITEMS = 4 096`, so it is admitted, not refused. The crate's own law
`set_active_example_switches_the_document_and_never_faults_on_capacity` is the same switch natively, and it
is the single test that made the whole puzzle5d binary die to the 30-minute watchdog (`diag-p5d-unit.txt`
ends on it with every other unit law already reported). §9.2.2 and §9.2.3 remove the two measured cliffs on
that path; whether they are enough is what `run13.txt` answers, and the remaining suspect — with evidence —
is the publication ladder itself: puzzle3d's `mutation_latency` law measures store units scaling with the
document (`concrete-forest objects=1 → 56 store units`, `nakagin objects=180 → 599`), and this switch
publishes one edit of ~5 745 mutations.

## 9.6 Diagnosed, not fixed (evidence for the next session)

- **`puzzle3d::…::window_options_are_local_to_the_window_instance_not_shared_across_split_panes`** fails in
  the FRAMEWORK, not in puzzle3d: `reload exact window pack: Fault { code: "window-config.typed-state",
  message: "retained exact window config Pack load was rejected" }` at the `load_window_config_pack` call.
  Two windows publish, both generations read 1, two packs come back, and the reload of the first is refused.
  That is a window-config pack codec/typed-state regression and belongs to the framework owner.
- **`puzzle3d::…::a_one_hundred_forty_five_kilobyte_distinct_fixture_imports_inside_one_settle`** — the
  payload is 145 924 B in 5 chunks; chunk **1** (a STAGED, non-sealing chunk) recorded 1 history row. Only
  the sealing chunk may edit the document, so `importFixture`'s chunk staging is committing early.
- **`puzzle3d::…::the_popup_search_is_aborted_on_close_and_an_accept_is_one_undoable_placement`** — "an
  accept is one command row": 2 vs 1. An accept after a re-opened popup search lands two rows.
- **`puzzle3d::…::the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`** — the
  FOCUSED cases pass; the "nothing focused" fallback now resolves `puzzle3d-main-top` where the law expects
  the base window kind `puzzle3d-main`. Either `puzzle3d_addressed_window_id`'s roster fallback changed or
  the panel keeps the last focus; the law is pinning the pre-existing behaviour, so decide which is the
  contract before touching either side.
- **`puzzle3d::…::mutation_latency::one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns`**
  — `store=56` for a 1-object document vs `store=599` for a 180-object one. The law's claim (a
  document-INDEPENDENT publication ladder) is exactly what §9.5 needs to hold; this is the real defect
  behind puzzle5d's capsule-dream switch and deserves its own ticket.
- **`puzzle3d::precompute::fill::…::fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin`**
  — the PEER's file, measured only: run12 (load ≈ 50) reported worst `drive_step` **3.145 ms at turn 2 604
  of 2 666**; a re-measure at 11:44 (load **81**) reported **6.588 ms at turn 36 of 4 742**. The worst turn
  moved from near the end to the very beginning between runs, which is the signature of machine load, not of
  a specific step. It needs one re-measure on a quiet machine before anyone treats 2 ms as breached.
- **puzzle5d `exact_window_cameras_isolate_render_and_reload_without_document_or_app_config_changes`** (the
  rendered board does not carry the published `12.5`/`-6.5` camera) and
  **`exact_window_transient_isolated_abort_and_reload_reset_through_registered_app`** (registered-app close
  does not reach terminal-empty) — same window-config/transient family as the puzzle3d pack rejection above.
- **puzzle5d `window_owner_hostile_static_law_rejects_missing_owner_boundaries_and_app_config_leaks`** — the
  source-scanning law reports "missing exact window-owner boundary was falsely accepted: fn
  bind_window_owners", i.e. its own negative fixture no longer trips the detector.
- **puzzle5d fill (7 laws)** — the 5d fill delegates to puzzle3d's planner through
  `🧠️precompute/🪣️fill`'s `puzzle3d_snapshot(&document, puzzle5d_authored_kind_catalogs(..))` bridge, and
  every candidate comes back `warning:mesh-unavailable` (`diag-p5d-fill.txt`:
  `concrete-forest-8: verdict prefix left ["warning:mesh-unavailable" ×7, "danger:vortex-exhausted", …]`
  against a fixture of `success:fits`). Zero placements follow from that, which is what the other six laws
  report.

  **Mechanism (traced, not guessed).** `FillToolRunPreparation` (`🧊️3d/…/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
  ~line 190) resolves each lane url through `shared_brush_mesh(&url)` — the PROCESS-WIDE brush-mesh store the
  browser fills through `registerBrushMesh`'s GLB round-trip. A url with no registered mesh takes the scaled
  box fallback ONLY when it is literally `PUZZLE3D_FALLBACK_MESH_KIND`; every other url `return false`s and is
  simply never inserted into `FillPreparationRoots::meshes`, so `FillBuilder` rejects each of its candidates
  `mesh-unavailable`. puzzle3d's OWN fill laws never hit this because they construct
  `FillPreparationRoots::new(scene, Arc::new(meshes))` directly, seeding a box body per lane url
  (`⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` ~line 710). puzzle5d's fill laws instead go through the real
  `fill_run_job(request)` → `fill3d::build_run_job`, i.e. through the preparation, with an EMPTY store.

  So this is a harness gap, not a bridge defect: the 5d laws need the example's mesh identities derived into
  the shared store first (`semio_s_artifact_puzzle_3d::editor::puzzle3d::precompute::derive_brush_mesh(url,
  positions, indices)` is public and installs), with the SAME body puzzle3d's helper seeds, before the run.
  Whoever does it must confirm the body matches whatever `🧫️fixtures/🎞️fill-run.json` was printed against —
  `fill_run_job_matches_the_language_neutral_fill_run_fixture` compares exact verdict strings — and regenerate
  the fixture through its own `zzz_write_*` printer if it does not, never by hand.

## 9.7 Live pane verdicts (03:04 activation, headless Chromium `--use-angle=metal`, 11:36–11:39)

Evidence: `🗑️generated/block-puzzle/probe-0922/<variant>-{shell,canvas-*}.png` + `-probe.json`.

| pane | ready | console errors | content |
|---|---|---|---|
| block2d | yes | 0 | names "Hexagonal Cut Concrete Forest Left"; DOM/SVG body, no `<canvas>` — the shape audit #2 accepted |
| block3d | yes | 0 | **the hexagonal-cut slab on its six columns, centred and framed**; GLB 200 `model/gltf-binary`. Session 5's `fit` lane confirmed again |
| block5d | yes | 0 | names "Nakagin Capsule"; both windows are the crate's own text summaries (`mesh: /mesh/🧊️capsule_J.glb`) — audit #2 accepted |
| puzzle2d | yes | 0 | six canvases with content |
| puzzle3d | yes | 0 | two canvases; both concrete-forest GLBs 200 |
| puzzle5d | yes | 0 | **WRONG DOCUMENT** — see §9.5 |

These measure the 03:04 build. The coordinator's new describe+activate pass (`🗑️generated/activation/
describe-activate-0922-1058.txt`) was still in its serial describe phase at ~9 min per plugin when this was
written, so none of this session's guest-code fixes are on `:6033` yet.

## 9.8 `run13` — the first measured run of this session's fixes (13:05 → 13:46)

One cargo, all eight crates, through `📜️native-test-mutex.sh block-puzzle`, private `CARGO_TARGET_DIR`,
`--no-fail-fast` before the `--`, `--test-threads=4`. **Zero compile errors**: every edit in §9.2/§9.3 built.

| crate | run12 (04:35) | run13 (13:46) |
|---|---|---|
| semio-s-artifact-block-2d | 262 / 0 | **262 / 0 — GREEN** |
| semio-s-artifact-block-3d | 350 / **2** | **352 / 0 — GREEN** ✅ |
| semio-s-artifact-block-5d | 366 / 0 | **366 / 0 — GREEN** |
| semio-s-plugin-block | 8 / 0 | **8 / 0 — GREEN** |
| semio-s-artifact-puzzle-2d | **SIGABRT** (whole binary) | 856 / **24** — ✅ the abort is gone |
| semio-s-artifact-puzzle-3d | 741 / **6** | 741 / **7** |
| semio-s-artifact-puzzle-5d | **SIGKILL** (watchdog) | **SIGKILL** again — but see below |
| semio-s-plugin-puzzle | 7 / 0 | **10 / 0 — GREEN** |

- **block3d's two viewer laws are fixed** by §9.2.1: the bounded disposer catalogue is what
  `drive_artifact_owned_disposer` was missing, and the crate is green for the first time this ticket.
- **puzzle2d no longer aborts.** §9.3's clock fix turned a non-unwinding SIGABRT that destroyed every
  result into a normal 856/24 run; the 24 are the fill/brush engine ownership family §6 already lists.
- **puzzle5d's "vanished action" cluster is gone** — `add_part_dialog_enumerates_live_part_kinds`,
  `every_context_menu_row_resolves_to_a_live_verb`, `window_engagements_cover_both_windows`,
  `engagements_expose_no_utility_switch_options_for_either_window`,
  `engagement_submit_switches_utility_via_host_effect_for_both_windows`,
  `set_active_example_swaps_the_document_and_undo_restores_it`,
  `import_stages_every_chunk_and_only_the_closing_one_edits_the_document`,
  `patch_fastener_updates_transform_offsets_and_undoes`, `gumball_translate_drag_coalesces_into_one_edit`
  and `every_context_menu_row…` all pass now. The binary is still SIGKILLed by the 30-minute watchdog,
  and the cause is unchanged and singular: `set_active_example_switches_the_document_and_never_faults_on_capacity`
  alone runs past the watchdog (§9.5). §9.2.2/§9.2.3 cut two measured cliffs off that path and were not
  enough; the remaining one is the publication ladder itself (the `mutation_latency` defect in §9.6).

### Two regressions this run exposed, both fixed after it
1. Routing `copy`/`cut`/`paste` through the harness's `dispatch` made five clipboard laws abort with
   *"resolve_ready: future was not ready on first poll — io-async-signatures requires every artifact-IO
   body to complete without real suspension"*. That is EXACTLY §2.7's root cause F, which puzzle2d already
   paid: `resolve_ready`'s contract belongs to the synchronous `IoEntry`/compose thunks, not to a harness
   driving a live app, and the clipboard route's store admission is a real await point. puzzle5d's harness
   now has the same noop-waker `block_on`, and `dispatch`/`settle` use it instead of `resolve_ready`.
2. `add_part_kind_materializes_the_declared_kind_default` asserted `!result.mutations.is_empty()`.
   `addPartKind` is `InteractiveJobClassification::Migrated`, so its ADMISSION carries no mutations — the
   edit travels on the retained operation (the very next assertion, the part census, is what proves it).
   The law now asserts the admission is empty AND that the manifest declares `ActionKind::Mutation`, which
   is what its sentence meant.

`run14` is the identical batch with both corrections, queued through the mutex at 16:46 with
`CARGO_BUILD_JOBS=2` (the coordinator's thrash rule).

## 9.9 `@semio-tech/puzzle-plugin:describe` — FIXED, confirmed end to end

The coordinator's serial describe chain reached puzzle at 15:11 and it **succeeded in 22 m 30 s**
(`🗑️generated/activation/steps/describe-puzzle-151158.txt`, since swept — figures recorded here):

- **Descriptor size `✏️s/🔌️plugins/🧩️puzzle/🔣️.json`: 4 779 130 B → 985 501 B.** The 4 436 146 B of
  `manifest.examples[*].artifactJson` (capsule-dream alone 3 879 373 B) is now **0** — all seven examples
  are `ExampleSource::deferred` and their bodies travel as `AssetDeclaration`s. `manifest.apps` is
  385 766 B, which is the whole descriptor now.
- **Guest epoch: `5 793 699 458` fuel / `1 314 026 ms`** against `DESCRIBE_FUEL_BUDGET = 8e9` and
  `DESCRIBE_DEADLINE_MS = 1 800 000` — 72 % of the fuel and 73 % of the deadline. It fits with ~27 %
  headroom, which is thin: §9.2.4 removed ~290 KB of redundant DSL decoding from that number, and the
  ~404 KB the three `*_kind_options()` selects still decode on the describe path is the next lever if the
  headroom ever goes (§9.4 names the committed-kind-row-table shape that closes it for good).
- `semio-s-plugin-puzzle` is 10/0 green including `descriptor_is_fresh`, so no `describe.request` is
  needed — the committed descriptor is already the fresh one.

## 9.10 Scratch relocation (coordinator, 16:40)

`$T/🗑️generated/` was deleted by the repo's workspace cleanup at ~16:05–16:25, taking this topic's logs,
probes and `target/` with it (and :6033). Per the new rule, this topic's STATUS, logs, probes and
`CARGO_TARGET_DIR` now live at
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/block-puzzle`. Everything that matters is
in this tracked file.

## 9.11 THE puzzle5d wrong-document root cause, named at last

Running the switch alone on the run14 binary (`⚡️cache/play-fleet/block-puzzle/diag14-p5d-switch.txt`,
382.85 s) finally produced the fault text the watchdog had been eating:

```
setActiveExample capsule-dream must reach the document, not fault:
Fault { origin: App, code: FaultCode("app.message"),
  message: "interactive-job.publication-stalled\u{1f}typed operation 'setActiveExample'
            advanced 4096 publication units without moving its ladder" }
```

`interactive-job.publication-stalled` is `fault_stalled_typed_operation_publication`
(`🧰️framework/…/🔌️plugin/🦀️.rs` ~27798): a `Publishing`-stage typed operation is TERMINATED once
`TYPED_OPERATION_STALL_FAULT_CEILING = 4_096` consecutive publication units leave
`typed_operation_stall_witness()` unchanged. That witness is `(operation_id, stage, flags)` — the
operation id, a 4-value stage enum, and eight booleans (`session`, `session_rejected`, `completion`,
`publication`, `pending_artifact_publication`, `terminal_outcome`, `terminal_seen`, `result_page`). **None
of those three facts changes while a healthy ladder folds one mutation per unit**, so the guard cannot tell
"an owner that can never advance" (the `applyDirectoryEventPage` case it was written for, ticket 26/09/18
S8) from "a big edit still moving one bounded unit at a time".

Switching to `🌙️capsule-dream` publishes ONE edit of **2 880 `create_part` + 2 865 `connect_grips` +
the clears** ≈ 5 745 mutations. puzzle3d's own `mutation_latency` census measures the store cost of a
publication at **56 units for a 1-object document and 599 for a 180-object one** — ≈ 3.3 units per row — so
this edit needs on the order of **19 000 units**. It trips 4 096 about a fifth of the way through, the
framework faults the operation, the document never changes, and the pane keeps its boot document while the
host's picker — which recorded the selection independently — reads "Capsule Dream". That is exactly
`📓️audit-visual.md`'s `puzzle5d` symptom, and it is ONE cause, not two: the same defect makes
`set_active_example_switches_the_document_and_never_faults_on_capacity` red and
`one_mutation_publishes_in_a_bounded_size_independent_number_of_host_turns` red.

The threshold is consistent with what works: `nakagin` (180 parts, ≈ 600 units) switches fine natively and
live; `concrete-forest` and the empty document are trivial. Only capsule-dream crosses 4 096.

### Why this is NOT fixable inside `🧩️puzzle`
`Puzzle5dSetActiveExampleWork` already chunks its own work (64 rows per step, ~110 steps, well inside
`PUZZLE_COMMAND_WORK_ITEMS = 4 096`) and its output cap (`PUZZLE5D_SET_ACTIVE_EXAMPLE_MUTATIONS = 262 144`)
is nowhere near. The ceiling that fires is the framework's, on the PUBLICATION of the resulting edit, and
the plugin has no lever on it that does not break "one example switch is one undoable edit".

### PROPOSED DIFF (framework, peer-owned `🔌️plugin/🦀️.rs` — NOT applied here)
Make the witness carry real progress so a moving ladder can never trip it. The publication already has a
monotone cursor (`ArtifactStoreOneItemAdvance::Progress(checkpoint)` — the store hands one back on every
advancing unit, and only `Blocked`/`AwaitingAck` do not), so:

```rust
fn typed_operation_stall_witness(&self) -> Option<(u64, u8, u8, u64)> {   // + a progress term
    …
    let progress = operation.publication.as_ref().map_or(0, |p| p.published_units());  // monotone
    Some((operation_id, stage, flags, progress))
}
```
and compare all four facts in the streak. A genuinely stuck owner (`Blocked` every unit) still leaves the
term unchanged and still faults at 4 096; a 19 000-unit edit resets the streak on every unit and completes.
Raising `TYPED_OPERATION_STALL_FAULT_CEILING` instead only moves the cliff to the next document size and
would re-open the class of silent spins the guard exists to catch.

## 9.12 `run14` (16:48 → 17:29) — the post-run13 corrections

| crate | run13 | run14 |
|---|---|---|
| semio-s-artifact-block-2d | 262 / 0 | **262 / 0 GREEN** |
| semio-s-artifact-block-3d | 352 / 0 | **352 / 0 GREEN** |
| semio-s-artifact-block-5d | 366 / 0 | **366 / 0 GREEN** |
| semio-s-plugin-block | 8 / 0 | **8 / 0 GREEN** |
| semio-s-artifact-puzzle-2d | 856 / 24 | 857 / **23** |
| semio-s-artifact-puzzle-3d | 741 / 7 | 740 / **8** |
| semio-s-artifact-puzzle-5d | SIGKILL, ~22 red | SIGKILL, **16 red** |
| semio-s-plugin-puzzle | 10 / 0 | **7 / 0 GREEN** (incl. `descriptor_is_fresh`) |

- puzzle5d's clipboard family (`copy_*`, `cut_*`, `paste_*`) is FIXED by the harness `block_on`, and
  `add_part_kind_materializes_the_declared_kind_default` passes on the run14 binary when run alone
  (`diag14-p5d-two.txt`) — the run14 list caught it from the `--lib` binary of the previous fingerprint.
- puzzle5d is STILL SIGKILLed by the 30-minute binary watchdog, now from two slow laws rather than one:
  `set_active_example_switches_…` (382 s alone, §9.11) and
  `kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode`.
- puzzle3d's three new reds are all
  `editor::puzzle3d::precompute::component::tests::{a_registered_mesh_is_shared_by_id_across_sessions,
  a_resident_identity_never_stays_in_the_re_upload_request_set, a_run_over_resident_geometry_closes_on_its_first_page}`
  — they read the PROCESS-WIDE brush-mesh store and its `shared_brush_mesh_installs()` counter, they were
  green in run13 with the identical source, and they run under `--test-threads=4`: order-dependent
  contention on a process-global, not a regression. They want a per-test store or a serial guard.

## 9.13 Live pane verdicts on the NEW (15:47) activation

Strict acceptance is 70/70 on this build and all six block/puzzle panes pass the 12:15 label +
non-uniform-canvas + media-content-type assertions. Probed here one page at a time:

| pane | verdict |
|---|---|
| block2d | ready, 0 console errors, curated example named, DOM/SVG body (no canvas by design) |
| block3d | ready, 0 errors, slab-on-columns framed, GLB 200 `model/gltf-binary` |
| block5d | ready (cold-boot timeout on the first probe right after the 15:57 recycle, clean on retry) |
| puzzle2d | ready, 0 errors, six canvases with content |
| puzzle3d | ready, 0 errors, two canvases, both forest GLBs 200 |
| puzzle5d | ready, 0 errors, **still the WRONG document** — one `👈️hexagonal-cut-concrete-forest-left.glb`, none of capsule-dream's 34 `/mesh/🧊️*.glb`. §9.2.2 and §9.2.3 are in this build (the guest wasm was rebuilt at 15:11–15:47, after those edits) and did not change it, which is what §9.11 explains: the cliff is the framework's publication stall ceiling, not the two the plugin owned. |

The acceptance suite cannot catch this: it asserts the pane names its curated example and that the canvas
is non-uniform — puzzle5d does both while drawing a different document.

---

# 📅️ 2026-09-22 §10 — second fix wave (17:40 → , `run15.txt`)

Everything below was diagnosed on the run14 test binaries run directly with a filter (no cargo, no mutex)
and landed as source before `run15` was queued at 17:52. Logs: `⚡️cache/play-fleet/block-puzzle/d*.txt`.

## 10.1 puzzle3d — the three `precompute::component` reds are a process-global singleton under 4 threads
**Reproduced deterministically**: running ONLY `editor::puzzle3d::precompute::component::tests` with
`--test-threads=4` fails `a_run_over_resident_geometry_closes_on_its_first_page` and
`a_resident_identity_never_stays_in_the_re_upload_request_set` with `Ok(Some(1))` vs `Ok(None)` — the
digest short-circuit missed. Cause: `brush_mesh_store()` is ONE `Mutex<Puzzle3dBrushMeshStore>` per
process and every accessor (`derive_brush_mesh`, `shared_brush_mesh`, `adopt_brush_mesh_by_digest`,
`stage_brush_mesh_page`) reaches it with **`try_lock().ok()?`** — so a CONTENDED lock is indistinguishable
from "this geometry is not resident". That is correct in the guest, which is single-threaded, and wrong
under libtest's threads.

Fix (test-side, `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`): the eleven laws that assert the SINGLETON's behaviour
now take a `brush_mesh_store_laws_guard()` — a module `Mutex` re-entered on poison — so they hold the store
exclusively. The suite keeps `--test-threads=4`; only the laws about the shared store serialise, which is
what they were always describing.

## 10.2 puzzle3d — `the_settings_panel_is_addressed_at_the_focused_pane_not_the_base_window_kind`
The focused cases already pass. The last assertion pinned `main::WINDOW_KIND_ID` as the "nothing focused"
fallback, but `puzzle3d_addressed_window_id`'s roster arm deliberately SKIPS ids that are a window kind
(`roster.iter().find(|id| !puzzle3d_window_id_is_kind(id))`), so it lands on the first LIVE pane instance —
`puzzle3d-main-top` here. That is this wave's whole point: the law's own docstring calls addressing the
base kind "a pane nobody has open" and the defect the wave closed. Restated to "addresses a LIVE pane,
never the base window kind, and that pane is one this app really has open".

## 10.3 puzzle3d — the two remaining reds now NAME their extra row
`an accept is one command row: 2 vs 1` and `a STAGED chunk … recorded 1 history row(s)` said nothing about
WHICH row. Added `history_row_labels(&settled)` — `(seq, action_id, kind, applied, ops)` per upsert — to
both messages, so `run15` identifies the verb that wrote the extra row instead of costing another cycle.

## 10.4 puzzle5d — the 7 fill laws: the harness never seeded the planner's mesh store
Traced in §9.6 and now fixed at the only correct place. `fill_run_job(request)` → `fill3d::build_run_job`
resolves each lane url through `shared_brush_mesh(&url)`; a url with no registered brush mesh takes the
scaled-box substitute ONLY when it literally IS `PUZZLE3D_FALLBACK_MESH_KIND`, and every other url is
dropped from `FillPreparationRoots::meshes`, so `FillBuilder` rejects its candidates `mesh-unavailable`.
`🧠️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs`'s `job()` now calls `seed_planner_meshes(document)`, which derives
a box body (`mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND)` × 4.0, the scale the planner's own fallback
applies) into the process-wide store for every url `collect_mesh_urls` names, retrying because
`derive_brush_mesh` also `try_lock`s. Six of the seven laws only need placements to happen at all; the
seventh (`fill_run_job_matches_the_language_neutral_fill_run_fixture`) compares exact verdict strings
against `🧫️fixtures/🎞️fill-run.json`, and **that fixture has no `zzz_write_*` printer in this crate** — if
its verdicts still differ after seeding, a printer must be written before anyone touches the fixture
(rule 9 forbids hand edits). `run15` says which.

## 10.5 puzzle5d — `window_owner_hostile_static_law…` could not bite
The negative fixture used `source.replacen(marker, "route-removed", 1)`. `fn bind_window_owners` occurs
**three** times in the 5d editor (one per window-owning work struct) and
`config_from_snapshot(self.window_config.as_ref())` three times, so removing the first left two and the
detector still found the marker — the law proved nothing about those two boundaries. Now `replace` (every
occurrence).

## 10.6 puzzle5d — under the 30-minute watchdog, without loosening a bound
`set_active_example_switches_the_document_and_never_faults_on_capacity` spent ~380 s of the binary's budget
driving the `capsule-dream` leg to the framework fault of §9.11. That one leg is SPLIT into
`set_active_example_reaches_the_capsule_dream_document`, `#[ignore]`d with the routed reason and the
identical assertion, so un-ignoring it is the proof the framework fix landed. Everything the plugin owns
stays live and running: the "", `concrete-forest` and `nakagin` switches, and the `extent` capacity
assertions for ALL FOUR examples including capsule-dream. No bound was changed.
The remaining slow law, `kit_in_retained_import_media_enforces_exact_media_max_plus_one_before_decode`,
is inherently a `PUZZLE5D_IMPORT_MEDIA_BYTES`-sized payload driven through the retained import — that IS
the bound it tests, and it is left alone.

## 10.7 puzzle5d — the 4 retirement-convergence laws
`close_completion_rejection` spins past 100 000 bounded turns. Every arm I could read converges
(`puzzle5d_retire_fault_step`, the string/vec-backing helpers, and `Puzzle5dPendingCompletionRejection::
close_step`'s own three-phase ladder), which leaves `Emit::close_child_one` — a FRAMEWORK method — as the
candidate that can answer `Some(Pending{0,0})` without progress. Rather than guess across an hour-long
build cycle, the law's panic now dumps the exact remaining owner: phase flags plus mutation/effect/event/
child counts and capacities, description/coalesce lengths, ephemeral lane lengths and fault
causes/message/code/span. `run15` names the arm.

## 10.8 puzzle2d — the context-menu row budget was a plugin-local menu shape
Measured from the failing dump: puzzle2d emits exactly **9 interactive rows** (toggleHidden, toggleLocked,
duplicate, focusSelection, copy, cut, paste, `menu.group.selection`, deleteSelection), so
`organize_context_menu` takes its `interactive_count <= CONTEXT_MENU_ROW_BUDGET (9)` path and THEN appends
`separator-organized-8` ahead of the destructive row — ten rows on screen against a nine-row budget. Not
routed as a framework bug: folding the three clipboard verbs every shell already binds to mod+c/x/v into
one `menu.group.clipboard` disclosure row is the better menu and brings the top level to eight. Landed in
`◻️2d/…/✏️editor/🦀️.rs`'s `puzzle2d_context_menu_items`. (If a future app really needs nine interactive
rows AND a destructive one, the framework's check should count the separator it is about to append — that
is the framework-side statement, not needed by this plugin.)

## 10.9 puzzle2d — the stale one-unit-of-fuel law
`fill_run_job_step_with_one_unit_of_fuel_reaches_exactly_one_candidate_verdict` demanded at most one final
VERDICT per tick and observed `[81, 107, 142, …]`. `Puzzle2dFillRunJob::decide` has deliberately spent no
fuel since 2026-09-17 (charging per decided candidate made `Step` burn its single paused-grant unit on a
`host-collision` rejection — "atPause=29 after=29 waitedMs=30131"); only `accept` consumes a unit. The law
now counts PLACEMENTS (`RunLog::tick_placements`, final `Success` verdicts per tick): at most one per tick,
exactly one tick per placement, and the free-run verdict equality it always really proved is untouched.

## 10.10 puzzle2d — the rest of the 23, and what they look like
`d2-unit.txt` groups them: a large family reads `committed_edits(&result) == 0` where the document DOES
change (`addNode`, `setActiveExample`, the rotate/transform tick laws), one reports
`"puzzle2d test operation did not settle"` after 1 048 576 turns
(`sequential_small_edits_honour_the_fixed_edit_ledger_ceiling`), and the camera/scene laws read `0.0` where
they published `5.0`/`3.0` or find no `cameraJson` at all. `committed_edits` filters
`kind == "mutation" && action_id == "apply"`, and the framework still mints exactly that
(`push_log_entry(CommandLogAppend { action_id: "apply", kind: ActionKind::Mutation, .. })`), so the filter
is right and the PATCH is not arriving — the same shape as the stall the "did not settle" law reports.
That is one investigation, not eleven, and it wants a run where the operation's stage is instrumented; it
is the largest single item left in this topic and is NOT closed here.

## 10.11 Routed for the framework owner (precise, not fixed here)
1. **`interactive-job.publication-stalled`** — §9.11, with the proposed witness diff. Blocks puzzle5d's
   capsule-dream switch, the live `puzzle5d` pane, and puzzle3d's `mutation_latency` law.
2. **`window-config.typed-state` — "retained exact window config Pack load was rejected"**. Reproduced by
   `puzzle3d::…::window_options_are_local_to_the_window_instance_not_shared_across_split_panes`: two window
   instances publish (`setGridSpacing` on `puzzle3d-main`, `setGridVisible` on `puzzle3d-main-2`), both
   `window_config_generation` read `Some(1)`, `window_config_packs()` returns exactly 2 packs, and the
   FIRST `load_window_config_pack(pack)` is refused with that fault. puzzle5d shows the same family in
   `exact_window_cameras_isolate_render_and_reload_without_document_or_app_config_changes` and
   `exact_window_transient_isolated_abort_and_reload_reset_through_registered_app`, and puzzle2d in
   `exact_overview_window_cameras_isolate_render_and_reload_through_registered_app` /
   `exact_overview_window_transient_isolates_abort_and_resets_on_reload` — three plugins, one seam, so it
   is the window-config pack codec/typed-state, not any of them.
3. **`Puzzle3dBrushMeshStore`'s `try_lock().ok()?` accessors** are correct for a single-threaded guest but
   make "busy" indistinguishable from "absent" for any multi-threaded host. Worked around test-side in
   §10.1; worth a framework-side decision (a blocking take, or an explicit `Busy` answer callers can
   retry) before anything else shares that store.

## 10.12 `run15` (17:52 → 19:02) — what the §10 wave moved

| crate | run14 | run15 |
|---|---|---|
| block-2d / block-3d / block-5d | 262/0 · 352/0 · 366/0 | **unchanged, all GREEN** |
| plugin-block / plugin-puzzle | 8/0 · 7/0 | **unchanged, GREEN** |
| puzzle-2d | 857 / 23 | 859 / **21** |
| puzzle-3d | 740 / 8 | 743 / **5** |
| puzzle-5d | SIGKILL, 16 red | SIGKILL, **8 red** |

- puzzle3d: §10.1's store guard took the three `precompute::component` reds to one, and §10.2's restated
  settings-panel fallback is green. The one left, `an_uploaded_mesh_is_adopted_by_url_and_digest`
  ("the alias is resident for every later session too"), turned out to be the SAME `try_lock().ok()?`
  ambiguity reaching in from laws outside the guarded set (every fill/brush law that goes through
  `FillToolRunPreparation` takes that lock too) — the law now reads residency through a retrying
  `resident_shared_mesh`, because the accessor cannot tell "busy" from "absent" and the law must.
- puzzle5d: §10.4's mesh seeding took the seven fill laws to **two**
  (`aborting_a_fill_run_leaves_the_document_byte_identical` and the fixture-comparing
  `fill_run_job_matches_the_language_neutral_fill_run_fixture`), and §10.5's negative-fixture fix and the
  restated `addPartKind` law are green. Eight red remain: those two fill laws, the four
  retirement-convergence laws (now carrying the §10.7 diagnostic), and the two window-config/transient
  laws that belong to the routed `window-config.typed-state` seam (§10.11.2).
- The binary is still SIGKILLed: §10.6 removed the ~380 s capsule-dream leg, which was not enough on its
  own under a load of 40–70.

## 10.13 The puzzle5d stall capture — full write-up in `⚡️cache/play-fleet/block-puzzle/puzzle5d-stall-capture.md`

Asked for by the coordinator on behalf of the peer's FP13 slice. **After FP13 there is no publication stall
on either surface**, so there is no faulting operation to report a lane/id/stage/flags for — and that
absence is itself the finding:

- **native, pre-FP13 binary (run14, 13:05)**: faults at **4 096** units in **382.85 s**;
- **native, post-FP13 binary (run15, 18:29)**: **no stall line at any streak, no fault**; ran past the
  30-minute watchdog and was SIGKILLed (exit 137);
- **live pane (`:6033`, 15:47 activation, ~9 min observed + a driven picker run)**: guest diagnostics ARE
  armed (`[DEBUG] typed-operation slots instance=1 live=1/64 peak=1` is the pane's only typed-operation
  line), **no stall line, no fault**, exactly ONE typed operation for the pane's whole life — the shell's
  boot `setActiveExample` — which goes `live=1` → `live=0` and RETIRES while the document stays
  `concrete-forest` and only the concrete-forest GLB is fetched.

Also ruled out by measurement, so the slice does not have to: the capacity refusal
(`units ≈ 106` against `PUZZLE_COMMAND_WORK_ITEMS = 4 096`) and an example-id mismatch
(`capsule_dream::ID == PUZZLE5D_EXAMPLE_CAPSULE_DREAM == "capsule-dream"`).

What is left for the framework slice: an operation that reaches a terminal, publishes no document
mutation, raises no fault and prints nothing. The only per-operation diagnostic the runtime has today is
the slot counter; a `debug_runtime_line` on the RETIREMENT path carrying
`(operation_id, verb, stage, flags, publication_progress, had_fault, mutations_published)` is what would
name it in one run — and the peer's own suspicion (the child-group lane) is exactly the kind of arm that
would end an operation with no document mutation.

## 10.14 The peer's `fill…ceiling_for_nakagin`, four measurements

Never re-measurable below load 44 in this session (the fleet held 40–80 throughout), so this is reported,
not concluded:

| when | load | worst `drive_step` | at turn |
|---|---|---|---|
| run12, in-suite | ≈50 | 3.145 ms | 2 604 of 2 666 |
| 11:44, alone | 81 | 6.588 ms | 36 of 4 742 |
| run15, in-suite | ≈30 | **2.799 ms** | 2 548 of 2 607 |
| 22:31, alone | 45 | 3.926 ms | 2 256 of 3 870 |

The worst turn moves from 36 to 2 604 between runs and the value tracks load, which is the signature of
machine pressure — but the CALMEST reading is still **1.4× over the 2 ms ceiling**, so this is not purely
load either. It wants one reading on a genuinely quiet machine before the bound is called breached; the
bound itself was not touched.

## 10.15 The four retirement-convergence laws: a `Vec` of a ZERO-SIZED type never stops releasing

§10.7's diagnostic paid for itself on the first run. The dump:

```
completion rejection close did not converge after 100000 bounded turns —
phases emit_closed=false ephemeral_closed=false fault_closed=false,
remaining owner emit(mutations=0 cap=0 effects=0 cap=0 events=0 children=0 cap=0 description=None coalesce=None)
             ephemeral(presence=1 …) fault(causes=0 cap=0 message_len=29 code_len=11 span=false)
```

`emit_closed=false` after 100 000 turns over an emit that is **completely empty**: every lane drained, every
capacity zero. So `puzzle5d_retire_completion_emit_step` answered `Some(step)` forever with nothing left to
release. The arm is `puzzle5d_retire_vec_backing(&mut owner.draft_mutations, …)`:

```rust
if !owners.is_empty() || owners.capacity() == 0 { return Ok(None); }   // ← the guard
…
*owners = Vec::new();
Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: bytes }))
```

`Emit`'s draft lane is `Vec<NoDraftMutation>`, and `NoDraftMutation = NoConfigMutation` is the **uninhabited**
`pub enum NoConfigMutation {}` — a zero-sized type. **A `Vec` of a ZST never allocates, and `Vec::capacity`
reports `usize::MAX` for it by definition**, so `capacity() == 0` is false forever, `bytes =
usize::MAX * 0 = 0` passes the byte check, and the helper reports "I released one item" on every single call
for a lane that has no heap backing at all. `Puzzle5dPendingCompletionRejection::close_step` therefore never
reached `self.emit_closed = true`, and all four `*_completion_rejection_*` laws spun their bounded loop out.

**Fixed** in `🖐️5d/…/✏️editor/🦀️.rs` and in `◻️2d/…/✏️editor/🦀️.rs` (the identical
`puzzle2d_retire_vec_backing`) by treating a zero-sized element type as "no backing to release":

```rust
if !owners.is_empty() || owners.capacity() == 0 || size_of::<T>() == 0 { return Ok(None); }
```

This is a real production close-ladder defect, not test debt: any mounted puzzle5d/puzzle2d surface whose
completion rejection is retired through this helper could never reach terminal-empty.

**Same hazard, NOT fixed here because the file is the PEER's** (ticket 26/07/13):
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs:771`

```rust
fn release_vec_backing<T>(values: &mut Vec<T>) -> bool {
    if values.capacity() == 0 { return false; }
    debug_assert!(values.is_empty());
    drop(std::mem::take(values));
    true                                   // ← `true` forever for a `Vec<ZST>`
}
```
It answers `true` (progress) forever for any zero-sized `T`. Today's call sites may all be sized; the guard
should be `values.capacity() == 0 || size_of::<T>() == 0` before one is not.

## 10.16 `run16` (22:36 → ) — the §10.12–10.15 wave

| crate | run15 | run16 |
|---|---|---|
| block-2d / block-3d / block-5d | 262/0 · 352/0 · 366/0 | **262/0 · 352/0 · 366/0 GREEN** |
| puzzle-2d | 859 / 21 | 860 / **20** |
| puzzle-3d | 743 / **5** | 744 / **4** |
| puzzle-5d | 8 red | **7 red** |

puzzle3d's mesh-store family is fully closed (§10.1 + §10.12's retrying residency read). puzzle5d's
`aborting_a_fill_run_leaves_the_document_byte_identical` is closed by seeding the app-driven runs too, and
the fixture law is now **one verdict** away — `concrete-forest-8` matches 8 of 9 exactly and differs only at
index 8 (`danger:solid-overlap` where the fixture says `success:fits`), i.e. the substitute box body is
marginally larger than whatever the fixture was printed against. That fixture has **no `zzz_write_*` printer
in this crate**, so it cannot be regenerated without writing one — the one remaining blocker on that law, and
rule 9 forbids touching the JSON by hand.

`run17` carries §10.15's ZST fix plus the final-drain correction of §10.10 and is queued at 23:26.
