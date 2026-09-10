# Unit suite — generation2d lib tests (2026-09-09 / 2026-09-10)

Lane: unit-suite-2d. Continues `📓️unit-suite-2026-09-09.md` §3.3, which left generation2d at
**179 passed / 46 failed** of 225 (`--features component-app-assembly --lib`).

Scope owned by this lane: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/**`.
Explicitly NOT this lane: the 24 `interactive-job.catalog-authority … migrated={}` failures and the
§3.1 node-graph surface / catalogue admission failures — those belong to the catalogue-surface lane
(`AppActionRegistry::from_definition`) and the flow/UI lane.

Private target `$S/target-g2d`, `RUSTC_WRAPPER=""`, `RUST_MIN_STACK=536870912`,
`--test-threads=2` (§0 of the parent report — `--test-threads=1` aborts on a main-thread stack
overflow). `cargo test` does not accept `--keep-going`; the equivalent is `--no-fail-fast`.

Raw logs: `🗑️generated/g2d-*.txt`.

---

## 0. Invocation

```
CARGO_TARGET_DIR=$S/target-g2d RUSTC_WRAPPER="" RUST_MIN_STACK=536870912 \
  cargo test -p semio-s-artifact-procedural-generation2d \
  --features component-app-assembly --lib --no-fail-fast -- --test-threads=2
```

## 1. Session log (appended as the lane runs)

- Two earlier instances of this lane died mid-flight (rate limit; 600 s stall). Their edits were
  auto-committed at `6ad7b0e7bc` (2026-09-10 01:31). This file is written first and appended to.
- Inventory of what `6ad7b0e7bc` landed under generation2d is in §2.
- Nine classes closed (§3), suite 221/6 → **228/2** (§5.1), native + wasm checks clean (§5.2-5.3),
  the two survivors handed to their real owners with evidence (§4). No `[DEBUG]` and no `#[ignore]`
  remain anywhere under `🌀️generation2d/**`.

## 2. Inventory at resume

`6ad7b0e7bc` (2026-09-10 01:31) landed the two dead instances' work across ~80 generation2d files
(editor commands + their unit tests, schema/mutations/snapshot binary retained paths, the
per-mutation fixture JSON, the mounted-registry wasm tests).

Re-measured at resume with the §0 invocation:

```
running 227 tests
test result: FAILED. 221 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s
```

So the suite moved **179/46 (225) → 221/6 (227)** before this instance touched anything, and the
24-strong `catalog-authority` class collapsed to 2. The 6 survivors:

| # | test | message |
|---|---|---|
| 1 | `editor::generation2d::component::tests::add_widget_undo_redo_round_trip` | `ordered-map root must be explicitly retired before drop` |
| 2 | `editor::generation2d::component::tests::import_params_in_patches_matching_input_slider` | `interactive-job.missing-reserved-builder: media port 'params:in' is registered but has no concrete resumable importer` |
| 3 | `editor::generation2d::component::tests::two_instances_converge_disjoint_widget_moves` | `interactive-job.catalog-authority … migrated={}` on `nodeGraphViewport` |
| 4 | `editor::generation2d::component::tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | same as 3 |
| 5 | `…::mutations::binary::retained_authority_laws::every_fourteen_variant_decodes_through_retained_structural_grants` | `generation2d-mutation.body-malformed` ← `LimitExceeded("retained value depth")` |
| 6 | `…::snapshot::binary::retained_mounted_laws::non_empty_canonical_snapshot_round_trips_one_grant_at_a_time` | `generation2d-mounted.widget-statements-owner` |

Also carried over from the dead instances: three temporary `[DEBUG]` `eprintln!`s in
`🧬️schema/🧬️mutations/💾️binary/🦀️.rs` and its retained-authority-laws test, which this lane removes
once #5 is closed.

## 3. Failure classes and fixes

### 3.1 `MutationDiff` never retired what it owned — product defect (#1)

**Root cause.** `impl MutationDiff<Generation2dSnapshot> for Generation2dDiff`
(`…/🧬️schema/🔺️diff/📝️text/🦀️.rs:119`) implemented only `apply` and `absorb`, so it inherited BOTH
cold-retirement defaults — `MutationDiff::retire_cold` (a plain drop) and
`MutationDiff::retire_projection` (a plain drop). An inhabited `fixture` owns an
`OrderedMap<WidgetLayout>` root whose `Drop` aborts
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:81`), and the generic replay seams reach the
artifact only through the CONTRACT, never through `Generation2dDiff::retire_cold`'s inherent twin —
which existed and was correct all along:

```
2: <OrderedMap<WidgetLayout> as Drop>::drop
6: drop_glue::<Generation2dDiff>
7: <Generation2dDiff as MutationDiff<Generation2dSnapshot>>::retire_cold
8: os_vcs::apply_mutation::<Generation2dSnapshot, Generation2dMutation>
9: ArtifactStore::redo_lane_position
```

so **every undo/redo of a 2d document that has a layout entry aborted the process** — i.e. all of
them. Exactly the generation3d `FlowDiff` defect the 3d lane found, one artifact over.

**Fix.** Both hooks overridden, delegating to the inherent `Generation2dDiff::retire_cold` and to
`Generation2dSnapshot::retire_cold`, mirroring `🧊️generation3d/…/🔺️diff/📝️text/🦀️.rs:158-165`.

**New law** (`…/🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs`):
`the_mutation_diff_contract_retires_an_inhabited_layout_delta_and_its_scratch_projection` builds a
delta whose `fixture.layout` is inhabited, applies it, and retires both the delta and the scratch
projection **through the trait**, which is the path the store actually takes.

### 3.2 The retained wire decoder read the wrong node role — product defect (#5, #6)

**Root cause.** `encode_wire_node` (`🎒️pack/🌱️value/🦀️.rs:618`) writes a node as
`id`, then `kind` (presence bit 0), then `port` (presence bit 1), and both retained owners build
their role table accordingly — `base+0 = id`, `base+1 = kind`, `base+2 = port`, with `base` 0 for
the `from` node and 3 for the `to` node. Both readers then assigned **role 1 → `from_port`** and
**role 4 → `to_port`**, i.e. they read the node KIND slot. `SynapseSpec`'s ports travel in
`WireNode.port` (`…/📸️snapshot/📝️text/🦀️.rs:144-145`), and `kind` is always `None` there, so every
retained `connect-synapse`/`replace-synapse` and every retained snapshot silently dropped both port
names.

**Fix.** `2 => from_port`, `5 => to_port` in
`…/🧬️mutations/💾️binary/🦀️.rs` and `…/📸️snapshot/💾️binary/🦀️.rs`.

### 3.3 One nesting bound, not two — product defect (#5, #6)

**Root cause.** The retained ingress cursors were preflighted with
`PackLimits::max_depth = 12` (`GENERATION2D_RETAINED_COMBINED_DEPTH`, `GENERATION2D_MOUNTED_TYPED_DEPTH`)
while the same modules' own initializer copy guards admit nesting up to
`GENERATION2D_RETAINED_STACK_CAPACITY = 64`. A `Widget::Neuron`'s `params` is a neural `Dictionary`
whose entries can themselves be `Value::Dictionary`, and each such level costs several pack frames —
a two-level dictionary already spends more than a dozen. So the wire rejected, with
`LimitExceeded("retained value depth")` → `generation2d-mutation.body-malformed` /
`generation2d-mounted.value-malformed`, documents the artifact's own initializer copies happily.

**Fix.** ONE bound. `GENERATION2D_RETAINED_COMBINED_DEPTH` is deleted and every one of its four
readers now takes `GENERATION2D_RETAINED_STACK_CAPACITY`; `GENERATION2D_MOUNTED_TYPED_DEPTH` is
raised to the same 64 and documented as its canonical-route twin.

### 3.4 The mounted snapshot route could not read a nested dictionary at all — product defect (#6)

**Root cause.** A neural `Dictionary` reaches the wire in two shapes: a columnar `Table` when it is
a record field, and a `List` of one-entry records when it is a nested `Value::Dictionary`. The
MUTATION owner handles both (`Generation2dMutationFrame::{DictionaryEntries, DictionaryEntry}` plus
a `TableRow`/`EntryRow` neural-owner discriminant); the SNAPSHOT owner handled only the table, so
the entry list was parsed as a list of strings and the first nested record died on
`generation2d-mounted.widget-statements-owner`.

**Fix.** The snapshot owner gains the mutation owner's exact shape:
`Generation2dMountedNeuralOwner::{TableRow, EntryRow}`,
`Generation2dMountedContainerOwner::{DictionaryEntries, DictionaryEntry}`, a
`DictionaryEntryKey` string target, the `FieldId` arm for the entry frame, and the two `End`
write-backs. `current_root_field` reports 2 for both new frames.

### 3.5 `move-widget` had no root string slot — product defect (#5)

`finish_string`'s `(ordinal, field) → slot` table listed ordinals 2, 5, 7, 9, 11 (and 12/13) but not
**6** (`MoveWidget`), whose field 0 is the widget id `Complete` then reads back as `first`. Every
retained `move-widget` failed with `generation2d-mutation.root-string-field`. Fixed by admitting 6.

### 3.6 `VcsArtifactApp::new` is unusable for this app (#3, #4)

Both remaining editor laws built their fixture with `VcsArtifactApp::<EditorApp<Generation2dPlayApp>>::new`
/ `testkit::assert_two_instances_converge`, i.e. **registryless**. This app publishes
`bounded_first_step_tool_proofs!`, so `AppActionRegistry`'s empty `migrated` set rejects every
bounded proof at CONSTRUCTION with `interactive-job.catalog-authority` — the app cannot be built at
all. Both now use the registry-backed twins (`testkit::app_with_registry`,
`testkit::assert_two_registered_instances_converge`), exactly as `🧊️generation3d` already does, and
the projections they hold are closing reads (`production_read`) with `close(app)` at every exit.

### 3.7 No concrete resumable importer — product defect (#2)

**Root cause.** `VcsArtifactApp::import_media` routes through `dispatch_import_media` →
`build_artifact_reserved_media_job`, which asks the APP for the job. The framework registers the
reserved `import-media` FACTORY for every app (`register_framework_reserved_factories`) but never a
concrete job, and generation2d implemented only the one-shot `ArtifactEditor::import_media` seam —
so the whole `params:in` workflow port was dead at runtime with
`interactive-job.missing-reserved-builder`.

**Fix.** `Generation2dImportJob` (`…/✏️editor/🦀️.rs`, region `🎞️ReservedImport`) — a two-step
resumable job (decode → publish) with the full retained close ladder and terminal-empty witness,
modelled on `🧩️puzzle/◻️2d`'s `Puzzle2dImportJob`, plus `ArtifactEditor::build_reserved_tool_job`
routing `import-media` into it. Mutations are retired through
`generation2d_retire_mutation_cold`, never dropped.

**New laws** (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`):
`import_media_fails_closed_off_its_own_params_in_contract` (unknown port, non-object root, invalid
JSON, binary payload — all reject and leave the document untouched) and
`import_params_skips_unmatched_keys_and_non_numeric_values`.

### 3.8 The publication-lease table is process-global (#7, flake)

`GENERATION2D_PUBLICATION_SLOTS` is 4 and the lease registry is a process-global fixed table, so two
tests holding leases at once saturate it and the loser fails `generation2d-publication.saturated` —
an order- and thread-count-dependent failure. `🧪️tests/🔬️publication-authority/🦀️.rs` adds the same
serialising mutex `🧊️generation3d` uses, taken by
`authoritative_publication_rejects_stale_generation_aba_and_parent` and
`vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`.

### 3.9 `BatchOnlyPendingRewrite` migration

**Nothing to migrate.** All 21 declared actions — including the seven the brief names
(`nodeGraphEdit`, `moveMediaNode`, `addWidget`, `removeWidget`, `connectMediaPorts`, `reorganize`,
`setEvalOutputs`) — already carry `InteractiveJobClassification::Migrated`, both on the
`AppDefinition` (`…/✏️editor/🦀️.rs:1311-1332`) and on
`Generation2dBoundedCommandJobFactory::classification`, and each is declared in
`bounded_first_step_tool_proofs!` with a real retained `ToolExecutionContract::bounded_first_step`
and a real `ArtifactCommandWork` completion. `grep -rn BatchOnlyPendingRewrite` over the whole
generation2d tree returns nothing.

## 4. Remaining 2, both cross-lane

| test | message | owner |
|---|---|---|
| `component::tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `left: Fault, right: Ready` | **3d lane, class D** (`📓️unit-suite-3d-2026-09-09.md` §0) — the identical failure in the identical test one artifact over. |
| `component::tests::two_instances_converge_disjoint_widget_moves` | `module.vcs: remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized` | **framework vcs**, pre-existing and DECLARED fail-closed — exactly the state `📓️unit-suite-2026-09-09.md` §3.2 records for the 3d twin. |

Both were `catalog-authority` failures at the start of this lane (registryless construction), i.e.
part of the 24 this lane's brief excludes. This lane carried them past that: the apps now construct,
the fixture is registry-backed and closing, and the retained decoder reads the envelope correctly.

**Evidence for the first, so the 3d lane does not have to re-derive it.** With the registry-backed
app the envelope IS admitted and sealed (`seal_artifact_envelope_ingress` returns `true`), but the
decode job never leaves `Pending`: instrumenting `drive_production_envelope` with the public
`poll_artifact_envelope_decode` shows `decode=Pending poll=Fault` for all 300 000 maintenance turns.
Two separate things are in play:

1. `advance_artifact_envelope_load` (`🧰️framework/…/🔌️plugin/🦀️.rs:19268-19276`) has no branch for a
   decode job that is still `Pending`: when `active.poll() != Ready` it falls straight through to
   `poll_artifact_store_replacement(handle)`, which `map_or`s a missing replacement job to
   `ArtifactEnvelopeDecodeOperationPoll::Fault`. So an in-progress decode is reported as a terminal
   Fault to every caller of this API — eight plugins share the loop shape. Unchanged since
   `9d7cabfd9c` (2026-08-23); `0a0bb74380` only collapsed the `if`.
2. Even ignoring (1), the decode never progresses: the job stays `Pending` across 300 000 turns, so
   `ActiveArtifactEnvelopeDecode::drive`'s `session.pump_one(pool, Lane::Interactive)` is not
   advancing the worker.

Neither is generation2d's; both are left untouched here.

## 5. Runs

### 5.1 generation2d lib suite

| features | before (parent §3.3) | at resume | after |
|---|---|---|---|
| `--features component-app-assembly --lib` | 179 passed / 46 failed (225) | 221 / 6 (227) | **228 passed / 2 failed (230)** |
| `--lib` (default features) | not measured | not measured | **169 passed / 0 failed (169)** |

```
test result: FAILED. 228 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
failures:
    editor::generation2d::component::tests::two_instances_converge_disjoint_widget_moves
    editor::generation2d::component::tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed
```

```
test result: ok. 169 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

Reproduced four times, stable across `--test-threads=2` and `--test-threads=4`. (Between runs a peer
lane briefly left `semio-framework-ui-runtime` uncompilable — `cannot find function
`reclaim_orphaned_handback_slots` in this scope`, `♻️reconcile.rs:3454` — which blocks the whole
workspace, this crate included; the final measurement was taken once that cleared.) No `#[ignore]`, no loosened assertion: the
three tests this lane rewrote (`vcs_artifact_app_…`, `two_instances_converge_…`,
`non_empty_canonical_snapshot_…`) kept every assertion they had and gained the registry-backed
fixture, the closing reads and the unwind-safe lease their laws require.

### 5.2 `cargo check -p semio-s-plugin-procedural --keep-going` (native)

`rc=0`, **0 errors**, 7 warnings — none this lane's: `semio-framework-os-flow`'s
`unused import: SpaceMember`, two `unused extern crate`, three `never used` in the framework, and the
pre-existing `unnecessary qualification` at
`🌀️generation2d/…/🧬️schema/🧬️mutations/🦀️.rs:171`. Warnings are present, so this is a real
type-check and not an aborted expansion. Log: `🗑️generated/g2d-check-native.txt`.

### 5.3 `cargo check -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`

`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `rc=0`, **0 errors**, the same peer warnings, `Finished
wasm-dev profile in 1m 14s`. Warnings are present in both checks, so both are real type-checks.
Log: `🗑️generated/g2d-check-wasm.txt`.

Both checks ran with `RUSTC_WRAPPER=""` in a private `CARGO_TARGET_DIR`. Mid-lane the shared
`/private/tmp` volume hit `No space left on device` with 632 MiB free (the scratchpad holds ~290 GB
of eight lanes' target dirs); the native check dir was reclaimed and the wasm check reused this
lane's own warm `target-suite-wasm`.

### 5.4 Raw logs

`🗑️generated/g2d-suite-resume.txt` (221/6 at resume), `g2d-suite-after.txt` (228/2),
`g2d-suite-after-default-features.txt` (169/0), `g2d-check-native.txt`, `g2d-check-wasm.txt`.

## 6. Files changed

**Product (generation2d)**

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs` — `MutationDiff::{retire_cold, retire_projection}` overrides (§3.1).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` — one nesting bound (§3.3), wire port roles (§3.2), the `move-widget` root string slot (§3.5), and the removal of the predecessors' three `[DEBUG]` `eprintln!`s.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs` — the dictionary-entry-list route (§3.4), wire port roles (§3.2), the canonical-route depth bound (§3.3).
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `Generation2dImportJob` + `build_reserved_tool_job` (§3.7).
- `🦀️.rs` (crate root) — declares the new test-only `publication_authority` module.

**Tests (generation2d)**

- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — registry-backed vcs/convergence fixtures, `production_read`, the `ProductionLease` unwind guard, the serialising lock, and the two new importer laws.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🧪️tests/🔬️unit/🦀️.rs` — the new `MutationDiff` retirement law.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🧪️tests/🔬️retained-mounted-laws/🦀️.rs` — retires the two owned projections the law holds.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs` — `[DEBUG]` removed.
- `🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs` — takes the serialising lock.
- `🧪️tests/🔬️publication-authority/🦀️.rs` — NEW, the lease serialiser.

Nothing outside `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/**` was edited.
