# 🔨 W2 — draw crate compile lane

Lane: W2 ("compile") of ticket `26/09/05/DRAW-PLUGIN-END-TO-END`.
Goal: `cargo check -p semio-s-plugin-draw --lib` → `--lib --tests` → `cargo test -p semio-s-plugin-draw --lib`, all green natively.
Target dir: `/Users/ueli/Documents/semio/target-draw-e2e` (shared with coordinator; cargo's lock serializes). `RUSTC_WRAPPER=""`, `CARGO_BUILD_JOBS=3`, no profile overrides.

## Log

- start: read `📓️status.md`, `📓️explore-compile-drift.md`, block precedent `w7a`.
- pre-check source fixes (before the first cargo run of this lane):
  1. **E0046 ×2 — `protocol::Mutation` missing `DESCRIPTORS`/`descriptor`.**
     - `…/✳️any/✏️editor/👥️presence/🦀️.rs:95` — added one `MutationLeafDescriptor` (`Snapshot`) + `descriptor()` match, shaped after `🧩️puzzle/…/◻️2d/…/👥️presence/🦀️.rs:98`.
     - `…/✳️any/✏️editor/🎚️config/🦀️.rs:186` — added six leaf descriptors (Snapshot, SetEngagementInput, SetCamera, SetActiveUtility, SetTracePointerProgress, SetLocale) + `descriptor()` match, shaped after `🧩️puzzle/…/◻️2d/…/🎚️config/🦀️.rs:335`. `owner` strings are provisional placeholders (no leaf dirs on disk), same as puzzle's.
  2. **E0053/E0407 — 6 more `async fn` on the sync `ArtifactEditor` trait** (W1's file `…/✳️any/✏️editor/🦀️.rs`, pure compile fix, minimal):
     `build_tool_job` (:1021), `app_schema` (:1047), `initial_snapshot` (:1051), `io` (:1055), `export_media` (:1062), `command_id` (:1082) → sync.
     Follow-on: `Self::io().await` → `Self::io()` (:1066); the 3 test call sites at `:1915/:1919/:1920` dropped their `semio_framework_plugin::resolve_ready(…)` wrapper (`resolve_ready` takes a `Future`; the fn is now sync).
     Trait oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` `ArtifactEditor` — only `command_from_intent` (:26840) and `media_ports` (:26921) are `async`; draw overrides neither.
  3. **`ArtifactViewer::initial_snapshot`** — `…/✳️any/👁️viewer/🦀️.rs:55` `async fn` → `fn`.
- verified statically: all `#[path]` + `include_str!/include_bytes!` in the crate's own mount tree resolve (0 missing) after the coordinator's rename passes.
