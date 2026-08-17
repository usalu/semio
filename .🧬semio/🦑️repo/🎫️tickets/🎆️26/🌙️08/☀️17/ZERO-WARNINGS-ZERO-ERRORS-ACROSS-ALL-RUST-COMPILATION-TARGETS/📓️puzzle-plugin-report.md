# Puzzle Plugin (`semio-s-plugin-puzzle`) — Zero Warnings Report

## Scope
Crate `semio-s-plugin-puzzle`, rooted at `✏️s/🔌️plugins/🧩️puzzle/`. `(lib)` target only —
`(lib test)` is blocked by the workspace-wide `Mutation::apply`/`::diff` trait migration
(~151 pre-existing errors), confirmed out of scope per the parent ticket and left untouched.

## Result
- Starting warning count (`(lib)` target): **282**
- Ending warning count (`(lib)` target): **0**
- New errors introduced: **0** (`(lib)` target confirmed compiling clean via repeated
  `cargo check -p semio-s-plugin-puzzle` — puzzle no longer even appears in the warning summary
  once `cargo fix`'s mechanical fixes were applied and cached).

## What was done

### 1. Mechanical fixes via `cargo fix --lib -p semio-s-plugin-puzzle --allow-dirty --allow-staged`
Auto-applied 257 suggested fixes across 110 files: unused imports, unnecessary qualifications,
unused `extern crate` (`glue.rs`), unused-variable underscore-prefixing, unnecessary `mut`. This
took the count from 282 → 25. Verified via `git diff --stat` that only files under
`✏️s/🔌️plugins/🧩️puzzle/` were touched.

### 2. Hidden lifetime parameters (3 warnings) — real fix, not suppression
`ComposeSource<'a>` (defined in `🧰️framework/🔨️modules/🚪️io/🦀️component.rs:861`) was used
elided as `&[ComposeSource]` in three near-identical `compose()` fn signatures. Fixed by writing
the explicit lifetime: `&[ComposeSource<'_>]`. Files:
- `🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`

### 3. Unused imports of `ArtifactAnalyzer as _` (3 warnings)
Same three `🚪️io/🦀️component.rs` files each had an unused
`use semio_framework_plugin::ArtifactAnalyzer as _;` — the associated-fn call
`Puzzle{2,3,5}dAnalyzer::analyze(...)` resolves without it (confirmed real: rustc's unused-import
lint already type-checked without the import). Removed.

### 4. Unused doc comments (3 warnings) — real fix, not suppression
Doc comments (`///`) sitting directly above a `thread_local! { ... }` macro invocation or above
one in a `//#region` don't attach to anything and are dropped by rustc with a warning.
- `🗿️artifacts/🖐️5d/🦀️component.rs`: moved the `PUZZLE5D_KIND_CATALOGS_SCRATCH` doc comment
  *inside* the `thread_local! { }` block, directly above the `static`, where it now legitimately
  documents that item.
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs` and the `🖐️5d`
  sibling: the doc comment described the whole `//#region 🔖️PlayApp` region (not the
  `thread_local!` specifically), so converted `///` → `//` (plain comment) instead of relocating.

### 5. Dead code (16 `never used` warnings) — triaged per the established method, not deleted blindly
- **`example_fixture`** (`◻2d/…/✏️editor/🦀️component.rs`): zero call sites anywhere in the crate,
  including tests. Deleted.
- **`puzzle2d_document_json_from_dwg`**: only called from `#[test] fn
  dwg_import_returns_empty_board_with_no_camera_field` inside `mod tests` (`#[cfg(test)]`) — a
  plain `cargo check` doesn't compile `#[cfg(test)]` code, so it legitimately warned dead in this
  compilation despite being a real test fixture. Gated the function itself with `#[cfg(test)]`
  (idiomatic fix, not `#[allow]`), matching the established pptx/`demo_diff_cases` precedent.
- **`puzzle2d_snapshot_to_drawing` / `puzzle2d_document_json_to_svg`**: genuinely dead — the
  function's own doc comment admitted the `register_media_io` caller that was supposed to wire it
  up was already deleted in a prior ticket (`register_dwg_import_handler` call ... is DELETED, not
  migrated"), and grep confirmed zero real call sites (not even in tests). Deleted the whole
  `//#region 🔖️SvgBridge` block (~175 lines) and updated the adjacent `🔖️Register` region's doc
  comment, which had referenced the now-deleted function, to explain what happened.
- **`with_puzzle3d_app_mut`** (`🧊️3d/…/✏️editor/🦀️component.rs`): zero call sites anywhere;
  additionally its body was byte-identical to `with_puzzle3d_app` (used `.borrow()`, not
  `.borrow_mut()` — looks like an unfinished copy-paste that was never wired up). Deleted.
- **`glb_frame_correct` / `quat_rotate_point` / `puzzle3d_mesh_from_document` /
  `puzzle3d_document_from_mesh`** (same file): a self-contained dead chain — the first two are
  only called by `puzzle3d_mesh_from_document`, which (like the puzzle2d SVG bridge above) was
  meant to be reached via a `register_mesh_io()` OS-host registrar that does not exist anywhere in
  the crate (confirmed: `grep -rn "fn register_mesh_io"` returns nothing, and
  `🧩️puzzle/🦀️component.rs`'s `plugin()` builder chain has no `.setup()` call at all despite its
  own doc comment claiming "`.setup()` still survives for the OS media-host bridges"). Deleted all
  four functions and the dangling `register_mesh_io` doc-comment block that described them.
  Confirmed their shared helpers (`PUZZLE3D_MESH_REGISTRY`, `resolve_object_mesh_url`,
  `object_scale_json`, `mesh_from_kind`, `PUZZLE3D_FALLBACK_MESH_KIND`) are still used elsewhere
  in the file (line ~995) so nothing else went newly dead.
- **`puzzle5d_document_from_mesh`** (`🖐️5d/…/✏️editor/🦀️component.rs`): same shape as the
  puzzle3d mesh bridge above — its own `register_mesh_io()` doesn't exist in this crate either.
  Deleted the function and the dangling `//#region 🚪️OsHostMeshIo` doc-only block that described
  it (confirmed `empty_document()`, its only helper, is still used at 4 other call sites in the
  file).
- **`json_value_to_serde`** (import side) / **`serde_to_json_value`** (export side), duplicated
  across `◻2d`/`🧊️3d`/`🖐️5d`'s `🚪️io/…/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs` pairs (6
  functions total): each file's real `deserialize`/`serialize` path actually goes through stdio's
  own `JsonSnapshot::to_serde_value()`/`JsonSnapshot::from_value()`, not the hand-rolled
  recursive converter sitting next to it — same "hand-rolled codec superseded by the real
  bridge, but never deleted" shape already seen in pptx/gltf this session. Deleted all 6, along
  with the now-unused `JsonValue`/`JsonMember`/`FromStr` imports, and updated each file's module
  doc comment (`//!`) to describe the real code path instead of the deleted one.

## Left alone (explicitly out of scope)
- `(lib test)` target's ~151 `Mutation::apply`/`::diff` trait-signature errors — another session's
  in-flight cross-cutting migration, per parent ticket instructions.
- No `#[allow(...)]` was used anywhere.

## Files touched (all under `✏️s/🔌️plugins/🧩️puzzle/`)
116 files total (110 from `cargo fix`'s mechanical pass + 6 files with additional manual edits
beyond what `cargo fix` could do — the `🚪️io`, editor, and json-codec component files discussed
above). Full list: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/🧪️puzzle-touched-files.txt`.
