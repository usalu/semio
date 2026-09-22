# 📓️ design — plugin native test debt (draw · layout · remodel · lowpoly · wfc)

Session 5 (fleet v4), 2026-09-21 14:00–17:25. Logs: `🗑️generated/design/` (`run-a1.txt`, `run-b1.txt`,
`run-c1.txt`, `run-c2.txt`, `run-c3.txt`, `probe-layout-all.txt`, `probe-layout-mut.txt`,
`probe-draw-1.txt`, `probe-draw-all.txt`, `probe-remodel-1.txt`), running note `🗑️generated/design/STATUS.md`.

## 1. Per crate — first run → final

Every number below comes from a run I made. The post-fix verification run did **not** complete: see §4.

| crate | first run (measured) | after the fixes |
|---|---|---|
| `semio-s-plugin-draw` | ok, 3 passed (`run-a1.txt`) | not re-run |
| `semio-s-plugin-draw-fsm` | ok, 26 passed (`run-a1.txt`) | not re-run |
| `semio-s-plugin-draw-fsm-macros` | ok, 9 passed (`run-a1.txt`) | not re-run |
| `semio-s-artifact-draw-drawing` | **SIGABRT** — the lib test binary aborts out of a fixture `Drop` (`run-a1.txt`, `probe-draw-1.txt`, `probe-draw-all.txt`) | not re-run |
| `semio-s-plugin-layout` | ok, 3 passed (`run-a1.txt`) | not re-run |
| `semio-s-artifact-layout-layout` | **92 failed / 260 passed** (`probe-layout-all.txt`, the built binary run directly with `--skip editor::layout::engine::export`); the cargo run itself was SIGKILLed by the then-10-min watchdog | not re-run |
| `semio-s-plugin-remodel` | ok, 3 passed (`run-a1.txt`) | not re-run |
| `semio-s-artifact-remodel-remodeling` | **5 failed / 1275 passed / 3 ignored** in 1301 s (`probe-remodel-1.txt`, long reconstruction lanes skipped); the cargo run was SIGKILLed by the 10-min watchdog | not re-run |
| `semio-s-plugin-lowpoly` | ok, 3 passed (`run-a1.txt`) | unchanged |
| `semio-s-artifact-lowpoly-lowpoly` | **ok, 299 passed** (`run-a1.txt`) | unchanged (no edits) |
| `semio-s-artifact-wfc-bitmap` | **compile error** (`run-b1.txt`) | not re-run |
| `semio-s-artifact-wfc-{2d,3d,grid2d,grid3d}`, `semio-s-plugin-wfc`, `semio-s-plugin-wfc-engine` | never got past the wfc-bitmap compile error — **no test result yet** | not re-run |

## 2. Root causes found and fixed

**A — Fixture float canonical form (layout, ~45 of the 92 failures).** The codecs print every `f64` with a
decimal; `serde_json::Value` distinguishes `Number(12)` from `Number(12.0)`, so every hand-written fixture that
spelled an f64 with an integer literal failed `committed_json_is_canonical` / `committed_diff_is_canonical` /
`produces_committed_diff`. Fixed on the fixture side (bucket 1), **schema-driven** rather than by key name:
`🗑️generated/design/🐍️canonicalize-fixture-floats.py` walks each fixture beside its committed JSON Schema
(`🧬️schema/**/🔣️.json`, generated from the Rust `ArtifactSchema` derive) and floats exactly the values the
schema types `"number"`, leaving `"integer"` fields (link width/height/dpi, column count, index) alone. It
resolves `$ref` across schema files, maps the externally tagged mutation fixtures (`{"ChangePageWidth": …}`)
onto their per-mutation payload schema by title, and upgrades a payload schema's shallow nested record types
(`Page`, `Frame`, …) to the artifact schema's full definitions. 62 layout fixture files rewritten; the rewrite
is idempotent and byte-identical in formatting for anything it does not convert.

**B — The layout demo DSL asset never parsed (layout, 10 failures).** `🖼️assets/🎬️demo/🗣️.dsl.semio` carried
`backgroundDrawing=[]`; every other line is hex, and the hand-rolled `key=<hex>` codec answers
`invalid digit found in string`. `LAYOUT_SAMPLE_TEXT` therefore failed to parse in every scene-export, binary
and text law. Fixed to `backgroundDrawing=6e756c6c` (hex of `null`, which is what `enc_json(&None)` prints).
The asset was committed broken (`git log` shows one commit, already with `[]`).

