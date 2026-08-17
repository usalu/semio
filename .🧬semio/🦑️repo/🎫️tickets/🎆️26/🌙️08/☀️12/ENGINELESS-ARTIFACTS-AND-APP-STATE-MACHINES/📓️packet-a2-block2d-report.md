# Packet A2 — Dissolve `block2d`'s `⚙️engine` (exemplar packet)

Target deleted: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (314 lines).
The `⚙️engine/` directory no longer exists under `◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

## Destination per region

| Region | Destination | Notes |
|---|---|---|
| 1. `🔖️ArtifactEngine` (`Block2dEngine` struct) | **deleted, not rehomed** | Confirmed 0 external references before deleting (`grep -rn "Block2dEngine"` found only the struct's own definition + itself in the engine file). |
| 2. `🔖️PuzzleCatalogFragment` (`puzzle2d_manifest_fragment`) | `🧬️schema/💡️inferences/🦀️component.rs`, new `//#region 🔖️PuzzleCatalogFragment` right after `🔖️ArtifactInferrer` | Derived compute over the snapshot = inference, per the map. Test moved into the file's existing `mod tests` as a new `//#region 🧪️PuzzleCatalogFragment` subregion. |
| 3. `🔖️DocumentHelpers` (`empty_block2d_snapshot`, `next_id`) | `🧬️schema/🦀️component.rs`, new `//#region 🔖️DocumentHelpers` after `🧬️DerivedArtifactFacets` | New `//#region 🧪️Tests` module added to this file (it had none before) carrying both tests verbatim. |
| 4. `🔖️Io` (`block2d_io`) | `🎛️apps/◻2d/🦀️component.rs`, new `//#region 🔖️Io` right after `🔖️Constants` | Function body unchanged; only qualification simplified since the imports (`AppIo`, `MediaType`, etc.) are now local `use`s instead of full `semio_framework_plugin::` paths. Its test (`block2d_io_declares_the_catalog_out_port`) added into this file's existing `mod tests`, in the `🔖️Manifest` subregion next to the pre-existing (different) `block2d_io_is_wired_into_the_manifest` test. |
| 5. `🚪️DerivedIoRegistry` (`pub mod io_registry` w/ `ComposerEntry`s + export composers) | `🚪️io/🦀️component.rs`, new `//#region 🚪️DerivedIoRegistry` after `🎹️DerivedComposition` | Moved verbatim (only home, no logic changes). This is the **low-level** `io_registry` (owns `entries() -> &'static [ComposerEntry]`); it is distinct from — and now the callee of — the artifact-top-level `io_registry` wrapper described below. |
| 6. `🔖️Register` / `🔖️ArtifactSchemaRegistry` / `🔖️ArtifactInferenceRegistry` (`register`, `register_pilot_languages`, `register_artifact_schema`, `register_artifact_inference`) | `🎛️apps/◻2d/🦀️component.rs`, new `//#region 🔌️Registration` right after the new `🔖️Io` region | **Diverges from the literal map instruction** — see "Mismatches" below. |
| 7. `🧪️Tests` | Split across destinations 2/3/4 above; no assertion dropped. | |

## Call sites updated

Directly touched files (all under `✏️s/🔌️plugins/🧱️block`), each verified with `git diff` against the pre-existing working tree:

