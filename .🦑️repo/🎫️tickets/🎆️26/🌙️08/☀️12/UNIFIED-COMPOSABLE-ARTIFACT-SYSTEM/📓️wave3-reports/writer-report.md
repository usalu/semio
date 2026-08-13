# W3 — `writer` composes stdio `document`

**ucas-status: complete — 100/100 tests passing, 0 compile errors, no open gaps of my own; 3 pre-existing baseline bugs fixed along the way (documented below, independently confirmed via `cargo check` before any edit and via git history)**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-writer --all-targets` was run BEFORE touching any file, per this ticket's verify-before-declaring-done discipline. It was already red: **16 pre-existing compile errors**, unrelated to composition:
- `WriterMutation::SetText`/`WriterMutation::SetSnapshot` referenced from the app layer (10 errors) — no such variants existed on the artifact-level `WriterMutation` enum (only `RenameWriter`/`ChangeUri`/`ChangeLanguage`/`EditText`).
- `WRITER_PLAY_BODY_DOCUMENT` unresolved import (renamed to `WRITER_PLAY_BODY_ARTIFACT` at some point, one call site never updated).
- `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot::PageDoc`/`PdfSnapshot` unresolved — the top-level `pdf::schema::snapshot::PdfSnapshot` re-export now points at the 1.7 subset (`pages: Vec<PdfPage>`), while writer's own pdf 1.4 serializer still assumed the frozen 1.4 `PageDoc` shape (caused by ticket `26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE`'s reorg of stdio's pdf artifact root, per that file's own doc comment — confirmed unrelated to this ticket).
- `DocxDocument.paragraphs` field doesn't exist (docx's schema moved to `body: Vec<DocxBlock>` + `styles: Vec<DocxStyle>`).

All four bug classes were fixed as part of this pass, since they blocked `cargo check` entirely and three of them (`SetText`/`SetSnapshot`, pdf, docx) sit inside files this migration was already rewriting for the composition change. See `## Pre-existing bugs fixed` below.

## What changed

### Snapshot / composed child

