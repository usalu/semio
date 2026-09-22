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


---

# 📓️ media — session 7 (2026-09-22, fleet v5)

Successor of sessions 5 and 6. Crates (8): `semio-s-artifact-sequence-sequence`, `semio-s-plugin-sequence`,
`semio-s-artifact-shooting-shooting`, `semio-s-plugin-shooting`, `semio-s-artifact-vcs-vcs`, `semio-s-plugin-vcs`,
`semio-s-plugin-demonstrator`, `semio-s-artifact-demonstrator-playground`. Panes: `shooting` (design group),
`sequence`, `vcs`, `demonstrator` (media group).

> ⚠️ `🗑️generated/` was swept for the SECOND time in this ticket, at ~16:33 on 2026-09-22, while this session's
> `run27` was compiling. Every `🗑️generated/media/` artefact of session 7 (STATUS.md, run24–run27 logs, the probe
> outputs, the proposed-diff file, the private `target/` dir) was destroyed, and the running cargo died with its
> log. The numbers below therefore cite runs that were made and read, with their logs no longer on disk where
> that is so; the relaunched `run28.txt` is a cold rebuild. **Everything durable is in THIS tracked file.**

## 1. On-disk state verified before any work

`git diff HEAD --stat` over `✏️s/🔌️plugins/{🎬️sequence,🎥️shooting,🌿️vcs,🎪️demonstrator}` at 11:03 showed exactly two
uncommitted files, both session-6 sequence TEST files (the packed-`TextEditorScene` read in the `🏃️run` unit tests,
and the rewritten document-archive round trip in `🚪️io/🧬️mutations/💾️binary/🧪️tests/🔬️unit`). Everything earlier was
auto-committed; nothing was half-applied. By 15:56 those two were committed too, and the only uncommitted files in
my dirs were the four `🔣️.json` + `🛂️.descriptor.semio` pairs the coordinator's describe chain had just regenerated.

`semio-s-plugin-demonstrator` declares **no** `component-app-assembly` feature of its own — it turns that feature ON
for its generation3d / puzzle-3d / gis-map artifact dependencies. No `--features` flag belongs on its `cargo test`.

## 2. Runs

| run | scope | outcome |
|---|---|---|
| `run24` | 8 crates | **aborted at compile**, zero tests: a peer mid-edit in `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs` (`native_codec_factory_receipts_for` not yet defined, `validate_native_openable_projection` arity) broke `semio-s-plugin-stdio`, on which `semio-s-plugin-vcs` depends. A `cargo test` compile error aborts the WHOLE invocation even with `--no-fail-fast`. The peer had both fixed within 3 minutes. |
| `run25` | 8 crates | never started a cargo — the wrapper was still WAITING under the old one-slot mutex when the coordinator rewrote it for two slots; killed my own wrapper and requeued, per the coordinator's instruction. |
| **`run26`** | **8 crates** | **the authoritative measurement of this session** (finished 12:47, `--test-threads=4`, `CARGO_BUILD_JOBS=4`, private `CARGO_TARGET_DIR`; log swept at 16:33) |
| `run27` | 8 crates | killed by the 16:33 sweep mid-compile (log deleted under it) |
| `run28` | 8 crates | relaunched after the sweep, cold rebuild — see §7 |

### `run26` per crate

| crate | result |
|---|---|
| `semio-s-artifact-shooting-shooting` | **357 passed / 0 failed ✅** |
| `semio-s-plugin-shooting` | **4 / 0 ✅** |
| `semio-s-artifact-vcs-vcs` | **121 / 0 ✅** |
| `semio-s-plugin-vcs` | native-codecs **2 / 0 ✅**, native-openable-identity **1 / 0 ✅**, lib **2 / 1** (`descriptor_is_fresh`) |
| `semio-s-artifact-demonstrator-playground` | **40 / 0 ✅** |
| `semio-s-plugin-demonstrator` | **9 / 1** (`descriptor_is_fresh`) |
| `semio-s-artifact-sequence-sequence` | **198 / 9** (all nine peer-guest, §4) |
| `semio-s-plugin-sequence` | **2 / 1** (`descriptor_is_fresh`) |

`sequence_document_archive_round_trips_app_with_applied_mutation` — the composed round trip session 6 rewrote — is
**GREEN**. So are `run_executes_default_snapshot_and_records_scope` and `set_viewport_writes_config_not_operations`.

## 3. The three plugin-crate reds were ONE test, and it is not mine to fix

