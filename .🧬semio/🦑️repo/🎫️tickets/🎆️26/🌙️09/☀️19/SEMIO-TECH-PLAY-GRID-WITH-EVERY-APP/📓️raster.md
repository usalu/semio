# 📓️ Raster — play-grid topic report (2026-09-21)

Topic `raster`, plugin `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster`.
Scratch + logs: `$T/🗑️generated/raster/` (`STATUS.md`, `test-1..8.txt`, `wasm-check.txt`, `browser/`,
`probe.mjs`, `probe2.mjs`, `probe3.mjs`, `canonical-floats.py`, `run-tests.sh`, `run-lib.sh`, `run-wasm.sh`).

## 1. Blank composite in the play pane — ROOT CAUSE FOUND AND FIXED

### Browser evidence (before the fix)
Headless playwright (repo `node_modules/playwright`, `PLAYWRIGHT_BROWSERS_PATH=…/⚡️cache/tools/ms-playwright`,
`--use-angle=metal`, one page at a time) against `http://127.0.0.1:6033/#raster`:

- `data-shell-ready` reached; the 2026-09-19 guest panic is GONE; Artifact panel lists `Backdrop pixel` +
  `Brighten adjustment`; example switcher on `Demo`; no page error, no refused input.
- Both WebGPU canvases (composite 1139×907, navigator 439×907) got a context and rendered **1 distinct colour,
  `rgba(0,0,0,0)`, 0 pixels with non-zero alpha** → `🗑️generated/raster/browser/raster-report.json`,
  screenshot `browser/raster-page.png`.
- `JSON.parse` hook: `Paint2dHost` parsed `assetsJson === "{}"` three times, and `atob` was NEVER called →
  zero textures uploaded (`browser/jsonparse.json`). The document and camera lanes were fine.
- Only console noise: the serve's own `[stale] … 58 staged plugin module(s)` warning and a stray
  `eprintln!("[DEBUG] typed-operation slots …")` left in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37827`
  (peer-owned file, do-not-edit this session — surfaces as console errors in the pane).

### Root cause
`assets_json` is built by `assets_json_from_document` → `crate::raster_asset(&document.assets, id)`, which
resolves a `RasterAssetChild`'s materialization and re-encodes it through the
`s.stdio.semio/v1/image → s.stdio.png` composer. **Raster never enabled `semio-s-artifact-stdio-semio`'s
`conversion-image` / `conversion-drawing` features**, so `semio_s_artifact_stdio_semio::register()` installed
NO composer entries at all (they are `#[cfg(feature = "conversion-image")]`). Consequently:

- `mint_raster_asset_child` always fell into its raw-bytes fallback → a handle with NO content;
- `raster_asset` always answered `None` → `assets_json` `{}` → the compositor received zero textures → blank canvas;
- the same gap failed ~10 io/composite/export tests with `no composer registered for s.stdio.semio/v1/image Export …`.

Two further defects on the same live lane (found by reading `RasterOneItemApply` → `RasterMutationCandidateAuthority`,
which is what `RasterStorePreparation` drives for every interactive document edit):

1. the retained `add-layer-asset` apply minted the child id from a RAW `(mime, data)` digest — disagreeing with the
   canonical content-addressed id every other route mints — and inserted the handle **without** the decoded
   `SemioImageSnapshot`;
2. `RasterSnapshotCloneAuthority` rebuilt each asset handle field by field and **dropped its materialization**, so
   every mutation after the asset was added lost the pixels again.

### Fixes (production code, absolute paths)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/📦️packages/🦀️rust/Cargo.toml`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/📦️packages/🦀️rust/Cargo.toml`
  — `semio-s-artifact-stdio-semio = { workspace = true, features = ["conversion-image", "conversion-drawing"] }`
  (same declaration `📸️remodel`, `🌍️gis`, `🗒️note`, `🖍️draw`, `📏️layout`, `🎥️shooting` already carry).
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` — new
  `adopt_raster_asset_owner(source, target)`, the single seam a field-by-field handle rebuild carries a
  materialization through.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
  — the candidate authority now mints through `crate::mint_raster_asset_child` (canonical id + owner) and hands the
  owner to the inserted handle; the snapshot clone authority adopts the source child's owner; the dead raw-hash
  `raster_asset_child_id` helper was removed; `asset_hasher`/`asset_hash` replaced by an `asset_mint` owner that is
  closed in `close_step` and covered by `terminal_is_empty`.

### Proof (native, from runs actually made)
- `standards::…::mutations::binary::unit_tests::retained_asset_apply_and_snapshot_clone_keep_the_composite_pixels`
  (NEW) — drives `RasterMutationCandidateAuthority` exactly as the document lane does: asserts the minted child id
  equals the artifact's own canonical one and that `raster_asset` resolves after the apply AND after a second
  mutation's clone. **ok** in `test-5..8.txt` (failed in `test-2/4`, i.e. it reproduces the bug).
- `editor::raster::component::unit_tests::mounted_boot_replays_the_demo_example_through_the_retained_route`
  — strengthened with the browser symptom: after the shell's boot `setActiveExample demo`, the booted document's
  `raster_scene(...).assets_json` must contain `semio-emblem` + `image/png`. **ok** from `test-5.txt` onward
  (FAILED in `test-4.txt` with the pixel-less pool).

### NOT yet verified in the browser
The pane still serves the 2026-09-20 19:37 wasm; a Rust fix needs a re-activation, which this session may not run.
`$T/🗑️generated/activate.request/raster` was touched (still pending when this report was written).
After the coordinator activates the `raster` lane and `:6033` is recycled, re-run:
`PLAYWRIGHT_BROWSERS_PATH=… node "$T/🗑️generated/raster/probe.mjs" http://127.0.0.1:6033 raster "$T/🗑️generated/raster/browser2"`
and expect `canvases[*].stats.distinctColors > 1` / `nonzeroAlpha > 0` and `atobCalls > 0`.

## 2. Whole-document JSON routes (export serializer + mutation JSON bridge)

The `RasterOwnedMap` serialization guard the brief refers to was already removed (committed 2026-09-20): both
`ToValue`/`FromValue` now carry a populated map, justified in the type's own doc comment (fixed 64-entry capacity).
What was still broken is the OTHER half: those routes materialize populated documents and let them fall off the
frame into `RasterOwnedMap`'s fail-closed `Drop`. Fixed in
`…/🧬️schema/🧬️mutations/🦀️.rs`: `bridge_decode_pair` retires the snapshot when the mutation does not decode,
`bridge_step` cold-retires the outcome's diff, `apply_raster_mutation_json` / `undo_raster_mutation_json` retire the
decoded, applied and every intermediate inverted document plus the operations themselves.

New tests (both **ok** since `test-4.txt`):
`mutations::component::tests::mutation_json_bridge_applies_and_inverts_a_populated_before_document`,
`mutations::component::tests::json_export_serializes_a_populated_document`.

## 3. Native test suites

