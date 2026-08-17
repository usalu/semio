# W4 (batch Cb) — `playbook` composes stdio `document` AND `flow`

**ucas-status: complete — 73/73 tests passing (reproduced stable across three consecutive runs), 0 compile errors; 1 pre-existing baseline bug fixed (2-line path drift from concurrent framework churn, documented below)**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-playbook --all-targets` was run BEFORE touching any file. First attempt failed to even reach playbook: `semio-framework-plugin` (the W1-owned, frozen framework crate) had a live uncommitted edit (`git status --porcelain` showed `MM` on `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, matching `📌️important.md`'s "THE AUTO-COMMIT MESSAGE'S DATE IS FAKE" churn-detection guidance — confirmed via `git log -1 --date=iso` that the last real commit on that file was `2026-08-13 16:49:56`, inside this ticket's active window, and `git status` still showed it dirty on top of that). Retried in the foreground (no background waits) until the framework side settled; the framework compiled clean on the next retry.

Once past that, the TRUE baseline was **2 pre-existing compile errors in playbook's own file**, unrelated to composition:
- `🎛️apps/📖️playbook/🦀️component.rs:77-78` — `type Transient = semio_framework_plugin::NoTransient;` / `type TransientMutation = semio_framework_plugin::NoTransientMutation;` — `NoTransient`/`NoTransientMutation` no longer re-export at `semio_framework_plugin`'s crate root; they only live at `semio_framework_plugin::app::NoTransient`/`::app::NoTransientMutation` now. Traced via `git log -1 --date=iso` on this file (last real commit `696b87d1`, `2026-08-13 16:49:56`, same commit as the framework's own in-flight rename) — this is the SAME repo-wide `ArtifactApp::Transient`/`TransientMutation` associated-type churn currently sweeping the codebase (confirmed independently: `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`, a **separate** crate not compiled by `-p semio-s-plugin-playbook`, has a live uncommitted edit adding the identical two lines with the identical wrong unqualified path — see `## Concurrent-churn observations`).

This blocked `cargo check` entirely (nothing compiles, including composition work, until fixed), so it was fixed directly — a 2-line path qualification (`semio_framework_plugin::app::NoTransient`/`::app::NoTransientMutation`) — per this ticket's "fix every compile error blocking cargo check, don't defer" instruction. Not composition work; independently traced, not touching any file outside this plugin's own crate.

## What changed

### Design: mapping playbook's domain onto `document` + `flow`

Playbook's kernel domain (`🧰️framework/…/📖️playbook/🦀️component.rs`, framework-owned, read-only) is `PlaybookSpec{schema,id,version,title,steps:Vec<PlaybookStep>}`, `PlaybookStep{id,title,description,blocks:Vec<PlaybookBlock>}` — a strict ordered list of steps, each holding a Blockly-like list of ~18-field form blocks (including a recursive `PlaybookExpr` visibility-condition tree). Two composed children, split by concern:

- **`flow` (`s.stdio.semio.flow`) — the LOSSLESS procedural source of truth.** One `FlowNode` per step (`kind = "step"`, `label` = step title); the step's full `blocks` vocabulary (every field, including nested `condition` trees) is JSON-encoded wholesale into one `blocksJson` param — the same "honest string boundary" flow-plugin's own `Widget -> FlowNode` converter established (`📓️wave4-reports/flow-report.md`). `description` becomes its own param. Steps are chained via sequential `FlowEdge`s (`kind = "sequence"`) as a redundant procedural witness of order — `nodes`' own `Vec` order is what's actually read back, never the edges.
- **`document` (`s.stdio.semio.document`) — an HONEST narrative projection, not a second source of truth.** One `Heading(1)` for the playbook title (if present) + one `Heading(2)`/`Paragraph` pair per step (title/description). Reconstructing FROM a bare document alone cannot recover `blocks`/`condition` — this is stated explicitly in both converter functions' doc comments, per this ticket's "say so explicitly" rule for lossy directions, and covered by a real test (`document_projection_round_trips_titles_and_descriptions_only`) that asserts the recovered `blocks` are empty.