**C — The layout fixture app never bound an instance and never closed its stores (layout, 24 failures).**
`layout_app_with_registry` returned the bare `VcsArtifactApp`, so every typed dispatch was refused with
`interactive-job.live-instance` (12 tests) and every test that did not close by hand panicked with
`artifact store reached Drop without its exact terminal-empty shallow-shell witness` (12 tests). Replaced the
`LayoutApp` type alias with the guard lowpoly's (passing) fixture uses: a newtype that binds `INSTANCE = 1`,
derefs to the framework app, closes on `Drop` unless panicking, and settles the typed operation inside
`dispatch`. Call sites that hand the app to a framework generic now pass `&mut app.0`.

**D — Draw installed the wrong presence retirement factory (draw, whole test binary).** `DrawingPlayApp` was
the only plugin in the repo installing `store::retirement::SharedValueRetirementFactory` for presence; every
other presence-carrying plugin installs `bounded_transient_root_retirement_factory`. `SharedRetirement` answers
`Blocked` for as long as the returned `Arc` has another strong reference, and nothing in the app close ladder
can release that reference — so `close_registered_fixture_app` spun its full 30 s deadline and then panicked
`presence returned local owner is held during app close`, which fired a second panic out of
`FixedOperationRegistry::drop` and **aborted the whole test binary** (`signal: 6, SIGABRT`). Both draw presence
factories switched to the bounded transient one.

**E — wfc bitmap vs the peer `LocalizedLabel` sweep (wfc, compile).** `mutation.label()` now returns
`protocol::LocalizedLabel`, which has no `is_empty()`. Updated the law to resolve and check every
`Terminology × Locale` cell instead of one string — same proof, current contract. (Peer edit, never reverted.)

**F — Stale/mis-aimed layout laws (layout, 8 failures).**
- the command roster pinned a literal `21` against a 20-row `app_commands!`; now asserts against
  `LayoutCommand::TOOL_JOB_IDS`, so it cannot drift again;
- the blueprint/preview scene laws grepped the projected tree's JSON text for layer ids, but a built surface
  carries its scene as a binary pack — they now read `layers_json` off the decoded scene
  (`decode_fixture_scene_with_lanes`, new `context::scene` helper);
- the "German document tree" law rendered with `ViewModel::default()` (English) and asserted on `"Rahmen"`;
  new `context::render_localized` renders for `de-DE`;
- the inspector semantic-contract law read `component.label` / `children[].component.value` off the panel
  **tree root**; heading and rows live on the summary `TreeSection` / `TreeItem` below it (`label` +
  `description`);
- `window_engagements_cover_both_windows` asked with no `window_id`; engagements are addressed per window, so
  it now asks for each window in turn;
- the binary-mutations store law used a bare `ArtifactStore::new` (`edit history insertion requires its exact
  mutation retirement factory`); added `new_layout_store`/`OwnedLayoutStore` beside cad's `new_cad_store`
  reference and used it.

**G — Remodel route-disposition law read the wrong list (remodel, 1 failure).**
`retained_route_dispositions_are_exact_and_exhaustive` built its `declared` map from the per-window action
lists only. `try_build_definition` fans an app action into a window kind only through that window's
`action_refs` or the framework's derived interaction/tool-run verbs — remodeling's windows declare
`actions: Vec::new()`, so that map never contained a single retained route and the law failed on the
alphabetically first id (`addGcp`). It now reads `definition.actions` chained with the per-window lists.

## 3. Files changed (absolute)