`descriptor_is_fresh` in demonstrator, sequence and vcs:
`🛂️.descriptor.semio is stale — re-run describe (📓️design-abi.md §3) and commit the refreshed 🛂️.descriptor.semio + 🔣️.json`.
The macro (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs`) builds the descriptor
natively and byte-compares it (hashes blanked) against the committed file. The committed file had drifted behind the
peers' manifest/`label()` churn. Refreshing it REQUIRES `describe`, a wasm32 target that brief v5 rule 2 forbids a fix
agent to run, so I requested it (`🗑️generated/describe.request/{sequence,vcs,demonstrator}`) and the coordinator's
12:11 chain ran it. **All four of my plugins' descriptors were regenerated and are on disk** (uncommitted
`🔣️.json` + `🛂️.descriptor.semio` pairs for vcs, shooting, demonstrator, sequence), so `run28` should clear all three.
`semio-s-plugin-shooting` never had this red (4/0 throughout), which is why this is per-plugin drift and not a
blanket staleness.

## 4. `semio-s-artifact-sequence-sequence` — nine reds, ALL peer-guest (slice S10), none test-side

Isolation proof, run directly against `run26`'s built test binary (`--test-threads=1 --exact`, no cargo, no mutex):

- **ORDER-DEPENDENT** — pass alone, fail inside the suite: `run_stores_result_and_renders_in_script`,
  `stop_command_clears_last_run_result`, `sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows`.
  Running just the two playback tests together is already enough (`--test-threads=1 playback` → 1 passed, 1 failed),
  so the leak is PROCESS-GLOBAL: an unretired run scope poisons the neural dictionary ledger for whatever runs next.
- **HARD** — fail alone: `replace_snapshot_preserves_next_serial_and_selection`,
  `repeated_drops_after_replace_snapshot_use_distinct_ids`, `set_step_params_json_updates_step_params`,
  `store_applies_and_undoes_step_create`, `import_media_steps_in_inserts_a_new_step_from_an_object_payload`,
  `import_media_steps_in_wraps_a_bare_scalar_payload`.

### For the `xcut-dict` owner
Five of the nine surface as `final Dictionary ownership must be explicitly retired or owned by a cold boundary` at
`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:101` — `repeated_drops_after_replace_snapshot_use_distinct_ids`,
`replace_snapshot_preserves_next_serial_and_selection`, `set_step_params_json_updates_step_params`,
`store_applies_and_undoes_step_create`, `sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows`.
**Every one of them has a sequence-LOCAL call site and needs no framework change** (S10-A/B/C/D below; S10-D in
particular is fixable with a plugin-local owner catalog, precedent `raster_document_store_owners`).
`xcut-dict` can skip sequence entirely.

### PROPOSED GUEST DIFFS (peer slice S10 owns `✏️s/🔌️plugins/🎬️sequence` guest code)

Shared rule: `neural_engine::Dictionary::drop` asserts unless the value was displaced through
`ColdRetire::retire_cold` or a `ValueRetirement` cursor. `ColdRetire` is already implemented for `StepParams`,
`SequenceStep`, `SequenceWorkingScene`, `SequenceHostSnapshot`, `SequenceSnapshot`, `SequenceMutation` and
`SequenceHost` — only the call sites are missing.

**S10-A — `SequenceHost::replace_snapshot` plain-drops the displaced snapshot.**
`…/🪆️subsets/✳️any/✏️editor/🦀️.rs:545`. Tests `replace_snapshot_preserves_next_serial_and_selection`,
`repeated_drops_after_replace_snapshot_use_distinct_ids`.
```diff
         self.next_serial = self.next_serial.max(max_serial_in_snapshot(&snapshot));
-        self.snapshot = snapshot;
+        neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut self.snapshot, snapshot));
         self.rebuild_dag();
```

**S10-B — `SequenceHost::set_step_params_json` plain-drops the displaced params.** Same file, line 661.
Test `set_step_params_json_updates_step_params`.
```diff
-        step.params = params;
+        neural_engine::ColdRetire::retire_cold(std::mem::replace(&mut step.params, params));
         self.rebuild_dag();
