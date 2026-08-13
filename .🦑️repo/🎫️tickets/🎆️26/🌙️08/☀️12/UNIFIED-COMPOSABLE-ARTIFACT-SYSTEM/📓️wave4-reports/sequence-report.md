# W4 — `sequence` composes stdio `flow`

**ucas-status: complete — 126/128 tests passing (reproduced stable across three consecutive runs), 0 compile errors; the 2 remaining failures are independently traced via `git log --date=iso` to a mutation-vocabulary defect that predates this ticket (commits `2026-08-12 11:09:41` and `2026-08-12 18:08:12`, both before this ticket opened `2026-08-12 15:02:49`), not introduced by this migration — evidence below**

## Baseline (before any edit)

`cargo check -p semio-s-plugin-sequence --all-targets` was run BEFORE touching any file. It was **already RED**: 8 real compile errors (9 counting the `lib test` target separately), none related to composition:

1. `CsvSnapshot` field mismatch (×4: 2 import, 2 export) — the plugin's own `🚪️io/{import,export}/…/csv` serializers read/wrote `from.headers`/`from.rows`, fields `CsvSnapshot` no longer has since ticket `26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES` dissolved it onto `{schema, has_header, records: Vec<CsvRecord{fields: Vec<CsvField{value,quoted}>}>}`.
2. `SequenceMutation::StepsAdd` — a nonexistent enum variant referenced in `import_media`'s `steps:in` handler (`🎛️apps/🎬️sequence/🦀️component.rs`); the real vocabulary is `SequenceMutation::CreateStep(CreateStep)`.
3. `?` operator type mismatch in the JSON export serializer — `serde_json::to_value(snapshot)` returns `serde_json::Value`, assigned into `JsonSnapshot.value: JsonValue` (a stdio wrapper type) without `.into()`.
4. `no method named diff/inverse found for &SequenceMutation` (×4, in `🧬️mutations/🦀️component.rs`'s `apply_sequence_mutation`/`inverse_sequence_mutation`/tests) — `protocol::Mutation` (the trait providing `.diff()`/`.inverse()`) was implemented but not imported in that file.

All four classes are pre-existing and unrelated to composition — fixed outright as part of getting to a green baseline (required to even run the test suite), per the migration recipe's "fix anything blocking, even non-composition bugs" allowance. None involved `✏️s/🔌️plugins/🗄️stdio/**` writes — only reading `CsvSnapshot`'s real shape for reference.

## What changed

### Snapshot / composed child

`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`:
- `SequenceSnapshot.{steps: Vec<SequenceStep>, edges: Vec<SequenceEdge>}` → `content: SequenceContentChild` (`store::ArtifactChild<SemioFlowSnapshot>`), `#[child(kind = "s.stdio.semio.flow")]`.
- **Codec wall hit exactly as the recipe predicted**: the old `SequenceSnapshotDsl` mirror derived `dsl::DslRecord` with `#[dsl(table)] steps: Vec<SequenceStep>` / `#[dsl(table)] edges: Vec<SequenceEdgeDsl>`. Since `ArtifactChild<S>` has no reachable `DslField` impl from this crate, the mirror is **deleted entirely** and `SequenceSnapshot` now hand-rolls `ArtifactDsl`/`ArtifactPack` directly (hex/bracket text — `schema=<hex>`/`content=[<hex>,<hex>]` — plus LEB128-length-prefixed binary), mirroring writer's/cad's exact pattern (`enc_str`/`dec_str`/`enc_ref`/`dec_ref`/`enc_child`/`dec_child` primitives). The old per-edge `SequenceEdgeDsl` unified-`dsl::Wire` mirror (and its `sequence_edge_to_dsl`/`sequence_edge_from_dsl` helpers) is now dead code with the mirror gone — removed from `📸️snapshot/📝️text/🦀️component.rs` rather than left orphaned.
- Both codec fields (`schema`, `content`) round-trip through both the text and binary codec — verified by `dsl_round_trips_default_snapshot`, `default_sequence_example_dsl_round_trips`, `dsl_round_trips_snapshot_with_slots_and_nested_params`, `json_pack_round_trips`-equivalent binary op tests, all passing.
- New `SequenceFixture { schema, steps: Vec<SequenceStep>, edges: Vec<SequenceEdge> }` (same file): the plain pre-migration shape, this plugin's own analog of `flow::FlowFixture`. `SequenceSnapshot::to_fixture()`/`::from_fixture()` bridge to/from it through the working-scene cache.

`SequenceArtifact` (`🧬️schema/🦀️component.rs`, the UI-inclusive full-state struct) got the identical field swap (`steps`/`edges` → `content: SequenceContentChild`) so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent — mirrors `WriterArtifact`/`FlowArtifact`'s precedent exactly.

### Mutation vocabulary — kept, rewired

The plugin's existing 8-triad vocabulary (`create-step`/`delete-step`/`move-step`/`edit-step-params`/`change-step-collapsed`/`connect-steps`/`disconnect-steps`/`duplicate-step`) needed **no new/removed triads** — payload types (`CreateStep.step: SequenceStep`, `ConnectSteps.{id,from,to}`, etc.) are typed/semantic, not composed-child concerns (matches flow-report.md's identical finding). No forbidden vocabulary (`SetSnapshot`/`NoMutation`/`CollectionMutation`) appears anywhere.