Both children are always present (never `Option`), matching writer's `document`/flow's `content` field pattern, not lowpoly's optional-slot shape.

### Composed child bridge + working scene (`🗿️artifacts/📖️playbook/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `PlaybookDocumentChild`/`PlaybookFlowChild` = `store::ArtifactChild<SemioDocumentSnapshot>`/`<SemioFlowSnapshot>`.
- **Two real, tested, bidirectional converters** (not stubs): `flow_content_snapshot_from_steps`/`steps_from_flow_content` (lossless — every `PlaybookStep`/`PlaybookBlock` field round-trips) and `document_snapshot_from_steps`/`steps_from_document` (honestly lossy in the document→steps direction, documented and tested).
- `flow_content_child_handle`/`document_child_handle` — content-addressed (`DefaultHasher` over the converted snapshot's JSON), same pattern as every prior exemplar.
- `PlaybookWorkingScene { steps: Vec<PlaybookStep> }` + `thread_local!` `PLAYBOOK_SCRATCH: RefCell<HashMap<child_id, Vec<PlaybookStep>>>`, keyed by the `flow` child's id (the lossless source). `playbook_working_scene`/`playbook_steps` are the read call sites; `playbook_content_handles_and_cache` mints+caches BOTH children together from one `Vec<PlaybookStep>`; `playbook_snapshot_with_steps` is the standard fixture/import constructor.

### ⚠️ §3 resolver-seam check — done, with a more precise finding than prior exemplars

Per the migration recipe's 2026-08-13 addition, checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only) **before** building the `thread_local!` workaround: `ArtifactView::with_children`/`ChildContentView` **is real** and **is** generically threaded through every `VcsArtifactApp` dispatch call site (`handle`, `render`, `import_media`, clipboard actions — all build `ArtifactView::with_children(snapshot, history, ChildContentView::new(children))`, not `::new`). This is a stronger finding than prior reports' "no such plumbing exists."

However, tracing further: `VcsArtifactApp.children` (the live child-store map `ChildContentView` wraps) is populated ONLY by `open_child`/`register_child`/`absorb_created_children`, and **no composed plugin in this ticket — including this one — ever calls any of them.** Child creation here, as everywhere else in this ticket, is pure content-addressed-HANDLE minting inline in a diff; there is no live child-store registration. So `doc.children` is unconditionally empty for playbook at every call site that DOES receive it. Separately and independently, `protocol::MutationKind::diff(&self, base: &Snapshot)` — the sole signature every mutation triad's `🔺️diff` leaf builds against — never receives a children view at all, real or empty, so the real seam couldn't help diff-building regardless.

Conclusion: the `thread_local!` working-scene cache (§4) is still required, and is reused uniformly for `render`/`import_media` too rather than mixing two different resolution strategies for the same data — documented precisely in `PlaybookWorkingScene`'s own doc comment.

### Mutation vocabulary — kept, rewired (no new triads, nothing forbidden)

Playbook's existing 9-triad vocabulary (`add-step`/`remove-step`/`move-step`/`add-block`/`remove-block`/`move-block`/`replace-block`/`update-step`/`change-title`) is unchanged in shape — payload types (`AddStep.step: PlaybookStep`, etc.) reference the framework kernel's typed domain, untouched by composition. What changed is **only the `🔺️diff`/`↩️inverse` construction** in 7 of 9 triads (2 — `add-step`'s inverse and `change-title`'s both leaves — never touched `.steps` and needed no change): each now reads the CURRENT steps off `base` via `playbook_working_scene(base).steps`, applies its own specific semantics to that `Vec<PlaybookStep>` (identical logic to before, just against the cache instead of a struct field), then (for `🔺️diff` leaves) calls the new shared builder `diff_replace_content(title, steps)` which mints+caches new `document`+`flow` handles — the "mint+cache whole handle, never apply-then-capture" pattern writer's `diff_set_text`/flow's `diff_replace_content` established.

`PlaybookDiff.steps: Option<PlaybookStepsDelta>` → `document: Option<PlaybookDocumentChild>` + `flow: Option<PlaybookFlowChild>` (single-Option each — the slots are never absent, only replaced). `PlaybookStepsDelta`/`PlaybookBlocksDelta`/`PlaybookStepPatch(Entry)`/`PlaybookBlockPatch(Entry)` deleted (dead — confirmed zero references remain). `🔺️diff/📝️text`'s `apply`/`apply_to_artifact`/`absorb` collapsed to whole-handle-replace for both slots; `apply_steps_delta`/`apply_blocks_delta`/`apply_step_patch`/`apply_block_patch` (all now-dead identified-collection appliers) removed.

No `whole_document_operation` override existed on `PlaybookPlayApp` (grepped — zero hits, matching flow's precedent: "nothing to remove"). No `SetSnapshot`/`NoMutation`/`CollectionMutation` anywhere (grepped, zero hits, before and after).

### Snapshot / codec (`📸️snapshot/🦀️component.rs`)

`PlaybookSnapshot.steps: Vec<PlaybookStep>` → `document`/`flow` composed children. This struct previously delegated its ENTIRE `ArtifactDsl`/`ArtifactPack` to the kernel `PlaybookSpec::__dsl_to_record`/`__dsl_spec` (no `dsl::DslRecord` derive on `PlaybookSnapshot` itself, but a 1:1 field mirror) — that 1:1 mirror is gone now that `steps` is replaced by two opaque handles, so both codecs are hand-rolled directly: `🔖️ChildCodecPrimitives` (hex/bracket handle codec, generic over the phantom `S` so one implementation backs both slots), `🔖️TextPrimitives`/`🔖️BinaryPrimitives` (one `key=hex` line / LEB128 length-prefixed field, matching writer's/flow's established shape), `🔖️HandcraftedArtifactCodecs`. `title: Option<String>` uses the `[0]`/`[1,<hex>]` bracket-tag convention already established in this ticket (raster's `enc_option`/`dec_option`).

`as_kernel()`/`to_kernel()`/`from_kernel()` kept (2 external call sites: `flatten_playbook_blocks`, `builder_window::render`) but rebuilt to read/write steps through the working-scene cache instead of a direct field. New `steps()` method — the single call site every render/inference/export/test path in this plugin now uses instead of the old `.steps` field access (61 call sites across the plugin, enumerated and fixed one-for-one).

`PlaybookArtifact` (`🧬️schema/🦀️component.rs`) got the identical field swap so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent (mirrors `WriterArtifact`/`FlowArtifact`'s precedent).

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was in the old kernel-delegated format (`steps=[ id=s title=Steps blocks=[ ] ]`), incompatible with the new hand-rolled `document=[…]`/`flow=[…]` handle-line codec. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that built the same one-empty-step default via `empty_playbook_snapshot()` and dumped the real `print_dsl()` output (`cargo test … dump_default_snapshot_dsl -- --nocapture`), captured, written as the new fixture, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

## Converter tests (real, not stubs)

Added to the artifact root's existing `mod tests` (no new test file, per repo policy):
- `flow_content_round_trips_every_step_field_losslessly` — a 2-step fixture with a `condition` tree, `required`, `description`, round-trips exactly through `flow_content_snapshot_from_steps`/`steps_from_flow_content`.
- `document_projection_round_trips_titles_and_descriptions_only` — same fixture through `document_snapshot_from_steps`/`steps_from_document`, asserting title/description recover exactly and `blocks` come back empty (the documented lossy direction).

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-playbook --all-targets
```
**0 errors**, confirmed on two consecutive clean runs after the final edit (down from 2 pre-existing baseline errors, both fixed). Remaining warnings are pre-existing/cosmetic (unused imports/qualifications across the workspace, `serde_to_json_value`/`json_value_to_serde` dead-code in the untouched json import/export serializers, `hidden lifetime parameters` idiom lints) — none introduced by this pass, none touched.

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-playbook --no-fail-fast
```
**73 run: 73 passed, 0 skipped.** Reproduced stable across three consecutive full runs (not flaky). No test was deleted; 2 new tests were added (the converter round-trip laws above); every pre-existing test that touched `.steps` (61 call sites) was rewired to `.steps()`, none weakened.

## sharedFileRequests

None. Every substantive change is inside `✏️s/🔌️plugins/📖️playbook/**` (own crate only — never touched `📦️glue.rs`/`📦️index.ts`, the extensions crate, or `🗄️stdio/**`; stdio's `document`/`flow` subset schemas were read-only reference).

## Concurrent-churn observations

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (W1-owned, frozen) was live-dirty at dispatch time (`git status` `MM`); settled to a clean-compiling state on retry without intervention. Root-caused this pass's 2-error baseline (an `ArtifactApp::Transient`/`TransientMutation` associated-type path move, `semio_framework_plugin::NoTransient` → `::app::NoTransient`) — fixed as a 2-line path qualification since it blocked `cargo check` entirely.
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` — a **separate crate** (`semio-s-plugin-playbook-procedural`, its own `📦️packages/🦀️rust/Cargo.toml`, NOT compiled by `-p semio-s-plugin-playbook`) has a live uncommitted 2-line edit (`git status` `MM`, mtime matching the same churn window) adding `type Transient = semio_framework_plugin::NoTransient;`/`TransientMutation` — the SAME wrong unqualified path this pass's own baseline fix corrected, presumably a repo-wide automated fan-out adding the new associated types. Left untouched: it is outside this plugin's own crate/compile scope, is mid-edit by another session, and its (likely-eventual) breakage does not block `semio-s-plugin-playbook`'s own build or test suite. Flagging here rather than "fixing" — the transient-failure protocol's "never fix someone else's file" applies even though the path is nominally inside this plugin's directory tree, since it's a distinct crate under active concurrent edit.
- `semio-s-plugin-stdio` and the framework crates were green throughout the rest of this pass — no retries needed beyond the one initial framework settle.

## Files touched this pass

- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️component.rs` — `PlaybookDocumentChild`/`PlaybookFlowChild`, `flow_content_snapshot_from_steps`/`steps_from_flow_content`, `document_snapshot_from_steps`/`steps_from_document`, `flow_content_child_handle`/`document_child_handle`, `PlaybookWorkingScene`, `PLAYBOOK_SCRATCH`, `playbook_working_scene(_for_handle)`, `playbook_steps`, `playbook_content_handles_and_cache`, `playbook_snapshot_with_steps`, 2 new converter round-trip tests.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `PlaybookSnapshot` field swap, hand-rolled codecs, `as_kernel`/`to_kernel`/`from_kernel`/`steps()`.
- `…/🧬️schema/🦀️component.rs` — `PlaybookArtifact` field swap, `to_snapshot`/`from_snapshot`/`set_snapshot`/`Default`.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `PlaybookDiff.document`/`.flow`, deleted dead delta/patch types.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — apply/apply_to_artifact/absorb rewire, `diff_replace_content` builder, deleted dead appliers.
- `…/🧬️schema/🧬️mutations/{↔️move-step,➕add-step,➖remove-step,🔀move-block,🔄replace-block,🗑️remove-block,🧱add-block,🩹update-step}/{🔺️diff,↩️inverse}/🦀️component.rs` (14 files with real changes) + `➕add-step/🦠️mutation/🦀️component.rs` (1 field-count read) — all rewired onto the working-scene + `diff_replace_content` pattern.
- `…/🧬️schema/🧬️mutations/🦀️component.rs`, `…/🧬️mutations/📝️text/🦀️component.rs` — test fixture/assertion fixes (`.steps()`, `playbook_snapshot_with_steps`).
- `…/🧬️schema/💡️inferences/🦀️component.rs` — `infer()` rewired through `.steps()`, JSON-literal test fixture replaced with Rust-constructed `PlaybookStep`/`PlaybookBlock`.
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — test fix (`.steps()`).
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `…/📚️examples/🎬️demo/🧪️tests/🦀️test.rs` — test fix (`.steps()`).
- `🎛️apps/📖️playbook/🦀️component.rs` — baseline `NoTransient`/`NoTransientMutation` path fix, `import_media` rewired through `.steps()`, 3 test fixes.
- `🎛️apps/📖️playbook/🎮️commands/{🪜️step,🧱️block,🗂️selection}/🦀️component.rs` — `.steps()` rewiring, 9 test fixes.

ucas-status: complete