1. `🎛️apps/◻2d/🦀️component.rs` — 8 references (3 doc comments + `initial_snapshot()`, `io()`, `export_media()`, `create_block2d_app()`'s `.io(...)`, plus the new `block2d_io`/`register*` functions added). +135/-23 lines.
2. `🎛️apps/◻2d/🎮️commands/🌱️handle/🦀️component.rs` — 1 (`next_id` call).
3. `🎛️apps/◻2d/🎮️commands/🔘️handle-kind/🦀️component.rs` — 1 (`next_id` call).
4. `🎛️apps/◻2d/🎮️commands/🔗️compatibility/🦀️component.rs` — 1 (`next_id` call).
5. `🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — 1 (`use ...engine::empty_block2d_snapshot` → `...schema::empty_block2d_snapshot`; the 12 subsequent unqualified call sites in that file's tests needed no edit).
6. `🗿️artifacts/◻2d/🦀️component.rs` (artifact top-level, pre-existing `io_registry` wrapper) — 1 (`use ...v1::engine::io_registry as v1` → `...v1::subsets::any::io::io_registry as v1`).
7. `📦️packages/🦀️rust/📦️glue.rs` — 3 edits: removed the `#[path=".../⚙️engine/🦀️component.rs"] pub mod engine;` mount under block2d's `v1`, removed the block2d-only shim `pub mod engine { pub use super::standards::v1::engine::*; }`, and changed `register_block_exports()`'s `crate::artifacts::block2d::engine::register();` to `crate::apps::block2d::register();`. block3d/block5d's own (untouched, still-`⚙️engine`-backed) lines were left exactly as-is.

Total: **16 call-site line edits** across 7 files, plus the 4 destination files gaining the moved code (schema, inferences, io, apps/◻2d) and the engine directory deletion. The ticket's "~41 references" estimate over-counts relative to actual distinct edit sites — most of the apparent volume was doc-comment repetition and the single-import + 12-unqualified-call pattern in the mutations test file.

Re-verified after all edits: `grep -rn "block2d::engine\|block2d\.engine\|Block2dEngine"` and `grep -rn "block2d::standards::v1::engine"` across `✏️s` return **zero** hits.

## Mismatches vs the map — most valuable finding for the other ~45 packets

**Region 6 (Register family) does not fit "the plugin root's registration path" literally.** The ticket instruction was to find where `semio_plugin!{ setup: … }` lives and move `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inference()` there. For this plugin that is `register_block_exports()` in `📦️glue.rs`. I did **not** put the function bodies there, for two reasons:

1. `glue.rs`'s own top-of-file docstring is explicit: "WIRING ONLY... Do not inline any component file back into this one: the taxonomy validator and the `TaxonomyLibShape` policy lint both fail on it." Adding ~90 lines of registration logic (5 `dsl::register_language` calls, schema/inference descriptor registration) would violate that self-imposed constraint.
2. `register()`'s body calls `semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Block2dPlayApp>(...)` — it depends on `Block2dPlayApp`, an **app** type. The engine's original placement under `🗿️artifacts` was itself already a constitutional violation (the ticket's own core principle: "an artifact must never depend on an app") — the same violation region 4 (`Io`) was explicitly called out for. Since `Io` and `Register` have the *same* app-dependency shape, I gave them the *same* destination: `🎛️apps/◻2d/🦀️component.rs`, under a new `//#region 🔌️Registration`. `register_block_exports()` in glue.rs now reads `crate::apps::block2d::register();` — a one-line call, consistent with the existing convention there (block3d/block5d's still-unconverted `crate::artifacts::block3d::engine::register()`/`block5d::engine::register()` are one-liners too).

**Recommendation for later packets:** when a `⚙️engine::register()`-family function's body touches the app type (`register_document_codec_for_app::<TheApp>`), treat it as app-constitutional and move it to the app's own top-level `🦀️component.rs`, not to `glue.rs` or to the artifact's own top-level file. Only update the plugin-root aggregator (`register_block_exports`-equivalent) to call the new one-line location. If a future artifact's `register()` body has *no* app dependency, `glue.rs`'s aggregator function itself is probably the right home instead — but I did not have that case here.

**Secondary note:** the artifact's own top-level `🦀️component.rs` (`◻2d/🦀️component.rs`, *not* the deleted engine file) already pre-dates this packet and had its own `pub mod io_registry` wrapper (thin `entries()`/`compose()`/`register()` over `&'static ComposerEntry` refs) that forwarded into the engine's nested `io_registry` via `use ...v1::engine::io_registry as v1`. This wrapper was **not** part of the 7 named regions but **was** a real call site — it had to be updated (`v1::engine::io_registry` → `v1::subsets::any::io::io_registry`) or the crate would not compile. Later packets should grep for `<artifact>::standards::v1::engine::` (not just `<artifact>::engine::`) before declaring their call-site sweep complete — this exact pattern (`🧊️3d`, `🖐️5d` artifact-top files) already exists for block3d/block5d too and will need the same fix when their turn comes.

## Verification — **UNVERIFIED, blocked upstream, not this packet's fault**

Per the repo's `.cargo/config.toml:2` (`rustc-wrapper = "sccache"`), all commands below were run with `RUSTC_WRAPPER=""` to defeat stale-cache false-greens, per the coordinator's correction mid-task.

```
RUSTC_WRAPPER="" cargo check -p semio-s-plugin-block --all-targets
```

This **cannot succeed right now regardless of correctness of this refactor**: `🧱️block`'s `Cargo.toml` depends on `semio-s-plugin-stdio`, which does not currently compile — another session is mid-refactor of `✳️brep`/`✳️drawing` mutation vocabularies there. Two consecutive runs of the same command surfaced two *different* upstream error sets (proof the stdio session is actively changing code under us), and in neither run did compilation ever reach `semio-s-plugin-block`:

Run 1 (`scratch-block2d-cargo-check-1.txt` in this ticket folder), tail:
```
error[E0599]: no variant, associated function, or constant named `DeleteLayer` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error[E0599]: no variant, associated function, or constant named `CreateLayer` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error[E0599]: no variant, associated function, or constant named `DeleteNode` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error[E0599]: no variant, associated function, or constant named `CreateNode` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error[E0599]: no variant, associated function, or constant named `MoveNode` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error[E0599]: no variant, associated function, or constant named `DragNodes` found for enum `drawing::schema::mutations::component::SemioDrawingMutation` in the current scope
error: could not compile `semio-s-plugin-stdio` (lib) due to 6 previous errors; 603 warnings emitted
```

Confirmed: `grep -c "^error" scratch-block2d-cargo-check-1.txt` = 7 (6 `E0599` + the final summary line); zero of those errors reference any `🧱️block` path (`grep -n "^error" -A3 ... | grep -i "🧱️block"` = empty); and `grep -n "Compiling semio-s-plugin-block\|Checking semio-s-plugin-block"` = empty — the build never got that far.

The coordinator separately measured a third error shape (`14 × E0432 unresolved import` in `subsets::brep`) moments earlier — three different failure signatures across three runs within one session, all confined to `semio-s-plugin-stdio`, all unrelated to `block`. **`cargo test -p semio-s-plugin-block` was not attempted** since `cargo check` for the same package already fails upstream of `block` — running it would only reproduce the identical stdio failure with no new information.

**I did not touch anything under `✏️s/🔌️plugins/🗄️stdio`.**

**Status: refactor complete, call sites swept clean (grep-verified zero remaining `block2d::engine` references), but compiler-UNVERIFIED.** Re-run `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-block --all-targets && RUSTC_WRAPPER="" cargo test -p semio-s-plugin-block` once `semio-s-plugin-stdio` is green again.

## Files touched (created/updated/removed)

- Removed: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (and the now-empty `⚙️engine/` directory).
- Updated: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎮️commands/🌱️handle/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎮️commands/🔘️handle-kind/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/🎛️apps/◻2d/🎮️commands/🔗️compatibility/🦀️component.rs`
- Updated: `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/📦️glue.rs`
- Scratch: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/scratch-block2d-cargo-check-1.txt`