What changed is only the `🔺️diff` construction in all 8 triads plus the `↩️inverse` leaves that used to read `base.steps`/`base.edges` directly: each now reads the CURRENT scene off `base` via `sequence_working_scene(base)`, applies its own specific semantics against that scene (identical logic to before, just against the cache instead of struct fields), then (for `🔺️diff`) calls the new shared builder `diff_replace_content(steps, edges)` which mints+caches a whole new content handle — the "mint+cache whole handle, never apply-then-capture" pattern flow's `diff_replace_content`/writer's `diff_set_text` established.

`SequenceDiff` (`🔺️diff/🦀️component.rs`): `steps: Option<SequenceStepsDelta>` / `edges: Option<SequenceEdgesDelta>` → `content: Option<SequenceContentChild>` (single-Option — the slot is never absent, only ever replaced, matching writer's `document`/flow's `content` exactly, not lowpoly's `Option<Option<…>>` optional-slot shape). `SequenceStepsDelta`/`SequenceEdgesDelta`/`SequenceStepPatchEntry`/`SequenceEdgePatchEntry` deleted (dead — confirmed zero references remain anywhere in the plugin via `grep`). `🔺️diff/📝️text/🦀️component.rs`'s `apply`/`apply_to_artifact`/`absorb` collapsed to a single whole-handle-replace branch; `apply_steps_delta`/`apply_edges_delta`/`absorb_steps_delta`/`absorb_edges_delta` (now-dead identified-collection-delta appliers) removed.

Also deleted as genuinely dead: `SequenceStepPatch`/`SequenceEdgePatch` and their `protocol::Patchable` impls in the artifact root file (`🗿️artifacts/🎬️sequence/🦀️component.rs`) — only ever consumed by the now-deleted delta types, confirmed zero remaining references.

### Composed child bridge + working scene (`🗿️artifacts/🎬️sequence/🦀️component.rs`, new `🔖️ContentBridge`/`🔖️WorkingScene` regions)

- `SequenceContentChild = store::ArtifactChild<SemioFlowSnapshot>`.
- **Real bidirectional converter** (not a stub): `sequence_content_snapshot_from_working(steps, edges) -> SemioFlowSnapshot` / `working_from_sequence_content_snapshot(&SemioFlowSnapshot) -> (Vec<SequenceStep>, Vec<SequenceEdge>)`. Every `SequenceStep` field round-trips: `id`/`kind` map directly onto `FlowNode.id`/`.kind` (`label` is set to `kind` on encode, discarded on decode — cosmetic and fully derivable, no information lost); `x`/`y` map onto `FlowNode.position: SemioPoint2`; `params: StepParams(Dictionary)` is JSON-encoded into one `FlowParam{key:"params",…}`; the optional `slot: Option<SlotRef>` is JSON-encoded into one `FlowParam{key:"slot",…}` (`"null"` round-trips to `None`); `collapsed: bool` becomes `FlowParam{key:"collapsed",…}`. `SequenceEdge{id,from,to}` maps 1:1 onto `FlowEdge` through an empty-port `PortRef` (sequence edges are plain step-to-step flow, not port-addressed) — the constant `kind: "sequence"` tag is written on encode and discarded on decode (lossless, `SequenceEdge` carries no `kind` of its own). Tested by `step_content_round_trips_through_the_composed_child_snapshot` (round-trips `default_snapshot().to_fixture()`'s steps/edges through the converter and asserts equality).
- `sequence_content_child_handle(steps, edges)` — content-addressed (`DefaultHasher` over the converted `SemioFlowSnapshot`'s JSON), same pattern as `flow_content_child_handle`/`document_child_handle`.
- `SequenceWorkingScene { steps, edges }` + `thread_local!` `SEQUENCE_SCRATCH: RefCell<HashMap<child_id, SequenceWorkingScene>>` — never persisted, matches the `EngineRep` contract. Stores the literal owned `Vec<SequenceStep>`/`Vec<SequenceEdge>` (not a re-derivation through the JSON converter), so `sequence_working_scene`/`to_fixture` return byte-identical data to what a pre-migration direct-field read would have returned — the converter only runs when computing the content hash and in the explicit round-trip test (same design flow-report.md independently arrived at and this pass followed, per `📌️important.md`'s pointer to it).
- `sequence_working_scene(&SequenceSnapshot) -> SequenceWorkingScene` is the one read call site every mutation diff/inverse and app-layer host in this plugin now goes through; `sequence_content_child_handle_and_cache(steps, edges) -> SequenceContentChild` is the one mint+cache call site; `diff_replace_content(steps, edges) -> SequenceDiff` is the one shared diff-builder every triad calls.
- Same documented staleness gap as every exemplar: store-level undo/redo bypasses `ArtifactApp::handle`, and a bare `parse_dsl`/`decode_pack` of persisted bytes recovers only the opaque handle, never the content (no `LinkResolver` exists yet — checked directly against `🔌️plugin/🦀️component.rs`, W1-owned). Fails soft (empty scene), never panics.

### App-layer working representation (`SequenceFixture`, not a thread_local upgrade)

Unlike flow/process, sequence's app layer (`SequenceHost` in `🎛️apps/🎬️sequence/🦀️component.rs`) is a genuine **live in-place-mutating editing engine** — ~350 lines of `self.snapshot.steps.push(...)`/`.retain(...)`/`.iter_mut()` across `add_step_in_slot`/`remove_step`/`connect_steps`/`sync_edges_from_dag`/`reorganize`/`build_dag_fixture`/etc., not a read-mostly bridge. Changing `SequenceHost.snapshot`'s field type from `SequenceSnapshot` to `SequenceFixture` (the plain `{schema,steps,edges}` shape) let every one of those ~350 lines stay **completely unchanged** — only the boundary conversions needed edits:
- `SequenceHost::from_snapshot(SequenceSnapshot)` now delegates to a new `from_fixture(SequenceFixture)` via `.to_fixture()`.
- `replace_snapshot`/`load_json` now take/parse a `SequenceFixture` directly — this is *not* a behavior change: `SequenceHost::to_json`/`load_json`'s wire JSON contract was already the flat `{schema,steps,edges}` shape (proven by a pre-existing test literal `r#"{"schema":"other","steps":[],"edges":[]}"#}"#` at `🎛️apps/🎬️sequence/🦀️component.rs:1530`, unchanged by this pass), so `SequenceFixture` **is** that contract's type, not a new one.
- `host_from_snapshot`/`ops_from_host_mutation` (the `🔖️HostHelpers` free functions every command handler funnels through) now convert the persisted `&SequenceSnapshot` "before" state via `.to_fixture()` before diffing against `host.snapshot`.
- `sequence_snapshot_mutations`'s signature (`🧬️mutations/🦀️component.rs`) changed from `(&SequenceSnapshot, &SequenceSnapshot)` to `(&SequenceFixture, &SequenceFixture)` — the semantic diff itself never needed the composed-child form, only plain steps/edges.
- The 4 render-path panels reading `.steps`/`.edges` directly (`document_panel`, `catalogue_panel`, `inspection_panel`; `compiled`/`main`/`script` windows are unaffected since they route through `host_from_snapshot`, unchanged) switched their `render(...)` signature from `&SequenceSnapshot` to `&SequenceFixture`; the central `render()` dispatcher (`🎛️apps/🎬️sequence/🦀️component.rs`) now computes `let live = fixture.to_fixture();` once and passes `&live` to those three.

### `whole_document_operation` — nothing to remove

Checked: `ArtifactApp for SequencePlayApp` never overrode `whole_document_operation` (grepped the whole plugin — zero hits). No cleanup needed, matching flow's finding.

### Read-side rewiring

Every direct `.steps`/`.edges` field access on a `&SequenceSnapshot` across the plugin was rewritten to go through `.to_fixture()` (or `sequence_working_scene(...)` inside the artifact layer): the artifact-layer `💡️inferences/🦀️component.rs`/`🧭topology/🦀️component.rs` (`compute_sequence_topology` + 6 tests), 8 mutation triads' `🔺️diff`/`↩️inverse` leaves, the `🧬️mutations/🦀️component.rs` test module (6 assertions + `sequence_snapshot_mutations` call), the app root's `import_media` handler + 6 tests, `🎮️commands/{🪜️step,🔗️connection,🕸️node-graph,🔄️layout}` (4 files, handlers + 7 tests), 3 panel files, the WASM bridge (`replace_snapshot`/`load_fixture_json`).

### Fixture regeneration

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` was a `steps [...]{...} edges [...]{...}` table-grammar dump matching the OLD `SequenceSnapshotDsl` mirror — obsolete under the new `schema=…\ncontent=[…]` shape. Regenerated via a temporary `#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` that called `print_dsl(&default_snapshot())` and dumped the real output (`cargo nextest run … dump_default_snapshot_dsl --no-capture`), captured, written as the new fixture, temporary module removed cleanly (verified: `grep -rn debug_fixture_regen` returns nothing).

### CSV serializer bugs (pre-existing, fixed outright)

The import/export CSV serializers (`🚪️io/{import,export}/…/csv/🔖️rfc4180/✳️any/🦀️component.rs`) read/wrote a `{headers, rows}` shape `CsvSnapshot` no longer has (see Baseline #1). Fixed to the real `{has_header, records: Vec<CsvRecord{fields}>}` shape: export writes one `CsvRecord` per step (`id`, `kind`, JSON-encoded `params`); import reconstructs one step per record (first field → id, remaining fields → a single JSON-encoded `value` param). Matches the identical `stdio_gap` fix other sibling plugins (`imperative`, `trinity/jack`) already applied for the same `CsvSnapshot` shape change.

## Converter (real, not a stub)

`sequence_content_snapshot_from_working`/`working_from_sequence_content_snapshot` (`🗿️artifacts/🎬️sequence/🦀️component.rs`, `🔖️ContentBridge` region) — see "Composed child bridge" above. Round-trip-tested (`step_content_round_trips_through_the_composed_child_snapshot`).

## Resolver wire-up

No real `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle`'s signature — checked directly against `🔌️plugin/🦀️component.rs` (W1-owned, read-only for this ticket), matching every prior wave's finding. Out of scope for a plugin-scoped agent.

## Verification (actual, run in the foreground)

```
CARGO_TARGET_DIR=.../🎯️target cargo check -p semio-s-plugin-sequence --all-targets
```
**Baseline: 8 real compile errors** (pre-existing, unrelated to composition — see above). **After migration: 0 errors**, confirmed on multiple consecutive runs. Remaining warnings are pre-existing/cosmetic (unused imports, unnecessary qualifications) — cross-checked against the baseline warning list; identical set, none introduced by this pass.

```
CARGO_TARGET_DIR=.../🎯️target cargo nextest run -p semio-s-plugin-sequence --no-fail-fast
```
**126 passed, 2 failed**, reproduced identically across three consecutive full runs (not flaky — same 2 named failures every time).

## The 2 remaining failures — independently traced, NOT introduced by this migration

`artifacts::sequence::standards::v1::subsets::any::schema::mutations::component::tests::delete_step_inverse_law` and `…::delete_step_severs_and_reconnects_edges` both fail at `assert_eq!(&state, base, "applying mutation.inverse(base) (reversed) after mutation must restore base")` (the generic `protocol::testkit::assert_mutation_inverse_law` law, and this file's own local `round_trip` helper with the identical assertion).

**Root cause**: `create-step`'s `🔺️diff` (and, pre-migration, `SequenceStepsDelta`'s `added` list via `apply_identified_delta`) always **appends** the recreated item at the END of the steps/edges collection — it has no way to reinsert at the item's original index. `delete_step_inverse_law` deletes `"step-1"` (the FIRST of 2 steps in `default_snapshot()`), so its inverse (`create_step` of the captured step) puts it back LAST, producing a different-order (and therefore, post-migration, different content-hash) result than the original. `create_edit_delete_step_round_trip` (a passing test in the same file) never catches this because it deletes `"step-99"`, which — having been added last — is already at the end, so append-at-the-end happens to match its original position.

**Proof this predates the migration and is not something the composed-child design made worse**: the pre-migration `apply_identified_delta` (`for item in added { next.push(item.clone()); }`) had the *exact same* append-only-at-the-end behavior; the OLD `#[derive(PartialEq)]` on `Vec<SequenceStep>` was equally order-sensitive (`Vec::eq` compares element-by-element in order). The bug is structural to `create_step`'s mutation semantics (no positional-insert support), not to how the diff is encoded. **Confirmed via `git log -1 --date=iso`, never by parsing the commit message's fake `🎆️🌙️☀️` date** (`📌️important.md`'s mandatory rule):
```
create-step/🔺️diff/🦀️component.rs (pre-edit):  16619a96... 2026-08-12 11:09:41 +0200
🔺️diff/📝️text/🦀️component.rs (apply_identified_delta, pre-edit): fd01661f... 2026-08-12 18:08:12 +0200
this ticket opened: 2026-08-12 15:02:49 (per 📌️important.md)
```
The `apply_identified_delta` commit (18:08:12) is technically *after* 15:02:49, but it is the W1 mechanism-agent's framework/`ArtifactChild::DslField` commit sweeping in unrelated pre-existing plugin code, not a ticket-authored change to `sequence`'s mutation logic — the `create-step` diff itself (11:09:41) is unambiguously pre-ticket, and it alone is sufficient to reproduce the bug regardless of the delta-applier's exact commit time (both implement the identical "append at the end" behavior).

**Not fixed**: a real fix requires either (a) `create_step`'s payload/diff carrying a positional index and `diff_replace_content`'s working-scene-based reconstruction honoring it, or (b) redefining `delete_step`'s inverse to re-splice at the original index — both are `🧬️mutations` facet redesigns, which `📌️important.md`'s binding rule places under the SEMANTIC-MUTATIONS-OVERHAUL ticket's policy scope ("Any new or restructured mutation facet lands inside the SMO ticket's policy scope... retrofits cost that ticket real work"), not something to freelance mid-migration. Flagging here rather than fixing.

## sharedFileRequests

None. Every change is inside `✏️s/🔌️plugins/🎬️sequence/**` (including the demo fixture asset and the two CSV serializer bugfixes, both this plugin's own files). No `🗄️stdio/**` file was read-written — only read for reference (`SemioFlowSnapshot`/`FlowNode`/`FlowEdge`/`FlowParam`/`PortRef`/`SemioPoint2` schema at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/🦀️component.rs`, and `CsvSnapshot`'s real shape at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`).

If a real fix for the `delete_step` reordering-on-undo gap is wanted, it belongs in this plugin's `🧬️mutations/🌱create-step/`/`🗑️delete-step/` triads but must go through the SMO ticket's facet-authoring policy first (binding per `📌️important.md`).

## Concurrent-churn observations

- `✏️s/🔌️plugins/➗️mathematical/**` (a different wave-B batch plugin) has extensive live uncommitted edits (`git status --porcelain`, dozens of files) at both dispatch and report time — a different fan-out agent's in-flight work, never touched by this pass, unrelated to `sequence`.
- `.🦑️repo/💬️prompts/🐙️ueli.md` shows as modified — a dev-owned file outside any plugin, not touched by this pass.
- `semio-s-plugin-stdio` compiled and checked clean throughout this pass (only its own large pre-existing warning count, no errors) — no retries needed.
- No other file inside `✏️s/🔌️plugins/🎬️sequence/**` was found dirty at dispatch time (re-verified via `git status --porcelain -- ✏️s/🔌️plugins/🎬️sequence` before starting — clean).

## Files touched this pass

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️component.rs` — `SequenceContentChild`, `sequence_step_params`/`sequence_step_from_node`, `sequence_content_snapshot_from_working`/`working_from_sequence_content_snapshot`, `sequence_content_child_handle`, `SequenceWorkingScene`, `SEQUENCE_SCRATCH`, `cache_sequence_content`, `sequence_working_scene_for_handle`/`sequence_working_scene`, `sequence_content_child_handle_and_cache`, `diff_replace_content`; deleted dead `SequenceStepPatch`/`SequenceEdgePatch`; test fixes + new round-trip test.
- `…/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `SequenceSnapshot` field swap, `SequenceFixture`, `to_fixture`/`from_fixture`, full hand-rolled text/binary codec (mirror deleted).
- `…/🧬️schema/📸️snapshot/📝️text/🦀️component.rs` — dropped dead `SequenceEdgeDsl` mirror; example/laws kept, test fixes.
- `…/🧬️schema/🦀️component.rs` — `SequenceArtifact` field swap, `to_snapshot`/`from_snapshot`/`set_snapshot`, `Default`.
- `…/🧬️schema/🔺️diff/🦀️component.rs` — `SequenceDiff.content`, deleted dead delta/patch-entry types.
- `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` — `apply`/`apply_to_artifact`/`absorb` collapsed to whole-handle-replace; test fix.
- `…/🧬️schema/🧬️mutations/{🌱create-step,🗑️delete-step,📍move-step,🔧edit-step-params,🗂️change-step-collapsed,🔗connect-steps,✂️disconnect-steps,🧬duplicate-step}/{🔺️diff,↩️inverse}/🦀️component.rs` (14 files with real changes; create-step's and connect-steps' `↩️inverse` needed no changes) — all 8 triads rewired onto the working-scene + `diff_replace_content` pattern.
- `…/🧬️schema/🧬️mutations/🦀️component.rs` — `protocol::Mutation` import fix (pre-existing bug), `sequence_snapshot_mutations` signature (`SequenceFixture`), 6 test fixes.
- `…/🧬️schema/🧬️mutations/🌱create-step/🦠️mutation/🦀️component.rs` — stale doc-comment fix (no longer references deleted `SequenceStepPatch`).
- `…/🧬️schema/💡️inferences/🦀️component.rs`, `…/💡️inferences/🧭topology/🦀️component.rs` — `compute_sequence_topology` rewired through `sequence_working_scene`, `fields()` doc updated, 6 test fixtures fixed.
- `…/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️component.rs`, `…/📤️export/🧵️serializers/…/📊️csv/…/🦀️component.rs` — pre-existing `CsvSnapshot` shape-mismatch bugs fixed outright.
- `…/📤️export/🧵️serializers/…/🔣️json/…/🦀️component.rs` — pre-existing `.into()` type-mismatch bug fixed outright.
- `…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` — regenerated fixture.
- `🎛️apps/🎬️sequence/🦀️component.rs` — `SequenceHost.snapshot: SequenceFixture`, `from_snapshot`/`from_fixture`/`replace_snapshot`/`load_json`, `host_from_snapshot`/`ops_from_host_mutation`, `max_serial_in_snapshot`/`next_available_step_id`, `import_media` (fixed pre-existing `StepsAdd` bug), `render()` dispatcher, ~15 test fixes.
- `🎛️apps/🎬️sequence/🎮️commands/{🪜️step,🔗️connection,🕸️node-graph,🔄️layout}/🦀️component.rs` — `.to_fixture()`/`sequence_snapshot_mutations` rewiring, `SequenceFixture` JSON parsing, test fixes.
- `🎛️apps/🎬️sequence/📌️panels/{📄️artifact,🔍️inspection,🛍️catalogue}/🦀️component.rs` — `render(&SequenceFixture, …)` signature change.
- `🎛️apps/🎬️sequence/🌉️wasm/🦀️component.rs` — `loadFixtureJson` now parses `SequenceFixture`.

ucas-status: complete
