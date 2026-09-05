# Opus lane D — one shared owned bounded tool-job factory for the fifteen norm apps

Ticket `26/09/05/S-END-TO-END`, dispatch-table row 2 of `📓️explore-action-migration-recipe.md`.
Scope: `✏️s/🔌️plugins/📕️norm` only. No framework crate was edited.

## 1. What the fifteen apps now share

All fifteen norm editors declare the byte-identical three-action shape
(`setSnapshot` / `evaluate` / `setSelectedCheckIndex`). Every one of those forty-five identities is now
`InteractiveJobClassification::Migrated` and dispatches through ONE generic factory.

New shared surface — `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` (region `🧵️RetainedCommands`,
`📬️StorePreparation`, `🔌️EditorOverrides`):

| item | `🖥️app-surface/🦀️.rs` line | what it is |
|---|---|---|
| `NORM_RETAINED_TOOL_IDS` | 317 | the three tool ids, in `app_commands!` row order — drives factory keys, `TOOL_IDS`, `PUBLICATION_CONTRACTS`, the proofs macro |
| `NORM_RETAINED_PAYLOAD_SCHEMA` = `norm.tool-command.v1` | 319 | payload schema id |
| `NORM_RETAINED_RAW_BYTES` = 8_192 | 322 | wire ceiling per dispatch |
| `NORM_ARTIFACT_STORE_MAXIMUM_BYTES` = 65_536 | 325 | artifact-lane one-item byte ceiling |
| `NORM_CONFIG_STORE_MAXIMUM_BYTES` = 4_096 | 327 | config-lane one-item byte ceiling |
| `NORM_PUBLICATION_CONTRACTS` | 332-336 | `setSnapshot → Artifact`, `evaluate → HostOnly`, `setSelectedCheckIndex → Config` |
| `norm_bounded_contract()` | 339 | `ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500)` |
| `trait NormRetainedEditor` | 347 | the ONLY per-app obligation: `dispatch_retained` → `command.dispatch(doc, cfg)` |
| `norm_retained_reduce<A>` | 359 | the shared reducer (see the LIFO note below) |
| `norm_bounded_extent<A>` | 378 | one bounded step per command |
| `NormBoundedCommandJobFactory<A>` | 385-443 | generic `ToolJobFactory` + `ArtifactOwnedToolJobFactory` |
| `norm_next_edit` / `NormOneItemPreparationFactory<P, M>` / `NormOneItemPreparation<P, M>` | 445, 499, 511-635 | ONE generic exact one-item Store preparation authority serving BOTH the artifact and the config lane (generation3d needed two hand-written copies; this collapses them via `protocol::Mutation::diff/inverse` + `MutationDiff::apply`) |
| `norm_artifact_store_preparation<A>` / `norm_config_store_preparation<A>` | 637 / 642 | the two `ArtifactEditor` store overrides |
| `norm_owned_tool_job_factory!` | 652-710 | declares one app's concrete factory newtype + its `register` entry point |
| `build_norm_tool_job<A>` | 712 | the `ArtifactEditor::build_tool_job` override |

Lane choice is read off the command bodies, not chosen by style:
`🎮️commands/📤️set-snapshot` commits `XMutation::from_snapshot(...)` (Artifact),
`🎮️commands/🧮️evaluate` returns `Emit::default()` — the report is derived on every read, so it emits
nothing at all (HostOnly, and HostOnly is its sole lane as the registry requires), and
`🎮️commands/☑️selected-check` emits `NormConfigMutation::ChangeSelectedCheckIndex` (Config). The three
command modules are untouched and remain the sole reducer authority.

### Why the factory needs a per-app newtype

`ArtifactBoundedFirstStepProof` joins the `factory:` literal against
`registration.factory_type_name.rsplit("::").next()`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12218`). For a generic instantiation
`std::any::type_name` ends in `…NormBoundedCommandJobFactory<…::Din4108PlayApp>`, whose last `::`
segment is `Din4108PlayApp>`. So the `norm_owned_tool_job_factory!` macro emits a thin per-app newtype
that delegates every method to the shared generic base. The macro body is written once; each app costs
one line.

### The LIFO drain compensation (real finding, not cosmetic)

`publish_mounted_typed_operation_unit` drains a completed emit with `emit.artifact_mutations.pop()`,
one `begin_apply_one` per maintenance turn
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22914`), i.e. **back-to-front**, whereas the
ordinary `dispatch_emit` path applies the same vector front-to-back inside a single edit
(`:21036` `ArtifactCommand::Apply { mutations, … }`). `XMutation::from_snapshot` emits ordered
`remove-layer` runs followed by `insert-layer` runs
(`…/🧬️schema/🧬️mutations/🦀️.rs:125-150`), so a mechanical migration of `setSnapshot` would have
published the layers in the wrong order. `norm_retained_reduce` therefore reverses
`artifact_mutations` on the way out, and the test
`set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document` drives the real
job path to prove the published document equals the payload. Note the undo granularity does change on
the retained lane (one store edit per mutation instead of one bundled edit) — that is the framework's
one-item publication contract, not something this lane introduced.

