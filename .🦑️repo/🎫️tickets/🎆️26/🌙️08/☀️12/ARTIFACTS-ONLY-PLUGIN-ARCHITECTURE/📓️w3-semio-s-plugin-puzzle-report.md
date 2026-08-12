# W3 — `🧩️puzzle` (crate `semio-s-plugin-puzzle`) — APA plugin migration report

Plugin directory: `✏️s/🔌️plugins/🧩️puzzle/`. This plugin was released to APA by both peer sessions (SMO, UCAS) before this wave started.

## Step 0 — baseline

`cd "/Users/ueli/Documents/semio" && CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-puzzle 2>&1 | tail -20` was queued but never returned output before the edits began — the shared build lock was held continuously by other concurrent sessions' checks (`ps aux` showed 3 other `cargo check -p semio-s-plugin-puzzle` processes and ~10 other plugins' checks in flight at once). No pre-existing red/green baseline was captured before editing; see `## Concurrent-churn observations` and the Step 6 result for the post-edit reading, which is the only real baseline this report can attest to.

## Step 1 — dead facet directories

All three read as **doc-only, 1-line stub, and unmounted** (confirmed by `grep -n "🛂️manifest\|🎟️capabilities\|🔧️setup" 📦️glue.rs` → zero hits):

- `🛂️manifest/🦀️component.rs` — `//! 🛂️ Manifest facet for \`🧩️puzzle\` — identity surfaces live on \`Plugin::builder\` in the parent.`
- `🎟️capabilities/🦀️component.rs` — `//! 🎟️ Capabilities facet for \`🧩️puzzle\` — declare rights via \`PluginBuilder::capability\` / \`.local_backbone_storage()\`.`
- `🔧️setup/🦀️component.rs` — `//! 🔧️ Setup facet for \`🧩️puzzle\` — codec/language/importer registration hooked via \`.setup(...)\`.`

**Deleted outright** — zero code impact, no glue.rs mount to remove:
- `✏️s/🔌️plugins/🧩️puzzle/🛂️manifest/` (removed)
- `✏️s/🔌️plugins/🧩️puzzle/🎟️capabilities/` (removed)
- `✏️s/🔌️plugins/🧩️puzzle/🔧️setup/` (removed)

Also removed: stray `✏️s/🔌️plugins/🧩️puzzle/.DS_Store`.

## Step 2 — plugin root closed

### `🔨️modules/🎲️board-2d/🦀️component.rs` (508 lines, real code — mounted)

Census (`📓️w0-b-plugin-shape.md` §5) proposed `🗿️artifacts/◻2d/…/⚙️engine/board-2d/`. **Deviated from that proposal after reading the file**: its own doc-comment states the module was split out specifically to keep the puzzle-2d *artifact* `⚙️engine` free of `wasm-bindgen`/`web-sys`/`wgpu` so a workflow runner can drive the engine headlessly — moving it back into `⚙️engine` would re-introduce exactly what it was built to avoid. The file is a `#[wasm_bindgen]` `BoardSession` bridge (WASM-only interop, all items already individually `#[cfg(target_arch = "wasm32")]`-gated), i.e. structurally identical to the puzzle-3d/puzzle-5d apps' own `🌉️wasm/🦀️component.rs` facet (`appChildDirs` already sanctions `🌉️wasm` as an app child — both `🎛️apps/🧊️3d/🌉️wasm/` and `🎛️apps/🖐️5d/🌉️wasm/` already exist with the identical "wasm-bindgen bridge lives at the app level, not the artifact engine" doc-comment rationale). `◻2d` was the one puzzle app missing this facet. Relocated to fill the missing symmetric slot:

- **Moved**: `🔨️modules/🎲️board-2d/🦀️component.rs` → `🎛️apps/◻2d/🌉️wasm/🦀️component.rs` (content byte-identical; zero internal Rust call sites referenced `crate::modules::board_2d::*` — confirmed by repo-wide grep before the move — so no source-level fixups were needed beyond the glue.rs mount).
- `📦️glue.rs`: removed the `//#region 🔨️Modules … //#endregion 🔨️Modules` block (was `pub mod modules { pub mod board_2d; }`); added `#[path = "../../🎛️apps/◻2d/🌉️wasm/🦀️component.rs"] pub mod wasm;` inside `pub mod puzzle2d { … }` under `pub mod apps`, mirroring the existing `puzzle3d`/`puzzle5d` `wasm` mounts exactly.
- Deleted the now-empty `🔨️modules/` directory tree.

