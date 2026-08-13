# Migration recipe — distilled from 3 exemplars (lowpoly, cad, writer)

Every W4 fan-out agent should read this file plus `📌️important.md` before touching its plugin. This is the concrete "how", not the "why" (see `📓️design-full-plan.md` for that). All three exemplars hit the same walls in the same order; this recipe exists so W4 doesn't rediscover them 29 more times.

## 1. The core transform

Replace an inline content field (a duplicated type, or an opaque blob like `mesh_json: String`) with a typed composed-child handle:

```rust
// before
pub struct FooSnapshot { pub content: SomeInlineType, ... }
// after
pub struct FooSnapshot { pub content: store::ArtifactChild<SemioXSnapshot>, ... }
// #[child(kind = "s.stdio.semio.<subset>")]
```

Use `Option<store::ArtifactChild<S>>` when the slot can be genuinely absent (lowpoly's `mesh`); a bare (non-`Option`) child when the slot always exists (writer's `document`, cad's per-pane model/drawing children).

Mint child handles content-addressed from a hash of the content being wrapped — never a random/incrementing id (peers must converge on replay). Pattern name varies per plugin but the shape is identical: `mesh_child_handle(object_id, content)`, `cad_model_child_handle(pane_id, content)`, `document_child_handle(content)`. Construct via `store::ArtifactChild::new(child_id, target)`.

## 2. The codec wall — you WILL hit this

`ArtifactChild<S>` has no `DslField`/`dsl::DslRecord` derive impl. If the snapshot type currently derives `dsl::DslRecord`, **drop that derive** and hand-roll `ArtifactDsl`/`ArtifactPack` for the whole struct. All three exemplars did this identically:
- Text codec: one `key=<hex-or-bracket>` line per field, matching whatever encoding convention the rest of that plugin's fields already use (`enc_str`/`dec_str`, or JSON-then-hex via `enc_json`/`dec_json` for structured fields — see cad's `🔖️JsonFieldPrimitives`).
- Binary codec: `write_str_lp`/`read_str_lp` (length-prefixed) per field, in the same field order as the text codec.
- The child handle itself encodes as its two strings (`child_id`, `target` — cad/writer call this the child-codec-primitives region, hex/bracket handle codec).

**Codec completeness is not optional and not caught by `cargo check`.** Every field on the snapshot struct must round-trip through BOTH codecs (text and binary), or you get silent data loss that only a real round-trip test catches (see §6). This bit cad hardest: two fields existed on the struct but were never wired into either codec, and `cargo check` stayed green the whole time.

**⚠️ UPDATE 2026-08-13, CHECK THIS BEFORE HAND-ROLLING ANYTHING**: `sourcing`'s wave-4 agent found `impl<S> crate::os_dsl::DslField for ArtifactChild<S>` now exists in `🏪️store/🦀️component.rs:523` — a real, generic, complete implementation (not a stub: proper `shape()`/`to_value()`/`from_value()` through a 2-field `RecordSpec`, verified by reading the source directly). **This may mean the whole codec wall above no longer applies** — a struct with an `ArtifactChild<S>` field might now keep its `#[derive(dsl::DslRecord)]` derive instead of dropping it and hand-rolling `ArtifactDsl`/`ArtifactPack` for the entire struct. **Check for this impl yourself before assuming you need to hand-roll anything** — if it's there and your snapshot's other fields don't have their own independent reasons to hand-roll (e.g. a field type that never supported the derive to begin with), try keeping the derive first; only fall back to hand-rolling if it genuinely doesn't compile. Every plugin migrated before this date hand-rolled its codec because this impl didn't exist yet — that was correct at the time, not a mistake to backport.

## 3. ⚠️ CHECK FIRST: a real resolver seam may already exist — `ArtifactView::with_children`

**Added 2026-08-13, after every plugin migrated before this point had to build the `thread_local!` workaround in §3 below.** `🔌️plugin/🦀️component.rs` now has `ArtifactView<'a, P> { snapshot, history, children: ChildContentView<'a> }` with `ArtifactView::with_children(snapshot, history, children)` — a real, live-wired view over a composing document's owned children, keyed `(slot, child_id)`. Its own doc comment states its purpose directly: *"the seam that replaces the `thread_local!`/session `HashMap<child_id, content>` caches every composed plugin used to carry. Those caches went STALE the moment anything moved a child's history without going through `ArtifactApp::handle`... Reading straight through the live `SpaceMember` cannot go stale by construction."*

**Before building yet another `thread_local!` scratch cache (§4 below), check whether this seam is wired into your plugin's actual dispatch path** (i.e. whether `ArtifactApp::handle`/`render` for your plugin actually receives an `ArtifactView` built via `::with_children` rather than always `::new`). If it's live and reachable, use it directly instead of the workaround — it's strictly better (no staleness gap to document, no undo/redo divergence risk). If it's present in the framework but not yet wired into your plugin's call path, note that gap precisely in your report rather than assuming either way.

