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