## 2. Per-app wiring (15 files, `🗿️artifacts/<app>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`)

Applied by `scratchpad/lane-d/apply_norm_factory.py` (kept for reference), identical in every file:

- inside `impl ArtifactEditor` (right after `const DOCUMENT_SCHEMA`, din4108 `:70` → `:72-97`):
  `build_artifact_store_one_item_preparation_factory`, `build_config_store_one_item_preparation_factory`,
  `register_tool_job_factories`, `build_tool_job`, and the
  `semio_framework_plugin::bounded_first_step_tool_proofs!` block with
  `factory_type: <Stem>BoundedCommandJobFactory` and the three tool ids.
- new `//#region 🧵️RetainedCommands` before `🧩️ComplianceFamily` (din4108 `:165-171`):
  `crate::norm_owned_tool_job_factory!(<Stem>BoundedCommandJobFactory, <Stem>PlayApp);` plus the
  three-line `NormRetainedEditor` impl.
- three `.action_interactive_job(…, InteractiveJobClassification::Migrated)` flips (din4108 `:213-215`),
  and the now-false "honest fail-closed dispositions" comment removed.

Two collateral repairs inside the same fifteen files:

- `testkit::new_app()` (registry-less `VcsArtifactApp::new`) is gone: with proofs declared it fails
  closed at construction with `interactive-job.catalog-authority`, because an empty `AppActionRegistry`
  has no migrated ids to join. All 180 call sites across 90 files now use `testkit::app_with_registry()`;
  the `new_app` helper and the `new_app as sdk_new_app` import were deleted (no alias left behind).