**Plugins migrated before 2026-08-13 all used the `thread_local!` workaround and have a documented staleness gap in their reports** (lowpoly, cad, writer, flow, process, gis, dag, mathematical, imperative, sequence, reasoning, norm, raster, shooting, layout, and any dispatched shortly after this note was added but before their own agent checked). That is NOT a bug to retroactively fix mid-fan-out — it's expected, sanctioned, and correct for when they were written. Retrofitting them onto the real seam is real, valuable, but separate follow-up work for a later wave (W5/W6-adjacent), not something to interrupt the fan-out for.

**Clarified by `animate`'s agent, confirmed real**: `ArtifactView::with_children` is live and generically wired into every plugin's app layer (`ArtifactApp::handle`/`render`), but is **NOT reachable from `MutationKind::diff`/`inverse`** — those are pure functions over `base: &Snapshot` with no session/store context to pull a live view from. So even with the real seam available, mutation-triad diff/inverse bodies still need the `thread_local!` working-scene cache (§4) to read live child content when building a sparse diff; the seam replaces the cache for RENDER/READ paths (panels, export, inference), not for mutation authoring. Expect to use both: the real seam where you have an `ArtifactView` in hand, the scratch cache where you only have `base: &Snapshot`.

**Further narrowed by `playbook`'s agent, more precise still**: `VcsArtifactApp.children` — the map `ArtifactView::with_children` actually reads from — is **never populated by any plugin migrated so far**, because populating it requires calling `open_child`/`register_child` at the store level, which no plugin (as of this note) does. So in practice, as of 2026-08-13, the real seam exists structurally and is wired into the dispatch signature, but has **zero live content behind it for any plugin in this fan-out** — meaning the `thread_local!` working-scene cache (§4) is still the correct, necessary choice for EVERY plugin today, on both the render path and the mutation path. Check `VcsArtifactApp.children`'s actual population for your specific plugin before assuming the seam is usable — don't just check that the type signature exists.

## 4. Ephemeral working-scene / session cache (the `EngineRep` pattern) — fallback if §3's real seam isn't reachable yet

No `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle` yet (checked directly against `🔌️plugin/🦀️component.rs` by all three exemplars — W1-owned, read-only for plugin agents, do not add one yourself). So the app layer cannot resolve a child handle back to its live content through the framework. Every exemplar solved this the same way:

- A small struct or `thread_local!`/session-field `HashMap<child_id, content>` cache, living **beside** the persisted view, never on the snapshot itself. Names so far: `LowpolyScratch.mesh_workspace`, `CadWorkingScene`, `WriterWorkingScene`/`WRITER_SCRATCH`.
- Populated at mutation-diff-build time (the only place that has the literal content) and at fixture-construction time.
- Read through one accessor every render/export/inference call site funnels through (`writer_text(&snapshot)`, `mesh_workspace(&self, id)`) — do not scatter direct cache reads.
- **Document the staleness gap honestly** in a doc comment: store-level undo/redo bypasses `ArtifactApp::handle` entirely, so the cache can go stale relative to the document's handle across an undo/redo of a create/delete. lowpoly added a fail-closed check (`StaleMeshWorkspace`, content-hash verify before trusting cache) — do this if your plugin has a similar sensitive-to-staleness read path (e.g. destructive geometry edits); a simple documented gap is sufficient otherwise (cad, writer both left it as documented, not fail-closed, since their read paths are lower-stakes).
- This is NOT a fix for the missing resolver — it's a bridge until W1 or a later wave adds one. Never make it a durable struct field, never let it outlive the process, never derive it incrementally from itself (matches the repo's independently-ratified `EngineRep` contract: `build(&P)` only, wholly derived, droppable at any instant).

## 5. Real bidirectional converters — no stubs

Every exemplar needed a real `child_content_from_app_state`/`app_state_from_child_content` pair, not a placeholder. Examples: cad's `cad_object_from_model_element`/inverse (full field-for-field typology + transform mapping), writer's `document_snapshot_from_text`/`text_from_document_snapshot` (text ↔ one `DocBlock::Code` leaf, honestly lossless because `Code` carries no structure to lose). If the composed subset's shape can't losslessly represent something your plugin's old inline type carried, **say so explicitly** in the converter's doc comment (which fields are lossy, why) rather than silently dropping data.

Write a round-trip test for the converter in the plugin's existing test module (not a new file — repo policy).

## 6. Mutation vocabulary: `SetSnapshot` is banned, full stop