Production:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (D)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (F, `new_layout_store`/`OwnedLayoutStore`)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` (B)

Tests:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (C, F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🧪️tests/🔬️unit/🦀️.rs` (F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs` (F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️semantic-contract/🦀️.rs` (F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🧪️tests/🔬️unit/🦀️.rs` (C)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/🦀️.rs` (C)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs` (F)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (G)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` (E)

Fixtures (all under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
62 files under `🧫️fixtures/🧬️mutations/**` plus `🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json` — float
canonicalisation only, by the schema-driven rewriter above.

No framework file was changed. No test was deleted, ignored, weakened or special-cased.

## 4. Left open, with the exact reason and next step

1. **The verification run never completed.** Three attempts:
   - `run-c1.txt` — starved on the shared build-dir artifact lock for 8 min (the first-run batch had already
     waited 36 min), stopped and relaunched with a private `CARGO_TARGET_DIR` per the coordinator's note;
   - `run-c2.txt` — `semio-framework-plugin` failed to compile on a peer in-flight edit
     (`crate::reactor::jobs::JobFn` / `register_job_kind` missing, `register_bounded_job_kind` arity);
   - `run-c3.txt` — same crate, the peer's next in-flight step (`store::SpaceMember::event_log_payload` called
     as a type, `ArtifactStore::{send_member_mutations, take_member_inbound}` gone).
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` is a peer no-touch path for this topic, so waiting is
   the only correct response. **Next step:** `🗑️generated/design/📜️run-design-crates.sh` is running detached
   (started 17:21); it polls that file and runs the full 17-crate batch into
   `🗑️generated/design/run-c4.txt` as soon as the peer's refactor compiles. Read that file for the final
   pass/fail.
2. **Layout `import_media_fields_in_sets_data_fields_json`** — a real production gap, not test debt:
   `fields:in` is a declared media port, but `LayoutPlayApp` implements no `build_reserved_tool_job`, so the
   framework refuses the import with `interactive-job.missing-reserved-builder` ("media port 'fields:in' is
   registered but has no concrete resumable importer"). **Next step:** implement a resumable import job for
   layout modelled on `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/✏️editor/🦀️.rs`
   (`Generation2dImportJob` + the `build_reserved_tool_job` arm that routes `ArtifactReservedToolInput::Media`).
3. **Layout `editor::layout::engine::export::tests` (3 tests)** — they were still running after 60 s when the
   10-min watchdog killed the binary, so they are unmeasured; my direct-binary probe skipped them. With the
   watchdog now at 30 min they need one measured run before anyone calls them a hang.
4. **Remodel worker-timing laws (4 tests)** — `images::{maximum_admitted_png_scanline…, production_png_decoder…}`
   and `mesh::{accepted_texture_bake…, accepted_tsdf_extraction…}` all fail as "step exceeded 8 ms" under fleet
   load (measured with load ≈ 65–100). Treat as load flakes until re-measured on a quiet machine; the first
   batch also flagged `images::accepted_worst_envelope_jpeg…` and
   `reconstruction::adversarial_feature_match_and_track…` the same way.
5. **Remodel long reconstruction lanes** (`engine::reconstruction::*`, `reconstruction_session::*`) — skipped in
   my probe to stay under the watchdog; included in the pending `run-c4.txt`.
6. **wasm32 check owed.** Two production edits (draw's presence factories, layout's `OwnedLayoutStore`) have not
   been checked for `wasm32-wasip2`. **Next step:** one batched command through the fleet mutex —
   `zsh ".../26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️wasm-build-mutex.sh" design -- cargo check -p semio-s-artifact-draw-drawing -p semio-s-artifact-layout-layout --target wasm32-wasip2`.

---

## Session 6 (successor, 2026-09-22 01:30–04:xx)

### 5. wfc panes reached `data-shell-error` — three drifted tool rosters (found, fixed, wasm unverified)

Probed the three failing panes headless against the coordinator's `:6033` (one page each, `--use-angle=metal`,
`🗑️generated/design/🧪️probe-pane.mjs`; logs `probe-wfc2d.txt`, `probe-wfc3d.txt`, `probe-grid3d.txt`). All three
reach `data-shell-error`, body reads "No plugins loaded", and the console carries a GUEST panic that aborts the
whole `semio_s_plugin_wfc` component (`wasm trap: unreachable`, `std::process::abort`):

| pane | guest panic | fault code |
|---|---|---|
| `#wfc2d` | `app-owned tool factories must preserve exact owner/controller/schema/tool authority` | `interactive-job.publication-contract` |
| `#grid3d` | same | `interactive-job.publication-contract` |
| `#wfc3d` | `tool proof catalog must exactly join migrated generated declarations to live concrete factories` | `interactive-job.catalog-incomplete` |

**Root cause (one defect, three spellings).** Each wfc editor keeps its app-owned factory roster in THREE hand-written
places that the framework then cross-checks at app registration
(`ArtifactToolFactoryRegistry::register`, `🔌️plugin/🦀️.rs:13956-13966`, and `tool_job_registration`): the
`TOOL_IDS`/`TOOL_JOB_IDS` constant, the `PUBLICATION_CONTRACTS` table and the `bounded_first_step_tool_proofs!`
`tools:` list. They had drifted apart, and the framework's refusal is a guest `panic!`, so ONE missing row kills
every pane of the component:
- **wfc2d** — `commit-fill` (the `fill` tool's commit verb, a real `Wfc2dEditorCommand::CommitFill` with a
  `Transient`-lane contract) was in `PUBLICATION_CONTRACTS` but missing from `WFC_2D_RETAINED_TOOL_IDS` and from
  the proofs roster. Added to both (wfc3d already had it in `TOOL_IDS`, which is the reference shape).
- **grid3d** — `"solve"` (a real `Grid3dEditorCommand::Solve`, `HostOnly` lane) was in `PUBLICATION_CONTRACTS` but
  missing from `GRID3D_RETAINED_TOOL_IDS` and the proofs roster. Added to both.
- **wfc3d** — `TOOL_IDS` and `PUBLICATION_CONTRACTS` already agreed (22 rows) but the proofs roster listed only 20:
  `"solve"` and `"commit-fill"` were missing, so two migrated generated commands had no owner-local bounded reducer
  proof. Added both.

**Regression law.** Each of the three editors' `✏️editor/🧪️tests/🔬️unit/🦀️.rs` gains
`the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster`: `TOOL_IDS`,
`PUBLICATION_CONTRACTS` keys and `bounded_first_step_tool_proofs()` tool ids must be the same set, and no lane list
may be empty. This is exactly the join the framework performs, asserted natively instead of only in a booted guest.

**These are Rust production edits**, so the shipped wasm is stale until the component is rebuilt: the running
`🔌️plugin-modules/🀄️wfc/semio_s_plugin_wfc_component.core.wasm` still carries the old rosters.
`🗑️generated/activate.request/wfc2d` is touched. One wfc component rebuild covers all five wfc apps
(bitmap/grid2d/wfc2d/grid3d/wfc3d), not just `wfc2d`. The `wasm32-wasip2` check is UNVERIFIED (wasm mutex; the
coordinator's activation compiles it).

Files changed:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- the three `✏️editor/🧪️tests/🔬️unit/🦀️.rs` beside them (the new law)

## Session 7 (successor, 2026-09-22 11:00–16:xx)

### 7.1 The wfc panes had a SECOND trap behind the first one

Session 6 fixed the roster drift that made `wfc2d`/`wfc3d`/`grid3d` abort with
`interactive-job.publication-contract`. That fix was necessary but not sufficient: the framework runs a
second join immediately after, and two of the three apps failed it.

`AppActionRegistry::validate_tool_job_rows` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13231`)
computes `expected = TOOL_JOB_IDS ∩ migrated_tool_ids(definition)` and requires every
`bounded_first_step_tool_proofs!` row to be inside it (`interactive-job.catalog-authority`) and the proof set to
equal it (`interactive-job.catalog-incomplete`). `migrated_tool_ids` reads the BUILT manifest — the app-level
action roster plus each window kind's own rows — so a verb can sit in `TOOL_IDS`, carry a publication-lane
contract AND carry a proof row and still abort the app, because no window and no `.action_*` call ever
DECLARED it. `EditorApp::with_registry_on_bus` `.expect()`s that result, so the refusal is a guest `panic!`
that aborts the whole `semio_s_plugin_wfc` component: every wfc pane, not just the guilty one.

Two verbs were in exactly that state after the peer's `jb1-describe-batch wfc` fill/solve work:

- **grid3d `"solve"`** — reached `GRID3D_RETAINED_TOOL_IDS`, `GRID3D_PUBLICATION_CONTRACTS` and the proofs
  macro, but `grid::definition()`'s action list never carried it. Fixed by one
  `ActionDefinition::bounded_catalog("solve", …)` row in the grid window (that file's own loop already stamps
  every row `Migrated` and attaches `action_args`).
- **wfc2d `commit-fill`** — same shape. wfc3d declares its own `commit-fill` at app level
  (`.action_with(ActionDefinition { in_palette: false, ..bounded_catalog(fill::COMMIT_FILL_ACTION_ID, …) })`);
  wfc2d was simply missed. Fixed the same way in `create_wfc2d_editor`, whose trailing
  `.interactive_jobs(Migrated)` classifies it.

I checked all six wfc apps' three rosters (`TOOL_IDS` / `PUBLICATION_CONTRACTS` / proofs) and their manifest
declarations: 2d 21, 3d 22, grid3d 22, bitmap 17, grid2d editor 28, grid2d viewer 6 — all coherent now.

**New regression laws.** `every_retained_tool_is_a_declared_migrated_action_of_the_built_manifest` in wfc2d's
and wfc3d's editor unit tests joins the built manifest through `semio_framework::window_kind_actions` — the
same join `AppActionRegistry::from_definition` performs — so this class of drift now fails in `cargo test`
instead of in a booted guest. grid3d already had the equivalent window-declaration law. The two grid2d apps had
NO roster law at all and now carry session 6's
`the_owned_factory_tool_ids_publication_contracts_and_proofs_are_one_exact_roster`, extended with the
HostOnly-lane exclusivity rule the framework enforces.

**Verified live.** On the coordinator's 15:47 activation (`activate-dev rc=0`, all 30 descriptors regenerated,
:6033 relaunched 15:57): `wfc2d`, `wfc3d` and `grid3d` all reach `data-shell-ready` with **0 console errors**
and their curated catalog example visible — "Terrain Ring", "Tower With A Cantilever", "3D Pipes" — with both
window kinds mounted. Evidence: `🗑️generated/design/probe2-{wfc2d,wfc3d,grid3d}.txt`.

### 7.2 wfc2d's own roster law could not compile

Session 6's law referenced `crate::editor::wfc2d::Wfc2dRetainedCommandJobFactory`, but the factory is private
and the module wiring is `mod component; pub use component::*;` — a glob re-export carries no private item, so
both sites were `E0425` and, because a compile error aborts the whole cargo invocation even with
`--no-fail-fast`, that one path killed the entire 17-crate run (`run-d4.txt`) before any wfc crate was reached.
Now an explicit `use crate::editor::wfc2d::component::Wfc2dRetainedCommandJobFactory;`.

### 7.3 The layout export close ladder livelocked (this is what the watchdog kept killing)

`layout-layout` was SIGKILLed by the 30-minute test-binary watchdog in every run, taking all of its results
with it. `sample` on the live binary (pid 97343) showed 814 of 873 samples inside
`BatchJobSession::close_step → … → LayoutExportJob::close_export_step → Fault::from` — the ladder was minting
one `Fault` per turn and releasing nothing.

`begin_close` parks the job on `LayoutExportCloseStage::Publication`, and `close_export_step` refuses that
stage outright by design: whichever ladder runs must drain the retained publication payload and advance the
stage itself. The `InteractiveJob` ladder did. `ArtifactReservedJob::close_step` — the reserved media-export
route that `LayoutMediaExportJobFactory` drives — delegated straight to `close_export_step` and took the
refusal on every call, so `while !terminal_is_empty() { close_step(…) }` never terminated. Three export laws
hung there forever.

Fixed in production code: a shared `LayoutExportJob::close_publication_step` that BOTH ladders call, with the
`InteractiveJob` arm now delegating to it instead of duplicating it. New bounded law
`the_reserved_close_ladder_leaves_the_publication_stage_in_bounded_slices` counts slices instead of hanging.

### 7.4 Numbers (run-e1, the first complete 17-crate result this fleet produced)

`🗑️generated/design/run-e1.txt` — one cargo invocation through `📜️native-test-mutex.sh design`, private
`CARGO_TARGET_DIR`, `--no-fail-fast` BEFORE `--`, `--test-threads=4`. Five of these crates
(`wfc-3d`, `wfc-grid2d`, `wfc-grid3d`, `plugin-wfc`, `plugin-wfc-engine`) had never had a test binary built by
any fleet run before this one.

| crate | run-e1 | note |
| --- | --- | --- |
| semio-s-artifact-draw-drawing | 281 / 1 | no SIGABRT any more; both of run-a1's reds now pass |
| semio-s-plugin-draw | 3 / 0 | |
| semio-s-plugin-draw-fsm | 26 / 0 | |
| semio-s-plugin-draw-fsm-macros | 9 / 0 | |
| semio-s-artifact-layout-layout | 334 ok / 38 FAILED, then watchdog SIGKILL | export livelock, fixed in §7.3 |
| semio-s-plugin-layout | 2 / 1 | `descriptor_is_fresh` |
| semio-s-artifact-remodel-remodeling | 353 ok / 6 FAILED, then watchdog SIGKILL | 1293 / 8 when run alone |
| semio-s-plugin-remodel | 3 / 0 | |
| semio-s-artifact-lowpoly-lowpoly | 299 / 0 | |
| semio-s-plugin-lowpoly | 2 / 1 | `descriptor_is_fresh` |
| semio-s-artifact-wfc-2d | 202 / 0 | |
| semio-s-artifact-wfc-3d | 256 / 0 (2 ignored) | |
| semio-s-artifact-wfc-bitmap | 201 / 2 | both pass alone — load flakes |
| semio-s-artifact-wfc-grid2d | 224 / 0 | |
| semio-s-artifact-wfc-grid3d | 216 / 1 | stale literal, repinned to the roster |
| semio-s-plugin-wfc | 13 / 1 lib + 10 / 1 close_ladder | `descriptor_is_fresh` + close-cost |
| semio-s-plugin-wfc-engine | 1 / 0 | |

remodel's `retained_route_dispositions_are_exact_and_exhaustive` (the `addGcp` red the previous status carried)
is GREEN — session 6's 16:13 fix holds.

### 7.5 Timing laws re-measured ALONE (brief rule 5)

- **wfc-bitmap ×2** — `the_solve_command_publishes_a_real_collapse_on_the_transient_lane` and
  `every_example_solve_terminates_with_a_verdict` both PASS alone (48.5 s, load ~58). Load flakes. The cause is
  the headless solve budget: bitmap declares `HEADLESS_STEP_BUDGET_US = 4_000` where wfc3d/grid3d declare
  `250_000`, and `SUSTAINED_OVERRUN_QUARANTINE_STEPS = 4` consecutive over-budget steps quarantine the job with
  `job-session.terminal-fault`.
- **remodel** — full suite alone at `--test-threads=8` finished in 1727 s with **1293 passed / 8 failed / 10
  ignored** (`probe-remodel-2.txt`; no watchdog kill). All 8 are 8 ms worker-step wall-clock laws. Repeated 3×
  single-threaded (`probe-timing-alone.txt`), `images::maximum_admitted_png_scanline…`,
  `images::production_png_decoder…` and the two `mesh::…` laws recover, but three fail 3/3 even alone:
  `images::accepted_worst_envelope_jpeg_and_malformed_entropy_steps_are_timed`,
  `reconstruction::adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded`,
  `sfm::maximum_seed_pair_and_degenerate_solve_steps_stay_below_hard_ceiling`. Those three are NOT load flakes.
  No bound was loosened.

### 7.6 Proposed diffs for code I must not edit

**`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (peer slice S10 no-touch)** —
`semio-s-artifact-draw-drawing`'s one red,
`drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle`,
fails with `Err(artifact-store.replacement-output-not-ready)`. `ActiveArtifactEnvelopeDecode::poll()` (:19988)
answers `Ready` for a decode job that reached `Ready` and was THEN cancelled, while
`try_begin_artifact_store_replacement` (:23226) filters `… && !active.cancel.is_cancelled_now()` and hard-faults.
The maintenance ladder flips `Ready + cancelled` to `ClosingCancelled` on its next turn, so this is a pure
ordering race between the caller's maintenance turn and its advance. Minimal fix in
`advance_artifact_envelope_load` (:23062), right after the `decode != Ready` early return:

```rust
// 🚫️ A Ready job that has been cancelled is not admissible into the store initializer
// (`try_begin_artifact_store_replacement` filters `!cancel.is_cancelled_now()`); its own maintenance
// ladder flips it to `ClosingCancelled` on the next turn, which polls `Cancelled`. Report progress
// rather than faulting the caller for asking one turn early.
if active.cancel.is_cancelled_now() {
    return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
}
```

### 7.7 Still open

- **`descriptor_is_fresh`** in `semio-s-plugin-layout`, `semio-s-plugin-lowpoly`, `semio-s-plugin-wfc`. The
  committed `🛂️.descriptor.semio` lagged the built manifest. `describe` is a wasm32 target this topic must not
  run; `🗑️generated/describe.request/{wfc,layout,lowpoly,draw,remodel}` was touched and the coordinator's 15:42
  chain regenerated all 30 descriptors — re-verified by `run-e4.txt`.
- **`plugin-wfc` `wfc_close_cost_is_independent_of_the_retained_session`** — cold 26 turns; warm 9 turns for 6
  retained units; long 45 turns for 32. Dilution 106 %, fixture floor 140 %
  (`✏️s/🔌️plugins/🀄️wfc/🧫️fixtures/🚪️close-ladder/🔣️.json`). The law's own doc comment says "a ladder that pages
  one retained item per turn scores a flat 100 %", so the wfc close ladder is paging exactly one retained unit
  per reactor turn instead of batching. Real, pre-existing, and not a wall-clock flake (it is a TURN COUNT).
  Next step: batch the wfc instance close ladder so one reactor turn retires several retained units.
- **The three deterministic remodel 8 ms laws** (§7.5). They need a genuinely cheaper worker step (jpeg entropy
  scan, pair-match microstep, 64-correspondence/32-hypothesis seed solve), not a looser ceiling.
- **`grid2d` and `bitmap` panes** — probed at load average 187 and both timed out at 180 s with ZERO console
  errors and no page error; re-probe when the machine is quieter. Every other design pane was green in
  `📓️audit-visual-2.md` and nothing I changed touches them.

### 7.8 Evidence-path notice (2026-09-22 16:05–16:34)

The repo's workspace cleanup deleted almost all of `$T/🗑️generated/` while this session was running, taking
every design log with it: `run-a1…run-e1`, `probe-remodel-1/2`, `probe-timing-alone`, `probe2-*`, the topic
`STATUS.md` and the in-flight verification cargo and its log. **Every number and root cause in §7.1–§7.7 was
read off those logs before they were swept**; they are transcribed here precisely because a tracked file is the
only durable place. The evidence paths §7.1–§7.5 name no longer exist.

Per the coordinator's 16:40 addendum the topic now writes to
`/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/design` (`STATUS.md`, run logs, probes and
`CARGO_TARGET_DIR`), which the cleanup does not touch. The 17-crate verification run carrying §7.1/§7.2/§7.3's
fixes is requeued there as `run-f2.txt`; because the previous `CARGO_TARGET_DIR` was swept too, it rebuilds the
deliverables from the shared build dir. Headless pane probing is suspended by the coordinator while the machine
is at load ~250, and :6033 was down at 16:45; the three wfc pane verdicts in §7.1 were measured at 15:58–16:05
on the 15:47 activation, before both events.

### 7.9 Correction to §7.5 — the remodel engine timing laws ARE load flakes

§7.5 reported three `editor::remodeling::engine::…` 8 ms laws as failing "3/3 even alone". That measurement was
taken at load ~50 with the fleet running, and it was wrong about the conclusion. Re-measured at 17:00–17:23 on a
calm machine (`⚡️cache/play-fleet/design/probe-timing-quiet.txt`), three IDENTICAL single-threaded repeats of the
whole `editor::remodeling::engine::` set (284 laws) gave:

| repeat | load (1 min) | result | wall |
| --- | --- | --- | --- |
| 1 | 20.4 | 279 passed / 5 failed | 718 s |
| 2 | 34.2 | 275 passed / 9 failed | 383 s |
| 3 | 38.2 | **284 passed / 0 failed** | 361 s |

The failing SET is different every repeat, and one repeat is fully green — including
`sfm::decompose_essential_recovers_relative_pose_within_tolerance`, a numerical-tolerance law that failed once
and is not a timing law at all. These are wall-clock laws in a DEBUG build on a machine this fleet keeps at
load 20–250; they carry no signal here. **No bound was loosened, and none should be**: the correct fix is to
measure them on an idle machine (or in release), not to widen the ceiling. Same verdict as wfc-bitmap's two
reds, which pass alone.

That leaves `semio-s-artifact-remodel-remodeling` at **1293 passed / 0 real failures** once the wall-clock laws
are measured fairly (its full-suite alone run was 1293 / 8, and all 8 are in that set).

### 7.10 Strict acceptance is 70/70

The coordinator reports the strict play acceptance suite at **70/70** on the 15:47 activation, with `wfc2d`,
`wfc3d` and `grid3d` booting with visible content — the three panes that were the fleet's only strict-acceptance
reds. Which change did it: the 15:47 activation is the FIRST restage that carries both halves of the trap fix —
session 6's `PUBLICATION_CONTRACTS` roster repair (`interactive-job.publication-contract`, §2 of session 6) and
session 7's two missing action declarations (`interactive-job.catalog-authority`, §7.1). The 03:04 build that
the audits measured had neither; no build ever existed with only one of them, because both landed before the
first describe of wfc in the coordinator's chain. §7.1's static check is what says both were needed: after
session 6's fix alone, grid3d's `"solve"` and wfc2d's `commit-fill` still had proof rows that no declared
`Migrated` action backed, which `validate_tool_job_rows` refuses with the same guest `panic!`.

### 7.11 All nine design panes verified green (17:40–17:50, 15:47 activation)

Re-probed one page at a time once the coordinator lifted the probe freeze; evidence in
`⚡️cache/play-fleet/design/probe4-<pane>.txt`. Every pane reaches `data-shell-ready` with **0 console errors**:

| pane | example shown |
| --- | --- |
| wfc2d | Terrain Ring |
| wfc3d | Tower With A Cantilever |
| grid3d | 3D Pipes |
| grid2d | Pipes |
| bitmap | Flowers 24 |
| remodel | Synthetic Orbit |
| draw | Demo |
| layout | (no example picker) |
| lowpoly | (no example picker) |

Each example is exactly the one `🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json` curates for that variant, and
both window kinds are mounted in each. The earlier `timeout` verdicts for grid2d, bitmap, layout and remodel
were taken at load average 187–224 and were load artefacts: zero console errors and no page error in any of them.

### 7.12 The layout crate, finally readable (runs g1 / g3, 22:00–23:00)

`layout-layout` had been SIGKILLed by the 30-minute watchdog in every fleet run since 2026-09-21, so its 375
laws had never produced a summary. §7.3 fixed one of the two livelocks; the second sat one stage later, and
§7.13's bounded harness turned it into a named failure. `run-g1.txt` then reported **336 passed / 40 failed in
1.10 s**, and `run-g3.txt` **338 / 38** — the first readable layout results this ticket has ever had. The 38
split into four clusters, three of them now fixed:

1. **25 × committed-JSON canonical form** (`committed_json_is_canonical` in 24 mutation fixtures plus
   `json_artifact_round_trip_preserves_the_language_neutral_snapshot`). The whole diff was one field:
   the committed fixtures wrote `"inset": {"x": 0, "y": 0, "w": 0, "h": 0}` while the crate's own encoder
   emits `0.0` for every member of that float rect. Session 6's schema-driven float canonicalizer had
   normalised 62 files but not `inset`, because `0` is legal JSON for a float and only the encoder settles
   it. FIXED: the `inset` members of all **49** fixture JSONs under
   `📏️layout/…/🧫️fixtures/🧬️mutations/**` rewritten to the canonical float form; a repo-wide scan then found
   zero integer `inset` members left. Nothing else in those fixtures was touched, and the assertion diffs
   confirm nothing else differed.
2. **~34 × `layout-window-view-required`** (run-g1 only; gone in run-g3). `ShellHost` never dispatches
   without an addressed window — it stamps `ActionMeta.view_state` with the focused window instance — but
   the fixture dispatched with bare `meta("local")`, so `ArtifactOwnedToolJobContext.view_state` was `None`
   and every window-lane verb (`setCamera`, the canvas pointer gestures, `focusPreflightIssue`) was refused
   by `addressed_window` before its assertion ever ran. FIXED: `context::dispatch` now sends
   `windowed_meta()` — a `ViewModel` carrying one `layout-blueprint` window instance resolved through
   `ViewModel::for_window_instance`, the same shape trinity jack's mounted tests use.
3. **8 × `assert_eq!(result.mutations.len(), 1)`** — brief-v2 stale-test bucket 3. A MOUNTED app publishes
   through its retained typed operation, so `result.mutations` is always empty (and, measured, so is
   `result.history_patch`). Restated onto the SETTLED document, which each of these laws was already
   asserting on the next line; `patch_page_supports_margins_and_columns` now reads back each margin/gutter
   it set, and `registry_backed_add_frame_emits_operation` asserts the projection grew by one frame.
4. **4 × export laws** — still open, see §7.14.

### 7.13 A close ladder that cannot progress must not hang the binary

`drive_test_job` (layout's export test oracle) closed with `while !terminal_is_empty() { let _ =
close_step(…) }` and ignored the result. A ladder answering `Blocked` — or an `Err` the `InteractiveJob`
wrapper maps to `Blocked` — therefore spun forever. That is what the 30-minute watchdog kept SIGKILLing, and
each kill destroyed all 375 layout results, not just the stuck law. The three close loops are now bounded by
`CLOSE_LADDER_SLICE_BUDGET = 100_000` and panic naming the stage, so a stuck ladder costs one red test
instead of a whole crate.

### 7.14 Still open after session 7

- **`semio-s-artifact-draw-drawing` 281/1** — the framework envelope cancel race, §7.6 proposed diff
  (peer-owned `🔌️plugin/🦀️.rs`). In run-g3 it was 280/2, the second being a load-sensitive sibling.
- **4 layout export laws.** `production_retained_wire_factory_…` and
  `terminal_candidate_is_empty_and_owned_chunks_never_exceed_four_kibibytes` now FAIL (bounded) instead of
  hanging: `close_export_step`'s `Snapshot` stage answers `Err(layout-export-close-snapshot-unwitnessed)`
  when the job holds the ONLY `Arc` to its snapshot, and neither test leaves a witness alive — the second
  one drops it on purpose. The invariant is deliberate (the ladder refuses to be the last owner, because
  dropping the final `Arc` frees the whole snapshot in one unbudgeted step), so the fix belongs on one of
  two sides, for the layout owner to choose: have the retained-wire factory install a `snapshot_close`
  lease (as the media factory's payload does), or have the ladder answer a terminal `Complete` when it is
  provably the sole owner. `checkpoint_is_lossless_bounded_and_authority_qualified` and
  `one_unit_budget_forces_multiple_yields_and_stale_context_faults` panic inside
  `🧰️framework/🔨️modules/🧵️job/🦀️.rs:713` and need the job owner's eye.
- **Two layout render laws** — `set_camera_preview_surface_updates_independently_of_blueprint` and
  `drag_over_emits_ghost_and_leave_clears` read a rendered body after dispatching. Now that the fixture
  dispatches INSIDE the blueprint window, the preview-surface law needs its own addressed window; the
  window-addressed `dispatch` is right, the two laws need the preview instance added.
- **Wall-clock laws under fleet load** — remodel's `editor::remodeling::engine::**` (0–9 red depending on
  load, fully green at load 38), wfc-bitmap's two solve laws, and `plugin-wfc-engine`'s
  `every_large_domain_unit_including_checkpoint_stays_below_watchdog` ("8 ms exceeded: 96.98 ms" under
  load, green otherwise). None were loosened.
- **`remodel-remodeling` needs a calm machine**: 1301 passed / 0 failed in 867 s on an idle box (run-f3),
  watchdog SIGKILL under fleet load (runs e1, g1) because ten reconstruction lanes each exceed 60 s.
