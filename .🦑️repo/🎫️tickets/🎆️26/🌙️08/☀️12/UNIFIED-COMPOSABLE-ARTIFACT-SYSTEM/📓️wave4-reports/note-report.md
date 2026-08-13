# W4 — `note` composes stdio `text` per text block, gains an `R:any` link slot

**ucas-status: complete — 90/90 tests passing (reproduced on two consecutive full runs), 0 compile errors, 2 pre-existing bugs found and fixed along the way (documented below, independently traced via `git log --date=iso`)**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-note --all-targets` was run BEFORE touching any file. It was clean: 0 errors, only pre-existing cosmetic warnings (unused imports, `testkit` glob-import ambiguity, unused functions in json import/export leaves). No baseline `cargo nextest run` was captured (recipe only requires `cargo check` before editing) — the 3 test failures found later (see `## Pre-existing bugs fixed`) were only surfaced once the full suite ran post-migration.

## What note actually duplicated (read before planning)

`🗿️artifacts/🗒️note/🦀️component.rs` is note's artifact root: an infinite-canvas block tree (`NoteBlockNode`: `Text`/`Image`/`Table`/`Math`/`Ink`/`Group` variants, each with `id/name/x/y/width/height/rotation/visible/locked`). Only `Text` carried content matching stdio's `s.stdio.semio.text` subset shape: `paragraphs: Vec<NoteTextParagraph>` → `NoteTextRun{text, bold, italic, underline, link}` — a genuine duplicate of `SemioTextRun{language, content, marks: Vec<SemioTextMark{Bold|Italic|Code|Link}>}`. No pre-existing cross-artifact reference field existed anywhere in the snapshot (unlike layout's `referenced_model`, which already had a role model to follow) — `R:any` is genuinely new capability, added the same way layout added its own new link slot.

## What changed

### Composition target: per-block, not per-document

Unlike writer (one document-level child) or lowpoly (one optional child), note is a multi-block canvas where only SOME blocks carry text. The correct unit of composition is therefore **one composed child per `Text` block**, not one child for the whole document — `NoteBlockNode::Text.paragraphs: Vec<NoteTextParagraph>` → `content: store::ArtifactChild<SemioTextSnapshot>` (type-aliased `NoteTextChild` in the artifact root). Verified this is safe against the framework: `ArtifactRefs::child_refs`/`links` both default to empty and no migrated plugin overrides them yet (`grep -rln "impl.*ArtifactRefs for"` across `✏️s/🔌️plugins` returns nothing) — so a child slot nested inside a `Vec<enum>` (unreachable by `#[derive(ArtifactSchema)]`'s own field-flat slot-table emission) costs nothing today; `#[child(...)]`/`#[state(...)]` attributes were deliberately NOT added to `NoteBlockNode`'s variant field since that enum isn't `ArtifactSchema`-derived.

`#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslEnum)]` were KEPT (not hand-rolled) per the migration recipe's 2026-08-13 update — `impl<S> DslField for ArtifactChild<S>` (`🏪️store/🦀️component.rs:523`) and `impl DslField for ArtifactLink` both exist and are generic/complete. Confirmed `dsl`'s derive macro only inspects `#[dsl(...)]`-pathed attributes (`parse_field_attrs`, `🗣️dsl/✨️derive/📦️glue.rs:85`), so `#[child]`/`#[link_slot]`/`#[state]` are silently ignored by it — safe to omit on `NoteBlockNode`. `NoteSnapshot` already carried the post-derive "hand-rolled `ArtifactDsl`/`ArtifactPack` calling `Self::__dsl_spec()`" shape from an earlier ticket (sourcing's `CurateSnapshot` is the live precedent for this exact pattern) — that boilerplate needed zero changes.

### Converter (real, not a stub) — `🗿️artifacts/🗒️note/🦀️component.rs`, `🔖️TextBridge` region

`text_snapshot_from_paragraphs`/`paragraphs_from_text_snapshot`: flattens multi-paragraph rich text into `SemioTextSnapshot`'s flat run list, encoding a paragraph boundary as a marks-free/language-free run whose `content == "\n"`. Bold/italic/link map directly to `SemioTextMark`; `NoteTextRun.underline` has **no equivalent** in stdio's closed Bold/Italic/Code/Link vocabulary and is honestly dropped (documented in the doc comment and covered by `text_bridge_drops_underline_honestly`). One documented lossy collapse: an empty paragraph list and a single paragraph with zero runs both flatten to zero runs, and the reverse always emits exactly one trailing paragraph (even an empty one) — so both restore as the SAME single empty paragraph, never as an empty list (`text_bridge_collapses_empty_paragraph_shapes`).

### Working scene — `🔖️WorkingScene` region

`NOTE_TEXT_SCRATCH: thread_local! RefCell<HashMap<child_id, Vec<NoteTextParagraph>>>`, generalizing writer's `WRITER_SCRATCH`/lowpoly's `mesh_workspace` pattern to N children instead of 1 (same cache, keyed the same way — nothing new architecturally). `note_text_child_handle(block_id, paragraphs)` mints a handle hashed from `(block_id, paragraph content)` — includes `block_id` (not content alone) so two distinct blocks with identical text never collide on one child slot, mirroring cad's `(pane_id, content)` convention. `note_text_child_handle_and_cache` mints+caches in one call; `note_block_text(&handle)` is the one read accessor every call site now uses (fails soft to `Vec::new()` on a cache miss — same documented staleness gap as every other exemplar).

**Duplicate-safety fix**: `clone_block`/`reid_block_tree` (schema `🦀️component.rs`) previously only reassigned `id`/`name` on duplicate; a duplicated Text block would have kept the SOURCE's `content` handle verbatim (same `child_id` as the original — a real correctness bug, since two live blocks would share one composed child slot). Fixed: `reid_block_tree` now recovers the source's live paragraphs from the cache BEFORE reassigning `id`, then remints under the NEW id and re-caches.

### `R:any` forward link slot

`NoteSnapshot.linked_artifact: Option<store::ArtifactLink>` (`#[link_slot(roles("any"))]`), mirrored onto `NoteArtifact` and `NoteDiff` (`Option<Option<ArtifactLink>>`, the standard optional-slot double-`Option` shape). Schema/codec-complete (round-trips through both hand-rolled-via-derive codecs, wired into `apply_to_artifact`/`MutationDiff::apply`/`absorb`'s `take!` list) but deliberately left **inert** — no mutation dispatch, no resolver read path — same honest scope layout's own `referenced_model` report used ("genuinely new capability with no existing UI/converter to preserve").

### Mutation vocabulary — unchanged, internals only

Per the recipe's "keep existing granular mutations' public payload shape unchanged" instruction: `EditBlockText.new_paragraphs: Vec<NoteTextParagraph>` did NOT change — only its diff/inverse internals now mint/read through the working scene instead of touching `paragraphs` directly. `CreateBlock`/`DuplicateBlock(s)`/`DeleteBlock(s)` all already carried the FULL `NoteBlockNode` value in their payload (no field-level awareness of `paragraphs`/`content`) and needed zero changes to their triad `diff`/`inverse` logic — `NoteBlockPatch.block_json` is already a whole-block JSON blob, so the diff layer is generically indifferent to the field-shape change. No new mutation triads added; `SetSnapshot` remains banned and untouched (note already routes whole-document replace through `reset_document_effect`, pre-existing, not part of this wave).

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was in the pre-migration `paragraphs=[ p runs=[ r "..." ] ]` shape, incompatible with the new `content=child_id=... target=...` record shape. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that built the real "semio" example `NoteSnapshot` via `note_text_child_handle_and_cache` and dumped real `print_dsl()` output (`cargo test … debug_fixture_regen -- --nocapture`), captured, written as the new fixture, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

## Pre-existing bugs fixed (independently traced via `git log --date=iso`, not this ticket's design work but blocking a green suite)

1. **`delete-blocks`'s inverse had a real block-reordering bug.** `🧬️mutations/🧺️delete-blocks/↩️inverse/🦀️component.rs` was created 2026-08-12 15:50:51 (48 min after this ticket opened, by an unrelated wave — file never touches `content`/`paragraphs`). It sorted its restore entries ASCENDING by original index, but every `MutationKind::inverse` caller (including `protocol::testkit::assert_mutation_inverse_law`) reverses the returned `Vec` before applying each step — so restoring two removed blocks landed them in the wrong relative order (proved via an isolated single-mutation repro that showed `duplicate_block` alone was innocent, isolating the bug to `delete_blocks`'s multi-entry restore). Fixed by sorting descending, so the caller's `.reverse()` turns it back into the correct ascending-index application order. Caught by `block_lifecycle_inverse_law_create_delete_duplicate`.
2. **`renders_document_tree` (note-play panel test) could never have passed as originally written, on any content.** `SetActiveExample`'s `reset_document_effect`/`HostEffect::LoadDocument` conversion predates this ticket's dispatch to note (`🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs` last touched 2026-08-13 16:49:56, `🎛️apps/🗒️note/🦀️component.rs` 18:52:17 — both before I started, neither ever edited by me). `dispatch_typed` only ever RETURNS a `HostEffect::LoadDocument` as data for a real host to re-apply — it never loops it back into the same app instance, so `app.snapshot()`/subsequent `render()` never reflected the loaded document, on ANY document, composed or not (structural, not content-shape). Fixed the test using the same technique writer's own `app_with_jack()`/cad's `two_instances_converge_…` tests already establish: call `PluginApp::load_document_pack` directly instead of trusting the dispatched effect.

Neither bug touches `✏️s/🔌️plugins/🗄️stdio/**` or any file outside note's own boundary.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-note --all-targets
```
**0 errors** (baseline was also 0 errors — no regressions), same pre-existing warnings only. Reproduced twice.

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-note --no-fail-fast
```
**90 tests run: 90 passed, 0 skipped.** Reproduced stable across two consecutive full runs (not flaky). New tests added (not new files — extended the existing artifact-root/snapshot test modules): `text_bridge_round_trips_paragraphs_through_semio_text_snapshot`, `text_bridge_collapses_empty_paragraph_shapes`, `text_bridge_drops_underline_honestly`, `working_scene_caches_by_child_id_and_block_id_never_collides`, `note_block_text_fails_soft_on_a_cache_miss` (artifact root); `linked_artifact_and_text_content_round_trip_through_text_and_binary`, `absent_linked_artifact_round_trips_as_none` (snapshot facet, new `round_trip_tests` module — that file had no prior test module).

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/🗒️note/**` (18 files, listed below), none of them `📦️glue.rs`/`📦️index.ts`. `✏️s/🔌️plugins/🗄️stdio/**` was only read for schema reference (`SemioTextSnapshot`/`SemioTextRun`/`SemioTextMark`/`SemioTextMarkKind`), never written.

## Concurrent-churn observations

`git diff --stat -- ✏️s/🔌️plugins/🗄️stdio` shows 2 files changed (`📊️csv/🦀️component.rs`, `✳️brep/🧬️schema/⚙️engine/🦀️component.rs`) that are **not mine** — I never touched stdio. Consistent with `📌️important.md`'s warning about DKM's in-flight `math`→`geometry`/`graph` extraction disrupting stdio tonight; did not block this pass (`semio-s-plugin-stdio` was green throughout, no retries needed).

## Files touched this pass

- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs` — `NoteBlockNode::Text.content` field swap, `NoteTextChild`, `text_snapshot_from_paragraphs`/`paragraphs_from_text_snapshot`, `note_text_child_handle`/`note_text_child_handle_and_cache`, `NOTE_TEXT_SCRATCH`/`note_block_text`, new converter/working-scene tests.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `NoteSnapshot.linked_artifact` (`#[link_slot(roles("any"))]`), `Default` update, new `round_trip_tests` module.
- `…/🧬️schema/🦀️component.rs` — `NoteArtifact.linked_artifact` mirror (to_snapshot/from_snapshot/set_snapshot/default_ui), `create_block_by_kind`, `reid_block_tree` duplicate-safety fix, `patch_block_field`'s `textContent` case, `empty_note_snapshot`.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `NoteDiff.linked_artifact` (double-`Option`).
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply_to_artifact`/`MutationDiff::apply`/`absorb` wired for `linked_artifact`.
- `…/🧬️schema/🧬️mutations/📝️edit-block-text/{🔺️diff,↩️inverse}/🦀️component.rs` — mint/cache-on-diff, cache-read-on-inverse.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — test literal fixes (3 `NoteBlockNode::Text` construction sites).
- `…/🧬️schema/🧬️mutations/🧺️delete-blocks/↩️inverse/🦀️component.rs` — pre-existing ordering bug fix (see above).
- `…/🧬️schema/💡️inferences/🧾outline/🦀️component.rs` — `block_word_count` rewired through `note_block_text`, test helper fix.
- `…/🧬️schema/📸️snapshot/{💾️binary,📝️text}/🦀️component.rs` — test literal fixes, fixture regen (text facet).
- `…/🚪️io/🦀️component.rs` — `draw_node_from_note_block`, `text_block_from_dwg`, media_tests literal fixes.
- `…/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` — `serialize` rewired through `note_block_text`.
- `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{📄️pdf/🔖️1.4,🎨️svg/🔖️1.1}/✳️any/🦀️component.rs` — construction sites rewired.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `🎛️apps/🗒️note/📌️panels/📄️artifact/🦀️component.rs` — `renders_document_tree` fixed (pre-existing bug, see above).

ucas-status: complete