Whole-document replace is not an in-history mutation (`📌️important.md`'s forbidden-vocabulary list: `SetSnapshot`/`NoMutation`/`CollectionMutation`). If your plugin currently has an app command that does a whole-snapshot replace (`OpenDocument`, `SetFixtureJson`, `SetActiveExample`, etc.):

1. Remove any `ArtifactApp::whole_document_operation` trait override — let it fall back to the trait default (`None`). Both cad and writer did exactly this.
2. Replace the replace-behavior with a `HostEffect::LoadDocument` built from a fresh envelope: `store::create_document_envelope(...)` + `store::print_document_spr(...)`, wrapped and returned as an effect, not dispatched as a mutation. Name it `reset_document_effect` (writer) or equivalent — every app command that used to do a whole-document replace now emits this effect instead.
3. If a command's payload used to carry `#[dsl(block)] snapshot: FooSnapshot`, that also breaks (the snapshot struct dropped its `DslField`-enabling derive per §2) — switch the payload to a `json: String` and parse it at the handler, same as writer's `SetSnapshot`/`SetSnapshotJson` collapse.

## 7. Verification discipline — the part every exemplar got wrong at least once before getting right

1. **Run `cargo check -p <crate> --all-targets` BEFORE touching anything**, and note the baseline. Writer's crate was already red (16 errors) before any migration edit — knowing that up front is what let the final report correctly separate "I introduced this" from "this was already broken."
2. Do the migration. Fix every resulting compile error across the whole cascade (artifact layer → app layer → commands → panels → tests) in one pass. Don't stop at "artifact layer compiles" and call it done — cad's round 1 left 84 app-layer errors for a round 2 to clean up; budget to avoid that if you can finish both in one dispatch.
3. Run `cargo nextest run -p <crate> --no-fail-fast`. A green `cargo check` proves nothing about test correctness — every exemplar's most important bug (lowpoly's round-trip law, cad's codec-completeness gap) was invisible to `cargo check` and only surfaced by actually running the test suite.
4. **For every failing test, before writing "pre-existing/unrelated" in your report: run `git log -1 --date=iso --format="%H %ad %s" -- <file>` on the specific failing assertion/function and confirm, by real commit hash and date, that it predates this ticket** (opened 2026-08-12 15:02:49 — anything from before that is out of scope, but only once actually traced via a REAL date, not assumed). **Never parse the digits inside a commit message's `🎆️🌙️☀️` glyphs as a date — every auto-commit message carries a fixed, stale template string that does not reflect when the commit actually happened; only `git log --date=iso`'s real timestamp is trustworthy.** This ticket has had multiple agents' "pre-existing/unrelated" classifications turn out wrong — twice from a genuinely pre-existing bug bundled with one the agent's own migration introduced, and twice more (cad's and process's own reports) from exactly this fake-date trap. Do it right the first time: real date, not message text.
5. If a failure IS something your migration introduced — even indirectly, e.g. a codec you forgot to update for a new/renamed field — **fix it**, don't defer it.
6. Reproduce the final test run at least twice (not flaky) before reporting done.

## 8. Fixture regeneration (obsolete demo assets)

If your plugin has a `.dsl.semio`/binary demo fixture written before this migration, it will very likely be in an obsolete format the new hand-rolled codec can't parse (all three exemplars hit this). Regenerate it for real, never hand-transcribe:

1. Add a temporary `#[cfg(test)] mod debug_fixture_regen { ... }` that constructs a representative snapshot (via your plugin's own fixture-builder helpers) and dumps real `print_dsl()`/pack-encode output (`cargo test ... debug_fixture_regen -- --nocapture`).
2. Capture that output, write it as the new fixture file content.
3. **Remove the temporary test module cleanly.** Watch for a dangling trailing `#[cfg(test)]` attribute left behind by a truncation script — this caused a real "expected item after attributes" + cascading import error in cad's round 3. Verify with `grep -rn debug_fixture_regen` returning nothing afterward.

## 9. Diff shape convention

- Always-present slot (never absent, only ever replaced): `Option<ArtifactChild<S>>` in the diff, single-Option (writer's `document`).
- Optional slot (can be present or absent, and that presence itself can change): `Option<Option<ArtifactChild<S>>>` in the diff — outer Option = "did the presence/identity change", inner Option = "is it now present" (lowpoly's `mesh`, matches `✳️object`'s own established pattern — don't invent a new shape).

## 10. Scope and hot-file boundaries (recap of `📌️important.md`, restated for this recipe)

- Your fan-out boundary is your own plugin subtree only, minus `📦️glue.rs`/`📦️index.ts` (shared, W5-owned).
- Never touch `✏️s/🔌️plugins/🗄️stdio/**` — read it for schema reference only. If you need a stdio change, write it up under `## sharedFileRequests` in your report and stop short.
- Never touch framework kernel files (`🔌️plugin`, `🚪️io`, `🧬️schema`, `🛂️manifest`, `📡️spr`, `🏪️store`, `🌿️vcs`, `🎠️kernel`) — W1-owned.
- If you need a new crate dependency (e.g. `semio-framework` for `HostEffect`, which writer needed and cad/lowpoly already had), add it to your own plugin's `Cargo.toml` — that's inside your boundary.
- Concurrent churn: if `cargo check` shows errors, `grep` them for the originating path before assuming they're yours. If every error traces outside your plugin's subtree, it's someone else's in-flight work settling — retry in the foreground (no background waits/polling loops — this has burned 500-600k tokens per incident in this ticket) until it clears.
- Never run `ticket_close`/`ticket_reopen` — this ticket is shared, only the orchestrating session closes it.

## 11. Report template

Match cad/writer/lowpoly's report shape: what changed (file-anchored), working-scene design, converter description, verification commands + exact output, honest gap list, `## sharedFileRequests`, final `ucas-status:` line (`complete` only if every test passes or every failure is independently traced to a pre-ticket commit).
