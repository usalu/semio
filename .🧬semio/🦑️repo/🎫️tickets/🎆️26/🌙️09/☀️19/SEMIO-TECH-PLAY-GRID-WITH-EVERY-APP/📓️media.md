# 📓️ media — native test debt for 🎥️shooting · 🌿️vcs · 🎪️demonstrator · 🎬️sequence

Fleet v4, session 5 (2026-09-21). Command for every run (one cargo invocation, many `-p`):

```
CARGO_TARGET_DIR=<⚡️cache>/cargo/target-media CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 \
RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools \
cargo test -p <crates…> --lib --tests --no-fail-fast -- --test-threads=4
```

No crate of mine declares a cargo feature, so `--features component-app-assembly` never applies
(the string in `🎪️demonstrator`'s `Cargo.toml` is a comment, not a `[features]` row).

## 1. Per crate: first run → final run

| crate | first run (`run1.txt`, 15:17) | final | log |
|---|---|---|---|
| `semio-s-artifact-shooting-shooting` | 356 pass / **1 fail** | **357 / 0 — GREEN** | `run12.txt` |
| `semio-s-plugin-shooting` | 4 / 0 | 4 / 0 — GREEN | `run12.txt` |
| `semio-s-artifact-vcs-vcs` | 107 / **14 fail** | **121 / 0 — GREEN** | `run13.txt` |
| `semio-s-plugin-vcs` | 3+2+1 / 0 | 3+2+1 / 0 — GREEN | `run12.txt` |
| `semio-s-artifact-demonstrator-playground` | 39 / **1 fail** | **40 / 0 — GREEN** | `run12.txt` |
| `semio-s-plugin-demonstrator` | 8 / **2 fail** | **10 / 0 — GREEN** | `run12.txt` |
| `semio-s-artifact-sequence-sequence` | 163 / **44 fail** | see §4 | `run16.txt` |
| `semio-s-plugin-sequence` | 3 / 0 | 3 / 0 — GREEN | `run12.txt` |

Logs live in `🗑️generated/media/run*.txt`; the per-step diary is `🗑️generated/media/STATUS.md`.

## 2. Root causes, grouped

### A. Missing store-lane owners (production, my crates)
The `ArtifactEditor` trait defaults these hooks to `None` and `EditorApp<E>` passes them through
unchanged, so an app that never overrides them cannot read presence, cannot close, or both.

- **vcs** declared no presence retirement owners → the FIRST command whose ephemeral leg read local
  presence was refused with `presence local read requires a live exact local retirement owner`,
  which took down all 14 reds. `no_presence_*` only types over `NoPresence`, and `VcsDemoPresence`
  is a real (empty) presence payload, so the owner is the generic bounded one, as in shooting.
- **vcs** also declared no draft-lane owners → every close faulted
  `app owner did not provide the required bounded disposer for draft-store`.
- **demonstrator playground** declared the presence *disposer* but not the presence *local-root
  retirement factory* the close loop demands → `presence close requires its installed local-root
  retirement factory`.

### B. Stale pins / stale API scans (tests, my crates)
- **shooting** `utility_registry_scopes_…` scanned only `window_kinds[].actions`; the seven shell
  verbs (`loadRequest`, `saveDownload`, …) live on the app-level `definition.actions` roster now.
  Updated to the repo-wide idiom `definition.actions.iter().chain(window_kinds…)`.
- **demonstrator manifest** pinned `setContributions` args as exactly `["json"]`; generation3d's
  contributions push outgrew the single-invocation wire ceiling and streams pages
  (`json`/`page`/`pageCount`). The assertion now admits both rosters and rejects any third.
- **demonstrator manifest** pinned process3d editor/viewer at 0 derived genesis children; process3d
  composes five now (its own `LoadDocument` fix). Pins updated to 5/5.

### C. Mounted-app test harness debt (tests) — buckets 2, 3, 8, 12
Applied to **vcs** and **sequence**:
- the fixture app is a GUARD with `Drop` → `close_registered_fixture_app` (bare `VcsArtifactApp`
  bindings panicked `artifact store reached Drop without its exact terminal-empty shallow-shell
  witness` at the end of every render/panel test);
- `bind_instance_id` before any dispatch (`interactive-job.live-instance`);
- `settle_registered_typed_operation` after every `dispatch_typed` — a mounted app publishes AFTER
  it answers, so `result.mutations` is ALWAYS empty and the document edit is witnessed by the
  settled receipt's `Artifact` lane (three vcs assertions rewritten to `edited_document()`);
- `settle_framework_reserved_admission` after `handle_action` for the reserved verbs
  (`interactionSelect`, `undo`/`redo`, `commitCheckpoint`, `checkoutCheckpoint`, `createAlternative`)
  — the fork/undo/history-changed evidence is produced by the spawned job, never by the admission;
- a parsed `ArtifactEnvelope` is NOT droppable: the shell refuses Drop before its owners are
  detached AND its populated `ArtifactHistoryLedger` refuses Drop before every entry owner is
  retired. vcs's `seeded_envelope` now hands back a guard that walks
  `store::retire_document_envelope` to Complete (🔌️wires' `retire_envelope` is the precedent).
- sequence's convergence law was typed through `assert_two_registered_instances_converge`, which is
  hardwired to `VcsArtifactApp<A>` = `NoMembers`; sequence composes an `s.stdio.semio@v1/flow`
  child, so that instance died at construction. The law is spelled out in the crate over the real
  `SemioMembers` pair (see §5 for the framework-side generalisation proposal).

### D. Composed-child reads (tests, sequence)
After a settled edit the republished parent carries its composed handle WITHOUT a local owner, so
`app.snapshot().to_host_snapshot()` faults `sequence child scene must be materialized before fixture
projection: Absent`. Every live-app read now goes through a new `context::live_host_snapshot`, which
decodes the `content` CHILD store — the same surface the windows read and the same shape as flow's
`flow_child_node_count`. 11 tests.

### E. Window-addressed verbs (tests, sequence)
`setViewport`/`setOrientation` (main window config) and `run`/`stop` (script window transient) refuse
an unaddressed dispatch (`sequence-window-view-required` / `sequence-script-window-view-required`),
and `addressed(view, …)` resolves `view.window_id` against an instance OF ITS OWN KIND. The context
grew `main_window_meta`/`script_window_meta`, `dispatch` (main), `dispatch_in_script` and `render_in`.

### F. XCUT-DICT root cause in the imperative executor (production, shared module)
`final Dictionary ownership must be explicitly retired or owned by a cold boundary` was raised from
`semio_s_imperative::engine::Executor::run_step`: every scope rebind (`*scope = merge_output_into_scope(…)`,
`*scope = scope.clone().insert("index", …)`, and each `merged = merged.merge(…)`/`.insert(…)` inside
`merge_output_into_scope`) dropped the displaced dictionary, and a `Dictionary` that is the LAST owner
of a non-empty pair root panics on drop. Added `replace_scope_cold`/`replaced_cold` so every displaced
root leaves through the cold boundary. This is the XCUT-DICT bucket's root cause for my crates (that
bucket's original owner is gone); it is in `✏️s/🔨️modules/📜️imperative`, a shared module, not in any
peer no-touch path.

### G. Cold-owned fixtures (tests, sequence)
The `create-step`/`edit-step-params` fixture cases carry non-empty `StepParams`, so every decoded
`SequenceMutation`/`SequenceSnapshot` is the final owner of a dictionary. `mutation()`, `before()` and
`expected_after()` now hand out `neural_engine::ColdOwner<…>` (the sibling `🔬️mutation-law` tests
already used that idiom); the `move-step` case passed only because its params are empty.

## 3. Files changed (absolute)

Production:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs`

Tests:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📈️increment-counter/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩺️patch-snapshot/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️history/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎪️demonstrator/🪪️manifest/🎪️demonstrator/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️layout/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connection/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪜️step/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📽️main/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🌱️create-step/🧪️tests/🚫️rejects-a-duplicate-step-id/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/🪜️step/🧬️schema/🧬️mutations/🔧️edit-step-params/🧪️tests/🟰️no-ops-when-the-params-are-already-identical/🦀️.rs`

No test was deleted, `#[ignore]`d, weakened or special-cased; no fixture was hand-edited.

## 4. Left failing — `semio-s-artifact-sequence-sequence`

Sequence's GUEST code is peer-owned (slice S10), so the remaining reds are diagnosed and handed over
as proposed diffs rather than applied. Exact counts are in `🗑️generated/media/run16.txt`.

### S10-1 `SequenceHost::replace_snapshot` drops the displaced snapshot
`✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
(tests `replace_snapshot_preserves_next_serial_and_selection`, `repeated_drops_after_replace_snapshot_use_distinct_ids`)

```diff
         self.next_serial = self.next_serial.max(max_serial_in_snapshot(&snapshot));
-        self.snapshot = snapshot;
+        neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut self.snapshot, snapshot));
         self.rebuild_dag();
