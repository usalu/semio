# App laws green run, 2026-11-27 / 28

Target: `@semio-tech/s-composition-laws-rs:test-generation3d-app-laws` via `app-laws-retry.sh`.

Each row is one failure found in the compile chain, with its root cause and fix.

| # | Failure | Root cause | Fix |
|---|---------|-----------|-----|
| 1 | `SequenceMainWindowConfigMutation: OpBinaryReceiving` not satisfied | Sequence plugin was not migrated when `OpBinaryReceiving` was introduced. `OpBinary` there is a canonical JSON codec, so the authored codec is the only wire decoder. | `original_decoded_operation_receiving!` and `artifact_persisted_encoded_binary_operation!` for the config mutation in `…/📽️main/🎚️config/🦀️.rs`; `SequenceMutation` already had both. Round-trip test `sequence_main_window_config_original_receiving_round_trips_every_mutation` (admit, step, take, funded close) in `…/🎚️config/🧪️tests/🔬️window/🦀️.rs`, with the `semio-framework-value-testing-admission` dev-dependency in the editor crate manifest. |
| 2 | `semio_plugin::tool_contract` is private | Module made private; the public path is `semio_plugin::app`. | `🎹️app/🦀️.rs` and `🎞️import/🦀️.rs` now use `semio_plugin::app::{ArtifactReservedWork, admit_reserved_work}`. |
| 3 | `ValueRetirement::close_step` / `push_dictionary` / `RegistryRetirement::close_step` take a funded `RetainedAllocationEffect`, not a grant | The neural retirement API became effect-based. The Sequence persistent and retained owners still drove it with `RetainedCloneGrant`. | `release_one` is now `release_one(effect) -> SequenceRelease`; the helpers `sequence_poll_step`, `sequence_drain_retirement`, `sequence_push_pending`, `sequence_push_params` and `sequence_unit` are effect-based. Registry supervisor close folded into the Run state. Both jobs close via `close_original_retained_effect_step`. Files: `🔁️persistent/🦀️.rs`, `🧵️retained/🦀️.rs`. |
| 4 | Import job against the new construction, receiving and emission API | `admit_original_construction` is now `admit_original_construction_settled` with a resident `OriginalConstructionAdmission`; `with_native_receiving` takes an outcome slot; `JsonGrammarCursor::step` needs an effect; the admission and receiver `take_output` / `advance` / `close_step` are effect-based; `prepare_child_original` takes the `StepContext`. | `🎞️import/🦀️.rs` rewritten on `OriginalPackReceiving<Dictionary>::admit_value / advance / take / close_step` (plugin job-receiving toolkit), settled construction with its failure custody and retirement, effect-based `close_turn`, `close_original_effect_step`. New path dependency `semio-plugin-job-receiving` in the sequence crate manifest. |
| 5 | Stale composition test support (generation3d law targets) | Facade no longer glob-exports `widget_id` and `create_generation3d_app`; `ActionMeta.view_state` is `Option<Arc<ViewModel>>`; `PluginApp::pending_effects`, `settle_framework_reserved_admission`, `owe_attached_previews`, `with_host`, `with_host_session`, `FlowEvalSession::tick` and `close_step` gained a context, grant or `Result`. | Test-only fixes under `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d`. Shared imports (`widget_id_for as widget_id`, `manifest::create_generation3d_app`), `Arc::new` view state, `fixture_pending_effects`, `tick_cold`, `close_step_cold`, `with_host(...).expect(...)`, `with_host_session(..., TURN, ...)`, `preview_eval` alias, `owe_attached_previews(..., TURN)`. |

## Environment notes

- Several runs failed with ENOSPC or an unrelated host failure (corrupt rlib after a full disk; `repo cache cas gc` fixed it).
- Two runs were stopped by concurrent edits by other agents (a missing private Nx engine import, a mid-edit `l3d_blockers` probe); both cleared by waiting and rerunning.
- One run was SIGTERMed by the host.