`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `WriterSnapshot.text: String` → `document: WriterDocumentChild` (`store::ArtifactChild<SemioDocumentSnapshot>`), `#[child(kind = "s.stdio.semio.document")]`.
- Dropped `dsl::DslRecord` derive (no `DslField` impl for `ArtifactChild<S>`, same wall cad/lowpoly hit) — hand-rolled `ArtifactDsl`/`ArtifactPack` added: `🔖️ChildCodecPrimitives` (hex/bracket handle codec, mirrors `📐️cad`'s), `🔖️TextPrimitives`/`🔖️BinaryPrimitives` (one `key=hex` line per field / LEB128 binary), `🔖️HandcraftedArtifactCodecs`.

`✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️component.rs` — new regions (mirrors `CadWorkingScene`'s home):
- `🔖️DocumentBridge`: `WriterDocumentChild` type alias; **real bidirectional converter** `document_snapshot_from_text`/`text_from_document_snapshot` (writer's whole authored body ↔ one `DocBlock::Code { language, text }` leaf — lossless, since `Code` carries no run/formatting structure to lose, exactly matching what the old `text: String` field carried verbatim); `document_child_handle` (content-addressed, mirrors `mesh_child_handle`/`cad_model_child_handle`).
- `🔖️WorkingScene`: `WriterWorkingScene { text, language_id }` + a `thread_local!` `WRITER_SCRATCH: RefCell<HashMap<child_id, text>>` cache — **never persisted**, matches the `EngineRep` contract (wholly derived, droppable, rebuilt). `writer_text(&WriterSnapshot) -> String` is the one read call site every render/inference/export path in the plugin now uses instead of the old `.text` field access. `document_child_handle_and_cache`/`writer_snapshot_with_text` are the standard mint-and-cache / fixture-builder helpers every mutation-diff and fixture in the plugin now goes through.

`WriterArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical field swap (`text` → `document`) so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent.

`WriterDiff` (`🔺️diff/🦀️component.rs`): `text: Option<WriterTextDelta>` → `document: Option<WriterDocumentChild>` (single-Option — the slot is never absent, only ever replaced, unlike lowpoly's `Option<Option<…>>` optional-slot pattern). `WriterTextDelta`/`WriterTextRangeEdit` deleted (dead — no byte-range edits on an opaque content-addressed handle). `🔺️diff/📝️text/🦀️component.rs`'s `apply`/`absorb` updated to whole-handle replace; `diff_set_text(text, id, language_id)` mints+caches a real handle (no apply-then-capture); `diff_text_range_edit` removed.

### Mutation vocabulary

No new mutation triads were added — `SetSnapshot` is explicitly **banned** (`📌️important.md`'s "Forbidden vocabulary": whole-document replace is not an in-history mutation). `EditText` (pre-existing, real, tested) is now writer's one and only content mutation; the app-layer bug where `TextEdit`/`SetText`/`FormatDocument`/`CommitRename`/engagement handlers all referenced the non-existent `WriterMutation::SetText` was fixed by routing them onto the real `EditText` variant. `🧬️mutations/✏️edit-text/{🔺️diff,↩️inverse}` updated: diff mints the handle via `base.id`/`base.language_id`; inverse reads `writer_text(base)` (cache lookup) instead of `base.text`.

### Whole-document replace → `reset_document_effect`

`ArtifactApp::whole_document_operation` override removed (trait default `None`) — matches `📐️cad`'s identical ruling. New `reset_document_effect(&WriterSnapshot) -> HostEffect` in `🎛️apps/✒️writer/🦀️component.rs` (`store::create_document_envelope` + `store::print_document_spr`, a fresh edit-free envelope wrapped in `HostEffect::LoadDocument`). Every former "replace the whole document" app command (`SetSnapshot`, `OpenDocument`, `SetSnapshotJson`, `SetFixtureJson`, `SetActiveExample`) now emits this effect instead of a banned mutation. `SetSnapshot`'s payload changed from a nested `#[dsl(block)] snapshot: WriterSnapshot` to `json: String` (`WriterSnapshot` no longer implements `dsl::DslField`, the same wall the top-level snapshot hit) — functionally identical to the pre-existing `SetSnapshotJson`, kept as its own wire row rather than deleting it mid-ticket. Added `semio-framework = { path = "…/🧰️framework/📦️packages/🦀️rust" }` to writer's `Cargo.toml` (writer had no path to `HostEffect` before; cad/lowpoly/sourcing/process already depend on it the same way).

### Read-side rewiring (`writer_text`)

~15 call sites across `🎭️modes/✏️edit/🪟️windows/✒️main`, `🎮️commands/{✍️text,💬️engagement,🗂️selection}`, `📌️panels/{📄️artifact,🔍️inspection}`, `🧬️schema/💡️inferences/{🦀️component.rs,🧾outline}`, and `apps/writer/🦀️component.rs`'s `editor_hover_context`/`writer_chapter_payload` switched from `document.text` to `writer_text(document)`.

### PDF/DOCX serializers (writer's own, fixed to match stdio's current shape)

`🚪️io/{📤️export,📥️import}/…/📄️pdf/🔖️1.4` now import the frozen 1.4 subset explicitly (`standards::v1_4::subsets::any::schema::snapshot::{PdfSnapshot, PageDoc}`) instead of the ambiguous top-level re-export (now 1.7). `…/📜️docx/🔖️ecma-376` rebuilt against `DocxDocument.body: Vec<DocxBlock>` (paragraph-per-line via `DocxBlock::paragraph`, non-`Paragraph` blocks honestly skipped on import rather than fabricating text).

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` and `🗣️dag-example.dsl.semio` were in the pre-migration fenced-code-block grammar (`text=```jack\n...\n````), incompatible with the new hand-rolled `document=[childId,target]` codec. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that built the real jack/dag-jack `WriterSnapshot`s via `writer_snapshot_with_text` and dumped real `print_dsl()` output (`cargo test … debug_fixture_regen -- --nocapture`), captured, written as the new fixtures, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

Because the new fixture format only carries the opaque `document=[childId,target]` handle (content-addressed, like every other composed-child DSL fixture in this ticket — cad's `shapeModel=`/lowpoly's `mesh=` lines are equally opaque), `jack_example_document()`/`dag_jack_example_document()` were extended to re-seed the working-scene cache with the real query text (`JACK_QUERY_TEXT`/`DAG_JACK_QUERY_TEXT` constants) after parsing — otherwise `writer_text()` would read back empty for a freshly-parsed fixture (the cache has no way to recover text from a bare hash). This is the same class of gap `WriterWorkingScene`'s doc comment already documents, applied honestly rather than left to silently regress the demo/example content to empty.

## Pre-existing bugs fixed (independently traced, not this ticket's design work but blocking compile)

1. **`WriterMutation::SetText`/`SetSnapshot` missing variants** — fixed by routing `TextEdit`/`SetText`/`FormatDocument`/`CommitRename`/engagement onto the real `EditText` variant (see above); `SetSnapshot` command retargeted onto `reset_document_effect` per the (independently, already-binding) `SetSnapshot`-mutation ban.
2. **`WRITER_PLAY_BODY_DOCUMENT` → `WRITER_PLAY_BODY_ARTIFACT`** — one stale test import (`🎮️commands/🗂️selection/🦀️component.rs`).
3. **pdf 1.4 / docx schema drift** — writer's own serializers updated to the current stdio shapes (see above); no stdio file touched.

These are NOT part of this ticket's composition work but were blocking `cargo check` entirely (nothing compiles, including my own changes, until fixed) — fixed directly rather than deferred, per the ticket's "fix every compile error, don't stop with unresolved errors" instruction. None required touching `✏️s/🔌️plugins/🗄️stdio/**`.

## Working-scene design

See `WriterWorkingScene`'s own doc comment (`🗿️artifacts/✒️writer/🦀️component.rs`, `🔖️WorkingScene` region) for the full rationale. Summary: a `thread_local!` `HashMap<child_id, text>` cache, exactly `LowpolyScratch.mesh_workspace`'s pattern — never persisted, populated at mutation-diff-build time (not apply time, since only the builder has the literal text) and at fixture-construction time. Same documented staleness gap as lowpoly's `StaleMeshWorkspace`: store-level undo/redo bypasses `ArtifactApp::handle` entirely, so a handle can in principle go uncached; `writer_text`/`writer_text_for_handle` fail soft (empty string), never panic. A real fix needs child-document resolution, which — checked directly against `🔌️plugin/🦀️component.rs` — no WASM-guest plugin in this repo has yet.

## Converter (real, not a stub)

`document_snapshot_from_text`/`text_from_document_snapshot` (`🗿️artifacts/✒️writer/🦀️component.rs`, `🔖️DocumentBridge` region) — writer's raw text buffer maps to exactly one `DocBlock::Code { language, text }` leaf in the composed `SemioDocumentSnapshot`'s block tree; the inverse concatenates every `Code` block's body (honestly skips non-`Code` blocks rather than fabricating text — writer never authors those). This is a real, lossless, tested round trip (`diff_set_text_mints_a_document_handle_and_caches_its_text`), not a placeholder.

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only for this ticket) before writing the working-scene cache, matching what cad's and lowpoly's reports already found. Out of scope for a plugin-scoped agent, per this ticket's own ruling on both prior exemplars.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-writer --all-targets
```
**0 errors** (down from 16 pre-existing), confirmed on two consecutive clean runs. Remaining warnings are pre-existing/cosmetic (unused imports in `🚪️io/🦀️component.rs`, elided-lifetime/idiom lints, `testkit` glob-import ambiguity between `os_spr`/`os_pack` in framework glue, a duplicate-symbol linker warning between this crate's test binary and `semio-s-plugin-trinity` — none block compilation or test execution, none touched by this pass).

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-writer --no-fail-fast
```
**100 run: 100 passed, 0 skipped.** Reproduced stable across two consecutive full runs (not flaky). No test was deleted to make this pass; `text_range_edit_honestly_patches_substring` (byte-range-edit law, retired capability) was replaced with `diff_set_text_mints_a_document_handle_and_caches_its_text` (the equivalent real-behavior law for the new content model), and `whole_document_operation_replaces_the_snapshot` was replaced with `whole_document_operation_stays_the_trait_default_none` (asserts the now-correct, banned-`SetSnapshot` behavior) — both documented in-place with the reason.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/✒️writer/**` (including its own `📦️packages/🦀️rust/Cargo.toml`, which is this plugin's own file, not the excluded `📦️glue.rs`/`📦️index.ts`). No `🗄️stdio/**` file was read-written — only read for reference (`SemioDocumentSnapshot`/`DocBlock` schema, `PdfSnapshot`/`PageDoc`/`DocxDocument` current shapes).

## Concurrent-churn observations

None encountered — `semio-s-plugin-stdio` was green throughout this pass's `cargo check`/`nextest` runs (no retries needed, unlike lowpoly/cad's reports during their sessions).

## Files touched this pass

- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🦀️component.rs` — `WriterDocumentChild`, `document_snapshot_from_text`/`text_from_document_snapshot`, `document_child_handle`, `WriterWorkingScene`, `WRITER_SCRATCH`, `writer_text`/`writer_text_for_handle`, `document_child_handle_and_cache`, `writer_snapshot_with_text`.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `WriterSnapshot` field swap, hand-rolled codecs.
- `…/🧬️schema/🦀️component.rs` — `WriterArtifact` field swap, `empty_writer_snapshot()`.
- `…/🧬️schema/🔺️diff/🦀️component.rs`, `…/🔺️diff/📝️text/🦀️component.rs` — `WriterDiff.document`, apply/absorb, `diff_set_text`, test fixes.
- `…/🧬️schema/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/✏️edit-text/{🔺️diff,↩️inverse}/🦀️component.rs`, `…/🧬️mutations/{💾️binary,📝️text}/🦀️component.rs` — mutation-side rewiring + test fixes.
- `…/🧬️schema/📸️snapshot/{💾️binary,📝️text}/🦀️component.rs` — fixture regen, test fixes, `JACK_QUERY_TEXT`/`DAG_JACK_QUERY_TEXT`.
- `…/🧬️schema/💡️inferences/{🦀️component.rs,🧾outline/🦀️component.rs}` — `writer_text` rewiring, outline test fixes.
- `…/🚪️io/{📤️export,📥️import}/…/{📄️pdf/🔖️1.4,📜️docx/🔖️ecma-376}/✳️any/🦀️component.rs` (4 files) — stdio-shape fixes.
- `…/📚️examples/🎬️demo/🖼️assets/{🗣️example.dsl.semio,🗣️dag-example.dsl.semio}` — regenerated fixtures.
- `🎛️apps/✒️writer/🦀️component.rs` — `reset_document_effect`, `whole_document_operation` removal, `writer_chapter_payload`, `editor_hover_context`, `app_with_jack` (loads via `load_document_pack` instead of a now-nonexistent in-history `SetSnapshot`), test fixes.
- `🎛️apps/✒️writer/🎮️commands/{✍️text,💬️engagement,🔍️inspect,🗂️selection}/🦀️component.rs` — `EditText` rewiring, `reset_document_effect` call sites, `writer_text` rewiring, test fixes (including `WRITER_PLAY_BODY_DOCUMENT` → `WRITER_PLAY_BODY_ARTIFACT`).
- `🎛️apps/✒️writer/📌️panels/{📄️artifact,🔍️inspection}/🦀️component.rs` — `writer_text` rewiring.
- `🎛️apps/✒️writer/🎭️modes/✏️edit/🪟️windows/✒️main/🦀️component.rs` — `writer_text` rewiring.
- `📦️packages/🦀️rust/Cargo.toml` — added `semio-framework` dependency (for `HostEffect`).

ucas-status: complete