```

### S10-2 `SequenceHost::set_step_params_json` drops the displaced params
same file (test `set_step_params_json_updates_step_params`)

```diff
-        step.params = params;
+        neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut step.params, params));
         self.rebuild_dag();
```

### S10-3 `steps:in` has no reserved media importer
`import_media("steps:in", …)` is refused by the framework with
`interactive-job.missing-reserved-builder: media port 'steps:in' is registered but has no concrete
resumable importer` — `ArtifactEditor::import_media` is implemented, but `build_reserved_tool_job`
is not, so `build_artifact_reserved_media_job` (plugin SDK) never gets a job for
`ArtifactReservedToolInput::Media`. Precedent implementations: `🌀️procedural/🌀️generation2d`,
`🧩️puzzle/◻️2d|🧊️3d|🖐️5d` editors' `fn build_reserved_tool_job`.
(tests `import_media_steps_in_inserts_a_new_step_from_an_object_payload`,
`import_media_steps_in_wraps_a_bare_scalar_payload`)

### S10-4 a standalone `SequenceStore` cannot retire a composed snapshot
`new_sequence_store` installs the generic `bounded_document_store_owners::<SequenceSnapshot, …>()`,
whose `BoundedConfigValueRetirement<SequenceSnapshot>` plainly drops the snapshot; the composed
child's `Arc<SequenceWorkingScene>` is then the last owner of its steps' dictionaries and the close
loop aborts with the neural `final Dictionary ownership` panic (backtrace in `run12.txt`:
`ArtifactStoreCursorDisposer::close_step` → `BoundedConfigValueRetirement::close_step` → `StepParams`).
It needs an owner catalog whose snapshot retirement routes through `SequenceSnapshot::retire_cold`.
(test `store_applies_and_undoes_step_create`, `🧬️schema/⚙️operations/🦀️.rs`)

### S10-5 the document text/pack round trip requires a materialized child
`assert_document_text_round_trip` re-parses the envelope and then hits
`sequence_working_scene` (`🗿️artifacts/🎬️sequence/🦀️.rs:285`) with a wire-only child →
`sequence child scene must be materialized before use`. Either the DSL printer must tolerate an
unmaterialized child, or the round-trip helper must carry the child pack.
(test `sequence_document_text_round_trips_store_with_applied_mutation`)

### S10-6 `run`'s retained job faults
`run_stores_result_and_renders_in_script` / `stop_command_clears_last_run_result` reach
`job-session.terminal-fault` after an inner `final Dictionary ownership` panic inside the retained
`run` job. Fix F (imperative executor) removes one source; re-measure after S10-1/2 land, the
remainder is in `SequencePersistentAdvance::CompleteRun`.

## 5. Framework-side proposal (peer-owned `🔌️plugin/🦀️.rs` — diff only)

`artifact_app_laws::assert_two_registered_instances_converge` and `paired_registered_apps` are typed
`VcsArtifactApp<A>`, i.e. `NoMembers`, so NO app that composes a derived genesis child can use them
(`derived child dialect … is not declared by this app's member roster` at construction). They should
gain the same `M: SpaceMember + MemberFactory` parameter `new_app_with_registry_and_members`,
`settle_history_verb` and `assert_undo_redo_round_trip` already carry:

```diff
-pub async fn paired_registered_apps<A, Manifest, Build>(channel: &str, manifest: Build) -> (VcsArtifactApp<A>, VcsArtifactApp<A>)
+pub async fn paired_registered_apps<A, M, Manifest, Build>(channel: &str, manifest: Build) -> (VcsArtifactApp<A, M>, VcsArtifactApp<A, M>)
 where
     A: ArtifactApp + Default,
+    M: super::SpaceMember + super::MemberFactory + Send + 'static,
```
(and the same for `assert_two_registered_instances_converge`/`exchange_and_assert_convergence`,
with `new_registered_app` gaining the member roster). Until then every composing app duplicates the
convergence law in its own crate, as sequence now does.

## 6. Notes for the fleet

- A `cargo test` **compile** error aborts the whole invocation even with `--no-fail-fast`: run2/run3
  produced zero test results because one crate of five failed to compile. Batch, but expect that.
- The shared build dir deadlocked three times during this topic (N cargos idle >20 min with 0 `rustc`
  while newer cargos progressed). Each time, stopping MY OWN cargo released its ~40 unit locks and a
  peer that had been idle for 63 min immediately grew `rustc` children. A private `CARGO_TARGET_DIR`
  removes the artifact-directory queue but not the per-unit build-dir locks.
- Peers broke the tree twice mid-run (`XmlDeclaration.quote` in `semio-s-artifact-stdio-semio`,
  `spend_close_byte_grant` in `🏪️store`); both cleared within minutes on a re-run, as brief v4 says.
