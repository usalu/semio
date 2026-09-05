# Fix panel + generate-mode window tests (async testkit)

Scope: the six panel/generate-mode-window files under
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`.
Each had a `#[cfg(test)] mod tests` calling the crate's now-`async` testkit
(`app()`, `render()`) with a plain `#[test] fn`. Converted every one to the
`#[semio_framework_async_macros::async_test]` / `async fn` form used elsewhere
in `…/✳️any/✏️editor/🦀️.rs`, awaiting every harness call. No assertions were
weakened, removed, or added. `test_support::lock()` calls were left
untouched (still sync, called with no `.await`), matching the harness's own
async tests. `app()` vs `app_with_registry()` choice was left as-is in all
six files (all already used `app()`; none needed `app_with_registry()` to
compile).

## Changes

1. `🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs` (line ~45)
   - `generate_mode_renders_surfaces`
   - `#[test] fn` → `#[semio_framework_async_macros::async_test] async fn`
   - `app()` → `app().await`
   - `render_body(&mut app, ...).contains(...)` → `render_body(&mut app, ...).await.contains(...)`

2. `🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` (line ~83)
   - `generate_preview_hints_without_evaluated_output`
   - same conversion pattern as above.

3. `🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs` (line ~52)
   - `generate_form_hints_without_a_selected_generation`
   - same conversion pattern as above.

4. `📌️panels/🗿️artifact/🦀️.rs` (line ~54)
   - `document_lists_widgets`
   - `#[test] fn` → `#[semio_framework_async_macros::async_test] async fn`
   - `app()` → `app().await`
   - `render_body(&mut app, GENERATION_3D_PLAY_BODY_DOCUMENT)` → `render_body(&mut app, GENERATION_3D_PLAY_BODY_DOCUMENT).await`
   - `app.snapshot().expect("snapshot")` left untouched (sync accessor via
     `PluginApp` trait, called after the `render_body` await completes —
     ordering and borrow shape unchanged).

5. `📌️panels/🔍️inspection/🦀️.rs` (line ~114)
   - `inspector_shows_no_selection_by_default`
   - same conversion pattern as #1-3; `test_support::lock()` left sync.

6. `📌️panels/🛍️catalogue/🦀️.rs` (line ~53)
   - `generation3d_labels_resolve_native_english_by_default`
   - same conversion pattern; `test_support::lock()` left sync.

All six diffs are minimal (attribute line, `async fn`, `.await` on `app()`
and on `render(...)` calls only) — verified via `git diff` after editing.

## Noted but not fixed (other compile blockers spotted in these six files)

None found. All six test modules, after this conversion, use only:
- `super::*` symbols already imported by the surrounding file,
- `crate::editor::generation3d::testkit::{app, render as render_body}`,
- `crate::editor::generation3d::test_support::lock()` (panels only),
- and, in the artifact panel, `semio_framework_plugin::PluginApp` for the
  sync `app.snapshot()` accessor.

No serde-on-`ToValue`/`FromValue` usage, no mutation-constructor-as-enum-variant
calls, no `protocol::testkit` helper references, and no direct
`VcsArtifactApp` construction appear in any of these six test modules — those
issues (if present) live elsewhere (e.g. the edit-mode window files owned by
the other agent, or the harness file itself).

## Not touched (out of lease)

- `…/✳️any/✏️editor/🦀️.rs` (harness/testkit source of truth — read only).
- `✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` and
  `.../👁️preview/🦀️.rs` — owned by another agent this session.
