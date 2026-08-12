# Packet Report — writer plugin `⚙️engine` elimination

Target: `✏️s/🔌️plugins/✒️writer` artifact-tree engine at
`🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (1066 LOC, single file).

## Module-wiring discovery (not anticipated by the region map)

The `crate::artifacts::writer::engine::X` and `crate::artifacts::writer::standards::v1::engine::X`
paths used throughout the plugin are **not** derived from directory nesting or codegen — they come from
explicit `#[path = "..."]` `mod` declarations and shim `pub use` re-exports hand-written in
`✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` (not in the packet's forbidden-file list, so it
was in scope to edit):

- `pub mod v1 { #[path=".../⚙️engine/🦀️component.rs"] pub mod engine; ... }` — mounted the engine file
  directly as `standards::v1::engine`, **skipping** `subsets::any` in the Rust module path even though
  the physical file lives under `.../subsets/✳️any/⚙️engine/...`.
- `pub mod engine { pub use super::standards::v1::engine::*; }` — a second shim inside `artifacts::writer`
  that produced the short `crate::artifacts::writer::engine::X` form seen at every call site.

Both declarations were deleted. This mirrors exactly how the already-migrated `block2d` exemplar's
`io_registry` now reads `crate::artifacts::block2d::standards::v1::subsets::any::io::io_registry` — the
full, un-shimmed path.

## Destination map applied