- three pre-existing `AppDefinition` field errors per file (`definition.actions`,
  `create_*_app().definition.modes`, `create_*_app().definition.io.ports`) — `AppDefinition`
  (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3419`) has neither an `actions` nor a `definition` field, so
  the editors' `#[cfg(test)]` modules did not compile before this lane. The disposition test now walks
  `window_kinds[*].actions`.

## 3. Tests

**Language-neutral fixture** (schemaVersion 2, migrated cohort):
`✏️s/🔌️plugins/📕️norm/🧪️fixtures/🧫️retained-command-dispositions/🔣️.json` + its
`🧬️.schema.json` — 15 apps × 3 routes = 45 identities, `retained: 45`,
`batchOnlyPendingRewrite: 0`, per-route `publicationLanes`, and a `factory` block pinning
`norm.tool-command.v1` / 8192 / `shared: true`.

**Rust oracle** (`🖥️app-surface/🦀️.rs`, `retained_disposition_oracle`): now cross-checks the fixture
against the LIVE `NORM_RETAINED_TOOL_IDS` / `NORM_PUBLICATION_CONTRACTS` / payload schema / raw byte
ceiling, and rejects four forgeries (downgraded admission, forged emitted lane, forged publication lane,
forged payload schema). Third-party `serde_json` stays behind the owned
`NormRetainedDispositionOracle` interface, test-only.

**Rust cohort walk** (`🖥️app-surface/🧪️tests/🦀️.rs`, `[[test]] surface_render`), new test
`every_norm_editor_action_is_migrated_onto_the_shared_owned_factory`: reads the fixture, checks it
against the live constants, then for all fifteen controllers asserts every window kind's three actions
are `Migrated` and that `plugin.create_app(controller)` succeeds — that call is the real gate, it runs
`register_tool_job_factories` + `validate_tool_job_rows` + the publication-lane availability check.
It also runs the framework conformance helpers `assert_editor_and_viewer_share_dialect::<E, V>()` and
`assert_viewer_never_mutates::<V>()` for all fifteen editor/viewer pairs.

**Rust functional job-path test** (din4108 editor):
`set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document` (`app_with_registry`
+ `bind_instance_id(1)` + `dispatch_typed` + repeated `maintenance_step`) and
`the_proof_catalog_covers_exactly_the_shared_retained_tool_ids` (three-way length/order equality).

**Independent TypeScript check** of the same fixture:
`✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript/📜️script.ts` `test` command
(`nx run @semio-tech/norm-js:test`), registered in `.vscode/launch.json` and
`.vscode/🧩️launch.seed.jsonc` as `⚖️gate📕️norm🟦️retained-cohort`. It re-derives the cohort invariants
from JSON alone and compares them against the committed `✏️s/🔌️plugins/📕️norm/🔣️.json` descriptor
(controller presence, per-window action presence, and `semantics.execution.interactiveJob` equality
wherever the descriptor carries one).

Run:

```
$ cd ✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript && bun ./📜️script.ts test
norm retained cohort ok: 15 editors × 3 migrated routes, 0 descriptor rows carried a classification
norm retained cohort: the committed 🔣️.json predates interactiveJob — re-run describe to make the drift check bite
```

## 4. Descriptor — deliberately NOT regenerated by this lane

The committed `✏️s/🔌️plugins/📕️norm/🔣️.json` / `🛂️.descriptor.semio` pair is stale and was NOT
hand-edited. It predates the `interactiveJob` field entirely: a scripted walk of its 30 manifest apps
found `setSnapshot`/`evaluate`/`setSelectedCheckIndex` 30x each under `windowKinds[*].actions` with
**zero** `semantics.execution.interactiveJob` keys, so the classification-drift clause of the TS check
is reported-but-vacuous until `describe` is re-run on a fresh `wasm32-wasip2` build.

Regeneration is deferred to the coordinator's Wave 2 catalog rebuild, which emits every plugin's core
and descriptor pair, norm included. This lane was explicitly told at 18:5x not to run
`cargo rustc -p semio-s-plugin-norm --target wasm32-wasip2` or `bun ./📜️script.ts describe`: the box was
carrying eight concurrent `rustc semio_s_plugin_stdio` wasm compilations from other sessions
(sourcing, procedural, lowpoly, space, stdio) at load average ~95. Verified that no wasm process
belonged to this lane before standing down (`ps -eo pid,ppid,args | grep -E "cargo rustc|wasm32-wasip2"`
listed only peer pids; lane D's only cargo process was the native `cargo test -p semio-s-plugin-norm`).

Once that rebuild lands, the TS oracle's `interactiveJob` comparison becomes non-vacuous with no code
change — it already asserts equality wherever the descriptor carries the field, and today prints
`0 descriptor rows carried a classification` plus an explicit warning.

## 5. Verification

All cargo work used the ticket's shared private target
(`RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e`) on a box carrying
~10 concurrent peer cargo runs (load average 100-290), so wall times below are contention, not code.

### 5.1 `cargo check -p semio-s-plugin-norm --lib` — GREEN

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e \
    cargo check -p semio-s-plugin-norm --lib --keep-going --message-format=short
…
warning: `semio-s-plugin-norm` (lib) generated 349 warnings (run `cargo fix --lib -p semio-s-plugin-norm` to apply 346 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 72m 41s
warning: the following packages contain code that will be rejected by a future version of Rust: semio-s-plugin-norm v0.1.0 (…/📕️norm/📦️packages/🦀️rust)
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
EXIT 0
```

Zero errors across the whole log (`grep -c 'error' … → 0`), and the run covered the two peer crates
this lane depends on: `semio-framework-os-kernel` and `semio-s-plugin-stdio` both compiled clean, so
the E2E plan's kernel `E0432` and "stdio non-compiling" blockers are no longer live for norm.
Full log: `scratchpad/norm-check.txt` (2512 lines, 349 of them norm warnings — all pre-existing
`unnecessary qualification` / `never used` lint noise, none from this lane's regions).

### 5.2 TypeScript cohort + example tests — GREEN

```
$ cd ✏️s/🔌️plugins/📕️norm/📦️packages/🟦️typescript && bun ./📜️script.ts test
norm retained cohort ok: 15 editors × 3 migrated routes, 0 descriptor rows carried a classification
norm retained cohort: the committed 🔣️.json predates interactiveJob — re-run describe to make the drift check bite
bun test v1.3.14 (0d9b296a)
 30 pass
 0 fail
 30 expect() calls
Ran 30 tests across 30 files. [1260.00ms]
```

### 5.3 `cargo test -p semio-s-plugin-norm --lib --test surface_render`

<!-- TESTRESULT -->


## 6. Remaining blockers / notes

<!-- BLOCKERS -->