One batched invocation per run:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools
cargo test -p semio-s-artifact-raster-raster -p semio-s-plugin-raster -p semio-framework-raster --lib --tests
--no-fail-fast -- --test-threads=4` (no crate declares `component-app-assembly`). From `test-8.txt` a private
`CARGO_TARGET_DIR` under the topic folder removed the ~45 min shared-build-dir queueing.

| crate | first run | final run |
|---|---|---|
| `semio-s-plugin-raster` (lib) | 3/3 ok (`test-1.txt`) | 3/3 ok (`test-8.txt`) |
| `semio-framework-raster` | 3/3 ok (`test-1.txt`) | 3/3 ok (`test-8.txt`) |
| `semio-s-artifact-raster-raster` (lib) | 80 FAILED, binary SIGKILLed by the 10-min watchdog (`test-1/2.txt`) | **186 ok / 43 FAILED** (`test-8.txt`) |

Cleared along the way (each verified by a run): composer gap (−10), fixture float canonical form (−13),
io/dwg fixture documents not retired (−10), add/remove-layer-asset fixture leaves not retired (−11),
snapshot/mutation round-trip projections not retired (−2).

Also fixed a genuine **hang**: `binary::unit_tests::raster_owned_map_cap_plus_one_…` granted only
`RASTER_OWNED_FIELD_BYTES` (4 KiB, the envelope decode page) to a retirement that must release 16 KiB page
backings, so it spun on `Pending { 0, 0 }` forever and ran the whole binary past the watchdog. It now grants
`RASTER_OWNED_FIELD_BYTES.max(conservative_page_credit_bytes())`, the same grant the sibling
`retirement::retire_raster_snapshot` helper already used; every assertion is unchanged.
**Open question for the framework owner:** the real close ladder (`ArtifactDocumentStoreDisposer`) is also driven
with `RASTER_OWNED_FIELD_BYTES` in `close_raster_candidate`; if the host's own grant is 4 KiB, a populated raster
document cannot close at runtime either and `RASTER_OWNED_MAP_PAGE_BACKING_BYTES` (16 KiB) is the wrong page size.
Not touched here — `🧰️framework/…/🔌️plugin/🦀️.rs` is peer-frozen this session.

### The 43 that remain (all pre-existing test debt, none on the pane's boot path)
- **15 × `editor::raster::component::unit_tests`** + 2 × `panels::document::tests` — mounted/app-level: 4 ×
  `edit history insertion requires its exact mutation retirement factory` (v2 stale-test bucket 2: bare
  `ArtifactStore::new`, needs an owners-installing guard), 3 × `artifact envelope terminal shell reached Drop
  before its app-owned bounded retirement authority detached every nested owner`, 2 × `fixed UI admission failed at
  image-window.source` (`ui.fixed-capacity`), plus tree/scene projection assertions
  (`compositeViewportJson`, `"componentKind":"paint-2d"`, window offsets), 1 × `interactive-job.missing-reserved-builder`
  for `image:in`, 1 × `Option::unwrap() on a None value`, 1 × `index out of bounds: the len is 0`.
- **14 × `mutations::binary::unit_tests`** — retirement/initializer laws: `nested Raster retirement did not reach
  terminal`, `maximum combined Raster owner did not reach terminal-empty`, `Raster candidate did not reach
  terminal-empty close`, 4 + 3 × `PoisonError` cascading from the two poisoned test locks (they clear once the
  primary panics do).
- **5 × `mutations::component::tests`** — 2 × `absorb(d1, d2).apply(base) must equal d2.apply(&d1.apply(base))`
  (absorb law), `store_applies_layer_create`, `every_variant_round_trips_via_inverse` (still one populated
  projection dropped inside `vcs::apply_mutation`'s own frame).
- **3 + 2 × `snapshot::binary` / `snapshot::text`** — the populated document is dropped inside the framework's
  `store::os_store::test_support::assert_dsl_round_trip` / `assert_dsl_pack_equivalence` helpers (framework-side,
  not raster-side; the raster locals are retired now).
- **2 × `move_layer::…::produces_committed_diff` / `committed_diff_is_canonical`** — the committed
  `🔺️diff/🔣️.json` disagrees with the produced diff beyond number form; needs a real read of that leaf's diff
  builder before the fixture is touched (NOT float-canonicalized by me: the diff fixture already carried floats).
- **2 × viewer composite/navigator `render_produces_a_scene_node_for_the_default_document`**.

## 4. Fixture correction (contract, not a test weakening)
`$T/🗑️generated/raster/canonical-floats.py` rewrote 27 committed fixture JSONs so every FLOAT-typed field carries a
decimal point (`opacity`, `newOpacity`, `newX`, `newY`, `transform.{x,y,scaleX,scaleY,rotation}`), leaving
integer-typed `width`/`height` alone. `pack::json::Number` keeps `Int`/`UInt`/`Float` apart and
`value_eq_ignoring_object_order` compares the variants, so a committed `1` for an `f32` field could never be a
fixed point of the artifact's own `ToValue`. 13 `committed_json_is_canonical` / `produces_committed_diff` tests
went green with no code or assertion change.

## 5. Other files changed (tests/fixtures, absolute paths)
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️dwg-import/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖇️add-layer-asset/🧪️tests/🖼️declines-to-4af870/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗂️remove-layer-asset/🧪️tests/🖼️rejects-removing-1c84a7/🦀️.rs`
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/{💾️binary,📝️text}/🧪️tests/🔬️unit/🦀️.rs`
- 27 fixture JSONs under `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/**`
(all under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/`).

---

# 📓️ Raster — session 2 (2026-09-21, after the coordinator's "continue on the 43")

`semio-s-artifact-raster-raster --lib`: **80 → 43 → 22 failures; 202 passing** (`🗑️generated/raster/test-14.txt`,
`--test-threads=4`; `test-13.txt` is the same tree run `--test-threads=2/1` and gives the SAME 22, so none of the
remaining failures is a parallel-execution artifact). `semio-s-plugin-raster` 3/3 and `semio-framework-raster` 3/3
stay green. A private `CARGO_TARGET_DIR` under the topic folder removed all lock queueing (~10 min per full cycle).

## Went green this session (each verified by the named run)
| bucket | what it was | fix |
|---|---|---|
| snapshot dsl/pack round trips (4) | the helper's own decoded document was dropped | `assert_dsl_round_trip_cold` / `assert_dsl_pack_equivalence_cold` (the framework's existing cold twins) with the artifact's `retire_raster_snapshot` — `test-11.txt` |
| `edit history insertion requires its exact mutation retirement factory` (4) | bare `ArtifactStore::new` in tests (v2 bucket 2) | `install_document_store_owners_exact(raster_document_store_owners())` + `close_plain_test_store` — `test-11.txt` |
| retirement/close laws (3) + the cap-plus-one hang | the loops granted `RASTER_OWNED_FIELD_BYTES` (4 KiB) to a retirement that must release a 16 KiB `RasterOwnedMap` page backing, so they spun on `Pending { 0, 0 }` | new `RASTER_CLOSE_GRANT_BYTES = max(RASTER_OWNED_FIELD_BYTES, RASTER_OWNED_MAP_PAGE_BACKING_BYTES)` used by the terminal-reaching loops only (the saturation laws deliberately keep the raw 4 KiB grant) — `test-11.txt` |
| mounted action tests (8) | v2 bucket 3: `context::dispatch` stopped at `dispatch_typed`, so a MOUNTED app queued every command and the document never changed; and the harness carried `ViewModel::default()` (no window instance) | `context::dispatch` now carries `raster_view_state()` (the composite window INSTANCE) and runs `settle_registered_typed_operation`; `main_window_measures` uses the same view state; two tests plant the layer they address (the store boots on the empty shell) — `test-11.txt` |
| stale scene assertions (3) | a `scene_surface` node publishes its payload as a PACKED document (`"kind":"paint-2d"`, `doc.bytes`), not as inline tree text, so `"componentKind":"paint-2d"` / `compositeViewportJson` / `\"zoom\":2.0` could never appear in the projected JSON | assert on `packed_scene_text(&json)` (the file's own decoder) — `test-11.txt` |
| `move-layer` diff fixture (2) | `transformX`/`transformY` are `Option<f64>`; the committed diff carried `16` / `-8` | extended `canonical-floats.py` to the patch keys and regenerated that one fixture — `test-12.txt` |
| `raster_owned_map_removal_…_populated_drop_refuses` | dropped a populated map outside its `catch_unwind` | retired — `test-12.txt` |
| `RASTER_MAXIMUM_CONTROL_BACKINGS` | the test pinned `64`, a literal from when the nesting depth was still 128; the derived value is `RASTER_RETIREMENT_STACK_PAGE_COUNT (15) + RASTER_NON_STACK_CONTROL_BACKINGS (13) = 28` | assert the derivation AND the current literal — `test-13.txt` |
| envelope terminal shells (2 of 3) | `semio_app()` / the convergence test built an envelope only to print a pack and dropped the terminal shell | new `context::retire_raster_envelope` driving `DocumentStoreOwners::retire_envelope_uninstalled` with the page-sized grant — `test-12.txt` |

## The 22 that remain, with the exact reason
**A. Genuine production defects in raster's diff algebra (3)** — these are NOT test debt and want a dedicated slice:
- `change_layer_opacity_satisfies_the_inverse_and_absorb_laws`: `absorb(d1, d2)` emits the SAME layer twice in
  `layers.patched`, so applying it fails `mutation.apply.duplicate-target` while `d2.apply(d1.apply(base))` succeeds.
  `RasterDiff`'s absorb does not merge two patches of one layer.
- `reorder_layers_satisfies_the_inverse_and_absorb_laws`: the absorbed `layers.moved` entry carries an index that is
  only valid against the pre-`d1` tree → `mutation.apply.invalid-index`.
- `every_variant_round_trips_via_inverse`: for at least one variant the computed inverse does not restore the base —
  the restored forest starts with the `adjust-1` adjustment where the base starts with the `l1` pixel (an ordering
  defect in a create/delete inverse), independent of the owned-map retirement this session added.

**B. Peer-frozen framework, proposed diffs (4)**
- `viewer …composite/navigator::render_produces_a_scene_node_for_the_default_document` (2):
  `ImageWindowKit::render` inlines the whole composite as a `data:` URL —
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32692`
  `UiText::try_format(format_args!("data:{};base64,{}", view.mime, view.base64)).ok_or_else(|| ui_assembly_error("image-window.source"))?`.
  A real composite never fits `UiText`'s fixed capacity, so every raster VIEWER window now refuses
  (`ui.fixed-capacity … image-window.source`). Before this session it passed only because no composer was registered
  and `composite_document_to_png` fell back to a 1×1 PNG — i.e. the test was green over a broken viewer.
  **Proposed diff**: publish `ImageView.base64` through the media transport lane (the same URL seam the icon/mesh
  transport uses) and keep the inline `data:` URL only when it fits, rather than failing the whole window.
- `renders_layers_tree`, `document_tree_binds_the_layers_interaction_domain` (2): after the envelope fix they reach
  `artifact_app_laws::close_registered_fixture_app` and fail with `registered fixture did not reach its exact
  terminal-empty witness` (`🔌️plugin/🦀️.rs:7417`). The close ladder grants 4 KiB, which cannot release a 16 KiB
  `RasterOwnedMap` page — the same page-credit law this session fixed on the raster side.
  **Proposed diff**: let the fixture close ladder grant `max(4096, the artifact's own page credit)`, or expose the
  grant so an artifact with fixed-capacity paged owners can raise it. The identical concern applies to the real
  runtime's `ArtifactDocumentStoreDisposer` grant: if the host also grants 4 KiB, a populated raster document cannot
  close at runtime.

**C. Composed-child persistence gap, framework-wide (1 + 1)**
- `composite_scene_syncs_document_and_assets`: the fixture goes `print_document_pack → load_document_pack`, and
  `ArtifactChild`'s materialization is serialization-skipped BY DESIGN, so a saved-and-reloaded raster document has
  handles without pixels (`assetsJson` empty). The play pane is unaffected because the demo is planted by
  `add-layer-asset` with real bytes on every boot, but **a raster document that is saved and reopened loses its
  images**. Same family as the architect/writer "derived children load empty" follow-up already in
  `📓️status.md`. Needs the pack to carry composed-child content (or a host resolver that re-materializes on load).
- `raster_import_media_appends_layer_from_incoming_image`: `interactive-job.missing-reserved-builder` — "media port
  'image:in' is registered but has no concrete resumable importer". Raster declares the port but never registers the
  reserved builder; the sibling pattern to copy still has to be located.

**D. Still raster-side, not finished (14 → the remaining)**
- `raster_arc_factory_full_saturation_…` fails its entry assertion `RASTER_STANDALONE_PROCESS_CONTROLS == 0`
  (measured 12) and, panicking while holding `RASTER_STANDALONE_RETIREMENT_TEST_LOCK`, POISONS that lock — which is
  the only reason the four `raster_populated_*` / `raster_standalone_control_max_plus_one` tests fail. Fixing that
  one entry assertion clears five. The counter is a PROCESS-wide credit pool and the file already has the right
  idiom a few tests above (`let baseline = …load(Acquire)` and assert relative to it, lines 454–475).
- `raster_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty`:
  `RetainedJobPayload requires one-page close to terminal-empty; ordinary Drop intentionally preserves page backing`
  — v2 bucket 7, the close needs `JOB_PAYLOAD_PAGE_BYTES` (16 KiB) grants.
- `add_layer_action_appends_and_undo_removes`, `patch_layer_renames_and_toggles_visibility_round_trip`,
  `two_instances_converge_disjoint_layer_edits_via_backbone`: the forward edits now land; `undo` (and the backbone
  convergence) still does not, so the history verb needs the `settle_history_verb` ladder the mounted harness uses
  (`mounted::history`) rather than a bare `handle_action("undo")`.
- `retained_route_dispositions_are_exact_and_exhaustive` ("action addLayer declared") and
  `utility_registry_declares_utilities_scoped_to_the_composite_window` (the framework no longer injects
  `SET_ACTIVE_UTILITY_ACTION_ID` into `create_raster_app()`'s raw window definition): both are contract drift
  between the app manifest and the current framework injection point; each needs that injection point read before
  the assertion is re-pinned.
- `panels::document::tests` ×2: tree-window offset/closed-container projections; the rendered tree in the failure
  text shows the rows, so these are assertion-shape drift, not a missing projection.

## wasm32
`cargo check -p semio-s-plugin-raster -p semio-s-artifact-raster-raster --target wasm32-wasip2` was queued through
the fleet mutex for ~1 h and then cancelled on the coordinator's instruction (a multi-hour peer release build holds
it). **UNVERIFIED** — the coordinator's `activate-raster-react-dev` compiles the same crates for wasm32 and will
surface any breakage. The native build of the identical production change is clean.

## Files changed this session (absolute)
Production: the two `Cargo.toml`s and the three `🦀️.rs` already listed above (unchanged since).
Tests/fixtures, all under `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/`:
`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`,
`🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`, `🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️unit/🦀️.rs`,
`🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs`,
`🧫️fixtures/🧬️mutations/↔️move-layer/📍️slides-the-stamp-b7bdca/🔺️diff/🔣️.json`.

---

# 📓️ Raster — session 7 (2026-09-22, successor of sessions 1–6)

Scratch/logs live under `$G = /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/raster` from 16:40
onward: the repo's workspace-cleanup deleted `$T/🗑️generated/raster/` (every log, probe and the private
target dir) at ~16:05–16:25. Readings quoted below were taken from runs I made before that sweep; the
numbers are restated here because this file is tracked and the logs are not.

## 1. Native suites

One batched invocation per run through `$T/📜️native-test-mutex.sh raster` with a private
`CARGO_TARGET_DIR`, `--no-fail-fast` before the `--`, `RUST_BACKTRACE=1`:
`cargo test -p semio-s-artifact-raster-raster -p semio-s-plugin-raster -p semio-framework-raster --lib --tests --no-fail-fast -- --test-threads=4`

| crate | session start (test-26, 04:37) | after my fixes (test-27 12:16 / test-28 13:31) |
|---|---|---|
| `semio-framework-raster` | 3/3 ok | **3/3 ok** |
| `semio-s-plugin-raster` | 3/3 ok | **3/3 ok** |
| `semio-s-artifact-raster-raster` (lib) | 219 passed / 5 failed | **223 passed / 2 failed** |

The brief's open item 1 ("`test-25`: `settle_framework_reserved_admission` no longer exists") was already
stale: `test-26.txt` (04:37) was a complete run of all three crates and compiled cleanly. The 12 reds of
session 4 were already down to 5 there. `test-29` is re-running the identical batch into `$G/test-29.txt`
purely to leave a durable log after the sweep.

The two remaining reds are the peer-frozen viewer windows (§3).

## 2. What I fixed (absolute paths, one line each)

Production:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` — removed the
  `[DEBUG] raster mint_raster_asset_child fell back content-less …` `eprintln!` a predecessor left in
  `mint_raster_asset_child` (guest `eprintln!` surfaces in the pane console).
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — removed the matching
  `[DEBUG] raster assets_json: …` `eprintln!` from `assets_json_from_document`.
- `…/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` — **real defect**: `replace_document_operations`
  emits a `remove-layer-asset` for EVERY asset the open document carries and then asked
  `example_media_operations(example_id, current)` whether to plant the example's own media — a guard that
  reads the pool as it was BEFORE those removals. Re-selecting the demo over a document that already
  carried the materialized emblem (any edit that makes the layer forests differ takes this path, so
  `handle`'s early return no longer applies) therefore removed the emblem and planted nothing: a
  pixel-less handle pool and a blank composite. Split out `example_media(example_id)`, which plants
  unconditionally, and used it from the replace batch; `example_media_operations` is now that plus the
  guard, which only a caller that KEEPS the pool may ask.

Tests (all under `…/🏅️standards/🔖️1/🪆️subsets/✳️any/`):
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` `composite_scene_syncs_document_and_assets` — `Paint2dScene::split_lanes`
  ALWAYS moves `documentSyncJson`/`assetsJson` out of the fixed-capacity surface doc into
  `paged_text_carrier` children, so `packed_scene_text` (which decodes only the doc's `"bytes":[…]`) could
  never contain `image/png`; the spine carries only the lane manifest. Now reads the assembled scene with
  `artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::Paint2dScene>` (trinity
  jack's idiom) and asserts `semio-emblem` + `image/png` on `scene.assets_json` — which also keeps the
  predecessor's composed-child pack fix (`read_child`/`write_child`, pack format 1→2) under test, since
  the fixture still goes `print_document_pack` → `load_document_pack`.
- same file, `raster_import_media_appends_layer_from_incoming_image` — three faults in one test:
  `ArtifactApp::snapshot()` hands back an OWNED `RasterSnapshot`, and after the import it owns a POPULATED
  asset pool, so reading `.layers.len()` off the temporary dropped a `RasterOwnedMap` that still held its
  page backing (its fail-closed `Drop`); the payload was `"aGVsbG8="` (5 bytes, no PNG signature) so the
  mint fell to its content-less fallback and the law proved nothing about pixels; and
  `assert!(!result.mutations.is_empty())` can never hold on a MOUNTED app (v2 bucket 3). Now: observations
  first then `schema::snapshot::retire_raster_snapshot`, the crate's own committed emblem PNG as the
  payload, and the import read off the DOCUMENT (the shape sequence/animate already use) including
  `crate::raster_asset(...)` resolving to real `image/png` bytes.
- `🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs` — the two saturation laws claimed a PREDICTED
  `RASTER_STANDALONE_PROCESS_CONTROL_CAPACITY - observed` number of credits; sibling laws on the other
  three test threads take credits from the same PROCESS pool between the observation and the loop, so a
  mid-loop claim came back with `control == None` (`left: None, right: Some((1, 4096))`). New
  `saturate_standalone_controls()` drives the pool to its own REFUSAL and hands back the credits held,
  the refused probe and that probe's producer-owner pointer; the refused probe holds no credit, so the
  caller closes it last. `RasterProcessCreditBaseline::standalone_headroom` is gone with its last caller,
  and the one exact `== CAPACITY - 1` pool read became the strict inequality that is actually provable
  (the law's exact part — the plus-one owner resuming into the returned control — is unchanged).
- `✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` — new law
  `replacing_a_materialized_document_plants_the_example_media_it_just_removed`.

## 3. Proposed diff (peer-frozen, NOT applied)

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:32706` (`ImageWindowKit::render`), still on the
`End-to-end repo completion` peer's no-touch list (brief v5 §"State at 11:00"):

```rust
let src = UiText::try_format(format_args!("data:{};base64,{}", view.mime, view.base64)).ok_or_else(|| ui_assembly_error("image-window.source"))?;
```

`UI_TEXT_MAX_BYTES` is 512 (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:18`), so a real composited
PNG never fits and EVERY raster viewer window refuses with `ui.fixed-capacity … image-window.source` — not
only in the two tests, but at runtime: the pane's `Viewer ⌘️⌥️V` mode cannot assemble. Before the composer
gap was fixed this passed only because `composite_document_to_png` fell back to a 1×1 PNG, i.e. the test
was green over a broken viewer. Raster cannot route around it: `👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs`
uses `ImageWindowKit` deliberately ("this artifact IS a pixel image", contract §2.6), and the paging
mechanism lives in the frozen file.

Proposed: publish the pixels through the framework's OWN out-of-doc lane mechanism instead of an inline
`data:` URL — `scene_surface` (same file, line 536) already pages arbitrarily large payloads with
`paged_text_carrier`, and `decode_fixture_scene_with_lanes` / the React Interpreter's `surfaceSceneLaneText`
already reassemble them. Concretely: give the image node a lane-key `src`
(`framework.window.image.pixels`), attach `paged_text_carrier("framework.window.image.pixels", &format!("data:{};base64,{}", view.mime, view.base64))`
beside it, and keep the inline `data:` URL only when it fits `UiText` — a refusal of the whole window is
the wrong failure mode for a payload whose size is a document property. (A transport-URL variant — the
`meshAssetTransportUrl`/icon pattern — also fits in 512 bytes and is the smaller change if a media route
for composed children already exists.)

## 4. Live pane — still blank on the NEW activation, and none of this session's fixes is the cause

Chain: `$T/🗑️generated/activation/describe-activate-0922-1211.txt` → `describe-raster ok` 14:28,
`activate-dev rc=0` 15:47; supervisor relaunched and `:6033` recycled 15:57 (`serve-…events.txt`).

Probe at 15:58 (repo playwright, `--use-angle=metal`, one page, `#raster`, wait `data-shell-ready`,
settle 15 s): shell ready, **0 page errors, 0 console errors** (only the dev staleness warning and the
peer's stray `[DEBUG] typed-operation slots` line), both webgpu canvases (1139×907 and 439×907)
**1 distinct colour `rgba(0,0,0,0)`, 0 non-zero-alpha pixels**, `atob` never called.

What that pins down, with the code read:
- `Paint2dHost` (`…/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx:252-260`) uploads one texture per
  `assetsJson` entry via `base64ToBytes`, whose ONLY decode path is `atob` (`:94-98`). Zero `atob` calls
  ⇒ `parsePaint2dAssets(scene.assetsJson)` returned `{}` ⇒ the guest published `assetsJson == "{}"`.
  A deeper probe also found no `JSON.parse` anywhere in the page carrying `image/png`/`semio-emblem`,
  and no `GPUQueue.writeTexture` call.
- The served module is the new one: `curl` of
  `/🔌️plugin-modules/🖨️raster/semio_s_plugin_raster_component.js` matches
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🖨️raster`
  (staged 12:10 UTC = 14:10 CEST, wasm 104 935 698 B), i.e. built AFTER every fix listed in §2 except
  nothing — the set-active-example fix landed 12:45 CEST, before that build.
- `conversion-image` IS compiled in: the served dev wasm carries `stdio_bmp`/`stdio_gif`/`stdio_tiff`/
  `stdio_jpg` symbols (the 06:26 release build is stripped, so its symbol count says nothing).
- So the 2026-09-21 composer-features fix, the composed-child pack fix and the set-active-example fix are
  all in the module the pane runs, the native mounted boot law
  (`mounted_boot_replays_the_demo_example_through_the_retained_route`) asserts real emblem pixels and
  passes, and the pane is still blank. **The remaining defect is live-only and no native law reproduces it.**

Next step for whoever picks this up (in this order, cheapest first):
1. The discriminator I had written but could not run (probes were stopped at 16:40; the script is re-created at
   `.🧬semio/🦑️repo/⚡️cache/play-fleet/raster/probe5.mjs`): re-select the curated example through the
   navbar switcher (`#playground.navbar.fixture`) and re-census the canvas. `set-active-example`'s replace
   batch now re-mints the emblem unconditionally, so **pixels after the re-selection ⇒ the BOOT lane never
   planted the media; still blank ⇒ minting/resolution itself fails in the guest.**
2. If (1) says minting: `raster_asset` (`🗿️artifacts/🖨️raster/🦀️.rs:693`) fails at one of exactly two
   points — `handle.local_owner::<SemioImageSnapshot>()` (a `TypeId` downcast on an
   `Arc<dyn Any + Send + Sync>`) or `io::raster_asset_from_semio_image_snapshot` (the PNG re-encode
   through the composer). A one-line guest-side distinction between those two is worth more than any
   further host-side probing.
3. If (1) says boot: read how the play shell reaches `setActiveExample` for this pane
   (`…/🐚️Shell/🟦️.tsx:292-301` `resolveBootExampleId`) against what the store boots with.

## 5. Session 7 addendum (21:40) — durable log, and the pane verdict taken off a SCREENSHOT

**Durable run**: `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/raster/test-29.txt`
(18:14, one batched invocation through `$T/📜️native-test-mutex.sh raster`, private
`CARGO_TARGET_DIR=$G/target`, `--no-fail-fast` before the `--`, `RUST_BACKTRACE=1`,
`--test-threads=4`): `semio-framework-raster` **3/3 ok**, `semio-s-plugin-raster` **3/3 ok**,
`semio-s-artifact-raster-raster` (lib) **223 passed / 2 failed**, `EXIT=101`. The two are
`viewer::raster::modes::view::windows::{composite,navigator}::component::tests::render_produces_a_scene_node_for_the_default_document`
— the peer-frozen `ImageWindowKit` laws of §3, kept as a PROPOSED DIFF, not applied.
(`test-27`/`test-28` proved the same numbers before the 16:05–16:25 sweep took their logs.)

**Live pane, 21:36, on the 15:47 activation** — evidence
`.🧬semio/🦑️repo/⚡️cache/play-fleet/raster/browser9/` (`page.png`, `canvas-0.png`, `canvas-1.png`,
`probe6.json`, `probe6-console.txt`; script `$G/probe6.mjs`):

- **VERDICT: the raster pane is BLANK.** The full-page screenshot shows the Composite and Navigator
  windows as uniform paper — no backdrop layer, no emblem, no document content at all. `data-shell-ready`
  reached, the example switcher reads **Demo**, **0 page errors, 0 console errors, no refused inputs**.
- Correction to my own earlier method: a WebGPU canvas read back through `drawImage` can come back fully
  transparent whether or not it drew, so the canvas census alone could not have settled this. The
  screenshot can, and does.
- Same correction cuts the other way for the acceptance suite: clipping a screenshot to the canvas'
  bounding box yields 118 / 316 distinct colours for a BLANK surface, because the window chrome (title
  bar, `Actions`/`Utilities` buttons, the red active-window accent) is composited over that box. **The
  strict acceptance suite's "non-uniform canvas" assertion is therefore a false green for raster** — it
  passes over an empty composite. Worth tightening before it is trusted for other panes.
- `atob` is never called in the page, and `Paint2dHost` (`…/🖌️Paint2dHost/🟦️.tsx:252-260`, `:94-98`) has
  no other decode path, so the guest is still publishing `assetsJson == "{}"` (three `JSON.parse("{}")`
  hits recorded). Everything in §4 about WHY still stands: the served module carries every fix.

**Discriminator attempt (inconclusive, for the next session)**: `$G/probe5.mjs` re-selects the curated
example to force `set-active-example`'s replace batch to re-mint the emblem. It could not drive the
navbar switcher headlessly — the Radix select popup renders OUTSIDE the viewport, so Playwright refuses
the option click ("element is outside of the viewport", at 1600×1000 and at 1600×1400), and the keyboard
route (`Enter` → `Home`/`End` → `Enter`) left the trigger on `Demo`, i.e. nothing was selected. Driving
that switcher needs the popup-geometry workaround the wgpu-parity topic wrote up
(`🎫️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/📓️astra-select-popup-geometry-and-reveal.md`).

**One more hypothesis worth testing first** (cheaper than the discriminator, and it explains why every
native law is green): the assets payload rides OUT of the surface doc as a `paged_text_carrier` lane of
`UiText` leaves capped at `UI_TEXT_MAX_BYTES` = 512 B. A real emblem PNG in base64 is tens of KB, i.e.
dozens of leaves over nested pages. If the guest's fixed-capacity UI arena refuses that carrier in the
wasm build (the `📓️memory` note "One UI Admission Fault Kills Every Later Refresh" is the same shape),
`paint2dSceneFromLanes` never sets `assetsJson`, and `parsePaint2dAssets`'s `try/catch` turns the missing
lane into `{}` **silently** — which is exactly what the page shows: a well-formed scene, no pixels, no
error. The native fixture path (`decode_fixture_scene_with_lanes`) reassembles the same lane and passes,
so only a guest-side reading can tell the two apart.

## 6. THE BLANK COMPOSITE — root cause found and fixed (2026-09-22 22:10)

**It was never raster.** `SurfaceView` in
`/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`
routed a surface through `PagedSurfaceView` — the component that reattaches a scene's out-of-doc
payload lanes — for `world-3d`, `canvas-2d`, `board-2d` and `tiled-map`. **`paint-2d` was missing from
that hand-written list**, while `Paint2dScene::split_lanes`
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:1719`) moves BOTH `documentSyncJson` and
`assetsJson` out of `SurfaceProps.doc` unconditionally. `Paint2dHost` therefore received `""` for both
fields, and its own `syncAll` swallowed the consequence:

```tsx
try { session.syncDocumentJson(scene.documentSyncJson); … } catch (error) { return; }   // 🖌️Paint2dHost/🟦️.tsx:245-250
```

an empty document JSON throws, the `catch` returns BEFORE the asset-upload block, and nothing is ever
drawn. That is the whole symptom set, exactly: composite AND navigator windows completely empty, zero
page errors, zero console errors, `atob` never called, `assetsJson` read as `{}`.

Five Rust `SceneDoc`s override `split_lanes` (Canvas2d, World3d, Paint2d, TiledMap, Board2d); paint-2d
was the only one whose host route was missing, which is why every other pane rendered and only raster
looked broken — and why no native law could ever have caught it: the split/merge pair is exercised on
the Rust side by `decode_fixture_scene_with_lanes`, and the host's routing table is TypeScript.

### Fix
- `…/🗣️Interpreter/🟦️.tsx` — one table `SURFACE_KIND_SCENE_LANES` (the five lane-publishing kinds) and
  `surfaceKindSceneLanes(kind)`; `PagedSurfaceView` takes its lane table from it and `SurfaceView` takes
  its routing decision from the same membership, so the two can no longer disagree. The hand-written
  kind lists are gone.
- `…/🗣️Interpreter/🧪️tests/🚚️surface-scene-lanes/🟦️.tsx` — a paint-2d carrier round trip against the
  committed `🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json`, and a routing law pinning the table against
  every lane declaration the scene module publishes.
- `bunx tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json` → **clean** (`$G/tsc-os.txt`, 0 lines).

### Live proof (no re-activation needed — this is host TypeScript; serve recycled 22:00)
| reading | before (21:36) | after (22:01) |
|---|---|---|
| `atob` calls (the host's only texture-decode path) | **0** | **2** |
| assets lane seen by the host | `{}` | `{"semio-emblem":{"mime":"image/png","data":"iVBORw0KGgoAAAANSUhEUgAAAAIAAAAC…"}}` |
| navigator composite-viewport overlay | absent | **drawn** |
| page / console errors | 0 | 0 |

Evidence: `.🧬semio/🦑️repo/⚡️cache/play-fleet/raster/browser10/` (probe6: screenshots + atob) and
`browser11/probe7.json` (the parsed assets lane), scripts `$G/probe6.mjs`, `$G/probe7.mjs`.

### What is left on the pane, and it is CONTENT, not code
That base64 decodes to a **2×2** PNG: the committed
`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🖼️semio-emblem.png`
is a 75-byte 2×2 RGBA swatch (red / green / blue / white) while the demo DSL's backdrop layer declares
`1024×1024`. The pipeline is whole now; the curated example simply carries placeholder media. Deciding
what the demo should actually show is a content call for the ticket owner.

**Capture caveat for whoever re-probes**: this headless Chromium composites neither into
`page.screenshot()` nor into a `drawImage` readback of a WebGPU canvas, so NEITHER can serve as the
visual verdict for a paint-2d/world-3d pane. The readings in the table above (lane payload, `atob`
count, overlay presence) are host-side and do not depend on GPU capture.

### Native law for the guest half (written, NOT yet verified)
`…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` gained
`mounted_boot_publishes_the_emblem_pixels_on_the_composite_assets_lane`: cold mounted boot →
`setActiveExample demo` → read the PUBLISHED surface through `decode_fixture_scene_with_lanes`, assert
the spine's own `SceneLaneRef` manifest measured >1 KiB onto the assets lane, that the carrier
reassembles to exactly those bytes, and that the entry is a real `image/png` payload. It could not be
run: `semio-framework-plugin` does not currently compile on anyone's machine —
`error[E0583] file not found for module interaction_selection_laws` (`🔌️plugin/🦀️.rs:22418`) and
`error[E0425] cannot find function history_row_applied_v1` (`:25235`), a peer's in-flight refactor of a
file this session must not touch (test-30/31/32, 22:06–22:11). Re-run once that lands:
`zsh $G/run-lib.sh $G/test-33.txt -- --test-threads=4`.

## 7. The curated demo now ships real media (2026-09-22 22:20)

The pane's content comes from `📚️examples/🎬️demo` — `emblem_image_asset()` plants the committed PNG on
every `setActiveExample demo`. That PNG was a **75-byte 2×2 RGBA swatch** (red / green / blue / white)
while the carrier DSL declared a `1024×1024` backdrop. It was real and decodable, so every codec law was
green — and it drew four pixels, which in a browser is indistinguishable from a pane that renders
nothing. That is precisely what masked §6's missing lane route for three sessions.

(The `semio_fixture_snapshot()` in `🧬️schema/🦀️.rs` keeps its own inline 2×2 PNG on purpose: it exists so
the codec laws have a minimal genuinely-decodable payload, and the committed
`🧫️fixtures/🖨️mutate-raster-1/🔣️.snapshot.json` is content-addressed against it. Only the CURATED
EXAMPLE's media changed.)

### Change
- `…/✳️any/🖼️assets/🎬️demo/🖼️semio-emblem.png` — now the repo's own shipped semio emblem raster,
  copied verbatim from `🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖼️raster.png`
  (**512×512 RGBA, 25 039 bytes**). The `🌘️dark-round` variant because this pane's background is paper
  (`rgb(240,236,221)`): the `☀️light`/`🌙️dark` rasters are white-field and the `⚪️round` one is cream
  (`rgb(247,243,227)`), i.e. invisible against it; the dark-round field (`rgb(0,17,23)`) with the white
  and red semio mark is unmistakable. Keeping the asset id `semio-emblem` honest mattered more than
  picking a photograph — this IS the semio emblem, at its shipped canonical size.
- `…/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` — the backdrop layer's declared size follows the asset:
  `[1,1024],[1,1024]` → `[1,512],[1,512]`.
- `…/✳️any/📚️examples/🎬️demo/🦀️.rs` — the doc comment now names the source asset, the variant and why.
- The DSL's asset line still carries the old content-addressed handle id
  (`raster-asset-81e1564e819d209c`). It is INERT: `replace_document_operations` emits a
  `remove-layer-asset` for every handle the carrier parsed and `example_media` re-mints the child from
  the real bytes in the same batch, so the id the document ends up with is always the canonical mint of
  whatever `emblem_image_asset()` returns (no test pins the carrier's id — `🔬️boot-document` compares
  documents, not handle ids). Regenerating it needs a `DefaultHasher` over `SemioImageSnapshot`'s own
  pack, i.e. a run of this crate, which is blocked below; the coordinator's `describe` will refresh
  `✏️s/🔌️plugins/🖨️raster/🔣️.json`'s copy of the carrier either way.

### Laws added / strengthened
- `…/🧬️schema/🧪️tests/🔬️boot-document/🦀️.rs` —
  `the_demo_carrier_ships_real_media_sized_exactly_as_its_backdrop_declares`: the committed asset is a
  PNG over 8 KiB, at least 256×256 read straight out of its IHDR (bytes 16..24, big-endian — no codec
  involved, so a re-encode cannot satisfy it), and the carrier's backdrop declares EXACTLY that size
  with `imageKey = semio-emblem`. The two can no longer drift.
- `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
  `mounted_boot_publishes_the_emblem_pixels_on_the_composite_assets_lane` — now base64-decodes the exact
  payload the host would hand `atob` and asserts >8 KiB and ≥256×256 from the IHDR, on top of the
  lane-manifest / carrier-round-trip assertions.

### Blocked on a peer, not on this work
`semio-framework-plugin` has not compiled since ~22:00: `error[E0583] file not found for module
interaction_selection_laws` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, the declaration
moved 22418 → 22394 while I watched, so the refactor is in flight) and `error[E0425] cannot find
function history_row_applied_v1`. Four runs hit it (`$G/test-30..33.txt`, 22:06–22:22). That same crate
is what `describe` compiles for wasm32, so it blocks BOTH the native re-run and the activation that
would put this media on the pane. When it lands:
1. `zsh $G/run-lib.sh $G/test-34.txt -- --test-threads=4` (all three raster crates, one invocation),
2. `$T/🗑️generated/{describe,activate}.request/raster` are already touched (22:11),
3. live proof through the assets lane, NOT a canvas census: `node $G/probe7.mjs http://127.0.0.1:6033 <out>`
   should show the decoded payload's IHDR at 512×512 instead of the 2×2 recorded in
   `$G/browser11/probe7.json`, with `atob` still firing twice and 0 console errors.

## 8. Session 7 successor (2026-09-23 02:00) — paint witness published by the host

State on arrival: every §6/§7 edit is in the 00:05 auto-commit `9c641044be` (verified by `git show --stat`:
Interpreter lane table + law, 512×512 emblem, DSL `[1,512]`, both new Rust laws). The peer's
`paged_text_carrier` proposal for `ImageWindowKit::render` HAS landed
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` `IMAGE_WINDOW_PIXELS_LANE_KEY`, ~line 33196), and the
`interaction_selection_laws`/`history_row_applied_v1` compile blockers are resolved (declared at ~22463/22481).
Native run `test-34` queued behind three peers in the play native mutex (01:56).

### Task 3 — `data-layers-json` / `data-assets-json` on the paint surface (done, law green)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx` —
  `paint2dAssetExtent` (decoded byte length + PNG IHDR extent from the first 32 base64 chars) and
  `paint2dPaintWitnessDom(documentSyncJson, assetsJson)`; the surface div publishes
  `data-layers-json` (the layer forest) and `data-assets-json` (`{key: {mime, bytes, width, height}}`),
  memoized on the two scene fields. Deliberate deviation from play-runtime's proposal: the raw base64 is
  NOT mirrored into the DOM (a real texture would put megabytes into an attribute on every scene change);
  the extent is exactly what the witness needs. `parsePaint2dAssets` now also refuses a non-object
  (`"null"` used to reach `Object.entries(null)`).
- Language-neutral fixture `…/🖌️Paint2dHost/🧫️fixtures/🔬️paint-witness/🔣️.json` (5 cases incl. the old 2×2
  swatch, a GIF, undecodable data, the empty strings of a lane-less surface, malformed JSON; plus the
  committed 512×512 emblem by path).
- Law `…/🖌️Paint2dHost/🧪️tests/🔬️paint-witness/🟦️.ts`, registered in the renderer vitest config
  (`elementSuite("🖌️Paint2dHost", "🔬️paint-witness")`); third-party oracle `pngjs` (fully decodes the same
  bytes and must agree on width/height) — declared as devDependency (`pngjs`, `@types/pngjs`) of
  `@semio-tech/framework-renderer-react`, `bun install --lockfile-only` (bun.lock gained only those two).
  `SEMIO_TEST_LEVEL=long bun node_modules/vitest/vitest.mjs run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts 🖌️Paint2dHost`
  → **7/7 passed** (`$G/vitest-paint-witness.txt`). `bunx tsc --noEmit -p 🧰️framework/🛍️products/💻️os/tsconfig.json` clean.
- `🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` — the React-fiber walk (`hostOf`) and the in-page IHDR
  peek are gone; a paint surface is read from its two attributes. A `.semio-paint-2d-canvas-surface` that
  publishes NO witness is an explicit red (not the DOM-body fallback, which would false-green on chrome text).

### Live on the 03:23 activation (serve recycled 03:24:24) — §7's real media is REFUSED by the store
`node $G/probe7.mjs http://127.0.0.1:6033 $G/browser12` (and `browser13`, with HTTP ≥400 URLs logged):
both paint surfaces now publish the witness (`data-layers-json="[]"`, `data-assets-json="{}"`), `atob` 0,
the document is `{"id":"raster","title":"Untitled","layers":[]}` — the demo never loads, because

```
input #1 setActiveExample refused: dispatch-failed (user window=raster-composite) — typed-operation
failed: plugin.internalvalidation failed: raster-store.mutation-asset-capacity
```

Root cause (production, `…/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`): the bounded candidate refused any
`add-layer-asset` whose data exceeded ONE 4 KiB owned-field page (`RASTER_OWNED_FIELD_BYTES`), in both
the candidate prepare step and the digest. That cap existed only because the retirement cursor's
`RasterRetirementOwner::Bytes` arm answered `Pending { 0, 0 }` for any buffer larger than its grant — the
exact "refuse instead of page" stall draw fixed on 2026-09-17 (memory: retirement cursor must page
oversized strings). With the 75-byte 2×2 swatch nothing ever crossed it; §7's 25 039-byte emblem does, so
the curated demo is refused on EVERY `setActiveExample` and raster could not hold any real image at all
(an `importMedia` of a photo would be refused the same way). The native law
`retained_asset_apply_and_snapshot_clone_keep_the_composite_pixels` drives `emblem_image_asset()` through
the same candidate, so it would have caught this had the crate compiled at 22:20.

Fix:
- `Bytes` retirement pages from the tail (`truncate` + `shrink_to`), `released_bytes ≤ grant`, full
  release once the remainder fits.
- The `add-layer-asset` data bound is the envelope ceiling `RASTER_MAXIMUM_NESTED_BYTES`
  (`ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES` = 256 KiB) in both the candidate and the digest (the digest
  already observes the bytes in 256-byte steps; the pixels live in the minted child's `Arc` owner, not in
  the fixed-capacity arena). The mime string keeps its one-page bound.
- New law `an_asset_over_one_retirement_grant_applies_and_its_operation_retires_within_every_grant`
  (`…/💾️binary/🧪️tests/🔬️unit/🦀️.rs`): the real emblem applies through the candidate, its pixels
  resolve, and the operation retires with every step inside a 4 KiB grant and all bytes released.
- `describe.request/raster` + `activate.request/raster` touched 03:40 (guest change).

Unrelated to raster, seen on the same page: `404 /🔌️plugin-modules/🪞️vendor/🔤️guestslim-typst-fonts.bin`
(the kernel's typst font shard, `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:2209`; materialization is supposed
to dump it). The strict suite tolerates resource 404s; reported, not touched.

### Native test-35 (04:26, `$G/test-35.txt`) — 227 / 1, and the one red is a missing framework hook
(test-34 died in a fine-grain build-dir lock cycle with flow's cargo — both idle in
`prebuild_lock_exclusive`, 0 rustc, 13 min; I killed only my own cargo and requeued.)
- `semio-framework-raster` **3/3**, `semio-s-plugin-raster` **3/3**, `semio-s-artifact-raster-raster`
  **227 passed / 1 failed**.
- GREEN now: `an_asset_over_one_retirement_grant_applies_and_its_operation_retires_within_every_grant`,
  `retained_asset_apply_and_snapshot_clone_keep_the_composite_pixels` (real emblem through the
  candidate), `mounted_boot_publishes_the_emblem_pixels_on_the_composite_assets_lane`,
  `the_demo_carrier_ships_real_media_sized_exactly_as_its_backdrop_declares`, and BOTH former
  peer-frozen viewer laws `viewer::…::{composite,navigator}::…::render_produces_a_scene_node_for_the_default_document`
  (the peer's `paged_text_carrier` for `ImageWindowKit::render` landed).
- RED: `raster_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed` —
  `editor did not declare a loaded-parent child projection`. The framework's `ArtifactEditor` /
  `ArtifactViewer` gained `child_restore_projection` with a faulting default and the live envelope load
  now calls it; every composing plugin (gis, animate, cad, flow, …) implements it via
  `store::ChildRestoreProjection::from_snapshot`, raster did not. Fixed in
  `…/✳️any/✏️editor/🦀️.rs` and `…/✳️any/👁️viewer/🦀️.rs` the same way (the projection is empty because
  `RasterSnapshot.assets` declares no `#[child]` slot — the documented schema-introspection gap in
  `🧬️schema/📸️snapshot/🦀️.rs`; restoring asset children after a document LOAD therefore still relies on
  the host resolver, as before). Re-run: `$G/test-36.txt`.

### LIVE PROOF — 04:47 activation (serve recycled 04:47:59): the raster pane paints the emblem
(The coordinator's 04:32 describe was a malformed batched nx target — `@semio-tech/energy flow gis
mathematical raster -plugin:describe`, "Cannot find project" — so `🔣️.json`/`🛂️.descriptor.semio` were NOT
re-described; `activate-dev` 04:32–04:47 did rebuild the guest after both Rust edits, and raster is absent
from the page's own "staged module behind source" list.)
- `node $G/probe7.mjs http://127.0.0.1:6033 $G/browser14`: decoded IHDR **512×512** (×4 reads), `atob`
  fires on the full 31 688-character payload twice (plus the witness's 32-char IHDR peeks), both
  surfaces publish `data-layers-json` = backdrop pixel layer 512×512 → `semio-emblem` + brighten
  adjustment, `data-assets-json` = `{"semio-emblem":{"mime":"image/png","bytes":23765,"width":512,"height":512}}`
  (23 765 B: the stdio canonical re-encode of the 25 039 B file). **No refused input, no page error**; the
  only console error is the framework-wide typst-font 404 the suite tolerates.
- Strict acceptance, one worker, from the repo root:
  `PLAYWRIGHT_BASE_URL=http://127.0.0.1:6033 … playwright/cli.js test --config 🏢️semio-tech/🎡️play/🔨️modules/🧪️e2e/🎚️config/🟦️.ts --grep raster --workers 1`
  → **1 passed (7.0 s)**, log `$G/e2e-raster.txt`, artifacts `$G/e2e-raster/`. The paint witness read
  the new attributes (no fiber walk left in the suite).

### Native test-36 (04:51, `$G/test-36.txt`, EXIT=0) — ALL GREEN
`semio-framework-raster` **3/3**, `semio-s-artifact-raster-raster` **228 passed / 0 failed**,
`semio-s-plugin-raster` **3/3**. Includes the live-envelope law (now that the hook exists), both former
peer-frozen viewer laws, the demo-media laws and the new paged-retirement law.

### Topic state at 04:55 — raster is DONE
- Native green (above); renderer TS law `🖌️Paint2dHost/🧪️tests/🔬️paint-witness` 7/7; strict e2e raster 1/1;
  live pane loads the curated demo and publishes a 512×512 emblem on both windows.
- Open, not raster's: (1) `🔣️.json`/`🛂️.descriptor.semio` were not re-described after this session's
  guest edits (coordinator's 04:32 describe batch was malformed) — `describe.request/raster` should be
  re-touched by whoever runs the next chain; no descriptor-visible surface changed (no new action, window
  or schema field), so the served pane is unaffected. (2) framework-wide 404 of
  `🪞️vendor/🔤️guestslim-typst-fonts.bin`. (3) Asset children are not declared as `#[child]` slots, so
  `child_restore_projection` is empty and a raster document LOADED from an envelope relies on the host
  resolver to re-materialize its images (pre-existing, documented in `🧬️schema/📸️snapshot/🦀️.rs`).
- 04:56: `describe.request/raster` re-touched for (1).