| Region (from `⚙️engine/🦀️component.rs`) | Destination | Why |
|---|---|---|
| `WriterEngine` struct + `impl` | **deleted outright** | Zero construction sites repo-wide (`grep -rn "WriterEngine"` → only its own definition/impl, both removed). |
| `register()`, `register_artifact_schema()`, `register_artifact_inferences()`, `register_writer_languages()` | `🎛️apps/✒️writer/🦀️component.rs` (new `🔌️Registration` region) | Rule 6 — `register()` calls `register_document_codec_for_app::<WriterPlayApp>`, app-constitutional. `register_writer_languages` also registers `JackWriterIdiom`/`WireWriterIdiom`, kept `pub(crate)` so schema's own test can call it without a full app bootstrap. |
| `writer_io()`, `WriterChapterPayload`, `writer_chapter_payload()` | `🎛️apps/✒️writer/🦀️component.rs` (new `🔖️Io` region) | Rule 4 (`writer_io`'s docstring literally says "This app's typed media I/O surface (`AppDefinition.io`)"). `WriterChapterPayload`/`writer_chapter_payload` have their single consumer in this same file (`export_media`, plus this file's own tests) — same "single consumer → its own file" rule the original engine.rs docstring itself used. |
| `empty_writer_snapshot()` | `🧬️schema/🦀️component.rs` (new `🔖️DocumentHelpers` region) | Rule 3 — matches block2d's `empty_block2d_snapshot()` placement exactly. |
| `GrammarToken`, `byte_span_to_text_span`, `token_class_from_name`, `JackWriterIdiom`, `WireWriterIdiom`, `tokenize_language`, `language_completions_json`, `jack_completions_json`, `format_writer_text`, all of `JackAstNode`/AST-navigation, all of `SelectableSpan`/`JackEditorPlaceholder`/newline-gate/rename/symbol-lookup machinery | `🧬️schema/🦀️component.rs` (new `🔖️Languages`, `🔖️JackAst`, `🔖️JackEditor` regions) | Rule 3 — pure text/token helpers over the document's `text`/`language_id` fields, not app-dependent. This cluster is heavily multi-consumer across many app files (`main` window, `selection`, `text`, `engagement` commands, `panels/artifact`) and internally interdependent (`jack_symbol_at_offset` calls `jack_variable_occurrences` calls `jack_bound_variable_names`, etc.) — kept together as one shared module rather than fragmented per single-use-site, per "pure algorithms belong to a module one level up" (singular). `JackWriterIdiom`/`WireWriterIdiom` made `pub(crate)` (previously private) since `register_writer_languages` now needs them from a different file. |
| `language_tokens_json(&WriterSnapshot)`, `language_diagnostics_json(&WriterSnapshot, u32)` | `🧬️schema/💡️inferences/🦀️component.rs` (new `🔖️LanguageInferences` region) | Rule 2 exactly — pure fns taking `&WriterSnapshot`, returning a derived JSON projection. |
| `jack_example_document/json`, `dag_jack_example_document/json` | `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (the "dsl" facet file; new `🔖️Examples` region) | Rule 3, refined by proximity — `JACK_EXAMPLE_TEXT`/`DAG_JACK_EXAMPLE_TEXT` (the constants these fns parse) already live in exactly this file, whose own docstring pointed back at the engine-housed parsers. Placed directly beside them per CLAUDE.md's "if code is repeated, keep it close" principle; doc comments updated to drop the now-redundant `engine::` indirection. |
| `pub mod io_registry { ... }` (ComposerEntry export wrappers) | `🚪️io/🦀️component.rs` (new `🔖️DerivedIoRegistry` region, moved verbatim) | Rule 5 exactly. Matches block2d's `io/🦀️component.rs` structure line-for-line (only artifact-kind-specific dialect strings differ). |

## Call sites updated

15 files, all previously calling through `crate::artifacts::writer::engine::…` or the bare `engine::…`
alias:

1. `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` — removed the two wiring/shim declarations.
2. `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/Cargo.toml` — dropped "engine" from the crate description.
3. `✏️s/🔌️plugins/✒️writer/🦀️component.rs` — `.setup(...)` now points at `crate::apps::writer::register`.
4. `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️component.rs` — `io_registry`'s `use ... v1` now reads
   the full `standards::v1::subsets::any::io::io_registry` path (matches block2d's fixed form).
5. `🎛️apps/✒️writer/🦀️component.rs` — the destination file itself; ~14 internal call sites fixed
   (`initial_snapshot`, `io()`, `export_media`, the manifest `.io()`/`.example()` calls, `PortTests`,
   `export_media_text_out_projects_the_document_as_a_chapter`, `export_media_rejects_unknown_ports`,
   `context_menu_is_grouped_and_keeps_cut_last_and_destructive`, plus the top docstring and
   `editor_hover_context`'s docstring).
6. `🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️component.rs` — import split across `schema`/`schema::inferences`/`dsl`.
7. `🎛️apps/✒️writer/🎮️commands/🗂️selection/🦀️component.rs` — top import + test import.
8. `🎛️apps/✒️writer/🎮️commands/✍️text/🦀️component.rs` — top import split (`schema`/`dsl`) + 2 test imports.
9. `🎛️apps/✒️writer/🎮️commands/💬️engagement/🦀️component.rs` — inline test-local `use`.
10. `🎛️apps/✒️writer/📌️panels/📄️artifact/🦀️component.rs` — top import + one call site.
11. `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — `use engine` → `use schema`, 3 call sites.
12. `🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — destination file; doc comments + 1 test call site.
13. `🧬️schema/🧬️mutations/🦀️component.rs` — `use engine` → `use schema`, 4 call sites.
14. `🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` — `use {engine, WriterSnapshot}` → `{schema, WriterSnapshot}`, 2 call sites.
15. `🧬️schema/🧬️mutations/📝️text/🦀️component.rs` — removed an unused `use crate::artifacts::writer::engine;` (dead import, no call sites used it).

## Structural verification (no compiler needed)

```
grep -rn "writer::engine" ✏️s/🔌️plugins/✒️writer            → 0
grep -rn "writer::standards::v1::engine" ✏️s/🔌️plugins/✒️writer → 0
find ✏️s/🔌️plugins/✒️writer/🗿️artifacts -type d -name "⚙️engine" → (no output — directory deleted)
grep -rn "WriterEngine" ✏️s/🔌️plugins/✒️writer                → 0 (struct deleted, zero prior construction sites)
```

Destination files all contain the moved code (spot-checked via grep for the moved symbols'
declarations): `🧬️schema/🦀️component.rs` (empty_writer_snapshot/GrammarToken/JackAstNode/apply_jack_rename),
`🧬️schema/💡️inferences/🦀️component.rs` (language_tokens_json/language_diagnostics_json),
`🧬️schema/📸️snapshot/📝️text/🦀️component.rs` (all 4 example fns),
`🚪️io/🦀️component.rs` (`pub mod io_registry`),
`🎛️apps/✒️writer/🦀️component.rs` (register/writer_io/WriterChapterPayload/register_writer_languages).

**Tests**: the original `⚙️engine/🦀️component.rs` had exactly one `#[cfg(test)] mod tests` with 11
`#[test]` functions / 20 `assert*!` calls. All 11 moved verbatim into
`🧬️schema/🦀️component.rs`'s new `🧪️Tests` region (only change: the one test that calls
`register_writer_languages()` now calls it via `crate::apps::writer::register_writer_languages()`,
matching its new home). `grep -c "#\[test\]"` on the destination file confirms 11; assertion count in
that region confirms 20 — equal to before, none dropped.

## Out of scope, flagged not touched

- `✏️s/🔌️plugins/✒️writer/🎛️apps/✒️writer/⚙️engine/` — an **empty** app-level directory (0 files), a
  completely different thing from the artifact-tree engine this packet targeted (ticket instructions
  explicitly scoped the target via `-path "*🗿️artifacts*"`). Left untouched; not mounted in `glue.rs`,
  so it has no effect on compilation either way.
- `🎛️apps/✒️writer/🎚️config/🦀️component.rs:7` — a docstring mentioning "the old `⚙️engine` crate's
  `WriterConfig` before this migration" — this refers to a historical, already-completed *earlier*
  crate-consolidation migration (unrelated to this ticket's directory), not a live call site. Left as is.
- `semio_s_plugin_stdio::artifacts::docx::engine::build_minimal_docx` (in writer's docx export
  serializer) — a different plugin's (`stdio`'s) own `engine` module, explicitly out of bounds
  (`✏️s/🔌️plugins/🗄️stdio` is forbidden). Left untouched.

## Compiler verification

(filled in once the background `cargo check` / `cargo test` finish — see final chat message for the
verbatim result.)