### `🧫️fixtures/` (4 files, plugin root)

Investigated rather than blind-moved per census's `UNVERIFIED` flag. Finding: these `🛂️manifest.json<descriptor>.manifest.json` files are graph-manifest codegen *sources* for `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts`'s `findManifestFiles()` — a repo-root-relative, filename-prefix-only, recursive walker (its own doc-comment: *"the `🗿️artifacts/<component>/🛂️manifest.json` taxonomy … sits directly under the component's own artifact folder with no 'manifest'-named parent directory at all … Matching on the filename prefix alone … needs no directory convention"*). This is a framework file I must not edit, but it is genuinely location-agnostic, so relocating the fixtures to the doc-blessed `🗿️artifacts/<kind>/` location requires **zero framework-side changes** and matches existing repo precedent (`✏️s/🔌️plugins/🌊️flow/🛂️manifest.json`, `💡️reasoning`, `✒️writer`, `🗒️note`, `📏️layout`, `🖍️draw` all already sit at a plugin/artifact root, same `"schema":"manifest"` shape, same codegen family).

Only **one** of the four fixtures is actually referenced anywhere in source (repo-wide grep for `include_str!` + filename, and for the fixture's `manifest.id` string): `◻2d`'s `default` fixture, consumed by a `#[cfg(test)]` cross-check in `board-host`. The other three (`◻2d` concrete-forest, `🖐️5d` default, `🧊️3d` default) have zero consumers anywhere in the repo (also zero consumers of their `puzzle3d-default`/`puzzle5d-default` manifest ids beyond the framework's own `math::graph::manifest::manifest_by_id` codegen family, which is filename-location-agnostic). No new wiring was invented for them (CLAUDE.md: don't fabricate example files) — they were relocated as-is, filenames unchanged:

- `🧫️fixtures/◻2d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/◻2d/🛂️manifest.jsondefault.manifest.json`
- `🧫️fixtures/◻2d/🛂️manifest.jsonconcrete-forest.manifest.json` → `🗿️artifacts/◻2d/🛂️manifest.jsonconcrete-forest.manifest.json`
- `🧫️fixtures/🖐️5d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/🖐️5d/🛂️manifest.jsondefault.manifest.json`
- `🧫️fixtures/🧊️3d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json`
- Deleted now-empty `🧫️fixtures/` tree.
- Updated the one real consumer: `🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🎲️board-host/🦀️component.rs` — `include_str!("../../../../../../🧫️fixtures/◻2d/🛂️manifest.jsondefault.manifest.json")` → `include_str!("../../../../🛂️manifest.jsondefault.manifest.json")` (4 `../` from `⚙️engine/🎲️board-host/` up to `🗿️artifacts/◻2d/`, matching the new location).

### Plugin root now contains exactly the six allowed entries
`AGENTS.md`, `README.md`, `🦀️component.rs`, `🎛️apps`, `🗿️artifacts`, `📦️packages` — verified in Step 6.

## Step 3 — escape-hatch call sites (same-plugin relocation)

Puzzle owns all three kinds it was mis-registering IO for (`2d.puzzle`, `3d.puzzle`, `5d.puzzle`), so per the ticket this is a same-plugin relocation of the *call site*, not new IO or a new artifact. Strategy chosen after reading each call site's dependency graph: relocating the whole `register_puzzleNd_exports` function bodies (and their callback fns) into the artifact engine risked dragging large private, file-local dependency graphs (`PUZZLE3D_MESH_REGISTRY`, `Puzzle3dFixture`, `resolve_object_mesh_url`, etc. for 3d; `Puzzle2dSnapshot`/stdio cross-plugin drawing bridge for 2d) across module boundaries for no architectural benefit — the rule under audit is about *where the OS-host `register_*` call site lives*, not where the callback body lives. So: the `semio_framework_os::register_*` call lines moved into the owning artifact's `⚙️engine`; each callback function they reference was widened from private `fn` to `pub(crate) fn` (still defined, still colocated with its app-local dependencies) and referenced via `crate::apps::<app>::<fn>`. `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]` gating was preserved exactly (same inner-block-of-an-unconditional-fn shape as before, per the plugin-specific instruction to preserve cfg semantics).

### `2d.puzzle` — `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/◻2d/🦀️component.rs`
- `puzzle2d_document_json_to_svg` (was line 1311) and `puzzle2d_document_json_from_dwg` (was line 1354): `fn` → `pub(crate) fn`.
- `register_puzzle2d_exports()` (was lines 1362-1369): removed the `register_2d_export_handlers`/`register_dwg_import_handler` block; now only calls `register_document_codec_for_app` (a `semio_framework_plugin` SDK call, not the OS-host family — left in place, out of scope for this audit).
- New home: `🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`, new `fn register_media_io()` in the `//#region 🔖️Register` region, called from `register()` right after `crate::apps::puzzle2d::register_puzzle2d_exports();`.

### `3d.puzzle` — `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🧊️3d/🦀️component.rs`
- `puzzle3d_mesh_from_document` (was line 2641) and `puzzle3d_document_from_mesh` (was line 2674): `fn` → `pub(crate) fn`. (Their own private helpers `glb_frame_correct`/`quat_rotate_point` stay untouched — used only by `puzzle3d_mesh_from_document`, which stays in the app file.)
- `register_puzzle3d_exports()` (was lines 2682-2695): removed the 8-call `register_mesh_*` block; now only calls `register_document_codec_for_app`.
- New home: `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`'s **pre-existing but previously-dead** `pub fn register_io()` (`//#region 🔖️IoFacet`) — extended with the 8 relocated calls. This function was never called from anywhere before this change (confirmed by repo-wide grep); it is now wired into the plugin boot sequence (see below), which both satisfies the architecture rule and fixes a latent dead-registration bug.

### `5d.puzzle` — `✏️s/🔌️plugins/🧩️puzzle/🎛️apps/🖐️5d/🦀️component.rs`
- `puzzle5d_document_from_mesh` (was line 1992): `fn` → `pub(crate) fn`. (The three exporter/dwg-export calls used an inline closure, not a named fn — moved verbatim, no visibility change needed.)
- `register_puzzle5d_exports()` (was lines 2000-2013): removed the 8-call `register_mesh_*` block; now only calls `register_document_codec_for_app`.
- New home: `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`'s pre-existing dead `pub fn register_io()` — same treatment as 3d.

### Boot-sequence wiring
`🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs::register()` is the single plugin-wide `setup:` hook (`Plugin::builder("puzzle").setup(crate::artifacts::puzzle2d::engine::register)` in root `🦀️component.rs`) — it already fanned out to all three apps' `register_puzzleNd_exports()`. Added two new calls so the relocated 3d/5d IO registration actually executes:
```rust
crate::apps::puzzle3d::register_puzzle3d_exports();
crate::artifacts::puzzle3d::engine::register_io();      // NEW
crate::apps::puzzle5d::register_puzzle5d_exports();
crate::artifacts::puzzle5d::engine::register_io();       // NEW
```
Without this, the relocated mesh registration for 3d/5d would have gone from "wrong layer, executes" to "right layer, never executes" — a regression this report needed to avoid, not just a taxonomy fix.

## Step 4 — dependency purge

`semio-framework-os` (Cargo.toml line 91, package `semio-framework-os`) is **retained** — it is still the only source of the mesh/2d-export registrars, and every call site that used it is still inside this crate, just moved from an app file to an artifact-engine file. Nothing to purge; no `## sharedFileRequests` needed for this.

## Step 5 — inventory only (no edits)

### `thread_local!` / interior-mutable app state — the Draft-lane debt

Per the binding prohibition, **nothing below was touched**; this is inventory for the SMO-reviewed Draft-lane wave.

**`🎛️apps/🧊️3d/🦀️component.rs:1876`** — `thread_local! { static PUZZLE3D_PLAY_SESSION: RefCell<Puzzle3dPlayApp> }`, `struct Puzzle3dPlayApp` fields:

| field | genuine draft state? | rationale |
|---|---|---|
| `precompute: RefCell<Puzzle3dPrecomputeSession>` | **No — derived** | precompute/gumball geometry engine, rebuilt from the projection each call |
| `transform_drag_active: RefCell<bool>` | **Yes** | genuine user-gesture flag: is a gumball drag in progress |
| `transform_base: RefCell<Option<Puzzle3dFixture>>` | **Yes** | genuine draft: fixture snapshot at drag start |
| `transform_scratch: RefCell<Option<Puzzle3dFixture>>` | **Yes** | genuine draft: in-progress scratch fixture accumulating drag deltas |
| `preview_seq: RefCell<u64>` | No — derived | monotone re-render counter, not content |
| `fill_display_memo: Mutex<Option<FillDisplayMemo>>` | No — derived | fingerprint-keyed memo cache |
| `geometry_cache: Mutex<Option<(u64,String,String)>>` | No — derived | fingerprint-keyed memo cache |
| `document_sections_cache: Mutex<Option<(u64,Vec<UiTreeSectionNode>)>>` | No — derived | fingerprint-keyed memo cache |

Proposed (report-only, not authored) `Draft` snapshot: `{ base: Puzzle3dFixture, scratch: Puzzle3dFixture }`, gated by a `bound: bool`/session-presence check replacing `transform_drag_active`. Proposed verb-slugs from the closed table, mapped from the existing action names in `transform_drag_tick`/`commit_transform`: `bind-transform` (session start, was `begin_transform_session`), `drag-transform` (was `"translateSelection"`), `rotate-transform` (was `"rotateSelection"`), `scale-transform` (was `"scaleSelection"`), `unbind-transform` (was `clear_transform_session`, cancel path). The final `commit_transform` fold-back into a real emitted operation is ordinary mutation-pipeline work, not itself a draft verb. The four derived-cache fields (`precompute`, `fill_display_memo`, `geometry_cache`, `document_sections_cache`) and the `preview_seq` counter are explicitly **not** draft state — they belong in an inference/derived-cache facet, per the ruling that a derived cache is never draft state.

**`🎛️apps/🖐️5d/🦀️component.rs:1295`** — `thread_local! { static PUZZLE5D_PLAY_SESSION: RefCell<Puzzle5dPlayApp> }`, `struct Puzzle5dPlayApp` fields:

| field | genuine draft state? | rationale |
|---|---|---|
| `precompute: RefCell<Puzzle5dPrecomputeSession>` | No — derived | precompute engine, rebuilt from the envelope |
| `registered_mesh_urls: RefCell<HashSet<String>>` | No — derived | memoized "which mesh URLs are already registered into `precompute`" set |

The struct's own doc-comment states this explicitly: *"Owns the precompute engine and the registered-mesh cache — both per-call scratch, never VCS-tracked; … the ephemeral view state lives in the wrapping store's real, VCS-tracked `Puzzle5dConfig` artifact."* **No genuine draft-state fields found** — this thread_local is entirely a derived-cache lane. No `Draft` snapshot/verb proposal applies; the pre-existing `Puzzle5dConfig`/`Emit` mutation pipeline already carries the real user-gesture state (brush placement, selection, camera) outside the thread_local, which is presumably why 5d never grew a gumball-style drag scratch the way 3d did.

### Other Step-5 inventory items
- `std::fs::`/`std::env::`/`Command::new` outside `#[cfg(test)]`: only in `📦️packages/🦀️rust/build.rs` (compile-time build-script IO — `CARGO_MANIFEST_DIR`/`OUT_DIR` reads, icon-asset copy — not runtime app impurity; out of scope for this inventory).
- `fn seed(`: zero matches anywhere in the plugin.
- Network calls (`reqwest`/`hyper`/`TcpStream`) outside test: zero matches.

## Step 6 — verify

1. `cargo check -p semio-s-plugin-puzzle` (scoped `CARGO_TARGET_DIR`): **queued behind the shared build lock for the full duration of this wave's edits** — every attempt printed only `Blocking waiting for file lock on build directory` for 10+ minutes with 3 other concurrent `cargo check -p semio-s-plugin-puzzle` processes and ~10 other plugins' checks running simultaneously (`ps aux` snapshot taken mid-wave). <REPLACE_WITH_REAL_TAIL_ONCE_UNBLOCKED>
2. `cargo test -p semio-s-plugin-puzzle --lib`: <REPLACE_WITH_REAL_OUTPUT_ONCE_CHECK_PASSES>
3. `bun nx run @semio-tech/puzzle-plugin:test-quick` — this target **does exist** exactly as named (confirmed in `📦️packages/🦀️rust/📋️project.json`; it shells out to `bun ./📜️script.ts test quick` → `runCargoTestBudgeted(["semio-s-plugin-puzzle"], repoRoot)`, i.e. the crate-scoped cargo test, not workspace-wide). <REPLACE_WITH_REAL_OUTPUT_IF_RUN>
4. `ls -a "✏️s/🔌️plugins/🧩️puzzle/"` → `. .. AGENTS.md README.md 🦀️component.rs 🎛️apps 📦️packages 🗿️artifacts` — **six allowed entries, confirmed**.

## Files touched

**Created**: `🎛️apps/◻2d/🌉️wasm/🦀️component.rs` (moved content, see below).

**Moved** (content unchanged except the one `include_str!` path):
- `🔨️modules/🎲️board-2d/🦀️component.rs` → `🎛️apps/◻2d/🌉️wasm/🦀️component.rs`
- `🧫️fixtures/◻2d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/◻2d/🛂️manifest.jsondefault.manifest.json`
- `🧫️fixtures/◻2d/🛂️manifest.jsonconcrete-forest.manifest.json` → `🗿️artifacts/◻2d/🛂️manifest.jsonconcrete-forest.manifest.json`
- `🧫️fixtures/🖐️5d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/🖐️5d/🛂️manifest.jsondefault.manifest.json`
- `🧫️fixtures/🧊️3d/🛂️manifest.jsondefault.manifest.json` → `🗿️artifacts/🧊️3d/🛂️manifest.jsondefault.manifest.json`

**Updated**:
- `📦️packages/🦀️rust/📦️glue.rs` — removed `🔨️Modules` region; added `puzzle2d::wasm` mount
- `🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🎲️board-host/🦀️component.rs` — `include_str!` path fix
- `🗿️artifacts/◻2d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — `register()` wiring + new `register_media_io()`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — extended `register_io()`
- `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` — extended `register_io()`
- `🎛️apps/◻2d/🦀️component.rs` — visibility + `register_puzzle2d_exports()` shrink
- `🎛️apps/🧊️3d/🦀️component.rs` — visibility + `register_puzzle3d_exports()` shrink
- `🎛️apps/🖐️5d/🦀️component.rs` — visibility + `register_puzzle5d_exports()` shrink

**Removed**:
- `🛂️manifest/🦀️component.rs`, `🎟️capabilities/🦀️component.rs`, `🔧️setup/🦀️component.rs` (+ their now-empty directories)
- `🔨️modules/` (now-empty after the move)
- `🧫️fixtures/` (now-empty after the move)
- `.DS_Store`

## sharedFileRequests

None. `🔣️taxonomy.json`'s `pluginChildDirs` flip is explicitly reserved as "the LAST thing APA does" per `📌️important.md` — not requested here.

## Concurrent-churn observations

Extremely heavy concurrent load on the shared `CARGO_TARGET_DIR` for this ticket at the time of this wave: `ps aux` showed simultaneous `cargo check` invocations for `semio-s-plugin-puzzle` (×3, including one from an apparently earlier/parallel attempt at this same plugin), `semio-s-plugin-space`, `semio-s-plugin-energy`, `semio-s-plugin-dag` (×2), `semio-s-plugin-raster`, `semio-s-plugin-sourcing`, `semio-s-plugin-playbook`, `semio-framework-plugin`, plus a bare `cargo check --workspace`. Every `cargo check -p semio-s-plugin-puzzle` invocation from this session sat at `Blocking waiting for file lock on build directory` for the whole edit window with zero further output — per `📌️important.md` this is documented as normal lock serialization, not a build failure, so it was not killed or worked around with a bare/unscoped check.

## apa-status: partial

Edits for this wave (steps 1-5) are complete and self-consistent by inspection and by grep-verification of every call site, visibility change, and path update described above. **Step 6 verification could not be completed within this session** — the scoped `cargo check -p semio-s-plugin-puzzle` never returned before this report was written, due to sustained build-lock contention from many concurrent sessions sharing this ticket's `CARGO_TARGET_DIR`. This report explicitly does **not** claim the build passes; a follow-up wave (or this session resumed) must re-run Step 6 verification, paste the real output in place of the `<REPLACE_WITH_REAL_…>` markers above, and only then can this plugin's APA migration be called done. If `cargo check` surfaces a real compile error (not lock contention) when it finally runs, the most likely fault points given the edits above are: the `pub(crate)` visibility changes not reaching `crate::apps::<app>::<fn>` through the `pub use component::*;` glob re-export in `📦️glue.rs`, or a stray reference to the deleted `crate::modules::board_2d` path that this session's greps missed.
