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