```
Two latent siblings in the same file (no test reaches them today because their scene is retired as a whole):
line 223 `step.params = value.params;` (`apply_mutation_to_scene`) and line 1256
`step.params = payload.params.clone();` (`EditStepParams` inverse builder). Fix in the same pass.

**S10-C — `SequenceRunState::advance` plain-drops the displaced scope.** Same file, lines 2352 / 2383 / 2426 / 2455.
Tests: the two playback tests and the exact-window law. `SequenceRunState` already owns a `ValueRetirement` cursor in
`self.retirement` and uses it correctly three lines above 2455 on the capacity-fault path; the four assignments to
`self.scope` bypass it.
```diff
-                self.scope = result.scope;
+                self.retirement.push_dictionary(std::mem::replace(&mut self.scope, result.scope));
```
```diff
-                self.scope = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
+                let next = self.scope.clone().insert("index", NeuralValue::Atom(neural_engine::Atom::Integer(index as i64)));
+                self.retirement.push_dictionary(std::mem::replace(&mut self.scope, next));
```
(same shape at 2426 with `Integer(0)`, and at 2352 with `Dictionary::new()`). Symptom while unfixed: the retained
`run` job ends in `job-session.terminal-fault` and the exact-window law reports
`interactive-job.app-owned-output: registered fixture typed operation fault: job-session.terminal-fault`.

**S10-D — `new_sequence_store` installs a plain-drop owner catalog.**
`…/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs:19`. Test `store_applies_and_undoes_step_create`.
`bounded_document_store_owners` → `bounded_config_store_owners` → `BoundedConfigValueRetirement::close_step`, whose
body is literally `drop(value)` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14928–14938`). For a snapshot/mutation
carrying a `Dictionary` that is a plain drop, so the store's close loop aborts.
**This needs no framework change** — a plugin-local catalog is the established pattern:
`raster_document_store_owners()` (`✏️s/🔌️plugins/🖨️raster/…/🧬️mutations/💾️binary/🦀️.rs:3442`) and
`gis_map_document_store_owners()`.
```diff
-    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<SequenceSnapshot, SequenceMutation>());
+    store.install_document_store_owners_exact(sequence_document_store_owners());
```
```rust
/// 🧊️ Sequence's own one-page retirement catalog: `bounded_document_store_owners` retires a displaced value with a
/// plain `drop`, and every Sequence snapshot/mutation carries `StepParams` (a neural `Dictionary`). Shape copied
/// from `raster_document_store_owners`.
pub fn sequence_document_store_owners() -> store::DocumentStoreOwners<SequenceSnapshot, SequenceMutation> {
    store::DocumentStoreOwners::new(
        std::sync::Arc::new(SequenceColdRetirementFactory::<SequenceSnapshot>::new()), // SnapshotRetirementFactory<P>
        std::sync::Arc::new(SequenceColdRetirementFactory::<SequenceSnapshot>::new()), // ArtifactOwnedValueRetirementFactory<P>
        std::sync::Arc::new(SequenceColdRetirementFactory::<SequenceMutation>::new()),
        Box::new(store::ArtifactStoreCursorDisposer::<SequenceSnapshot, SequenceMutation>::new()),
    )
}

struct SequenceColdRetirement<T: neural_engine::ColdRetire> {
    owner: std::mem::ManuallyDrop<Option<std::sync::Arc<T>>>,
    value: std::mem::ManuallyDrop<Option<T>>,
}

impl<T: neural_engine::ColdRetire + Send + Sync + 'static> store::ErasedSnapshotRetirement for SequenceColdRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 || maximum_bytes < store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(owner) = self.owner.take() {
            // 🔁️ Arc handback exactly as raster's `RasterSnapshotRootRetirement` does it: a still-shared snapshot is
            // `Blocked`, never dropped, so the composed child's scene owner is never left holding the last reference.
            return match std::sync::Arc::try_unwrap(owner) {
                Ok(value) => { *self.value = Some(value); Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES }) }
                Err(owner) => { *self.owner = Some(owner); Ok(store::SnapshotRetirementStep::Blocked) }
            };
        }
        if let Some(value) = self.value.take() {
            neural_engine::ColdRetire::retire_cold(value);
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.value.is_none() }
}

impl<T: neural_engine::ColdRetire> Drop for SequenceColdRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.owner.is_none() && self.value.is_none()), "Sequence cold retirement reached Drop before exact terminal emptiness");
    }
}
```
plus the two one-line `ArtifactOwnedValueRetirementFactory<T>` / `SnapshotRetirementFactory<T>` impls on
`SequenceColdRetirementFactory<T>`. (`DocumentStoreOwners::new`'s argument order is verified against
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2309`.)

**S10-E — `steps:in` has no concrete reserved media importer.** Tests
`import_media_steps_in_inserts_a_new_step_from_an_object_payload`, `import_media_steps_in_wraps_a_bare_scalar_payload`;
fault `interactive-job.missing-reserved-builder: media port 'steps:in' is registered but has no concrete resumable importer`.
`ArtifactEditor::import_media` is implemented for the sequence editor, but `fn build_reserved_tool_job` is NOT
(`grep -rn build_reserved_tool_job ✏️s/🔌️plugins/🎬️sequence` → no hits), so the plugin SDK's
`build_artifact_reserved_media_job` never receives a job for `ArtifactReservedToolInput::Media` and the framework fails
the port closed. Precedents to copy: `🌀️procedural/🌀️generation2d`, `🧩️puzzle/{◻️2d,🧊️3d,🖐️5d}`.

(S10-5 of the session-6 report — "the document text/pack round trip requires a materialized child" — is CLOSED: the
composed document-archive round trip replaced it and is green.)

### Framework-side proposal (peer-owned `🔌️plugin/🦀️.rs` — diff only, unchanged)
`artifact_app_laws::assert_two_registered_instances_converge` and `paired_registered_apps` are typed
`VcsArtifactApp<A>` (i.e. `NoMembers`), so no app composing a derived genesis child can use them. They should gain the
`M: SpaceMember + MemberFactory` parameter `new_app_with_registry_and_members` / `settle_history_verb` /
`assert_undo_redo_round_trip` already carry; until then every composing app duplicates the convergence law locally, as
sequence now does.

## 5. Live panes

`nx run @semio-tech/semio-tech-play:test-e2e` `dependsOn` `activate-dev`, which rule 2 forbids a fix agent to run, so I
restated the strict suite's three assertions (`🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts`) in a read-only probe that
runs against a live :6033 — curated example LABEL in `[data-play-pane=…] #playground.navbar.fixture`, the 12-bit paint
census of the largest ≥10 000 px² canvas (screenshot read-back, never `drawImage`) or the largest
`[data-slot="window-body"]`, and no media-extension path answered `text/html`.

Measured twice — on the 03:04 activation (11:45 / 13:00) and again on the recycled server at 15:58 — identical both times:

| pane | shell | example label | main window paints | page errors | console errors | refused inputs | SPA-fallback assets |
|---|---|---|---|---|---|---|---|
| `shooting` | ready | "Hexagonal Cut Concrete Forest Left" ✅ | canvas 967×809, 143 colours, 99 666/771 683 differ (needs 772) ✅ | 0 | 0 | 0 | 0 |
| `sequence` | ready | "Demo" ✅ | canvas 708×808, 25 colours, 49 200/563 004 (needs 563) ✅ | 0 | 0 | 0 | 0 |
| `vcs` | ready | "Demo" ✅ | DOM window body #1, 19 chars / 51 elements ✅ | 0 | 0 | 0 | 0 |
| `demonstrator` | ready | none declared — assertion skipped exactly as the suite skips it | canvas 1428×808, 24 colours, 4 622/1 140 444 (needs 1 140) ✅ | 0 | 0 | 0 | 0 |

By eye (screenshots, 11:45): `shooting` shows the concrete slab in 3D plus its 256×256 SVG icon projection;
`sequence` shows the Demo document with the Script window (`state.set(key="counter", value=0)`,
`log.print(message="hello sequence")`) and the DSL window listing both steps and the edge;
`vcs` shows `VCS Demo · Counter 2`, `demo notes`, Commit/Branch/Undo/Redo; `demonstrator` shows only the genesis title
`playground.playground`.

**`demonstrator` "blank" is a missing curated document, not a code defect.** `📓️audit-visual-2.md` (rows 138/176/224)
already ruled it expected-empty because the catalog declares no `example`. Going further: the artifact DOES declare one
(`…/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`) but its asset `🖼️assets/🎬️demo/🗣️.dsl.semio` is only
`semio playground.playground.dsl v1` + `schema=playground.playground` — byte-for-byte the genesis document. Wiring
`example: "demo"` into the catalog would change nothing on screen; a curated composed playground document (children
from cad / procedural / puzzle / sourcing / process / gis) has to be AUTHORED. Content authoring, not a defect in my
crates — for the coordinator / `default-example` topic.

**`vcs`'s empty History window is correct**: the curated `demo` document (`🌿️vcs/…/🖼️assets/🎬️demo/🗣️.dsl.semio`) is a
one-line snapshot with no commits or branches, so an empty edit history is its faithful projection.

**Not a media defect, for the graph owners:** in the sequence graph window the two step nodes draw as ~44×22 px boxes
with no visible label and no edge line, although the DSL window lists the edge. `flow`'s own pane renders the same way
(`audit-visual-2/screenshots/flow.png`: tiny `Number` / `Math.add` nodes, no edges), so this is a shared dag/graph
rendering trait at the default camera, not a sequence fault.

The six `entwerfen-mit-bestand-*` demonstrator variants are deliberately NOT play panes — `🧩️runtime/🔣️.json` sets
`excludedBrandPrefix: "entwerfen-mit-bestand-"`. `demonstrator` (the playground) is my only demonstrator-group pane.

## 6. FLEET FINDING — the 15:47 activation did not restage

The 12:11 chain's second `activate-dev` reported `rc=0` at 15:47 and :6033 was recycled onto it at 15:57, but the served
page prints **32 distinct `[stale] …: source-newer` lines**, including all four of mine:
`sequence` staged 12:22:43 < `🔣️.json` 13:49:37 · `vcs` staged 12:23:42 < 13:03:47 ·
`shooting` staged 12:12:01 < 13:04:46 · `demonstrator` staged 11:54:35 < 13:10:54.
The descriptors that same chain regenerated between 12:26 and 13:56 are NEWER than the modules :6033 serves — the second
`activate-dev` was a cache hit over the pre-describe staging. A fresh `activate-dev` AFTER the describe pass is required
before any pane verdict can honestly be called "on the new activation". My four panes are green on what is served today
either way.

Two further environment notes: the play supervisor had WEDGED (it ignored `serve-restart.request` from 14:25 to 15:57,
until the coordinator killed and relaunched it), and load average reached **255**, which is what timed out the
screenshot probe at 16:31 — the acceptance probe on the same server half an hour earlier reported `ready` for all four
panes in seconds.

## 7. What remains

1. **`descriptor_is_fresh` × 3** (demonstrator, sequence, vcs) — the refreshed descriptors are on disk; `run28`
   (relaunched 16:35 after the sweep, cold rebuild) is the confirmation run. Nothing to fix.
2. **The nine `semio-s-artifact-sequence-sequence` reds** — all peer-guest, diffs in §4. Not fixable from the test side
   and not `xcut-dict`'s.
3. **A curated `demonstrator` document** — content authoring (§5).
4. **A fresh `activate-dev` after the describe pass** — coordinator (§6).


---

## 8. FINAL RESULT — `run31.txt` (2026-09-22 18:18) — supersedes §2 and §7

Log: `.🧬semio/🦑️repo/⚡️cache/play-fleet/media/run31.txt` (the durable location the coordinator's 16:40 addendum
introduced after the second `🗑️generated` sweep). One invocation, all 8 crates, `CARGO_BUILD_JOBS=2`,
`CARGO_INCREMENTAL=0`, private `CARGO_TARGET_DIR=$G/target`, `--test-threads=4`, through the play native mutex.

**17 of 18 test targets green; one red target with two tests.**

| crate | target | result |
|---|---|---|
| `semio-s-artifact-shooting-shooting` | lib | **357 / 0 ✅** |
| `semio-s-artifact-vcs-vcs` | lib | **121 / 0 ✅** |
| `semio-s-artifact-demonstrator-playground` | lib | **40 / 0 ✅** |
| `semio-s-plugin-demonstrator` | lib | **10 / 0 ✅** |
| `semio-s-plugin-shooting` | lib | **4 / 0 ✅** |
| `semio-s-plugin-sequence` | lib | **3 / 0 ✅** |
| `semio-s-plugin-vcs` | lib · native-codecs · native-openable-identity | **3 / 0 · 2 / 0 · 1 / 0 ✅** |
| all 8 crates | doc-tests | 0 / 0 ✅ |
| `semio-s-artifact-sequence-sequence` | lib | **205 / 2** |

### The three `descriptor_is_fresh` reds are CLOSED
demonstrator, sequence and vcs all pass now. The coordinator's 12:11 describe chain regenerated the committed
`🛂️.descriptor.semio` + `🔣️.json` of all four of my plugins (on-disk mtimes 15:03 vcs, 15:04 shooting, 15:10
demonstrator, 15:49 sequence, local CEST). Nothing in my crates had to change: the test is a byte comparison
against a file only `describe` may write, and rule 2 forbids a fix agent to run it.

### Seven of the nine sequence reds are CLOSED
Everything that panicked `final Dictionary ownership must be explicitly retired or owned by a cold boundary` is
green — `replace_snapshot_preserves_next_serial_and_selection`,
`repeated_drops_after_replace_snapshot_use_distinct_ids`, `set_step_params_json_updates_step_params`,
`store_applies_and_undoes_step_create`,
`sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows` — and with them the two
order-dependent playback tests `run_stores_result_and_renders_in_script` and
`stop_command_clears_last_run_result`. The peer's S11 slice applied the retirement diffs (S10-A/B/C/D of §4)
to the sequence guest; the order dependence went with them, which confirms the diagnosis that the unretired run
scope was poisoning the process-global dictionary ledger for whatever test ran next.

### The two that remain are S10-E, and they are NOT Dictionary and NOT test-side
```
editor::sequence::component::unit_tests::import_media_steps_in_inserts_a_new_step_from_an_object_payload
editor::sequence::component::unit_tests::import_media_steps_in_wraps_a_bare_scalar_payload
  → Fault { origin: Framework, code: "interactive-job.missing-reserved-builder",
            message: "media port 'steps:in' is registered but has no concrete resumable importer" }
```
Re-verified against the tree at 21:40:

- The port is real and the law that declares it is green: `sequence_io_declares_steps_in_and_document_ports`
  passes, so `steps:in` IS a registered media port of `SequencePlayApp`.
- The synchronous importer is implemented in guest code:
  `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3447`
  `fn import_media(port, media, doc)` — it decodes the Structured payload, wraps a bare scalar as `{"value": …}`,
  mints `step-<n>`, sets `kind = "computation.import"` and emits the child replace. Exactly what the two tests assert.
- What is missing is the RESUMABLE builder. `ArtifactApp::import_media` on a mounted app routes through
  `build_artifact_reserved_media_job` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26843`), which builds an
  `ArtifactReservedToolJobRequest` with `tool_id = "import-media"` and
  `input = ArtifactReservedToolInput::Media { port, media }`, then calls `A::build_reserved_tool_job(request)`
  and faults `interactive-job.missing-reserved-builder` when it answers `None` — the trait default.
  `grep -rn build_reserved_tool_job ✏️s/🔌️plugins/🎬️sequence` returns **nothing**.

So the tests are right, the sync importer is right, and the guest is missing one trait method plus its job type.
**Nothing here is fixable from the test side**, and the file is peer-owned guest code, so it stays a proposed diff.

#### PROPOSED GUEST DIFF S10-E (unchanged in substance, now with exact anchors)
In `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`, next to
`fn import_media` (line 3447), add the reserved-tool builder and its job, copying
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` line for line:

- `const SEQUENCE_IMPORT_TOOL_ID: &str = "import-media";` (generation2d's own constant is that exact string, line 1107)
- `fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault>`
  (generation2d line 1448): answer `Ok(None)` for any other `tool_id`, refuse a non-empty `raw_wire`
  ("sequence import-media admits a decoded media value, never a wire payload"), destructure
  `ArtifactReservedToolInput::Media { port, media }`, and return
  `Ok(Some(ArtifactReservedToolJob::new(SequenceImportJob::new(request, port, media))))`.
- `struct SequenceImportJob` with `impl SequenceImportJob` + `impl InteractiveJob` + `impl ArtifactReservedJob`
  (generation2d lines 1116 / 1144 / 1188 / 1241). Its per-step work is one call into the existing
  `import_media` body, so the decode/insert logic is not duplicated — only the resumable envelope around it.

The `puzzle/{◻️2d,🧊️3d,🖐️5d}` editors carry the same shape if a second reference is wanted.

### Definition of done, as measured
- 7 of 8 crates fully green through a real run log (`run31.txt`).
- The 8th, `semio-s-artifact-sequence-sequence`, is 205/2, and both reds are proven peer-guest (S10-E), with the
  proposed diff above. No red of mine is xcut-dict's any more: every `final Dictionary ownership` red closed when
  the peer applied the sequence-local diffs, which is what §4 predicted.
- Panes: `shooting`, `sequence`, `vcs`, `demonstrator` all pass the three strict-acceptance assertions with
  0 page errors, 0 console errors, 0 refused inputs and 0 SPA-fallback assets (§5); the coordinator reports the
  full strict suite at 70/70 on the 15:47 activation.
- Still open and NOT mine: the S10-E guest diff (peer), a curated `demonstrator` document (content authoring,
  §5), and — if a pane verdict is ever to be called "on the newest bytes" — a fresh `activate-dev` after the
  describe pass (§6).
