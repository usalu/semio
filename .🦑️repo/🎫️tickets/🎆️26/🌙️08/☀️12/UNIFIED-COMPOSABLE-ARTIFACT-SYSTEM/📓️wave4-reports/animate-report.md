# W4 batch Cb — `animate` composes stdio `presentation` (real content) + `animation` (composed, honestly empty today)

**ucas-status: complete — baseline was already green, 0 compile errors after migration, 228/228 tests passing, reproduced stable across TWO independent full runs (`cargo test -p semio-s-plugin-animate --lib` / `--all-targets`, both 228 passed / 0 failed / 0 ignored), 0 failures.**

## Baseline (before any edit)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-animate --all-targets
```
Green from the start: `Finished `dev` profile [unoptimized] target(s) in 2m 02s`, only pre-existing warnings (ambiguous `testkit` glob, unused imports, an unnecessary-qualification lint) — none touched by this pass, none inside animate's own boundary in a way that blocked anything. No pre-existing defect to separate from this migration's own.

## What animate's `present` artifact was duplicating

Read the artifact root (`🗿️artifacts/🎬️present/🦀️component.rs`) and its snapshot (`🧬️schema/📸️snapshot/🦀️component.rs`) before touching anything, per the dispatch brief. `PresentSnapshot` was two fields: `source: FigureTileSource` (one shared background figure — `src`/`kind`/`frame`/`sourceAspect`/`pdfPage`) and `tiles: Vec<FigureTileDraft>` (named normalized crop rectangles over that source — a filmstrip/contact-sheet tool, NOT a literal slide deck and NOT time-based animation). The 9-mutation-triad vocabulary (`create/delete/delete-tiles/rename/resize-tile-crop/reorder/replace-tiles/replace-source/resize-source-frame`) operates entirely on this shape.

Separately, `🎛️apps/🎬️present/⚙️engine/**` hosts a large (~1.5k line) Manim-class scene/keyframe animation engine (`Scene`/`Sobject`/`Camera`/rate-curves/headless video renderer) used by `compile_scene_to_assets`/`export_video_from_scene`. This is genuinely unrelated to `PresentSnapshot`'s persisted content — scenes are constructed by Rust trait impls (`Scene::construct`), never derived from the document — confirmed by reading every file in that subtree: the only touchpoints are `compiler::site_manifest` (reads `tiles.len()`/`tiles.first()` for a JSON manifest) and `slide::PresentScene.deck: Option<PresentSnapshot>` (holds a reference, no field-level coupling). Neither needed structural changes beyond the same `.tiles`/`.source` → working-scene-accessor rewiring every other call site got.

Mapping per `📓️design-full-plan.md` §4 (`animate→C:presentation,animation`): `source` + `tiles` map onto stdio `presentation`'s slide-deck shape (`source` → the deck's one `SlideMaster`, a `Picture` shape spanning `source.frame`; each `tile` → its own `Slide`, one `Picture` shape at `tile.crop`, `tile.name` carried as the slide's `notes`). `animation` is composed too, per the design mapping, but is **honestly empty** — this artifact carries no time-based/keyframe data at all today (see `animation_child_handle`'s doc comment). Fabricating fake keyframes to "use" the slot would violate the recipe's honesty requirement; an always-empty-but-real, typed, composed child is the same precedent `gismap`'s `image` slot and `shooting`'s declined-`table` reasoning both already established for this ticket.

## §3 resolver-seam finding (relevant to every fan-out agent still to come)

Checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only) per the recipe's mandatory §3 check, going further than a grep for existing usage (found none — no plugin references `ArtifactView::with_children`/`ChildContentView` yet):

**`ArtifactView::with_children` IS real, IS live, and IS already wired into every plugin's dispatch path generically** — `VcsArtifactApp<A>`'s own `handle_action`/`dispatch_typed_command`/`copy`/`cut`/`paste`/render call sites (`🔌️plugin/🦀️component.rs`, confirmed at 10+ call sites) construct `ArtifactView::with_children(snapshot, history, ChildContentView::new(&self.children))` before calling into the app's own `ArtifactApp::handle`/`render`/etc. — not an opt-in a plugin has to wire up itself. `ChildContentView::typed::<S>(slot, child_id)` is the real, live, non-stale accessor (`self.children` is populated by `open_child`/`register_child`, keyed exactly like `ArtifactChild` handles).

**But this does NOT retire the `thread_local!` working-scene pattern (§4) for mutation diff/inverse construction.** `protocol::MutationKind::diff`/`::inverse` (the pure functions every `🧬️mutations/<kind>/{🔺️diff,↩️inverse}` leaf implements) receive only `base: &PresentSnapshot` — never an `ArtifactView`, never live child-store access. The `with_children` seam is real and reachable **only from the app layer** (`ArtifactApp::handle`/`render`, which DO receive `doc: &ArtifactView<Snapshot>`) — animate's own `render()`/panel functions could in principle be rewired onto `doc.children.typed(...)` instead of the working-scene cache for READING, but the WRITE side (every mutation's `diff()` needing the pre-existing `(source, tiles)` to build the next content-addressed handle) has no seam to reach at all, by construction of the `MutationKind` trait. This migration used §4's thread_local cache throughout (matching every prior exemplar), and left the app-render-layer opt-in-to-`with_children` refinement as documented future work (see `## Honest gaps`) rather than mixing two access patterns mid-migration.

## What changed

### Domain root (`🗿️artifacts/🎬️present/🦀️component.rs`)

New `🔖️PresentationBridge` + `🔖️WorkingScene` regions (domain types `FigureTileFrame`/`FigureTileSource`/`FigureTileDraft`/`FigureTileDraftPatch` unchanged — still the mutation payload shapes):
- `PresentationChild`/`AnimationChild` type aliases (`store::ArtifactChild<SemioPresentationSnapshot>`/`<SemioAnimationSnapshot>`).
- `presentation_snapshot_from_source_tiles`/`source_tiles_from_presentation_snapshot` — real bidirectional converters. `source.kind` is reused verbatim as `SlidePictureImage.mime` (an honest, lossless choice over inventing a MIME taxonomy `presentation` has no field for). **Documented lossy**: `source.source_aspect`/`source.pdf_page` have no representable slot in `presentation`'s schema and are dropped by the forward conversion — every in-process mutation round-trip still preserves them exactly via the working-scene cache (never routed through this projection for restoration), so this only matters for a genuinely fresh reload with an empty cache, the same class of gap every exemplar (lowpoly/writer) already carries for its own slot.
- `presentation_child_handle`/`animation_child_handle` — content-addressed minting (`DefaultHasher` over `serde_json::to_string`), matching lowpoly's/writer's convention exactly. `animation_child_handle()` is deterministically constant (content is always `SemioAnimationSnapshot::default()`).
- Working scene: `PRESENT_SCRATCH: thread_local! RefCell<HashMap<child_id, (FigureTileSource, Vec<FigureTileDraft>)>>`, `cache_present_working_scene`/`present_working_scene_for_handle`/`present_working_scene` (the one read accessor every call site now funnels through — fails soft to `(default_figure_tile_source(), Vec::new())` on a cache miss, documented) / `presentation_child_handle_and_cache` (mint + cache in one call) / `present_snapshot_with_tiles` (the fixture constructor replacing the old 3-field struct literal).

### Snapshot (`🧬️schema/📸️snapshot/🦀️component.rs`)

`PresentSnapshot` now `{ schema, presentation: PresentationChild, animation: AnimationChild }` — both `#[child(kind = "s.stdio.semio.presentation"/"s.stdio.semio.animation")]`, both bare (never absent — this artifact always composes exactly one of each). Dropped the `dsl::DslRecord`-derived `PresentSnapshotDsl` mirror entirely (recipe §2 — `ArtifactChild<S>` has no `DslField` impl) and hand-rolled `ArtifactDsl`/`ArtifactPack` directly on `PresentSnapshot`, following writer's exact hex/bracket (text, `[hex(child_id),hex(target.to_uri())]`) + LEB128-length-prefixed (binary) child-handle convention. Both codecs verified round-tripping for both children in the same test (`populated_snapshot_pack_and_dsl_round_trip`).

### Diff (`🔺️diff/🦀️component.rs`, `🔺️diff/📝️text/🦀️component.rs`)

`PresentDiff.source: Option<FigureTileSource>` + `.tiles: Option<PresentTilesDelta>` replaced by a single `presentation: Option<PresentationChild>` — the recipe §9 "always-present slot" shape (single `Option`, not the double-`Option` an optional slot needs), matching writer's `document` field precedent exactly. `PresentTilesDelta`/`PresentTilePatchEntry` (the old sparse id-keyed collection-delta types) and `apply_tiles_delta`/`absorb_tiles_delta`/`tiles_delta_from_set_tiles` are removed — dead once `tiles` stopped being a top-level snapshot field; the granular add/remove/patch/reorder tile semantics now live entirely inside each mutation triad's own diff-construction logic against the working scene (still real, still sparse-built from `(payload, base)`, never apply-then-capture — see below), not as a separate delta type threaded through `PresentDiff::apply`. `PresentDiff.artifact: Option<Box<PresentArtifact>>` (the pre-existing whole-artifact-replace escape hatch, never populated by any of the 9 mutations, `diff.artifact.is_none()` asserted by an existing test) is UNCHANGED — same pattern writer's own `WriterDiff.artifact` keeps, not something this migration introduced or removed. New `diff_set_presentation(source, tiles) -> PresentDiff` builder (mirrors writer's `diff_set_text`) — mints the handle, caches it, wraps it — used by all 9 mutation triads' diff functions.

### Mutations — all 9 triads rewired, payload shapes unchanged

Every mutation's public payload struct (`CreateTile{index,tile}`, `DeleteTile{id}`, `DeleteTiles{ids}`, `RenameTile{id,new_name}`, `ResizeTileCrop{id,new_crop}`, `ReorderTiles{id,to_index}`, `ReplaceTiles{new_tiles}`, `ReplaceSource{new_source}`, `ResizeSourceFrame{new_frame}`) is **byte-for-byte unchanged** — same fields, same `#[dsl(keyword=...)]`, same wire format (confirmed by the existing `optional_field_rows_keep_their_pre_migration_bytes`/`every_printed_op_line_starts_with_the_rows_wire_keyword` tests passing unmodified). Only the internals of each `diff()`/`inverse()` pair changed:
- `diff()`: read `(source, tiles)` via `present_working_scene(base)`, apply the mutation's own edit to the in-memory tuple, call `diff_set_presentation(&source, &tiles)`.
- `inverse()`: read the pre-mutation `(source, tiles)` via `present_working_scene(base)` (was `base.tiles`/`base.source` directly) to reconstruct the undo mutation.
No exceptions/deviations needed — every triad fit the same pattern cleanly (unlike lowpoly's per-object-nested-field limitation or shooting's declined-`table` case, animate's whole `{source,tiles}` content moving to one composed child is structurally identical to writer's single-`document`-child shape).

### `PresentArtifact` (`🧬️schema/🦀️component.rs`)

Same field swap (`source`/`tiles` → `presentation`/`animation`, both `#[child(...)]`), `Default`/`to_snapshot`/`from_snapshot`/`set_snapshot` updated to match.

### App layer — every `.tiles`/`.source` call site rewired to `present_working_scene`

11 files: app root (`🎛️apps/🎬️present/🦀️component.rs`), engine `compiler` (`⚙️engine/🦀️component.rs`), tile-editor window render (`🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️component.rs`), panels (`📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue}/🦀️component.rs`), commands (`🎮️commands/{⌨️engagement,👁️view,🀄️tile,🖼️source,🌐️grid}/🦀️component.rs`). Every read of `deck.tiles`/`deck.source` became `crate::artifacts::present::present_working_scene(deck)`; every test constructing a `PresentSnapshot{tiles,..}`/`{source,..}` struct literal now builds via `present_snapshot_with_tiles`/mutates the working-scene tuple before rebuilding. Purely mechanical — no behavioral changes, confirmed by every existing app-layer test passing unmodified in intent (same assertions, same expected counts/names/crops).

## Whole-document replace — nothing to remove

Checked (grep, before any edit): no `whole_document_operation` override in `AnimatePresentPlayApp` (confirmed still the trait default, `None`) and the one whole-document-replace GESTURE (`setActiveExample`/"reset to demo") already went through `HostEffect::LoadDocument` (`reset_present_document_effect`), never an `artifact_mutations` entry — pre-existing, correct, untouched by this pass.

## Fixture regeneration (recipe §8, temporary-debug-test technique)

The demo fixture (`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`) was in the OLD `source {...} tiles [...] {}` DSL-record-derive format — incompatible with the new hand-rolled `schema=<hex>` / `presentation=<hex,hex>` / `animation=<hex,hex>` line format. Added a temporary `#[cfg(test)] mod debug_fixture_regen` to `📸️snapshot/📝️text/🦀️component.rs`, ran `cargo test ... debug_fixture_regen -- --nocapture`, captured real `print_dsl(&default_present_snapshot())` output, wrote it as the new fixture, removed the temp module cleanly (verified with `grep -rn debug_fixture_regen` returning nothing afterward). `present_dsl_round_trips_bundled_default_example` (parses the checked-in fixture) passes against the regenerated file.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-animate --all-targets
```
**0 errors** (`Finished `dev` profile`), confirmed twice across the session (once immediately after the schema/diff/mutations layer landed with 3 residual errors — `default_present_snapshot` missing import in the temp regen module, two `.tiles` misses in `🧬️mutations/💾️binary/🦀️component.rs` I'd missed on the first grep pass — fixed, then 0 errors on every subsequent check).

```
CARGO_TARGET_DIR=.../🎯️target cargo test -p semio-s-plugin-animate --lib
CARGO_TARGET_DIR=.../🎯️target cargo test -p semio-s-plugin-animate --all-targets
```
**228 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out** both times (0.44s–1.80s), single test binary (`📦️glue.rs` unittests — no separate integration-test target). Covers every one of the 9 mutation triads' round-trip/inverse/diff-absorb laws, the DSL text + binary codec round trips (default + populated), the regenerated fixture's own round trip, every app-layer command/panel/window test, the video/scene/rate/camera/geometry engine's own (untouched) test suite, and the inference/topology laws. No skips, no flakes across two independent runs.

## Fixed outright

- `🧬️mutations/💾️binary/🦀️component.rs` tests (`envelope_helpers_round_trip`, `present_deck_materializes`) — missed on the first grep pass (this file wasn't in the initial `.tiles`/`.source` grep because it lives in a `💾️binary` subdirectory I hadn't listed explicitly); caught by `cargo check`'s E0609 and fixed the same way as every other call site.

No pre-existing failures were present to triage (baseline was green) — nothing to trace via `git log --date=iso`.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/🎞️animate/**`, never touching `📦️glue.rs`/`📦️index.ts` or any `✏️s/🔌️plugins/🗄️stdio/**` file (stdio's `presentation`/`animation` subset schemas were read-only reference: `SemioPresentationSnapshot`/`Slide`/`SlideMaster`/`SlideShape`/`SlidePictureImage`/`SlideFrame`, `SemioAnimationSnapshot`, and the shared `SemioPoint2`/`document::DocBlock` types they themselves reuse).

## Concurrent-churn observations

One pre-existing, unrelated staged change noticed in `git status` after finishing: `📸️snapshot/🟦️component.ts` shows `M ` (staged, not further modified) with a one-line docstring diff (`"persistent fields only"` → `"artifact-lane fields only"`) that does NOT match this migration's Rust changes (the staged version still declares the old `source`/`tiles` TS shape). This was already staged before this session touched the file — not something this pass created or needs to revert; flagged here per `📌️important.md`'s "read the actual diff before attributing blame" guidance, not acted on (out of this migration's Rust-only scope).

No other concurrent churn encountered — `cargo check`/`cargo test` were green on every single invocation this session, no framework-layer errors ever surfaced to retry against.

## Honest gaps (non-blocking)

1. **Non-Rust facet mirrors left stale** — `🟦️component.ts`/`🔣️component.json`/`🔗️component.graphql`/`🛰️component.proto` for the `snapshot`/`diff`/`artifact`/`mutations` facets, plus the static `📖️component.grammar.semio` declaration file, still describe the pre-migration `source`/`tiles` shape. Matches every prior wave-4 report's own documented scope boundary (verification is `cargo check`/`cargo test`-scoped per the recipe; no exemplar in this ticket's `📓️wave4-reports/` touches these non-Rust files either) — flagged for whichever later wave reconciles static facet mirrors with the real derived Rust shape across the whole fan-out.
2. **`presentation_snapshot_from_source_tiles` drops `source_aspect`/`pdf_page`** on the forward conversion (no representable slot in stdio's `presentation` schema) — documented in the converter's own doc comment; does not affect any in-process mutation round-trip (the working-scene cache is the source of truth for those), only a genuinely fresh reload with an empty cache.
3. **§3's real `ArtifactView::with_children` seam was not adopted for animate's own `render()`/panel read paths** — those still read through the same `present_working_scene` accessor the mutation-diff layer needs anyway (which has no seam to use, see `## §3 resolver-seam finding`). Using `doc.children.typed::<SemioPresentationSnapshot>(...)` in `render()` instead would be a real, valuable, but separate follow-up (matches the recipe's own "not something to interrupt the fan-out for" framing for pre-2026-08-13 exemplars); documented here so it isn't lost.
4. **Working-scene staleness gap** — identical, documented class of gap every exemplar (lowpoly/writer) carries: store-level undo/redo bypasses `ArtifactApp::handle`, so a `presentation` handle can in principle go uncached in a fresh process or after an undo past this session's history; `present_working_scene` fails soft (`default_figure_tile_source()`, no tiles) rather than panicking.

## Files touched this pass

- `🗿️artifacts/🎬️present/🦀️component.rs` — new `🔖️PresentationBridge`/`🔖️WorkingScene` regions (converters, child-handle minting, working-scene cache, fixture constructor).
- `…/🧬️schema/📸️snapshot/🦀️component.rs` — `PresentSnapshot` field swap, dropped `PresentSnapshotDsl` mirror, hand-rolled `ArtifactDsl`/`ArtifactPack`.
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — test fixes (working-scene accessor), fixture regeneration (temp module added and removed).
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `PresentDiff` field swap, `PresentTilesDelta`/`PresentTilePatchEntry` removed.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_to_artifact`/`MutationDiff::apply`/`absorb` rewired; `diff_set_presentation` builder added; `tiles_delta_from_set_tiles`/`apply_tiles_delta`/`absorb_tiles_delta` removed.
- `…/🧬️schema/🦀️component.rs` — `PresentArtifact` field swap; `empty_present_snapshot` rewired.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — dispatch enum unchanged; tests rewired to working-scene accessor.
- `…/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`, `…/🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — test fixes.
- `…/🧬️schema/🧬️mutations/{✂resize-tile-crop,✏rename-tile,🆕create-tile,🔀reorder-tiles,🔁replace-tiles,🔲resize-source-frame,🖼replace-source,🗑delete-tile,🧹delete-tiles}/{🔺️diff,↩️inverse}/🦀️component.rs` — all 9 triads rewired to the working scene; payload shapes unchanged.
- `…/🧬️schema/💡️inferences/🦀️component.rs`, `…/🧬️schema/💡️inferences/🧭topology/🦀️component.rs` — working-scene accessor.
- `…/🚪️io/🦀️component.rs` — one struct-literal fixture fix (`animate_present_document_json_from_dwg`), test fixes.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture (new codec format).
- `🎛️apps/🎬️present/🦀️component.rs` — app-root helpers/tests rewired.
- `…/⚙️engine/🦀️component.rs` — `compiler::site_manifest`/test rewired.
- `…/🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️component.rs` — canvas-layer render + tests rewired.
- `…/📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue}/🦀️component.rs` — render functions rewired.
- `…/🎮️commands/{⌨️engagement,👁️view,🀄️tile,🖼️source,🌐️grid}/🦀️component.rs` — command handlers + tests rewired.

ucas-status: complete
